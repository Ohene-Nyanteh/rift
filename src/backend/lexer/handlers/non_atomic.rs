use crate::backend::tokens::{Tokens, NonAtomic, SymbolVal};

pub fn handle_non_atomic(tokens: &mut Vec<Tokens>, current_char: &char, index: &mut usize) -> Option<usize> {
    match *current_char {
        ':' => {
            tokens.push(Tokens::NonAtomic(NonAtomic::Colon));
            *index += 1;
            Some(*index)
        },
        ';' => {
            tokens.push(Tokens::NonAtomic(NonAtomic::SemiColon));
            *index += 1;
            Some(*index)
        },
        '{' | '}' => {
            if *current_char == '{' {
                tokens.push(Tokens::NonAtomic(NonAtomic::CurlyBraces(SymbolVal::Open)));
                *index += 1;
                Some(*index)
            }
            else {
                tokens.push(Tokens::NonAtomic(NonAtomic::CurlyBraces(SymbolVal::Close)));
                *index += 1;
                Some(*index)
            }

        },
        ',' => {
            tokens.push(Tokens::NonAtomic(NonAtomic::Commar));
            *index += 1;
            Some(*index)
        },
        '(' | ')'  => {
            if *current_char == '(' {
                tokens.push(Tokens::NonAtomic(NonAtomic::Paren(SymbolVal::Open)));
                *index += 1;
                Some(*index)
            }
            else {
                tokens.push(Tokens::NonAtomic(NonAtomic::Paren(SymbolVal::Close)));
                *index += 1;
                Some(*index)
            }

        },
        _ => None
    }
}
