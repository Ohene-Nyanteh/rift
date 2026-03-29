use crate::backend::{
    errors::Error,
    nodes::Statement,
    parser::Parser,
    tokens::{NonAtomic, Tokens},
};

impl Parser {
    pub fn parse_break(&mut self) -> Result<Statement, Error> {
        // expected semicolon after break
        match self.expect(Tokens::NonAtomic(NonAtomic::SemiColon)) {
            Ok(()) => Ok(Statement::Break),
            Err(error) => Err(error),
        }
    }
}
