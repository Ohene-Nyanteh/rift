use crate::backend::{
    errors::Error,
    nodes::{Identifier, Statement, StructDecl},
    parser::Parser,
    tokens::{NonAtomic, Token, Tokens},
};

impl Parser {
    // struct Point { x, y, z }
    pub fn parse_struct(&mut self) -> Result<Statement, Error> {
        // consume struct name
        let name = match self.next() {
            Some(t) => match t.kind {
                Tokens::Variable(name) => Identifier(name),
                unexpected => {
                    return Err(Error::InvalidSyntax(format!(
                        "Expected struct name, got {:?}",
                        unexpected
                    )));
                }
            },
            None => return Err(Error::UnexpectedEOF),
        };

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

        // parse fields until }
        let mut fields: Vec<Identifier> = vec![];
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
                    // consume field name
                    match self.next() {
                        Some(t) => match t.kind {
                            Tokens::Variable(name) => fields.push(Identifier(name)),
                            unexpected => {
                                return Err(Error::InvalidSyntax(format!(
                                    "Expected field name, got {:?}",
                                    unexpected
                                )));
                            }
                        },
                        None => return Err(Error::UnexpectedEOF),
                    }

                    // optional trailing comma
                    if let Some(Token {
                        kind: Tokens::NonAtomic(NonAtomic::Comma),
                        ..
                    }) = self.peek()
                    {
                        self.next();
                    }
                }
            }
        }

        Ok(Statement::Struct(Box::new(StructDecl { name, fields })))
    }
}
