use crate::backend::error_parser::Error;
use crate::backend::nodes::{Block, Identifier, Statement};
use crate::backend::parser::{Parser, col_for};
use crate::backend::tokens::{Keywords, NonAtomic, Primary, Token, Tokens};

impl Parser {
    // for (item in iterable) { body }
    pub fn parse_for(&mut self) -> Result<Statement, Error> {
        // consume (
        match self.next() {
            Some(t) if t.kind == Tokens::NonAtomic(NonAtomic::LParen) => {}
            Some(t) => return  Err(Error::UnexpectedToken {
                expected: Tokens::NonAtomic(NonAtomic::LParen),
                error_line: self.line_text(t.span.row),
                col_start: col_for(t.span.start, t.span.row, &self.line_starts),
                col_end: col_for(t.span.end, t.span.row, &self.line_starts),
                found: t.kind,
                at: t.span
            }),
            None => return Err(Error::UnexpectedEOF),
        }

        // consume variable name
        let var = match self.next() {
            Some(t) => match t.kind {
                Tokens::Variable(name) => Identifier(name),
                unexpected => {
                    return  Err(Error::UnexpectedToken{
                        expected: Tokens::Primary(Primary::Str("variable name in for-loop".to_string())),
                        error_line: self.line_text(t.span.row),
                        col_start: col_for(t.span.start, t.span.row, &self.line_starts),
                        col_end: col_for(t.span.end, t.span.row, &self.line_starts),
                        found: unexpected,
                        at: t.span
                    });
                }
            },
            None => return Err(Error::UnexpectedEOF),
        };

        // consume `in`
        self.expect(Tokens::Keyword(Keywords::In))?;

        // parse iterable expression
        let expected = Tokens::Primary(Primary::Str("Expression".to_string()));
        let iterable = self.parse_expressions(0, expected)?;

        // consume )
        self.expect(Tokens::NonAtomic(NonAtomic::RParen))?;

        // consume {
        self.expect(Tokens::NonAtomic(NonAtomic::LCurlyBraces))?;

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
