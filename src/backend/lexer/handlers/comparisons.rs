use crate::backend::tokens::{
    NonAtomic, Operations, Span, Token,
    Tokens::{self},
};

pub fn handle_comparison_operators(
    tokens: &mut Vec<Token>,
    start: usize,
    current_char: &char,
    index: &mut usize,
    next_char: Option<char>,
    row: &usize,
) -> Option<usize> {
    let t = match current_char {
        '>' => {
            if next_char == Some('=') {
                *index += 2;
                Token {
                    kind: Tokens::Atomic(Operations::GreaterOrEquals),
                    span: Span {
                        start,
                        end: *index,
                        row: *row,
                    },
                }
            } else {
                *index += 1;
                Token {
                    kind: Tokens::Atomic(Operations::GreaterThan),
                    span: Span {
                        start,
                        end: *index,
                        row: *row,
                    },
                }
            }
        }
        '<' => {
            if next_char == Some('=') {
                *index += 2;
                Token {
                    kind: Tokens::Atomic(Operations::LessOrEquals),
                    span: Span {
                        start,
                        end: *index,
                        row: *row,
                    },
                }
            } else {
                *index += 1;
                Token {
                    kind: Tokens::Atomic(Operations::LessThan),
                    span: Span {
                        start,
                        end: *index,
                        row: *row,
                    },
                }
            }
        }
        '=' => {
            if next_char == Some('=') {
                *index += 2;
                Token {
                    kind: Tokens::Atomic(Operations::EqualTo),
                    span: Span {
                        start,
                        end: *index,
                        row: *row,
                    },
                }
            } else if next_char == Some('>') {
                *index += 2;
                Token {
                    kind: Tokens::NonAtomic(NonAtomic::FatArrow),
                    span: Span {
                        start,
                        end: *index,
                        row: *row,
                    },
                }
            } else {
                *index += 1;
                Token {
                    kind: Tokens::NonAtomic(NonAtomic::Assignment),
                    span: Span {
                        start,
                        end: *index,
                        row: *row,
                    },
                }
            }
        }
        '!' => {
            if next_char == Some('=') {
                *index += 2;
                Token {
                    kind: Tokens::Atomic(Operations::NotEqualTo),
                    span: Span {
                        start,
                        end: *index,
                        row: *row,
                    },
                }
            } else {
                *index += 1;
                Token {
                    kind: Tokens::Atomic(Operations::Not),
                    span: Span {
                        start,
                        end: *index,
                        row: *row,
                    },
                }
            }
        }
        _ => return None,
    };

    tokens.push(t);
    Some(*index)
}
