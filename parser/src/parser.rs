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
            .then(items_parser())
            .map(|((name, imports), items)| ast::Module {
                name,
                imports,
                items,
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

fn items_parser() -> impl Parser<char, Vec<ast::Item>, Error = Simple<char>> {
    any().repeated().collect::<String>().map(|rest| {
        let trimmed = rest.trim();
        if trimmed.is_empty() {
            Vec::new()
        } else {
            vec![ast::Item::Other(trimmed.to_string())]
        }
    })
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

    let line_comment = just("//")
        .ignore_then(filter(|c: &char| *c != '\n').repeated().ignored())
        .then_ignore(just('\n').ignored().or(end()))
        .ignored();

    let block_comment = just("/*")
        .ignore_then(take_until(just("*/")).ignored())
        .then_ignore(just("*/"))
        .ignored();

    choice((spaces, line_comment, block_comment))
        .repeated()
        .ignored()
}
