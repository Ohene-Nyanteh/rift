use crate::backend::tokens::{Tokens, Operations, NonAtomic, ComparisonOp, LogicalOp};

pub fn handle_comparison_operators(tokens: &mut Vec<Tokens>, current_char: &char, index: &mut usize, next_char: Option<char>) -> Option<usize> {
    match current_char {
        '>' => {
            if next_char == Some('=') {
                tokens.push(Tokens::Atomic(Operations::Comparison(ComparisonOp::GreaterOrEquals)));
                *index += 2;
            } else {
                tokens.push(Tokens::Atomic(Operations::Comparison(ComparisonOp::GreaterThan)));
                *index += 1;
            }
            Some(*index)
        },
        '<' => {
            if next_char == Some('=') {
                tokens.push(Tokens::Atomic(Operations::Comparison(ComparisonOp::LessOrEquals)));
                *index += 2;
            } else {
                tokens.push(Tokens::Atomic(Operations::Comparison(ComparisonOp::LessThan)));
                *index += 1;
            }
            Some(*index)
        },
        '=' => {
            if next_char == Some('=') {
                tokens.push(Tokens::Atomic(Operations::Comparison(ComparisonOp::EqualTo)));
                *index += 2;
            } else {
                tokens.push(Tokens::NonAtomic(NonAtomic::Assignment));
                *index += 1;
            }
            Some(*index)
        },
        '!' => {
            if next_char == Some('=') {
                tokens.push(Tokens::Atomic(Operations::Comparison(ComparisonOp::NotEqualTo)));
                *index += 2;
            } else {
                tokens.push(Tokens::Atomic(Operations::Logical(LogicalOp::Not)));
                *index += 1;
            }
            Some(*index)
        },
        _ => None
    }
}
