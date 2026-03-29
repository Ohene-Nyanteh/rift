use crate::backend::{
    errors::Error,
    nodes::Statement,
    parser::Parser,
    tokens::{NonAtomic, Token, Tokens},
};

impl Parser {
    pub fn parse_return(&mut self) -> Result<Statement, Error> {
        // if next token is ; or } return nothing
        match self.peek() {
            Some(Token {
                kind: Tokens::NonAtomic(NonAtomic::SemiColon),
                ..
            })
            | Some(Token {
                kind: Tokens::NonAtomic(NonAtomic::RCurlyBraces),
                ..
            }) => {
                self.next(); // consume the semicolon
                return Ok(Statement::Return(None));
            }
            _ => {}
        }

        let value = self.parse_expressions(0)?;

        // consume ;
        match self.next() {
            Some(t) if t.kind == Tokens::NonAtomic(NonAtomic::SemiColon) => {}
            Some(t) => {
                return Err(Error::InvalidSyntax(format!(
                    "Expected ; after return, got {:?}",
                    t.kind
                )));
            }
            None => return Err(Error::UnexpectedEOF),
        }

        Ok(Statement::Return(Some(value)))
    }
}
