use super::errors::Error;
use super::nodes::Statement;
use crate::backend::tokens::{Keywords, NonAtomic, Token, Tokens};
pub mod handlers;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn expect(&mut self, expected_token: Tokens) -> Result<(), Error> {
        let token = self.next().ok_or(Error::UnexpectedEOF)?;

        if token.kind != expected_token {
            return Err(Error::UnexpectedToken {
                expected: expected_token,
                found: token.kind,
            });
        }

        Ok(())
    }

    pub fn next(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).cloned();
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    pub fn parse_code(&mut self) -> Result<Vec<Statement>, Error> {
        let mut statements = Vec::new();
        while let Some(token) = self.peek() {
            if token.kind == Tokens::EOF {
                break;
            }
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    pub fn parse_statement(&mut self) -> Result<Statement, Error> {
        let token = self.peek().ok_or(Error::UnexpectedEOF)?;

        match &token.kind {
            Tokens::Keyword(Keywords::Let) => {
                self.next();
                self.parse_let()
            }
            Tokens::Keyword(Keywords::Enum) => {
                self.next();
                self.parse_enum()
            }
            Tokens::Keyword(Keywords::Fn) => {
                self.next();
                self.parse_functions()
            }
            Tokens::Keyword(Keywords::While) => {
                self.next();
                self.parse_while()
            }
            Tokens::Keyword(Keywords::If) => {
                self.next();
                self.parse_if()
            }
            Tokens::Keyword(Keywords::For) => {
                self.next();
                self.parse_for()
            }
            Tokens::Keyword(Keywords::Return) => {
                self.next();
                self.parse_return()
            }
            Tokens::Keyword(Keywords::Break) => {
                self.next();
                self.parse_break()
            }
            Tokens::Keyword(Keywords::Continue) => {
                self.next();
                self.parse_continue()
            }
            Tokens::Keyword(Keywords::Match) => {
                self.next();
                self.parse_match()
            }
            Tokens::Keyword(Keywords::Struct) => {
                self.next();
                self.parse_struct()
            }
            Tokens::EOF => Err(Error::UnexpectedEOF),
            _ => {
                let expr = self.parse_expressions(0)?;
                match self.next() {
                    Some(t) if t.kind == Tokens::NonAtomic(NonAtomic::SemiColon) => {
                        Ok(Statement::Expression(expr))
                    }
                    Some(t) => Err(Error::InvalidSyntax(format!(
                        "Expected ; after expression, got {:?}",
                        t.kind
                    ))),
                    None => Err(Error::UnexpectedEOF),
                }
            }
        }
    }
}
