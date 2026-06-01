use crate::backend::{
    errors::Error,
    nodes::{Expression, Identifier},
    parser::Parser,
    tokens::{NonAtomic, Operations, Primary, Tokens},
};

impl Parser {
    pub fn parse_expressions(&mut self, min_bp: u8) -> Result<Box<Expression>, Error> {
        let next_token = self.peek().ok_or(Error::UnexpectedEOF)?;

        let mut lhs: Box<Expression> = match next_token.kind.clone() {
            Tokens::Primary(Primary::Int(val)) => {
                self.next();
                Box::new(Expression::Literal(Primary::Int(val)))
            }
            Tokens::Primary(Primary::Float(val)) => {
                self.next();
                Box::new(Expression::Literal(Primary::Float(val)))
            }
            Tokens::Primary(Primary::Bool(val)) => {
                self.next();
                Box::new(Expression::Literal(Primary::Bool(val)))
            }
            Tokens::Primary(Primary::Str(val)) => {
                self.next();
                Box::new(Expression::Literal(Primary::Str(val)))
            }
            Tokens::NonAtomic(NonAtomic::LSquareBraces) => {
                self.next();
                let array = self.handle_arrays()?;
                Box::new(array)
            }
            Tokens::Variable(v) => {
                self.next();

                // check if it's a function call: name(...)
                match self.peek() {
                    Some(t) if t.kind == Tokens::NonAtomic(NonAtomic::LParen) => {
                        // parse as a call expression (no semicolon consumed)
                        let call_expr = self.parse_call_expr(Identifier(v))?;
                        Box::new(call_expr)
                    }
                    Some(t) if t.kind == Tokens::NonAtomic(NonAtomic::LSquareBraces) => {
                        // parse the array index with no semi colon
                        let array_index_exp = self.parse_array_index_expr(Identifier(v))?;
                        Box::new(array_index_exp)
                    }
                    Some(t) if t.kind == Tokens::NonAtomic(NonAtomic::Colon) => {
                        // parse the enums
                        let enum_exp = self.parse_enum_calls(Identifier(v))?;
                        Box::new(enum_exp)
                    }
                    Some(t) if t.kind == Tokens::NonAtomic(NonAtomic::Dot) => {
                        let struct_exp = self.parse_struct_call(Identifier(v))?;
                        struct_exp
                    }
                    _ => {
                        // plain variable reference
                        Box::new(Expression::Variable(Identifier(v)))
                    }
                }
            }
            // unary: -x or !x
            Tokens::Atomic(Operations::Sub) => {
                let expr = self.parse_expressions(7)?; // higher than everything else
                Box::new(Expression::Unary {
                    op: Operations::Sub,
                    expr,
                })
            }
            Tokens::Atomic(Operations::Not) => {
                let expr = self.parse_expressions(7)?;
                Box::new(Expression::Unary {
                    op: Operations::Not,
                    expr,
                })
            }

            // grouped expression: (2 + 3)
            Tokens::NonAtomic(NonAtomic::LParen) => {
                let inner = self.parse_expressions(0)?;
                self.expect_rparen()?; // consume the closing )
                inner
            }

            t => {
                return Err(Error::UnexpectedToken {
                    expected: Tokens::NonAtomic(NonAtomic::RParen),
                    found: t,
                });
            }
        };

        loop {
            // peek — if no token or EOF, stop
            let op_token = match self.peek() {
                Some(t) => t.clone(),
                None => break,
            };

            let op = match op_token.kind {
                Tokens::Atomic(operation) => operation,
                _ => break, // not an operator (e.g. RParen, EOF, semicolon) — stop
            };

            let (lbp, rbp) = match infix_binding_power(&op) {
                Some(bp) => bp,
                None => break, // operator not valid in infix position
            };

            if lbp <= min_bp {
                break;
            }

            self.next(); // consume operator
            let rhs = self.parse_expressions(rbp)?;
            lhs = Box::new(Expression::Binary { op, lhs, rhs });
        }

        Ok(lhs)
    }

    fn expect_rparen(&mut self) -> Result<(), Error> {
        match self.next() {
            Some(t) if t.kind == Tokens::NonAtomic(NonAtomic::RParen) => Ok(()),
            Some(t) => Err(Error::UnexpectedToken {
                expected: Tokens::NonAtomic(NonAtomic::RParen),
                found: t.kind,
            }),
            None => Err(Error::UnexpectedEOF),
        }
    }
}

fn infix_binding_power(op: &Operations) -> Option<(u8, u8)> {
    match op {
        // logical — lowest precedence
        Operations::Or => Some((1, 2)),
        // Operations::Nor => Some((1, 2)),
        // Operations::Xor => Some((1, 2)),
        Operations::And => Some((3, 4)),

        // comparison — middle
        Operations::EqualTo | Operations::NotEqualTo => Some((5, 6)),
        Operations::GreaterThan
        | Operations::LessThan
        | Operations::GreaterOrEquals
        | Operations::LessOrEquals => Some((5, 6)),

        // arithmetic — highest
        Operations::Add | Operations::Sub => Some((7, 8)),
        Operations::Mul | Operations::Div | Operations::Mod => Some((9, 10)),

        // Not/Sub handled as unary in the nud, not here
        _ => None,
    }
}
