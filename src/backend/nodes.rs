use super::tokens::{Operations, Primary};

#[derive(Debug, Clone)]
pub enum Statement {
    Function(Box<FunctionDecl>),
    Let(Box<LetDecl>),
    Enum(Box<EnumDecl>),
    Struct(Box<StructDecl>),
    If {
        condition: Box<Expression>,
        body: Block,
        elif_branches: Vec<(Box<Expression>, Block)>,
        else_body: Option<Block>,
    },
    While {
        condition: Box<Expression>,
        body: Block,
    },
    For {
        var: Identifier,
        iterable: Box<Expression>,
        body: Block,
    },
    Match {
        value: Box<Expression>,
        arms: Vec<(Box<Expression>, Block)>,
    },
    Return(Option<Box<Expression>>),
    Break,
    Continue,
    Expression(Box<Expression>),
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: Identifier,
    pub args: Vec<Identifier>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct LetDecl {
    pub name: Identifier,
    pub value: Option<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub name: Identifier,
    pub variants: Vec<Identifier>,
}

#[derive(Debug, Clone)]
pub struct StructDecl {
    pub name: Identifier,
    pub fields: Vec<Identifier>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Unary {
        op: Operations,
        expr: Box<Expression>,
    },
    Binary {
        op: Operations,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Literal(Primary),
    Variable(Identifier),
    Call {
        callee: Identifier,
        args: Vec<Expression>,
    },
}

#[derive(Debug, Clone)]
pub struct Identifier(pub String);
