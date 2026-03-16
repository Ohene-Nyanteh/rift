use crate::backend::tokens::{Tokens, Secondary, Keywords};

pub fn handle_variable(
    current_char: &char,
    index: &mut usize,
    input: &Vec<(usize, char)>,
    tokens: &mut Vec<Tokens>,
) -> Option<usize> {
    if !current_char.is_alphabetic() {
        return None;
    }

    let mut value = String::new();

    while *index < input.len() {
        let ch = input[*index].1;
        if !ch.is_alphanumeric() && ch != '_' {
            break;
        }
        value.push(ch);
        *index += 1;
    }

    tokens.push(to_keyword_or_variable(value));
    Some(*index)
}

fn to_keyword_or_variable(value: String) -> Tokens {
    match value.as_str() {
        "fn"       => Tokens::Secondary(Secondary::Keyword(Keywords::Fn)),
        "if"       => Tokens::Secondary(Secondary::Keyword(Keywords::If)),
        "else"     => Tokens::Secondary(Secondary::Keyword(Keywords::Else)),
        "elif"     => Tokens::Secondary(Secondary::Keyword(Keywords::Elif)),
        "let"      => Tokens::Secondary(Secondary::Keyword(Keywords::Let)),
        "while"    => Tokens::Secondary(Secondary::Keyword(Keywords::While)),
        "for"      => Tokens::Secondary(Secondary::Keyword(Keywords::For)),
        "break"    => Tokens::Secondary(Secondary::Keyword(Keywords::Break)),
        "continue" => Tokens::Secondary(Secondary::Keyword(Keywords::Continue)),
        "match"    => Tokens::Secondary(Secondary::Keyword(Keywords::Match)),
        _          => Tokens::Secondary(Secondary::Variable { val: value }),
    }
}
