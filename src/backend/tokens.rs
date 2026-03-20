#[derive(Debug, Clone, PartialEq)]
pub enum Operations {
    Add,
    Sub,
    Div,
    Mul,
    And,
    Or,
    Not,
    Nor,
    Xor,
    GreaterThan,
    LessThan,
    GreaterOrEquals,
    LessOrEquals,
    EqualTo,
    NotEqualTo
}


#[derive(Debug, Clone, PartialEq)]
pub enum Primary {
    Int { val: i64 },
    Bool {val: bool},
    Float {val: f64},
    Str { val: String},
    Char { val: char}
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
    Class
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
    Assignment
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Atomic(Operations),
    Primary(Primary),
    Variable { val: String },
    NonAtomic(NonAtomic),
    Keyword(Keywords),
    EOF
}


#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: Tokens,
    pub span: Span
}


#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub row: usize
}
