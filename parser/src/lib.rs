pub mod ast;
pub mod error;
mod parser;

pub use error::HiloParseError;

/// Parse a HILO source file into an abstract syntax tree.
pub fn parse_module(source: &str) -> Result<ast::Module, HiloParseError> {
    parser::parse_module(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_returns_todo_error_for_now() {
        let err = parse_module("").expect_err("parser not implemented yet");
        matches!(err, HiloParseError::NotImplemented);
    }
}
