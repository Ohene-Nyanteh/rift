use crate::backend::{
    errors::Error,
    nodes::{Expression, Identifier, Statement},
    parser::Parser,
    tokens::{NonAtomic, Primary, Tokens},
};

impl Parser {
    pub fn parse_print(&mut self) -> Result<Statement, Error> {
        // expect the next (
        self.expect(Tokens::NonAtomic(NonAtomic::LParen))?;

        // read the expression value
        let value = match self.peek() {
            None => return Err(Error::UnexpectedEOF)?,
            Some(v) => match v.clone().kind {
                Tokens::Atomic(_) | Tokens::Variable(_) => match self.parse_expressions(0) {
                    Ok(exp) => exp,
                    Err(_) => panic!("Error parsing expression"),
                },
                Tokens::Primary(value) => {
                    self.next();
                    Box::new(Expression::Literal(value))
                }
                unexpected => {
                    return Err(Error::UnexpectedToken {
                        expected: Tokens::Primary(Primary::Str("".to_string())),
                        found: unexpected,
                    })?;
                }
            },
        };

        // expect and consume } and ;
        self.expect(Tokens::NonAtomic(NonAtomic::RParen))?;
        self.expect(Tokens::NonAtomic(NonAtomic::SemiColon))?;

        Ok(Statement::Print(value))
    }
}
