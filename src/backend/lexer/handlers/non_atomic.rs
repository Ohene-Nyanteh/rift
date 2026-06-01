use crate::backend::tokens::{NonAtomic, Span, Token, Tokens};

pub fn handle_non_atomic(
    tokens: &mut Vec<Token>,
    start: usize,
    current_char: &char,
    index: &mut usize,
    row: &usize,
) -> Option<usize> {
    let t = match *current_char {
        ':' => {
            *index += 1;
            Token {
                kind: Tokens::NonAtomic(NonAtomic::Colon),
                span: Span {
                    start,
                    end: *index,
                    row: *row,
                },
            }
        }
        ';' => {
            *index += 1;
            Token {
                kind: Tokens::NonAtomic(NonAtomic::SemiColon),
                span: Span {
                    start,
                    end: *index,
                    row: *row,
                },
            }
        }
        '{' | '}' => {
            *index += 1;
            let kind = if *current_char == '{' {
                NonAtomic::LCurlyBraces
            } else {
                NonAtomic::RCurlyBraces
            };
            Token {
                kind: Tokens::NonAtomic(kind),
                span: Span {
                    start,
                    end: *index,
                    row: *row,
                },
            }
        }
        ',' => {
            *index += 1;
            Token {
                kind: Tokens::NonAtomic(NonAtomic::Comma),
                span: Span {
                    start,
                    end: *index,
                    row: *row,
                },
            }
        }
        '(' | ')' => {
            *index += 1;
            let kind = if *current_char == '(' {
                NonAtomic::LParen
            } else {
                NonAtomic::RParen
            };
            Token {
                kind: Tokens::NonAtomic(kind),
                span: Span {
                    start,
                    end: *index,
                    row: *row,
                },
            }
        }
        '[' | ']' => {
            *index += 1;
            let kind = if *current_char == '[' {
                NonAtomic::LSquareBraces
            } else {
                NonAtomic::RSquareBraces
            };
            Token {
                kind: Tokens::NonAtomic(kind),
                span: Span {
                    start,
                    end: *index,
                    row: *row,
                },
            }
        }
        '.' => {
            *index += 1;
            Token {
                kind: Tokens::NonAtomic(NonAtomic::Dot),
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
