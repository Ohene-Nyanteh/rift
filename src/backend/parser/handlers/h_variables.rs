use std::panic;

use crate::backend::error_parser::Error;
use crate::backend::nodes::{Identifier, Statement};
use crate::backend::parser::{Parser, col_for};
use crate::backend::tokens::{NonAtomic, Primary, Tokens};

impl Parser {
    pub fn parse_variables_or_function_calls(&mut self) -> Result<Statement, Error> {
        //consume the `name`
        let name_token = self.next().ok_or(Error::UnexpectedEOF)?;
        let name = match name_token.kind {
            Tokens::Variable(val) => Identifier(val.to_string()),
            unexpected => {
                return Err(Error::UnexpectedToken{
                    expected: Tokens::Primary(Primary::Str("A variable name".to_string())),
                    error_line: self.line_text(name_token.span.row),
                    col_start: col_for(name_token.span.start, name_token.span.row, &self.line_starts),
                    col_end: col_for(name_token.span.end, name_token.span.row, &self.line_starts),
                    found: unexpected,
                    at: name_token.span
                });
            }
        };

        // check if its a function call or variable assignment
        let next_token = self.peek().ok_or(Error::UnexpectedEOF)?;

        match next_token.kind.clone() {
            Tokens::NonAtomic(NonAtomic::Assignment) => self.parse_variables(name),
            Tokens::NonAtomic(NonAtomic::LSquareBraces) => self.parse_array_index(name),
            Tokens::NonAtomic(NonAtomic::LParen) => self.parse_function_call(name),
            Tokens::NonAtomic(NonAtomic::Dot) => {
                let value = self.parse_struct_call(name)?;
                self.expect(Tokens::NonAtomic(NonAtomic::SemiColon))?;
                Ok(Statement::Expression(value))
            }
            // Tokens::NonAtomic(NonAtomic::Colon) => self.parse_enum_calls(name),
            unexpected => {
                return  Err(Error::UnexpectedToken {
                    expected: Tokens::Primary(Primary::Str(vec!["'('", "'='"].join(", ").to_string())),
                    error_line: self.line_text(next_token.span.row),
                    col_start: col_for(next_token.span.start, next_token.span.row, &self.line_starts),
                    col_end: col_for(next_token.span.end, next_token.span.row, &self.line_starts),
                    found: unexpected,
                    at: next_token.span.clone()
                });
            }
        }
    }

    pub fn parse_variables(&mut self, name: Identifier) -> Result<Statement, Error> {
        // expect `=`
        self.expect(Tokens::NonAtomic(NonAtomic::Assignment))?;

        let expected = Tokens::Primary(Primary::Str(vec!["expression", "value", "enum|struct variant"].join(" or ").to_string()));
        let exp = self.parse_expressions(0, expected);
        let value = match exp {
            Ok(value) => *value,
            Err(_) => panic!("Error: Couldnt Parse "),
        };

        // expect `;`
        self.expect(Tokens::NonAtomic(NonAtomic::SemiColon))?;

        Ok(Statement::VariableAssignment {
            var: name,
            exp: Box::new(value),
        })
    }
}
