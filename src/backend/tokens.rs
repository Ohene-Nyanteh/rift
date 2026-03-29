#[derive(Debug, Clone, PartialEq)]
pub enum Operations {
    Add,
    Sub,
    Div,
    Mul,
    And,
    Or,
    Not,
    Xor,
    Nor,
    GreaterThan,
    LessThan,
    GreaterOrEquals,
    LessOrEquals,
    EqualTo,
    NotEqualTo,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Primary {
    Int(i64),
    Bool(bool),
    Float(f64),
    Str(String),
    Char(char), // to do
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keywords {
    Fn,
    If,
    Else,
    Elif,
    Let,
    While,
    For,
    Break,
    Continue,
    Match,
    Enum,
    Struct,
    Return,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NonAtomic {
    Colon,
    Comma,
    SemiColon,
    LParen,
    RParen,
    LCurlyBraces,
    RCurlyBraces,
    LSquareBraces,
    RSquareBraces,
    Assignment,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Atomic(Operations),
    Primary(Primary),
    Variable(String),
    NonAtomic(NonAtomic),
    Keyword(Keywords),
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: Tokens,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub row: usize,
}
