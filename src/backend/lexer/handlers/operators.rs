use crate::backend::tokens::{Operations, Span, Token, Tokens};

pub fn handle_operators(
    tokens: &mut Vec<Token>,
    start: usize,
    current_char: &char,
    index: &mut usize,
    row: &usize,
) -> Option<usize> {
    let t = match current_char {
        '+' => {
            *index += 1;
            Token {
                kind: Tokens::Atomic(Operations::Add),
                span: Span {
                    start,
                    end: *index,
                    row: *row,
                },
            }
        }
        '-' => {
            *index += 1;
            Token {
                kind: Tokens::Atomic(Operations::Sub),
                span: Span {
                    start,
                    end: *index,
                    row: *row,
                },
            }
        }
        '/' => {
            *index += 1;
            Token {
                kind: Tokens::Atomic(Operations::Div),
                span: Span {
                    start,
                    end: *index,
                    row: *row,
                },
            }
        }
        '*' => {
            *index += 1;
            Token {
                kind: Tokens::Atomic(Operations::Mul),
                span: Span {
                    start,
                    end: *index,
                    row: *row,
                },
            }
        }
        '&' => {
            *index += 1;
            Token {
                kind: Tokens::Atomic(Operations::And),
                span: Span {
                    start,
                    end: *index,
                    row: *row,
                },
            }
        }
        '|' => {
            *index += 1;
            Token {
                kind: Tokens::Atomic(Operations::Or),
                span: Span {
                    start,
                    end: *index,
                    row: *row,
                },
            }
        }
        '%' => {
            *index += 1;
            Token {
                kind: Tokens::Atomic(Operations::Mod),
                span: Span {
                    start,
                    end: *index,
                    row: *row,
                },
            }
        }
        _ => return None,
    };

    tokens.push(t);
    Some(*index)
}
