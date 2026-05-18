use std::{cell::RefCell, rc::Rc};

use crate::backend::{
    environment::Environment,
    executor::Value,
    nodes::Expression,
    tokens::{Operations, Primary},
};

pub fn execute_expressions(expression: Box<Expression>, env: &Rc<RefCell<Environment>>) -> Value {
    match *expression {
        Expression::Literal(value) => match value {
            Primary::Int(v) => Value::Int(v),
            Primary::Float(v) => Value::Float(v),
            Primary::Bool(v) => Value::Bool(v),
            Primary::Str(v) => Value::Str(v),
        },

        Expression::Unary { op, expr } => {
            let value = execute_expressions(expr, env);

            match op {
                Operations::Sub => match value {
                    Value::Int(v) => Value::Int(-v),
                    Value::Float(v) => Value::Float(-v),
                    _ => panic!("Unary '-' not supported for this type"),
                },

                Operations::Not => match value {
                    Value::Bool(v) => Value::Bool(!v),
                    _ => panic!("Unary 'not' only works on bool"),
                },

                _ => panic!("Unsupported unary operation"),
            }
        }
        Expression::Binary { op, lhs, rhs } => {
            let left = execute_expressions(lhs, env);
            let right = execute_expressions(rhs, env);
            match op {
                Operations::Add => match (left, right) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                    (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
                    (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 + b),
                    (Value::Float(a), Value::Int(b)) => Value::Float(a + b as f64),
                    (Value::Str(a), Value::Int(b)) => Value::Str(a + &b.to_string()),
                    (Value::Str(a), Value::Float(b)) => Value::Str(a + &b.to_string()),
                    (Value::Str(a), Value::Bool(b)) => Value::Str(a + &b.to_string()),
                    (Value::Str(a), Value::Str(b)) => Value::Str(a + &b),

                    _ => panic!("Invalid types for +"),
                },

                Operations::Sub => match (left, right) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a - b),
                    (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
                    (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 - b),
                    (Value::Float(a), Value::Int(b)) => Value::Float(a - b as f64),
                    _ => panic!("Invalid types for -"),
                },

                Operations::Mul => match (left, right) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                    (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
                    (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 * b),
                    (Value::Float(a), Value::Int(b)) => Value::Float(a * b as f64),
                    _ => panic!("Invalid types for *"),
                },

                Operations::Div => match (left, right) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
                    (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
                    (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 / b),
                    (Value::Float(a), Value::Int(b)) => Value::Float(a / b as f64),
                    _ => panic!("Invalid types for /"),
                },

                Operations::Mod => match (left, right) {
                    (Value::Int(a), Value::Int(b)) => Value::Int(a % b),
                    (Value::Float(a), Value::Float(b)) => Value::Float(a % b),
                    (Value::Int(a), Value::Float(b)) => Value::Float(a as f64 % b),
                    (Value::Float(a), Value::Int(b)) => Value::Float(a % b as f64),
                    _ => panic!("Invalid types for /"),
                },

                Operations::And => match (left, right) {
                    (Value::Bool(a), Value::Bool(b)) => Value::Bool(a && b),
                    _ => panic!("AND only works on bool"),
                },

                Operations::Or => match (left, right) {
                    (Value::Bool(a), Value::Bool(b)) => Value::Bool(a || b),
                    _ => panic!("OR only works on bool"),
                },

                Operations::EqualTo => Value::Bool(left == right),
                Operations::NotEqualTo => Value::Bool(left != right),

                Operations::GreaterThan => match (left, right) {
                    (Value::Int(a), Value::Int(b)) => Value::Bool(a > b),
                    (Value::Float(a), Value::Float(b)) => Value::Bool(a > b),
                    _ => panic!("Invalid comparison"),
                },

                Operations::LessThan => match (left, right) {
                    (Value::Int(a), Value::Int(b)) => Value::Bool(a < b),
                    (Value::Float(a), Value::Float(b)) => Value::Bool(a < b),
                    _ => panic!("Invalid comparison"),
                },

                Operations::LessOrEquals => match (left, right) {
                    (Value::Int(a), Value::Int(b)) => Value::Bool(a <= b),
                    (Value::Float(a), Value::Float(b)) => Value::Bool(a <= b),
                    _ => panic!("Invalid comparison"),
                },

                Operations::GreaterOrEquals => match (left, right) {
                    (Value::Int(a), Value::Int(b)) => Value::Bool(a >= b),
                    (Value::Float(a), Value::Float(b)) => Value::Bool(a >= b),
                    _ => panic!("Invalid comparison"),
                },

                _ => panic!("Operation not implemented yet"),
            }
        }

        Expression::Variable(key) => {
            let value = env.borrow_mut().get(&key);
            let var = match value {
                Some(value) => value,
                None => {
                    panic!("Variable value doesn't exist");
                }
            };

            var.clone()
        }

        _ => panic!("Expression type not supported yet"),
    }
}
