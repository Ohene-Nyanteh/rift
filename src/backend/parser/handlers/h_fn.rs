use crate::backend::error_parser::Error;
use crate::backend::nodes::{Block, FunctionDecl, Identifier, Statement};
use crate::backend::parser::{Parser, col_for};
use crate::backend::tokens::{NonAtomic, Primary, Token, Tokens};

impl Parser {
    pub fn parse_functions(&mut self) -> Result<Statement, Error> {
        /*
         * fn main(parse_args) {
         *  create a block here and call parse_statements
         * }
         */

        let fn_name_token = self.next().ok_or(Error::UnexpectedEOF)?;
        let fn_name = match &fn_name_token.kind {
            Tokens::Variable(val) => Identifier(val.to_string()),
            unexpected => {
                return Err(Error::UnexpectedToken{
                    expected: Tokens::Primary(Primary::Str("fn name".to_string())),
                    error_line: self.line_text(fn_name_token.span.row),
                    col_start: col_for(fn_name_token.span.start, fn_name_token.span.row, &self.line_starts),
                    col_end: col_for(fn_name_token.span.end, fn_name_token.span.row, &self.line_starts),
                    found: unexpected.clone(),
                    at: fn_name_token.span
                });
            }
        };

        // expect the opening tag
        self.expect(Tokens::NonAtomic(NonAtomic::LParen))?;

        // run a simple loop till we see )
        let mut args: Vec<Identifier> = vec![];
        loop {
            let next_token = self.next().ok_or(Error::UnexpectedEOF)?;

            match &next_token.kind {
                Tokens::NonAtomic(NonAtomic::RParen) => break,
                Tokens::Variable(val) => args.push(Identifier(val.to_string())),
                Tokens::NonAtomic(NonAtomic::Comma) => continue,
                unexpected => {
                    return Err(Error::UnexpectedToken{
                        expected: Tokens::Primary(Primary::Str(") or args,".to_string())),
                        error_line: self.line_text(next_token.span.row),
                        col_start: col_for(next_token.span.start, next_token.span.row, &self.line_starts),
                        col_end: col_for(next_token.span.end, next_token.span.row, &self.line_starts),
                        found: unexpected.clone(),
                        at: next_token.span
                    });
                }
            }
        }

        // expect left curly braces
        self.expect(Tokens::NonAtomic(NonAtomic::LCurlyBraces))?;

        // pass the body of the function till you see }
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
                | None => {
                    return Err(Error::UnexpectedEOF);
                }
                _ => {
                    body.push(self.parse_statement()?);
                }
            }
        }

        Ok(Statement::Function(Box::new(FunctionDecl {
            name: fn_name,
            args,
            body: Block { statements: body },
        })))
    }
}
