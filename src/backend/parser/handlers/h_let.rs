use crate::backend::error_parser::Error::{self};
use crate::backend::nodes::{Identifier, LetDecl, Statement};
use crate::backend::parser::{Parser, col_for};
use crate::backend::tokens::{NonAtomic, Primary, Tokens};

impl Parser {
    pub fn parse_let(&mut self) -> Result<Statement, Error> {
        // consume the `let` keyword (already matched by caller, or consume here)
        // self.next() was already called before entering, so next token should be the name

        let name_token = self.next().ok_or(Error::UnexpectedEOF)?;
        let name = match &name_token.kind {
            Tokens::Variable(val) => Identifier(val.to_string()),
            unexpected => {
                return  Err(Error::UnexpectedToken{
                    expected: Tokens::Primary(Primary::Str("variable name".to_string())),
                    error_line: self.line_text(name_token.span.row),
                    col_start: col_for(name_token.span.start, name_token.span.row, &self.line_starts),
                    col_end: col_for(name_token.span.end, name_token.span.row, &self.line_starts),
                    found: unexpected.clone(),
                    at: name_token.span
                });
            }
        };

        // expect `=`
        self.expect(Tokens::NonAtomic(NonAtomic::Assignment))?;

        // parse the value
        let expected = Tokens::Primary(Primary::Str(vec!["value", "struct", "enum variant"].join(" or ").to_string()));
        let value = *self.parse_expressions(0, expected)?;

        // expect `;`
        self.expect(Tokens::NonAtomic(NonAtomic::SemiColon))?;

        Ok(Statement::Let(Box::new(LetDecl {
            name: name,
            value: Some(Box::new(value)),
        })))
    }
}
