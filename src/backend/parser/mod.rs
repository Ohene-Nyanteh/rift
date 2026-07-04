use super::nodes::Statement;
use crate::{LexerOutput, backend::{error_parser::Error, tokens::{Keywords, NonAtomic, Primary, Token, Tokens}}};
pub mod handlers;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    source_map: Vec<String>, // source map for errors
    line_starts: Vec<usize>,
    pos: usize,
}


pub fn col_for(offset: usize, row: usize, line_starts: &[usize]) -> usize {
    offset - line_starts[row]
}


impl Parser {
    pub fn new(lexer_output: LexerOutput) -> Self {
        Self { tokens: lexer_output.tokens, source_map: lexer_output.source_map,line_starts: lexer_output.line_starts, pos: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos).clone()
    }

    fn line_text(&self, row: usize) -> String {
        self.source_map.get(row).cloned().unwrap_or_default()
    }

    pub fn expect(&mut self, expected_token: Tokens) -> Result<(), Error> {
        let token = self.next().ok_or(Error::UnexpectedEOF)?;

        if token.kind != expected_token {
            return Err(Error::UnexpectedToken {
                expected: expected_token,
                found: token.kind,
                at: token.span.clone(),
                error_line: self.line_text(token.span.row),
                col_start: col_for(token.span.start, token.span.row, &self.line_starts),
                col_end: col_for(token.span.end, token.span.row, &self.line_starts),
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
            Tokens::Keyword(Keywords::Loop) => {
                self.next();
                self.parse_loop()
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
            Tokens::Keyword(Keywords::Print) => {
                self.next();
                self.parse_print()
            }
            Tokens::Variable(_) => self.parse_variables_or_function_calls(),
            Tokens::EOF => Err(Error::UnexpectedEOF),
            _ => {
                let expected = Tokens::Primary(Primary::Str("Expression".to_string()));
                let expr = self.parse_expressions(0, expected)?;
                match self.next() {
                    Some(t) if t.kind == Tokens::NonAtomic(NonAtomic::SemiColon) => {
                        Ok(Statement::Expression(expr))
                    }
                    Some(t) => Err(Error::UnexpectedToken {
                        expected: Tokens::NonAtomic(NonAtomic::SemiColon),
                        found: t.kind,
                        error_line: self.line_text(t.span.row),
                        col_start: col_for(t.span.start, t.span.row, &self.line_starts),
                        col_end: col_for(t.span.end, t.span.row, &self.line_starts),
                        at: t.span
                    }),
                    None => Err(Error::UnexpectedEOF),
                }
            }
        }
    }
}
