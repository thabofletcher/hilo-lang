//! Error types emitted by the HILO parser.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HiloParseError {
    #[error("parser not implemented yet")]
    NotImplemented,

    #[error("lexing error: {0}")]
    Lex(String),

    #[error("parse error: {0}")]
    Parse(String),
}
