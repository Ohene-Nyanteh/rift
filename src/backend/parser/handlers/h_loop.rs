use crate::backend::{
    errors::Error,
    nodes::{Block, Expression, Identifier, Statement},
    parser::Parser,
    tokens::{Keywords, NonAtomic, Primary, Token, Tokens},
};

impl Parser {
    pub fn parse_loop(&mut self) -> Result<Statement, Error> {
        // expect a variable name next
        let variable_name_token = self.next().ok_or(Error::UnexpectedEOF)?;
        let variable_name = match &variable_name_token.kind {
            Tokens::Variable(val) => Identifier(val.to_string()),
            unexpected => {
                return Err(Error::UnexpectedToken {
                    expected: Tokens::Variable(String::from("Variable Name")),
                    found: unexpected.clone(),
                });
            }
        };

        // expect the from
        self.expect(Tokens::Keyword(Keywords::From))?;

        // get the value
        let value_token = self.next().ok_or(Error::UnexpectedEOF)?;
        let value = match &value_token.kind {
            Tokens::Primary(Primary::Int(val)) => Expression::Literal(Primary::Int(*val)),
            unexpected => {
                return Err(Error::UnexpectedToken {
                    expected: Tokens::Keyword(Keywords::From),
                    found: unexpected.clone(),
                });
            }
        };

        // expect {
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

        Ok(Statement::Loop {
            variable: Box::new(Expression::Variable(variable_name)),
            body: Block { statements: body },
            value: Box::new(value),
        })
    }
}
