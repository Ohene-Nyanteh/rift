use crate::backend::tokens::{Tokens, Operations, ArithmeticOp};

pub fn handle_operators(tokens: &mut Vec<Tokens>, current_char: &char, index: &mut usize) -> Option<usize> {
    match current_char {
        '+' => {
            tokens.push(Tokens::Atomic(Operations::Arithmetic(ArithmeticOp::Add)));
            *index += 1;
            Some(*index)
        },
        '-' => {
            tokens.push(Tokens::Atomic(Operations::Arithmetic(ArithmeticOp::Sub)));
            *index += 1;
            Some(*index)
        },
        '/' => {
            tokens.push(Tokens::Atomic(Operations::Arithmetic(ArithmeticOp::Div)));
            *index += 1;
            Some(*index)
        },
        '*' => {
            tokens.push(Tokens::Atomic(Operations::Arithmetic(ArithmeticOp::Mul)));
            *index += 1;
            Some(*index)
        },
        _ => None
    }
}
