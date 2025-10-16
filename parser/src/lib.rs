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
    fn parses_module_and_imports() {
        let src = r#"
            module org.example.test
            import core.io
            import core.text { trim, join } as text
        "#;

        let module = parse_module(src).expect("parser should succeed");
        assert_eq!(
            module.name,
            Some(vec![
                String::from("org"),
                String::from("example"),
                String::from("test")
            ])
        );
        assert_eq!(module.imports.len(), 2);

        let import0 = &module.imports[0];
        assert_eq!(import0.path, vec![String::from("core"), String::from("io")]);
        assert!(import0.members.is_none());
        assert!(import0.alias.is_none());

        let import1 = &module.imports[1];
        assert_eq!(
            import1.path,
            vec![String::from("core"), String::from("text")]
        );
        assert_eq!(
            import1.members.as_ref().unwrap(),
            &vec![String::from("trim"), String::from("join")]
        );
        assert_eq!(import1.alias.as_deref(), Some("text"));
    }

    #[test]
    fn parses_import_alias_after_member_list() {
        let src = r#"
            import core.text { trim } as txt
        "#;

        let module = parse_module(src).expect("parser should succeed");
        assert_eq!(module.name, None);
        assert_eq!(module.imports.len(), 1);

        let import = &module.imports[0];
        assert_eq!(
            import.path,
            vec![String::from("core"), String::from("text")]
        );
        assert_eq!(
            import.members.as_ref().unwrap(),
            &vec![String::from("trim")]
        );
        assert_eq!(import.alias.as_deref(), Some("txt"));
    }
}
