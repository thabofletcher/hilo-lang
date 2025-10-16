//! Core Abstract Syntax Tree definitions for the HILO language.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub name: Option<String>,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Placeholder,
}
