use std::panic;

use crate::backend::errors::Error;
use crate::backend::nodes::{Identifier, Statement};
use crate::backend::parser::Parser;
use crate::backend::tokens::{NonAtomic, Tokens};

impl Parser {
    pub fn parse_variables_or_function_calls(&mut self) -> Result<Statement, Error> {
        //consume the `name`
        let name_token = self.next().ok_or(Error::UnexpectedEOF)?;
        let name = match &name_token.kind {
            Tokens::Variable(val) => Identifier(val.to_string()),
            _ => {
                return Err(Error::InvalidSyntax(
                    "Expected a variable name ".to_string(),
                ));
            }
        };

        // check if its a function call or variable assignment
        let next_token = self.peek().ok_or(Error::UnexpectedEOF)?;

        match next_token.kind.clone() {
            Tokens::NonAtomic(NonAtomic::Assignment) => self.parse_variables(name),
            Tokens::NonAtomic(NonAtomic::LSquareBraces) => self.parse_array_index(name),
            Tokens::NonAtomic(NonAtomic::LParen) => self.parse_function_call(name),
            unexpected => {
                return Err(Error::InvalidSyntax(format!(
                    "Expected = or ( got, {:?}",
                    unexpected
                )));
            }
        }
    }

    pub fn parse_variables(&mut self, name: Identifier) -> Result<Statement, Error> {
        // expect `=`
        self.expect(Tokens::NonAtomic(NonAtomic::Assignment))?;

        let exp = self.parse_expressions(0);
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
