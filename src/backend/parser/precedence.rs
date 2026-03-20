// Precedence levels for Pratt parser
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest = 0,
    LogicalOr = 1,
    LogicalAnd = 2,
    Equality = 3,
    Comparison = 4,
    Sum = 5,
    Product = 6,
    Prefix = 7,
    Call = 8,
}
