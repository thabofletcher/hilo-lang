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
    module_decl()
        .then(import_parser().repeated())
        .map(|(name, imports)| ast::Module {
            name,
            imports,
            items: Vec::new(),
        })
        .then_ignore(end())
}

fn module_decl() -> impl Parser<char, Option<ast::QualifiedName>, Error = Simple<char>> {
    text::keyword("module")
        .padded()
        .ignore_then(qualified_name().padded())
        .map(Some)
        .or_not()
        .map(|opt| opt.flatten())
}

fn import_parser() -> impl Parser<char, ast::Import, Error = Simple<char>> {
    text::keyword("import")
        .padded()
        .ignore_then(qualified_name().padded())
        .then(import_tail())
        .map(|(path, (alias, members))| ast::Import {
            path,
            members,
            alias,
        })
        .padded()
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

fn qualified_name() -> impl Parser<char, ast::QualifiedName, Error = Simple<char>> {
    identifier().separated_by(just('.')).at_least(1).collect()
}

fn identifier() -> impl Parser<char, String, Error = Simple<char>> {
    text::ident().map(|s: String| s)
}

fn alias_parser() -> impl Parser<char, String, Error = Simple<char>> {
    text::keyword("as")
        .padded()
        .ignore_then(identifier().padded())
}

fn member_list_parser() -> impl Parser<char, Vec<String>, Error = Simple<char>> {
    just('{')
        .padded()
        .ignore_then(
            identifier()
                .padded()
                .separated_by(just(',').padded())
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .then_ignore(just('}').padded())
}
