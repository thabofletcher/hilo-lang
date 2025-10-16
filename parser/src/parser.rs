//! Top-level parser entry points.

#[allow(unused_imports)]
use chumsky::prelude::*;

use crate::{ast, error::HiloParseError};

pub fn parse_module(_source: &str) -> Result<ast::Module, HiloParseError> {
    Err(HiloParseError::NotImplemented)
}
