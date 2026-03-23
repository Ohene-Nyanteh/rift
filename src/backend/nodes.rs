use super::tokens::{Operations, Primary};

#[derive(Debug)]
pub enum Statement {
    Function(Box<FunctionDecl>),
    Let(Box<LetDecl>),
    While(Box<WhileDecl>),
    Expression(Expression),
    If {
        condition: Expression,
        body: Block,
        else_body: Option<Block>,
    }
}

#[derive(Debug)]
pub struct FunctionDecl {
    pub name: Identifier,
    pub args: Vec<Identifier>,
    pub body: Block,
}

#[derive(Debug)]
pub struct LetDecl {
    pub name: Option<Identifier>,
    pub value: Option<Expression>,
}

#[derive(Debug)]
pub struct WhileDecl {
    pub condition: Expression,
    pub body: Block
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Expression {
    Unary { op: Operations, expr: Box<Expression> },
    Binary {
        left: Box<Expression>,
        op: Operations,
        right: Box<Expression>,
    },
    Literal(Primary),
    Variable(Identifier),
    Call {
        callee: Identifier,
        args: Vec<Expression>,
    }
}

#[derive(Debug)]
pub struct Identifier(pub String);
