use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        executor::{Value, handlers::h_call::execute_fn_call},
        nodes::{Expression, Signal},
        tokens::{Operations, Primary},
    },
};

pub fn execute_expressions(
    expression: Box<Expression>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Value {
    match *expression {
        Expression::Literal(value) => match value {
            Primary::Int(v) => Value::Int(v),
            Primary::Float(v) => Value::Float(v),
            Primary::Bool(v) => Value::Bool(v),
            Primary::Str(v) => Value::Str(v),
        },

        Expression::Unary { op, expr } => {
            let value = execute_expressions(expr, env, call_stack);

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
            let left = execute_expressions(lhs, env, call_stack);
            let right = execute_expressions(rhs, env, call_stack);
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
            let value = env.borrow().get(&key);
            let var = match value {
                Some(value) => value,
                None => {
                    panic!("Variable value doesn't exist");
                }
            };

            var.clone()
        }

        Expression::FnCall(fn_call) => {
            let value = match execute_fn_call(fn_call.callee, fn_call.args, env, call_stack) {
                Signal::Return(v) => v,
                _ => Value::Int(0),
            };

            value
        }

        // Expression::EnumCall { name, variant } => {

        // },
        Expression::ArrayLiteral(items) => {
            let values = items
                .into_iter()
                .map(|item| execute_expressions(item, env, call_stack))
                .collect();
            Value::Array(values)
        }

        Expression::ArrayIndex { target, index } => {
            let arr = match execute_expressions(target, env, call_stack) {
                Value::Array(v) => v,
                _ => panic!("Cannot index into a non-array"),
            };
            let idx = match execute_expressions(index, env, call_stack) {
                Value::Int(i) => i as usize,
                _ => panic!("Array index must be an integer"),
            };
            arr[idx].clone()
        }

        Expression::EnumCall { name, variant } => {
            let var = env.borrow().get(&name);
            let variants = match var {
                Some(value) => value,
                None => {
                    panic!("Enum value doesn't exist");
                }
            };

            let is_in_var = match variants {
                Value::Enum(v) => v.contains(&Value::Str(variant.0.clone())),
                unexpected => panic!("Expected An Enum, got a {unexpected:?}"),
            };

            if !is_in_var {
                panic!("Variant doesnt exist in Enum")
            }

            Value::Str(variant.0)
        }
    }
}
