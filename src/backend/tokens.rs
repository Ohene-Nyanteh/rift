
#[derive(Debug)]
pub enum ArithmeticOp {
    Add,
    Sub,
    Div,
    Mul
}


#[derive(Debug)]
pub enum LogicalOp {
    And,
    Or,
    Not,
    Nor,
    Xor
}

#[derive(Debug)]
pub enum ComparisonOp {
    GreaterThan,
    LessThan,
    GreaterOrEquals,
    LessOrEquals,
    EqualTo,
    NotEqualTo
}


#[derive(Debug)]
pub enum Operations {
    Arithmetic(ArithmeticOp),
    Logical(LogicalOp),
    Comparison(ComparisonOp)
}


#[derive(Debug)]
pub enum Primary {
    Int { val: String },
    Bool,
    Float,
    Str { val: String},
    Char
}

#[derive(Debug)]
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
    // These will be added later
    // Enum,
    // Struct,
    // Class,
    // Public,
    // Private
}

#[derive(Debug)]
pub enum Secondary {
    Variable {val: String},
    Keyword(Keywords)
}

#[derive(Debug)]
pub enum NonAtomic {
    Colon,
    Commar,
    SemiColon,
    Paren(SymbolVal),
    CurlyBraces(SymbolVal),
    SquareBraces(SymbolVal),
    Assignment
}

#[derive(Debug)]
pub enum SymbolVal {
    Open,
    Close
}


#[derive(Debug)]
pub enum Tokens {
    Atomic(Operations),
    Primary(Primary),
    NonAtomic(NonAtomic),
    Secondary(Secondary)
}
