use crate::backend::errors::Error;
use crate::backend::nodes::{Block, Statement};
use crate::backend::parser::Parser;
use crate::backend::tokens::{Keywords, NonAtomic, Token, Tokens};

impl Parser {
    // if (condition) { body } elif (condition) { body } else { body }
    pub fn parse_if(&mut self) -> Result<Statement, Error> {
        let condition = self.parse_condition()?;
        let body = self.parse_block()?;

        let mut elif_branches: Vec<(Box<crate::backend::nodes::Expression>, Block)> = vec![];
        let mut else_body: Option<Block> = None;

        loop {
            match self.peek() {
                Some(Token {
                    kind: Tokens::Keyword(Keywords::Elif),
                    ..
                }) => {
                    self.next(); // consume elif
                    let elif_condition = self.parse_condition()?;
                    let elif_body = self.parse_block()?;
                    elif_branches.push((elif_condition, elif_body));
                }
                Some(Token {
                    kind: Tokens::Keyword(Keywords::Else),
                    ..
                }) => {
                    self.next(); // consume else
                    else_body = Some(self.parse_block()?);
                    break;
                }
                _ => break,
            }
        }

        Ok(Statement::If {
            condition,
            body,
            elif_branches,
            else_body,
        })
    }
}

// --- helpers ---
impl Parser {
    // parses (expression) — used by if/while
    fn parse_condition(&mut self) -> Result<Box<crate::backend::nodes::Expression>, Error> {
        self.expect_token(Tokens::NonAtomic(NonAtomic::LParen))?;
        let condition = self.parse_expressions(0)?;
        self.expect_token(Tokens::NonAtomic(NonAtomic::RParen))?;
        Ok(condition)
    }

    // parses { statements }
    pub fn parse_block(&mut self) -> Result<Block, Error> {
        self.expect_token(Tokens::NonAtomic(NonAtomic::LCurlyBraces))?;
        let mut statements: Vec<Statement> = vec![];
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
                | None => {
                    return Err(Error::UnexpectedEOF);
                }
                _ => {
                    statements.push(self.parse_statement()?);
                }
            }
        }
        Ok(Block { statements })
    }

    fn expect_token(&mut self, expected: Tokens) -> Result<(), Error> {
        let token = self.next().ok_or(Error::UnexpectedEOF)?;
        if token.kind != expected {
            return Err(Error::InvalidSyntax(format!(
                "Expected {:?}, got {:?}",
                expected, token.kind
            )));
        }
        Ok(())
    }

    fn expect_semicolon(&mut self) -> Result<(), Error> {
        self.expect_token(Tokens::NonAtomic(NonAtomic::SemiColon))
    }
}
