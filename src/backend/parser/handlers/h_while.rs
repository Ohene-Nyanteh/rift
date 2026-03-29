use crate::backend::{
    errors::Error,
    nodes::{Block, Statement},
    parser::Parser,
    tokens::{NonAtomic, Token, Tokens},
};

impl Parser {
    // while (condition) { body }
    pub fn parse_while(&mut self) -> Result<Statement, Error> {
        // consume (
        match self.next() {
            Some(t) if t.kind == Tokens::NonAtomic(NonAtomic::LParen) => {}
            Some(t) => return Err(Error::InvalidSyntax(format!("Expected ( got {:?}", t.kind))),
            None => return Err(Error::UnexpectedEOF),
        }

        let condition = self.parse_expressions(0)?;

        // consume )
        match self.next() {
            Some(t) if t.kind == Tokens::NonAtomic(NonAtomic::RParen) => {}
            Some(t) => return Err(Error::InvalidSyntax(format!("Expected ) got {:?}", t.kind))),
            None => return Err(Error::UnexpectedEOF),
        }

        // consume {
        match self.next() {
            Some(t) if t.kind == Tokens::NonAtomic(NonAtomic::LCurlyBraces) => {}
            Some(t) => {
                return Err(Error::InvalidSyntax(format!(
                    "Expected {{ got {:?}",
                    t.kind
                )));
            }
            None => return Err(Error::UnexpectedEOF),
        }

        let mut body: Vec<Statement> = vec![];
        loop {
            match self.peek() {
                Some(Token {
                    kind: Tokens::NonAtomic(NonAtomic::RCurlyBraces),
                    ..
                }) => {
                    self.next();
                    break;
                }
                Some(Token {
                    kind: Tokens::EOF, ..
                })
                | None => return Err(Error::UnexpectedEOF),
                _ => body.push(self.parse_statement()?),
            }
        }

        Ok(Statement::While {
            condition: condition,
            body: Block { statements: body },
        })
    }
}
