use crate::backend::{
    errors::Error,
    nodes::Statement,
    parser::Parser,
    tokens::{NonAtomic, Tokens},
};

impl Parser {
    pub fn parse_continue(&mut self) -> Result<Statement, Error> {
        // expected semicolon after continue
        match self.expect(Tokens::NonAtomic(NonAtomic::SemiColon)) {
            Ok(()) => Ok(Statement::Continue),
            Err(error) => Err(error),
        }
    }
}
