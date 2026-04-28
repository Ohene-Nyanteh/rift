use crate::backend::{nodes::Expression, tokens::Primary};

pub fn converter(exp_literal: Box<Expression>) -> Result<Primary, ()> {
    match *exp_literal {
        Expression::Literal(value) => Ok(value),
        _ => Err(()),
    }
}
