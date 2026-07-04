use crate::backend::{
    error_parser::Error,
    nodes::{Call, Expression, Identifier, Statement},
    parser::Parser,
    tokens::{NonAtomic, Primary, Tokens},
};

// fn_call.rs
impl Parser {
    /// Parses a function call as an Expression (no trailing semicolon).
    /// Used when a call appears inside an expression: return foo(x) + 1
    pub fn parse_call_expr(&mut self, fn_name: Identifier) -> Result<Expression, Error> {
        self.expect(Tokens::NonAtomic(NonAtomic::LParen))?;

        let mut args: Vec<Expression> = vec![];
        loop {
            match self.peek().ok_or(Error::UnexpectedEOF)?.kind.clone() {
                Tokens::NonAtomic(NonAtomic::RParen) => {
                    self.next(); // consume )
                    break;
                }
                Tokens::NonAtomic(NonAtomic::Comma) => {
                    self.next(); // consume ,
                    continue;
                }
                _ => {
                    // parse a full expression as the argument (handles n-1, n+2, etc.)
                    let expected = Tokens::Primary(Primary::Str("args".to_string()));
                    let arg = self.parse_expressions(0, expected)?;
                    args.push(*arg);
                }
            }
        }

        Ok(Expression::FnCall(Box::new(Call {
            callee: fn_name,
            args,
        })))
    }

    /// Parses a standalone function call statement (consumes trailing semicolon).
    pub fn parse_function_call(&mut self, fn_name: Identifier) -> Result<Statement, Error> {
        let expr = self.parse_call_expr(fn_name)?;
        self.expect(Tokens::NonAtomic(NonAtomic::SemiColon))?;
        match expr {
            Expression::FnCall(call) => Ok(Statement::FnCall(call)),
            _ => unreachable!(),
        }
    }
}
