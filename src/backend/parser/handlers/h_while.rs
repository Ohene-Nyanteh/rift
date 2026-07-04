use crate::backend::{
    error_parser::Error,
    nodes::{Block, Statement},
    parser::Parser,
    tokens::{NonAtomic, Primary, Token, Tokens},
};

impl Parser {
    // while (condition) { body }
    pub fn parse_while(&mut self) -> Result<Statement, Error> {
        // consume (
        self.expect(Tokens::NonAtomic(NonAtomic::LParen))?;

        let expected = Tokens::Primary(Primary::Str("condition".to_string()));
        let condition = self.parse_expressions(0, expected)?;

        // consume )
        self.expect(Tokens::NonAtomic(NonAtomic::RParen))?;

        // consume {
        self.expect(Tokens::NonAtomic(NonAtomic::LCurlyBraces))?;

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

        Ok(Statement::While {
            condition: condition,
            body: Block { statements: body },
        })
    }
}
