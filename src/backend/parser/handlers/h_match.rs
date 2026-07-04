use crate::backend::{
    error_parser::Error,
    nodes::{Block, Statement},
    parser::Parser,
    tokens::{NonAtomic, Primary, Token, Tokens},
};

impl Parser {
    // match value {}
    pub fn parse_match(&mut self) -> Result<Statement, Error> {
          let expected = Tokens::Primary(Primary::Str(vec!["value", "variable", "expression", "enum variant", "struct"].join(" or ").to_string()));
        let value = self.parse_expressions(0, expected)?;

        // consume {
        self.expect(Tokens::NonAtomic(NonAtomic::LCurlyBraces))?;

        let mut arms: Vec<(Box<crate::backend::nodes::Expression>, Block)> = vec![];

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
                _ => {
                    let expected = Tokens::Primary(Primary::Str("Expression".to_string()));
                    let pattern = self.parse_expressions(0, expected)?;

                    // expect =>
                    self.expect(Tokens::NonAtomic(NonAtomic::FatArrow))?;

                    // expect {
                    self.expect(Tokens::NonAtomic(NonAtomic::LCurlyBraces))?;
                    // parse arm body until }
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

                    // optional trailing comma
                    if let Some(Token {
                        kind: Tokens::NonAtomic(NonAtomic::Comma),
                        ..
                    }) = self.peek()
                    {
                        self.next();
                    }

                    arms.push((pattern, Block { statements: body }));
                }
            }
        }

        Ok(Statement::Match { value, arms })
    }
}
