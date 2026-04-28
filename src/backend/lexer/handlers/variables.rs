use crate::backend::tokens::{Keywords, Primary, Span, Token, Tokens};

pub fn handle_variable(
    start: usize,
    current_char: &char,
    index: &mut usize,
    input: &Vec<(usize, char)>,
    tokens: &mut Vec<Token>,
    row: &usize,
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

    let token_kind = to_keyword_or_variable(value);
    tokens.push(Token {
        kind: token_kind,
        span: Span {
            start,
            end: *index,
            row: *row,
        },
    });

    Some(*index)
}

fn to_keyword_or_variable(value: String) -> Tokens {
    match value.as_str() {
        "fn" => Tokens::Keyword(Keywords::Fn),
        "if" => Tokens::Keyword(Keywords::If),
        "else" => Tokens::Keyword(Keywords::Else),
        "elif" => Tokens::Keyword(Keywords::Elif),
        "let" => Tokens::Keyword(Keywords::Let),
        "while" => Tokens::Keyword(Keywords::While),
        "for" => Tokens::Keyword(Keywords::For),
        "break" => Tokens::Keyword(Keywords::Break),
        "continue" => Tokens::Keyword(Keywords::Continue),
        "struct" => Tokens::Keyword(Keywords::Struct),
        "return" => Tokens::Keyword(Keywords::Return),
        "enum" => Tokens::Keyword(Keywords::Enum),
        "match" => Tokens::Keyword(Keywords::Match),
        "print" => Tokens::Keyword(Keywords::Print),

        // bool data type
        "true" => Tokens::Primary(Primary::Bool(true)),
        "false" => Tokens::Primary(Primary::Bool(false)),
        _ => Tokens::Variable(value),
    }
}
