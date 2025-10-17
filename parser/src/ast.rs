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
    Record(RecordDecl),
    Task(TaskDecl),
    Workflow(WorkflowDecl),
    Test(TestDecl),
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordDecl {
    pub name: Ident,
    pub type_params: Vec<Ident>,
    pub fields: Vec<RecordField>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordField {
    pub name: Ident,
    pub optional: bool,
    pub ty: TypeExpr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDecl {
    pub name: Ident,
    pub params: Vec<Param>,
    pub return_type: Option<TypeExpr>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowDecl {
    pub name: Ident,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestDecl {
    pub name: String,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: Ident,
    pub ty: TypeExpr,
    pub default: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub raw: String,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Let {
        name: Ident,
        ty: Option<TypeExpr>,
        value: Option<Expression>,
    },
    Return {
        value: Option<Expression>,
    },
    Expr(Expression),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier(Ident),
    Literal(String),
    Call {
        target: Box<Expression>,
        args: Vec<Expression>,
    },
    Member {
        target: Box<Expression>,
        property: Ident,
    },
    Binary {
        left: Box<Expression>,
        op: String,
        right: Box<Expression>,
    },
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeExpr {
    Simple(QualifiedName),
    Generic {
        base: QualifiedName,
        arguments: Vec<TypeExpr>,
    },
    List(Box<TypeExpr>),
    Struct(Vec<StructFieldType>),
    Optional(Box<TypeExpr>),
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructFieldType {
    pub name: Ident,
    pub optional: bool,
    pub ty: TypeExpr,
}
