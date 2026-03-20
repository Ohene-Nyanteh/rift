mod handlers;
use handlers::{handle_comments, handle_operators, handle_newline, handle_non_atomic, handle_whitespace, handle_comparison_operators, handle_numbers, handle_str, handle_variable};
use crate::backend::tokens::{Token, Tokens, Span};

pub fn tokenizer(input: String) -> Result<Vec<Token>, ()> {
    let mut index = 0;
    let mut tokens: Vec<Token> = vec![];
    let input_chars: Vec<(usize, char)> = input.char_indices().collect();
    let mut row: usize = 0;

    while index < input_chars.len() {
        let start = index;

        let (_, current_char) = input_chars[index];
        let next_char = input_chars.get(index + 1).map(|(_, c)| *c);
        if handle_newline(&current_char, &mut index, &mut row).is_some() { continue; }
        if handle_comments(start, &current_char, &mut index, &input_chars, &mut row).is_some() { continue; }
        if handle_non_atomic(&mut tokens, start, &current_char, &mut index, &row).is_some() { continue; }
        if handle_whitespace(&current_char, &mut index).is_some() { continue; }
        if handle_comparison_operators(&mut tokens, start, &current_char, &mut index, next_char, &row).is_some() { continue; }
        if handle_numbers(start, &current_char, &mut index, &input_chars, &mut tokens, &row).is_some() { continue; }
        if handle_str(start, &current_char, &mut index, &input_chars, &mut tokens, &row).is_some() { continue; }
        if handle_variable(start, &current_char, &mut index, &input_chars, &mut tokens, &row).is_some() { continue; }
        if handle_operators(&mut tokens, start, &current_char, &mut index, &row).is_some() { continue; }
        println!("Unknown character: {}", current_char);
        return Err(());
    }
    tokens.push(Token {kind: Tokens::EOF, span: Span {start: tokens.len(), end: tokens.len() + 1, row: row}});
    Ok(tokens)
}
