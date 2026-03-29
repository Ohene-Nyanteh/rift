use crate::backend::errors::Error;
use crate::backend::nodes::{Block, Identifier, Statement};
use crate::backend::parser::Parser;
use crate::backend::tokens::{NonAtomic, Token, Tokens};

impl Parser {
    // for (item in iterable) { body }
    pub fn parse_for(&mut self) -> Result<Statement, Error> {
        // consume (
        match self.next() {
            Some(t) if t.kind == Tokens::NonAtomic(NonAtomic::LParen) => {}
            Some(t) => return Err(Error::InvalidSyntax(format!("Expected ( got {:?}", t.kind))),
            None => return Err(Error::UnexpectedEOF),
        }

        // consume variable name
        let var = match self.next() {
            Some(t) => match t.kind {
                Tokens::Variable(name) => Identifier(name),
                unexpected => {
                    return Err(Error::InvalidSyntax(format!(
                        "Expected variable name in for loop, got {:?}",
                        unexpected
                    )));
                }
            },
            None => return Err(Error::UnexpectedEOF),
        };

        // consume `in`
        match self.next() {
            Some(t) => match &t.kind {
                Tokens::Variable(s) if s == "in" => {}
                unexpected => {
                    return Err(Error::InvalidSyntax(format!(
                        "Expected 'in' in for loop, got {:?}",
                        unexpected
                    )));
                }
            },
            None => return Err(Error::UnexpectedEOF),
        }

        // parse iterable expression
        let iterable = self.parse_expressions(0)?;

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

        // parse body until }
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

        Ok(Statement::For {
            var,
            iterable,
            body: Block { statements: body },
        })
    }
}
