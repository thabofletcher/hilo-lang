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

    #[test]
    fn parses_sample_project_main() {
        let src = include_str!("../../project/src/main.hilo");
        let module = parse_module(src).expect("parser should succeed on sample project");

        assert_eq!(
            module.name,
            Some(vec![
                String::from("org"),
                String::from("example"),
                String::from("hilo"),
                String::from("project")
            ])
        );

        assert_eq!(module.imports.len(), 5);
        let text_import = &module.imports[1];
        assert_eq!(
            text_import.path,
            vec![String::from("core"), String::from("text")]
        );
        assert_eq!(
            text_import.members.as_ref().unwrap(),
            &vec![String::from("trim"), String::from("join")]
        );
        assert_eq!(text_import.alias.as_deref(), Some("T"));

        assert_eq!(module.items.len(), 3);

        match &module.items[0] {
            ast::Item::Record(record) => {
                assert_eq!(record.name, "Brief");
                assert_eq!(record.fields.len(), 3);
                assert_eq!(record.fields[0].name, "title");
                match &record.fields[0].ty {
                    ast::TypeExpr::Simple(path) => {
                        assert_eq!(path, &vec![String::from("String")]);
                    }
                    other => panic!("expected simple string type, got {:?}", other),
                }
                match &record.fields[2].ty {
                    ast::TypeExpr::List(inner) => match inner.as_ref() {
                        ast::TypeExpr::Simple(path) => {
                            assert_eq!(path, &vec![String::from("String")]);
                        }
                        other => panic!("expected list of string type, got {:?}", other),
                    },
                    other => panic!("expected list type, got {:?}", other),
                }
            }
            other => panic!("expected record, got {:?}", other),
        }

        match &module.items[1] {
            ast::Item::Task(task) => {
                assert_eq!(task.name, "ProduceBrief");
                assert_eq!(task.params.len(), 1);
                assert_eq!(task.params[0].name, "topic");
                assert!(task.body.raw.contains("Writer.run"));
                match task.body.statements.get(0) {
                    Some(ast::Statement::Let { name, .. }) => {
                        assert_eq!(name, "research");
                    }
                    other => panic!("expected let statement, got {:?}", other),
                }
                assert!(
                    task.body
                        .statements
                        .iter()
                        .any(|stmt| matches!(stmt, ast::Statement::Return { .. })),
                    "expected a return statement in task body"
                );
            }
            other => panic!("expected task, got {:?}", other),
        }

        match &module.items[2] {
            ast::Item::Workflow(flow) => {
                assert_eq!(flow.name, "Main");
                assert!(flow.body.raw.contains("start"));
                assert!(!flow.body.statements.is_empty());
            }
            other => panic!("expected workflow, got {:?}", other),
        }
    }

    #[test]
    fn parses_complex_type_shapes() {
        let src = r#"
            record Complex<T> {
              items?: List[Map[String, Int]?]
              props: { key: String, value?: Int }
            }
        "#;

        let module = parse_module(src).expect("parser should succeed");
        assert_eq!(module.items.len(), 1);

        let record = match &module.items[0] {
            ast::Item::Record(record) => record,
            other => panic!("expected record, got {:?}", other),
        };

        assert_eq!(record.name, "Complex");
        assert_eq!(record.type_params, vec![String::from("T")]);
        assert_eq!(record.fields.len(), 2);

        let items_field = &record.fields[0];
        assert_eq!(items_field.name, "items");
        assert!(items_field.optional);
        match &items_field.ty {
            ast::TypeExpr::List(inner) => match inner.as_ref() {
                ast::TypeExpr::Optional(inner) => match inner.as_ref() {
                    ast::TypeExpr::Generic { base, arguments } => {
                        assert_eq!(base, &vec![String::from("Map")]);
                        assert_eq!(arguments.len(), 2);
                    }
                    other => panic!("expected generic map, got {:?}", other),
                },
                other => panic!("expected optional inner, got {:?}", other),
            },
            other => panic!("expected list type, got {:?}", other),
        }

        let props_field = &record.fields[1];
        match &props_field.ty {
            ast::TypeExpr::Struct(fields) => {
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].name, "key");
                assert!(!fields[0].optional);
                assert_eq!(fields[1].name, "value");
                assert!(fields[1].optional);
            }
            other => panic!("expected struct type, got {:?}", other),
        }
    }
}
