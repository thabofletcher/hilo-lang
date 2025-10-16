//! Core Abstract Syntax Tree definitions for the HILO language.

pub type Ident = String;
pub type QualifiedName = Vec<Ident>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub name: Option<QualifiedName>,
    pub imports: Vec<Import>,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub path: QualifiedName,
    pub members: Option<Vec<Ident>>,
    pub alias: Option<Ident>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Unknown(String),
}
