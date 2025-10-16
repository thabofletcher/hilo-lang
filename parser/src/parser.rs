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
            body: ast::Block {
                raw: body_src.trim().to_string(),
            },
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
            body: ast::Block {
                raw: body_src.trim().to_string(),
            },
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
            body: ast::Block {
                raw: body_src.trim().to_string(),
            },
        }),
        idx,
    ))
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
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return ast::TypeExpr::Unknown(String::new());
    }
    if trimmed.ends_with('?') {
        let inner = trimmed[..trimmed.len() - 1].trim();
        return ast::TypeExpr::Optional(Box::new(parse_type_expr(inner)));
    }
    if trimmed.contains(' ')
        || trimmed.contains('[')
        || trimmed.contains('<')
        || trimmed.contains('{')
    {
        return ast::TypeExpr::Unknown(trimmed.to_string());
    }
    let parts: Vec<_> = trimmed
        .split('.')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if parts.is_empty() {
        ast::TypeExpr::Unknown(trimmed.to_string())
    } else {
        ast::TypeExpr::Simple(parts)
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
