use crate::backend::{
    error_parser::Error,
    nodes::{Block, Expression, Identifier, Statement},
    parser::{Parser, col_for},
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
                    error_line: self.line_text(variable_name_token.span.row),
                    col_start: col_for(variable_name_token.span.start, variable_name_token.span.row, &self.line_starts),
                    col_end: col_for(variable_name_token.span.end, variable_name_token.span.row, &self.line_starts),
                    found: unexpected.clone(),
                    at: variable_name_token.span
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
                    error_line: self.line_text(value_token.span.row),
                    col_start: col_for(value_token.span.start, value_token.span.row, &self.line_starts),
                    col_end: col_for(value_token.span.end, value_token.span.row, &self.line_starts),
                    found: unexpected.clone(),
                    at: value_token.span
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
