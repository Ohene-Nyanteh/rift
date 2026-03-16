mod handlers;
use handlers::{handle_comments, handle_operators, handle_non_atomic, handle_whitespace, handle_comparison_operators, handle_numbers, handle_str, handle_variable};
use crate::backend::tokens::Tokens;

pub fn tokenizer(input: String) -> Result<(), ()> {
    let mut index = 0;
    let mut tokens: Vec<Tokens> = vec![];
    let input_chars: Vec<(usize, char)> = input.char_indices().collect();

    while index < input_chars.len() {
        let (_, current_char) = input_chars[index];
        let next_char = input_chars.get(index + 1).map(|(_, c)| *c);

        if handle_comments(&current_char, &mut index, &input_chars).is_some() { continue; }
        if handle_non_atomic(&mut tokens, &current_char, &mut index).is_some() { continue; }
        if handle_whitespace(&current_char, &mut index).is_some() { continue; }
        if handle_comparison_operators(&mut tokens, &current_char, &mut index, next_char).is_some() { continue; }
        if handle_numbers(&current_char, &mut index, &input_chars, &mut tokens).is_some() { continue; }
        if handle_str(&current_char, &mut index, &input_chars, &mut tokens).is_some() { continue; }
        if handle_variable(&current_char, &mut index, &input_chars, &mut tokens).is_some() { continue; }
        if handle_operators(&mut tokens, &current_char, &mut index).is_some() { continue; }
        println!("Unknown character: {}", current_char);
        return Err(());
    }

    println!("{:?}", tokens);
    Ok(())
}
