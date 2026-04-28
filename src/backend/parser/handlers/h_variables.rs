use std::panic;

use crate::backend::errors::Error;
use crate::backend::nodes::{Identifier, Statement};
use crate::backend::parser::Parser;
use crate::backend::tokens::{NonAtomic, Tokens};

impl Parser {
    pub fn parse_variables(&mut self) -> Result<Statement, Error> {
        // // consume the `name`
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
        // let value_token = self.next().ok_or(Error::UnexpectedEOF)?;
        // let value = match &value_token.kind {
        //     Tokens::Primary(val) => Expression::Literal(val.clone()),
        //     _ => return Err(Error::InvalidSyntax("Expected a value ".to_string())),
        // };

        let exp = self.parse_expressions(0);
        let value = match exp {
            Ok(value) => *value,
            Err(_) => panic!("Error: Couldnt Parse "),
        };

        // expect `;`
        let semi_token = self.next().ok_or(Error::UnexpectedEOF)?;
        if semi_token.kind != Tokens::NonAtomic(NonAtomic::SemiColon) {
            return Err(Error::InvalidSyntax("Expected a ; ".to_string()));
        }

        Ok(Statement::VariableAssignment {
            var: name,
            exp: Box::new(value),
        })
    }
}
