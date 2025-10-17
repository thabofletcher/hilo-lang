//! Top-level parser entry points.

use chumsky::prelude::*;
use chumsky::{Parser, error::Simple};

use crate::{ast, error::HiloParseError};

pub fn parse_module(source: &str) -> Result<ast::Module, HiloParseError> {
    module_parser().parse(source).map_err(|errs| {
        let msg = errs
            .into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        HiloParseError::Parse(msg)
    })
}

fn module_parser() -> impl Parser<char, ast::Module, Error = Simple<char>> {
    ws().ignore_then(
        module_decl()
            .then(import_parser().repeated())
            .then(remainder())
            .map(|((name, imports), body)| {
                let items = parse_items_from_remainder(&body);
                ast::Module {
                    name,
                    imports,
                    items,
                }
            }),
    )
    .then_ignore(ws())
    .then_ignore(end())
}

fn module_decl() -> impl Parser<char, Option<ast::QualifiedName>, Error = Simple<char>> {
    text::keyword("module")
        .then_ignore(ws())
        .ignore_then(qualified_name())
        .then_ignore(ws())
        .map(Some)
        .or_not()
        .map(|opt| opt.flatten())
}

fn import_parser() -> impl Parser<char, ast::Import, Error = Simple<char>> {
    ws().ignore_then(text::keyword("import"))
        .then_ignore(ws())
        .ignore_then(qualified_name())
        .then_ignore(ws())
        .then(import_tail())
        .map(|(path, (alias, members))| ast::Import {
            path,
            members,
            alias,
        })
}

fn import_tail() -> impl Parser<char, (Option<String>, Option<Vec<String>>), Error = Simple<char>> {
    let alias_then_members = alias_parser()
        .map(Some)
        .then(member_list_parser().or_not())
        .map(|(alias, members)| (alias, members));

    let members_then_alias = member_list_parser()
        .map(Some)
        .then(alias_parser().or_not())
        .map(|(members, alias)| (alias, members));

    alias_then_members
        .or(members_then_alias)
        .or_not()
        .map(|opt| opt.unwrap_or((None, None)))
}

fn remainder() -> impl Parser<char, String, Error = Simple<char>> {
    any().repeated().collect::<String>()
}

fn qualified_name() -> impl Parser<char, ast::QualifiedName, Error = Simple<char>> {
    identifier()
        .then_ignore(ws())
        .separated_by(just('.').then_ignore(ws()))
        .at_least(1)
        .collect()
}

fn identifier() -> impl Parser<char, String, Error = Simple<char>> {
    text::ident().map(|s: String| s)
}

fn alias_parser() -> impl Parser<char, String, Error = Simple<char>> {
    ws().ignore_then(text::keyword("as"))
        .then_ignore(ws())
        .ignore_then(identifier())
        .then_ignore(ws())
}

fn member_list_parser() -> impl Parser<char, Vec<String>, Error = Simple<char>> {
    ws().ignore_then(just('{'))
        .then_ignore(ws())
        .ignore_then(
            identifier()
                .then_ignore(ws())
                .separated_by(just(',').then_ignore(ws()))
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .then_ignore(ws())
        .then_ignore(just('}'))
        .then_ignore(ws())
}

fn ws() -> impl Parser<char, (), Error = Simple<char>> {
    let spaces = filter(|c: &char| c.is_whitespace())
        .repeated()
        .at_least(1)
        .ignored();

    let doc_comment = just("///")
        .ignore_then(filter(|c: &char| *c != '\n').repeated().ignored())
        .then_ignore(just('\n').ignored().or(end()))
        .ignored();

    let line_comment = just("//")
        .ignore_then(filter(|c: &char| *c != '\n').repeated().ignored())
        .then_ignore(just('\n').ignored().or(end()))
        .ignored();

    let block_comment = just("/*")
        .ignore_then(take_until(just("*/")).ignored())
        .then_ignore(just("*/"))
        .ignored();

    choice((spaces, doc_comment, line_comment, block_comment))
        .repeated()
        .ignored()
}

fn parse_items_from_remainder(src: &str) -> Vec<ast::Item> {
    let mut items = Vec::new();
    let mut offset = skip_ws(src, 0);
    while offset < src.len() {
        if let Some((item, next)) = parse_record_decl(src, offset) {
            items.push(item);
            offset = skip_ws(src, next);
            continue;
        }
        if let Some((item, next)) = parse_task_decl(src, offset) {
            items.push(item);
            offset = skip_ws(src, next);
            continue;
        }
        if let Some((item, next)) = parse_workflow_decl(src, offset) {
            items.push(item);
            offset = skip_ws(src, next);
            continue;
        }
        if let Some((item, next)) = parse_test_decl(src, offset) {
            items.push(item);
            offset = skip_ws(src, next);
            continue;
        }

        let remainder = src[offset..].trim();
        if remainder.is_empty() {
            break;
        }
        items.push(ast::Item::Other(remainder.to_string()));
        break;
    }
    items
}

fn parse_record_decl(src: &str, start: usize) -> Option<(ast::Item, usize)> {
    let mut idx = skip_doc_comments(src, start);
    if !starts_with_keyword(src, idx, "record") {
        return None;
    }
    idx += "record".len();
    idx = skip_ws(src, idx);
    let (name, mut idx) = take_ident(src, idx)?;
    idx = skip_ws(src, idx);

    let mut type_params = Vec::new();
    if src[idx..].starts_with('<') {
        let (params_src, consumed) = extract_balanced(src, idx, '<', '>')?;
        idx = consumed;
        type_params = params_src
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        idx = skip_ws(src, idx);
    }

    if !src[idx..].starts_with('{') {
        return None;
    }
    let (fields_src, consumed) = extract_balanced(src, idx, '{', '}')?;
    idx = consumed;
    let fields = parse_record_fields(&fields_src);
    idx = skip_ws(src, idx);

    Some((
        ast::Item::Record(ast::RecordDecl {
            name,
            type_params,
            fields,
        }),
        idx,
    ))
}

fn parse_task_decl(src: &str, start: usize) -> Option<(ast::Item, usize)> {
    let mut idx = skip_doc_comments(src, start);
    if !starts_with_keyword(src, idx, "task") {
        return None;
    }
    idx += "task".len();
    idx = skip_ws(src, idx);
    let (name, mut idx) = take_ident(src, idx)?;
    idx = skip_ws(src, idx);

    if !src[idx..].starts_with('(') {
        return None;
    }
    let (params_src, consumed) = extract_balanced(src, idx, '(', ')')?;
    idx = consumed;
    let params = parse_params(&params_src);
    idx = skip_ws(src, idx);

    let mut return_type = None;
    if idx < src.len() && src[idx..].starts_with("->") {
        idx += 2;
        idx = skip_ws(src, idx);
        let type_start = idx;
        while idx < src.len() && !src[idx..].starts_with('{') {
            if let Some(ch) = peek_char(src, idx) {
                idx += ch.len_utf8();
            } else {
                break;
            }
        }
        let ty_str = src[type_start..idx].trim();
        if !ty_str.is_empty() {
            return_type = Some(parse_type_expr(ty_str));
        }
    }
    idx = skip_ws(src, idx);

    if !src[idx..].starts_with('{') {
        return None;
    }
    let (body_src, consumed) = extract_balanced(src, idx, '{', '}')?;
    idx = consumed;
    idx = skip_ws(src, idx);

    Some((
        ast::Item::Task(ast::TaskDecl {
            name,
            params,
            return_type,
            body: build_block(&body_src),
        }),
        idx,
    ))
}

fn parse_workflow_decl(src: &str, start: usize) -> Option<(ast::Item, usize)> {
    let mut idx = skip_doc_comments(src, start);
    if !starts_with_keyword(src, idx, "workflow") {
        return None;
    }
    idx += "workflow".len();
    idx = skip_ws(src, idx);
    let (name, mut idx) = take_ident(src, idx)?;
    idx = skip_ws(src, idx);
    if !src[idx..].starts_with('{') {
        return None;
    }
    let (body_src, consumed) = extract_balanced(src, idx, '{', '}')?;
    idx = consumed;
    idx = skip_ws(src, idx);
    Some((
        ast::Item::Workflow(ast::WorkflowDecl {
            name,
            body: build_block(&body_src),
        }),
        idx,
    ))
}

fn parse_test_decl(src: &str, start: usize) -> Option<(ast::Item, usize)> {
    let mut idx = skip_doc_comments(src, start);
    if !starts_with_keyword(src, idx, "test") {
        return None;
    }
    idx += "test".len();
    idx = skip_ws(src, idx);
    let (name, idx_after_name) = if src[idx..].starts_with('"') {
        take_string_literal(src, idx)?
    } else {
        take_ident(src, idx)?
    };
    let mut idx = skip_ws(src, idx_after_name);
    if !src[idx..].starts_with('{') {
        return None;
    }
    let (body_src, consumed) = extract_balanced(src, idx, '{', '}')?;
    idx = consumed;
    idx = skip_ws(src, idx);
    Some((
        ast::Item::Test(ast::TestDecl {
            name,
            body: build_block(&body_src),
        }),
        idx,
    ))
}

fn build_block(body_src: &str) -> ast::Block {
    let raw = body_src.trim().to_string();
    let statements = raw
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter(|line| *line != "{" && *line != "}" && *line != "}" && *line != "{")
        .map(parse_statement)
        .collect();
    ast::Block { raw, statements }
}

fn parse_statement(line: &str) -> ast::Statement {
    if let Some(rest) = line.strip_prefix("let ") {
        return parse_let_statement(rest.trim());
    }
    if let Some(rest) = line.strip_prefix("return") {
        let value = rest.trim();
        return ast::Statement::Return {
            value: if value.is_empty() {
                None
            } else {
                Some(parse_expression(value))
            },
        };
    }
    ast::Statement::Expr(parse_expression(line))
}

fn parse_let_statement(rest: &str) -> ast::Statement {
    let mut name_part = rest;
    let mut value_part = None;
    if let Some((lhs, rhs)) = rest.split_once('=') {
        name_part = lhs.trim();
        value_part = Some(rhs.trim().to_string());
    }

    let (name, ty) = if let Some((name, ty_str)) = name_part.split_once(':') {
        (
            name.trim().to_string(),
            Some(parse_type_expr(ty_str.trim())),
        )
    } else {
        (name_part.trim().to_string(), None)
    };

    ast::Statement::Let {
        name,
        ty,
        value: value_part.map(|v| parse_expression(&v)),
    }
}

fn parse_expression(src: &str) -> ast::Expression {
    let trimmed = src.trim();
    if trimmed.is_empty() {
        return ast::Expression::Raw(String::new());
    }
    if let Some((target, args)) = parse_call_expression(trimmed) {
        return ast::Expression::Call {
            target: Box::new(parse_expression(target)),
            args: args.into_iter().map(parse_expression).collect(),
        };
    }
    if let Some((left, op, right)) = parse_binary_expression(trimmed) {
        return ast::Expression::Binary {
            left: Box::new(parse_expression(left)),
            op: op.to_string(),
            right: Box::new(parse_expression(right)),
        };
    }
    if let Some((target, property)) = parse_member_expression(trimmed) {
        return ast::Expression::Member {
            target: Box::new(parse_expression(target)),
            property: property.to_string(),
        };
    }
    if is_identifier(trimmed) {
        return ast::Expression::Identifier(trimmed.to_string());
    }
    if is_literal(trimmed) {
        return ast::Expression::Literal(trimmed.to_string());
    }
    ast::Expression::Raw(trimmed.to_string())
}

fn parse_call_expression(src: &str) -> Option<(&str, Vec<&str>)> {
    let open_paren = src.find('(')?;
    let close_paren = src.rfind(')')?;
    if close_paren < open_paren {
        return None;
    }
    let target = src[..open_paren].trim();
    if target.is_empty() {
        return None;
    }
    let args_str = &src[open_paren + 1..close_paren];
    let args = split_args(args_str);
    Some((target, args))
}

fn split_args(src: &str) -> Vec<&str> {
    let mut args = Vec::new();
    let mut depth = 0;
    let mut start = 0;
    let chars: Vec<char> = src.chars().collect();
    for (idx, ch) in chars.iter().enumerate() {
        match ch {
            '(' | '{' | '[' => depth += 1,
            ')' | '}' | ']' => {
                if depth > 0 {
                    depth -= 1
                }
            }
            ',' if depth == 0 => {
                args.push(src[start..idx].trim());
                start = idx + 1;
            }
            _ => {}
        }
    }
    let tail = src[start..].trim();
    if !tail.is_empty() {
        args.push(tail);
    }
    args
}

fn parse_member_expression(src: &str) -> Option<(&str, &str)> {
    let mut depth = 0;
    let chars: Vec<char> = src.chars().collect();
    for (idx, ch) in chars.iter().enumerate().rev() {
        match ch {
            ')' | ']' | '}' => depth += 1,
            '(' | '[' | '{' => depth -= 1,
            '.' if depth == 0 => {
                let target = src[..idx].trim();
                let property = src[idx + 1..].trim();
                if !target.is_empty() && is_identifier(property) {
                    return Some((target, property));
                }
            }
            _ => {}
        }
    }
    None
}

fn parse_binary_expression(src: &str) -> Option<(&str, &str, &str)> {
    let ops = [
        "==", "!=", "<=", ">=", "&&", "||", "+", "-", "*", "/", "%", "<", ">",
    ];
    let mut depth = 0;
    let chars: Vec<char> = src.chars().collect();
    for idx in (0..chars.len()).rev() {
        let ch = chars[idx];
        match ch {
            ')' | ']' | '}' => depth += 1,
            '(' | '[' | '{' => depth -= 1,
            _ if depth == 0 => {
                for op in ops.iter() {
                    if idx + 1 >= op.len() {
                        let candidate = &src[idx + 1 - op.len()..=idx];
                        if candidate == *op {
                            let left = src[..idx + 1 - op.len()].trim();
                            let right = src[idx + 1..].trim();
                            if !left.is_empty() && !right.is_empty() {
                                return Some((left, *op, right));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    None
}

fn is_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(ch) if ch == '_' || ch.is_alphabetic() => {
            chars.all(|c| c == '_' || c.is_alphanumeric())
        }
        _ => false,
    }
}

fn is_literal(s: &str) -> bool {
    s.starts_with('"') && s.ends_with('"')
        || s.parse::<f64>().is_ok()
        || matches!(s, "true" | "false")
}

fn parse_record_fields(body: &str) -> Vec<ast::RecordField> {
    body.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty()
                || trimmed.starts_with("//")
                || trimmed.starts_with("/*")
                || trimmed.starts_with("}")
            {
                return None;
            }
            let (name_part, rest) = trimmed.split_once(':')?;
            let mut name = name_part.trim().to_string();
            let optional = name.ends_with('?');
            if optional {
                name.pop();
            }
            name = name.trim_end_matches('?').trim().to_string();
            let ty_str = rest
                .split_once('=')
                .map(|(ty, _)| ty)
                .unwrap_or(rest)
                .trim()
                .trim_end_matches(',')
                .trim();
            Some(ast::RecordField {
                name,
                optional,
                ty: parse_type_expr(ty_str),
            })
        })
        .collect()
}

fn parse_params(src: &str) -> Vec<ast::Param> {
    src.split(',')
        .filter_map(|part| {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                return None;
            }
            let (name_part, rest) = trimmed.split_once(':')?;
            let name = name_part.trim().to_string();
            let rest = rest.trim();
            let (ty_part, default) = if let Some((ty, default)) = rest.split_once('=') {
                (ty.trim(), Some(default.trim().to_string()))
            } else {
                (rest, None)
            };
            Some(ast::Param {
                name,
                ty: parse_type_expr(ty_part),
                default,
            })
        })
        .collect()
}

fn parse_type_expr(raw: &str) -> ast::TypeExpr {
    TypeParser::new(raw).parse()
}

struct TypeParser<'a> {
    src: &'a str,
    idx: usize,
}

impl<'a> TypeParser<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            src: src.trim(),
            idx: 0,
        }
    }

    fn parse(mut self) -> ast::TypeExpr {
        if self.src.is_empty() {
            return ast::TypeExpr::Unknown(String::new());
        }
        match self.parse_type_with_optional() {
            Some(ty) => {
                self.skip_ws();
                if self.idx < self.src.len() {
                    ast::TypeExpr::Unknown(self.src.trim().to_string())
                } else {
                    ty
                }
            }
            None => ast::TypeExpr::Unknown(self.src.trim().to_string()),
        }
    }

    fn parse_type_with_optional(&mut self) -> Option<ast::TypeExpr> {
        let mut ty = self.parse_type_inner()?;
        self.skip_ws();
        if self.peek_char() == Some('?') {
            self.idx += 1;
            ty = ast::TypeExpr::Optional(Box::new(ty));
        }
        Some(ty)
    }

    fn parse_type_inner(&mut self) -> Option<ast::TypeExpr> {
        self.skip_ws();
        if self.idx >= self.src.len() {
            return None;
        }

        if self.peek_char() == Some('{') {
            self.idx += 1;
            let fields = self.parse_struct_fields();
            return Some(ast::TypeExpr::Struct(fields));
        }

        let base = self.parse_qualified_identifier();
        if base.is_empty() {
            return None;
        }

        self.skip_ws();
        if self.consume('<') {
            let args = self.parse_type_arguments('>');
            return Some(ast::TypeExpr::Generic {
                base,
                arguments: args,
            });
        }

        self.skip_ws();
        if self.consume('[') {
            self.skip_ws();
            if base.len() == 1 && base[0] == "List" {
                let elem_ty = if self.peek_char() == Some(']') {
                    self.idx += 1;
                    ast::TypeExpr::Simple(base)
                } else {
                    let ty = self
                        .parse_type_with_optional()
                        .unwrap_or(ast::TypeExpr::Unknown(String::new()));
                    self.skip_ws();
                    let _ = self.consume(']');
                    ty
                };
                return Some(ast::TypeExpr::List(Box::new(elem_ty)));
            } else {
                let args = self.parse_type_arguments(']');
                return Some(ast::TypeExpr::Generic {
                    base,
                    arguments: args,
                });
            }
        }

        Some(ast::TypeExpr::Simple(base))
    }

    fn parse_struct_fields(&mut self) -> Vec<ast::StructFieldType> {
        let mut fields = Vec::new();
        loop {
            self.skip_ws();
            if self.peek_char() == Some('}') {
                self.idx += 1;
                break;
            }

            let mut name = self.parse_identifier();
            if name.is_empty() {
                break;
            }
            let mut optional = false;
            if name.ends_with('?') {
                name = name.trim_end_matches('?').to_string();
                optional = true;
            }

            self.skip_ws();
            if !self.consume(':') {
                break;
            }

            let ty = self
                .parse_type_with_optional()
                .unwrap_or(ast::TypeExpr::Unknown(String::new()));
            fields.push(ast::StructFieldType { name, optional, ty });

            self.skip_ws();
            if !self.consume(',') {
                self.skip_ws();
                if self.peek_char() == Some('}') {
                    self.idx += 1;
                }
                break;
            }
        }
        fields
    }

    fn parse_type_arguments(&mut self, closing: char) -> Vec<ast::TypeExpr> {
        let mut args = Vec::new();
        loop {
            self.skip_ws();
            if self.peek_char() == Some(closing) {
                self.idx += closing.len_utf8();
                break;
            }
            let arg = self
                .parse_type_with_optional()
                .unwrap_or(ast::TypeExpr::Unknown(String::new()));
            args.push(arg);
            self.skip_ws();
            if self.consume(closing) {
                break;
            }
            let _ = self.consume(',');
        }
        args
    }

    fn parse_qualified_identifier(&mut self) -> Vec<String> {
        let mut parts = Vec::new();
        loop {
            let ident = self.parse_identifier();
            if ident.is_empty() {
                break;
            }
            parts.push(ident);
            self.skip_ws();
            if !self.consume('.') {
                break;
            }
        }
        parts
    }

    fn parse_identifier(&mut self) -> String {
        self.skip_ws();
        let start = self.idx;
        while self.idx < self.src.len() {
            if let Some(ch) = self.peek_char() {
                if ch == '_' || ch.is_alphanumeric() || ch == '?' {
                    self.idx += ch.len_utf8();
                    continue;
                }
            }
            break;
        }
        self.src[start..self.idx].trim().to_string()
    }

    fn skip_ws(&mut self) {
        while self.idx < self.src.len() {
            if let Some(ch) = self.peek_char() {
                if ch.is_whitespace() {
                    self.idx += ch.len_utf8();
                    continue;
                }
            }
            break;
        }
    }

    fn consume(&mut self, ch: char) -> bool {
        self.skip_ws();
        if self.peek_char() == Some(ch) {
            self.idx += ch.len_utf8();
            true
        } else {
            false
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.src[self.idx..].chars().next()
    }
}

fn starts_with_keyword(src: &str, idx: usize, keyword: &str) -> bool {
    if idx >= src.len() || !src[idx..].starts_with(keyword) {
        return false;
    }
    let next = idx + keyword.len();
    !is_ident_continue(peek_char(src, next))
}

fn skip_doc_comments(src: &str, mut idx: usize) -> usize {
    loop {
        idx = skip_ws_spaces(src, idx);
        if idx < src.len() && src[idx..].starts_with("///") {
            idx = skip_line_comment(src, idx + 3);
            continue;
        }
        break;
    }
    idx
}

fn skip_ws(src: &str, mut idx: usize) -> usize {
    loop {
        let mut advanced = false;
        let new_idx = skip_ws_spaces(src, idx);
        if new_idx != idx {
            idx = new_idx;
            advanced = true;
        }
        if idx < src.len() && src[idx..].starts_with("///") {
            idx = skip_line_comment(src, idx + 3);
            advanced = true;
        } else if idx < src.len() && src[idx..].starts_with("//") {
            idx = skip_line_comment(src, idx + 2);
            advanced = true;
        } else if idx < src.len() && src[idx..].starts_with("/*") {
            idx = skip_block_comment(src, idx + 2);
            advanced = true;
        }
        if !advanced {
            break;
        }
    }
    idx
}

fn skip_ws_spaces(src: &str, mut idx: usize) -> usize {
    while idx < src.len() {
        let ch = match peek_char(src, idx) {
            Some(ch) => ch,
            None => break,
        };
        if ch.is_whitespace() {
            idx += ch.len_utf8();
        } else {
            break;
        }
    }
    idx
}

fn skip_line_comment(src: &str, mut idx: usize) -> usize {
    while idx < src.len() {
        let ch = match peek_char(src, idx) {
            Some(ch) => ch,
            None => break,
        };
        idx += ch.len_utf8();
        if ch == '\n' {
            break;
        }
    }
    idx
}

fn skip_block_comment(src: &str, mut idx: usize) -> usize {
    while idx + 1 < src.len() {
        if src[idx..].starts_with("*/") {
            idx += 2;
            break;
        }
        if let Some(ch) = peek_char(src, idx) {
            idx += ch.len_utf8();
        } else {
            break;
        }
    }
    idx
}

fn take_ident(src: &str, start: usize) -> Option<(String, usize)> {
    if start >= src.len() {
        return None;
    }
    let mut chars = src[start..].char_indices();
    let (first_offset, first_char) = chars.next()?;
    if first_offset != 0 || !is_ident_start(first_char) {
        return None;
    }
    let mut end = start + first_char.len_utf8();
    for (offset, ch) in chars {
        if is_ident_continue(Some(ch)) {
            end = start + offset + ch.len_utf8();
        } else {
            break;
        }
    }
    Some((src[start..end].to_string(), end))
}

fn take_string_literal(src: &str, start: usize) -> Option<(String, usize)> {
    if start >= src.len() {
        return None;
    }
    let mut chars = src[start..].char_indices();
    let (first_offset, first_char) = chars.next()?;
    if first_offset != 0 || first_char != '"' {
        return None;
    }
    let mut result = String::new();
    let mut idx = start + 1;
    let mut escape = false;
    while idx < src.len() {
        let ch = peek_char(src, idx)?;
        idx += ch.len_utf8();
        if escape {
            result.push(ch);
            escape = false;
            continue;
        }
        match ch {
            '\\' => escape = true,
            '"' => return Some((result, idx)),
            _ => result.push(ch),
        }
    }
    None
}

fn extract_balanced(src: &str, start: usize, open: char, close: char) -> Option<(String, usize)> {
    if start >= src.len() || peek_char(src, start)? != open {
        return None;
    }
    let mut depth = 1;
    let mut idx = start + open.len_utf8();
    let content_start = idx;
    let mut in_string = false;
    let mut escape = false;
    while idx < src.len() {
        let ch = peek_char(src, idx)?;
        idx += ch.len_utf8();
        if in_string {
            if escape {
                escape = false;
                continue;
            }
            match ch {
                '\\' => escape = true,
                '"' => in_string = false,
                _ => {}
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            _ if ch == open => depth += 1,
            _ if ch == close => {
                depth -= 1;
                if depth == 0 {
                    let content = &src[content_start..idx - close.len_utf8()];
                    return Some((content.to_string(), idx));
                }
            }
            _ => {}
        }
    }
    None
}

fn peek_char(src: &str, idx: usize) -> Option<char> {
    src.get(idx..)?.chars().next()
}

fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_alphabetic()
}

fn is_ident_continue(ch: Option<char>) -> bool {
    match ch {
        Some(c) => c == '_' || c.is_alphanumeric(),
        None => false,
    }
}
