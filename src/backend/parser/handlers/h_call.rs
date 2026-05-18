use crate::backend::{
    errors::Error,
    nodes::{Call, Expression, Identifier, Statement},
    parser::Parser,
    tokens::{NonAtomic, Tokens},
};

impl Parser {
    pub fn parse_function_call(&mut self, fn_name: Identifier) -> Result<Statement, Error> {
        // expect a parenthesis next
        self.expect(Tokens::NonAtomic(NonAtomic::LParen))?;

        // run a simple loop to store all the args passed until it sees a )
        let mut args: Vec<Expression> = vec![];
        loop {
            let next_token = self.next().ok_or(Error::UnexpectedEOF)?;
            match next_token.kind {
                Tokens::NonAtomic(NonAtomic::RParen) => break,
                Tokens::Variable(var) => args.push(Expression::Variable(Identifier(var))),
                Tokens::NonAtomic(NonAtomic::Comma) => continue,
                Tokens::Primary(value) => args.push(Expression::Literal(value)),
                unexpected => {
                    return Err(Error::InvalidSyntax(
                        format!("Expected a variable, or a value, got {:?}", unexpected)
                            .to_string(),
                    ));
                }
            }
        }

        // expect semicolon
        self.expect(Tokens::NonAtomic(NonAtomic::SemiColon))?;

        Ok(Statement::FnCall(Box::new(Call {
            callee: fn_name,
            args: args,
        })))
    }
}
