use std::panic;

use crate::backend::errors::Error;
use crate::backend::nodes::{Identifier, LetDecl, Statement};
use crate::backend::parser::Parser;
use crate::backend::tokens::{NonAtomic, Tokens};

impl Parser {
    pub fn parse_let(&mut self) -> Result<Statement, Error> {
        // consume the `let` keyword (already matched by caller, or consume here)
        // self.next() was already called before entering, so next token should be the name

        let name_token = self.next().ok_or(Error::UnexpectedEOF)?;
        let name = match &name_token.kind {
            Tokens::Variable(val) => Identifier(val.to_string()),
            _ => {
                return Err(Error::InvalidSyntax(
                    "Expected a variable name ".to_string(),
                ));
            }
        };

        // expect `=`
        let eq_token = self.next().ok_or(Error::UnexpectedEOF)?;
        if eq_token.kind != Tokens::NonAtomic(NonAtomic::Assignment) {
            return Err(Error::InvalidSyntax("Expected = ".to_string()));
        }

        // parse the value
        let exp = self.parse_expressions(0);
        let value = match exp {
            Ok(value) => *value,
            Err(err) => panic!("Error: Couldnt Parse the Value {err:?}"),
        };

        // expect `;`
        self.expect(Tokens::NonAtomic(NonAtomic::SemiColon))?;

        Ok(Statement::Let(Box::new(LetDecl {
            name: name,
            value: Some(Box::new(value)),
        })))
    }
}
