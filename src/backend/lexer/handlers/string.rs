use crate::backend::tokens::{Tokens, Primary};

pub fn handle_str(
    current_char: &char,
    index: &mut usize,
    input: &Vec<(usize, char)>,
    tokens: &mut Vec<Tokens>,
) -> Option<usize> {
    if *current_char != '"' {
        return None;
    }

    let mut value = String::new();
    *index += 1;

    while *index < input.len() {
        let ch = input[*index].1;
        if ch == '"' {
            break;
        }
        value.push(ch);
        *index += 1;
    }

    *index += 1; // skip closing quote
    tokens.push(Tokens::Primary(Primary::Str { val: value }));
    Some(*index)
}
