mod handlers;
use std::vec;

use crate::{LexerOutput, backend::tokens::{Span, Token, Tokens}};
use handlers::{
    handle_comments, handle_comparison_operators, handle_newline, handle_non_atomic,
    handle_numbers, handle_operators, handle_str, handle_variable, handle_whitespace,
};

pub fn tokenizer(input: &String) -> Result<LexerOutput, ()> {
    let mut index = 0;
    let mut tokens: Vec<Token> = vec![];
    let input_chars: Vec<(usize, char)> = input.char_indices().collect();
    let mut row: usize = 0;
    let source_map: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let mut line_starts: Vec<usize> = vec![0]; // line 0 begins at char-index 0

    while index < input_chars.len() {
        let start = index;

        let (_, current_char) = input_chars[index];
        let next_char = input_chars.get(index + 1).map(|(_, c)| *c);
        if handle_newline(&current_char, &mut index, &mut row).is_some() {
            line_starts.push(index);
            continue;
        }
        if handle_comments(&current_char, &mut index, &input_chars).is_some() {
            continue;
        }
        if handle_non_atomic(&mut tokens, start, &current_char, &mut index, &row).is_some() {
            continue;
        }
        if handle_whitespace(&current_char, &mut index).is_some() {
            continue;
        }
        if handle_comparison_operators(
            &mut tokens,
            start,
            &current_char,
            &mut index,
            next_char,
            &row,
        )
        .is_some()
        {
            continue;
        }
        if handle_numbers(
            start,
            &current_char,
            &mut index,
            &input_chars,
            &mut tokens,
            &row,
        )
        .is_some()
        {
            continue;
        }
        if handle_str(
            start,
            &current_char,
            &mut index,
            &input_chars,
            &mut tokens,
            &row,
        )
        .is_some()
        {
            continue;
        }
        if handle_variable(
            start,
            &current_char,
            &mut index,
            &input_chars,
            &mut tokens,
            &row,
        )
        .is_some()
        {
            continue;
        }
        if handle_operators(&mut tokens, start, &current_char, &mut index, &row).is_some() {
            continue;
        }
        println!("Unknown character: {}", current_char);
        return Err(());
    }

    tokens.push(Token {
        kind: Tokens::EOF,
        span: Span {
            start: index,
            end: index + 1,
            row: row,
        },
    });


    Ok(LexerOutput { tokens, source_map: source_map, line_starts})
}
