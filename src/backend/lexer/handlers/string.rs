use crate::backend::tokens::{Tokens, Primary, Token, Span};

pub fn handle_str(
    start: usize,
    current_char: &char,
    index: &mut usize,
    input: &Vec<(usize, char)>,
    tokens: &mut Vec<Token>,
    row: &usize
) -> Option<usize> {
    if *current_char != '"' {
        return None;
    }

    let mut value = String::new();
    *index += 1; // skip opening quote

    while *index < input.len() {
        let ch = input[*index].1;
        if ch == '"' {
            break;
        }
        value.push(ch);
        *index += 1;
    }

    *index += 1; // skip closing quote

    tokens.push(Token {
        kind: Tokens::Primary(Primary::Str { val: value.into() }),
        span: Span { start, end: *index, row: *row},
    });

    Some(*index)
}
