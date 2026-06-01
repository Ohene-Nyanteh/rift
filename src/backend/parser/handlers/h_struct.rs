use crate::backend::{
    errors::Error,
    nodes::{Expression, Identifier, Statement, StructDecl},
    parser::Parser,
    tokens::{
        NonAtomic, Token,
        Tokens::{self},
    },
};

impl Parser {
    // struct Point { x: value, y: value, z: value }
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
        let mut fields: Vec<(Identifier, Expression)> = vec![];
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
                            Tokens::Variable(name) => {
                                self.expect(Tokens::NonAtomic(NonAtomic::Colon))?;

                                // parse next value: Primary / Variable
                                let token = self.next().ok_or(Error::UnexpectedEOF)?;
                                let v = self.parse_struct_value_exp(&token, &Identifier(name))?;
                                fields.push(v);
                            }
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

    fn parse_struct_value_exp(
        &mut self,
        token: &Token,
        name: &Identifier,
    ) -> Result<(Identifier, Expression), Error> {
        match token.kind.clone() {
            Tokens::Primary(v) => Ok((name.clone(), Expression::Literal(v))),
            Tokens::Variable(var) => {
                // check if its a normal var or an enum
                let next_token = self.peek().ok_or(Error::UnexpectedEOF)?;
                match next_token.kind {
                    Tokens::NonAtomic(NonAtomic::Colon) => {
                        let exp = self.parse_enum_calls(Identifier(var))?;
                        Ok((name.clone(), exp))
                    }
                    _ => {
                        // store it as a normal variable
                        Ok((name.clone(), Expression::Variable(Identifier(var))))
                    }
                }
            }
            unexpected => {
                return Err(Error::InvalidSyntax(format!(
                    "Expected a variable, value or a enum variant got {unexpected:?}"
                )));
            }
        }
    }
}
