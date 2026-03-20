use crate::backend::tokens::{Tokens, Primary, Token, Span};

pub fn handle_numbers(
    start: usize,
    current_char: &char,
    index: &mut usize,
    input: &Vec<(usize, char)>,
    tokens: &mut Vec<Token>,
    row: &usize
) -> Option<usize> {
    if !current_char.is_ascii_digit() {
        return None;
    }

    let mut value = String::new();

    while *index < input.len() {
        let ch = input[*index].1;
        if !ch.is_ascii_digit() {
            break;
        }
        value.push(ch);
        *index += 1;
    }

    tokens.push(Token {
        kind: Tokens::Primary(Primary::Int { val: value.parse().unwrap() }),
        span: Span { start, end: *index, row: *row},
    });

    Some(*index)
}
