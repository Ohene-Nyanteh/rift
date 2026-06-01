use crate::backend::{
    errors::Error,
    nodes::{EnumDecl, Identifier, Statement},
    parser::Parser,
    tokens::{NonAtomic, Token, Tokens},
};

impl Parser {
    // enum Color { Red, Green, Blue }
    pub fn parse_enum(&mut self) -> Result<Statement, Error> {
        // consume enum name
        let name = match self.next() {
            Some(t) => match t.kind {
                Tokens::Variable(name) => Identifier(name),
                unexpected => {
                    return Err(Error::InvalidSyntax(format!(
                        "Expected enum name, got {:?}",
                        unexpected
                    )));
                }
            },
            None => return Err(Error::UnexpectedEOF),
        };

        // consume {
        self.expect(Tokens::NonAtomic(NonAtomic::LCurlyBraces))?;

        // parse variants until }
        let mut variants: Vec<Identifier> = vec![];
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
                    // consume variant name
                    match self.next() {
                        Some(t) => match t.kind {
                            Tokens::Variable(name) => variants.push(Identifier(name)),
                            unexpected => {
                                return Err(Error::InvalidSyntax(format!(
                                    "Expected variant name, got {:?}",
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

        // self.expect(Tokens::NonAtomic(NonAtomic::SemiColon))?;

        Ok(Statement::Enum(Box::new(EnumDecl { name, variants })))
    }
}
