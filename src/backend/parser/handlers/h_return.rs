use crate::backend::{
    error_parser::Error,
    nodes::Statement,
    parser::Parser,
    tokens::{NonAtomic, Primary, Token, Tokens},
};

impl Parser {
    pub fn parse_return(&mut self) -> Result<Statement, Error> {
        match self.peek() {
            Some(Token {
                kind: Tokens::NonAtomic(NonAtomic::SemiColon),
                ..
            })
            | Some(Token {
                kind: Tokens::NonAtomic(NonAtomic::RCurlyBraces),
                ..
            }) => {
                self.next();
                return Ok(Statement::Return(None));
            }
            _ => {}
        }

        let expected = Tokens::Primary(Primary::Str(vec!["value", "variable", "enum variant"].join(" or ").to_string()));
        let value = self.parse_expressions(0, expected)?;

        // only consume semicolon; the expression already consumed everything else
        self.expect(Tokens::NonAtomic(NonAtomic::SemiColon))?;

        Ok(Statement::Return(Some(value)))
    }
}
