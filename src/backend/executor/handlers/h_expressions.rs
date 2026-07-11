use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        error_parser::Error,
        executor::{Value, handlers::h_call::execute_fn_call},
        nodes::{Expression, Signal},
        tokens::{Operations, Primary},
    },
};

pub fn execute_expressions(
    expression: &Expression,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Result<Value, Error> {
    match expression {
        Expression::Literal(value) => Ok(match value {
            Primary::Int(v) => Value::Int(*v),
            Primary::Float(v) => Value::Float(*v),
            Primary::Bool(v) => Value::Bool(*v),
            Primary::Str(v) => Value::Str(v.clone()),
        }),

        Expression::Unary { op, expr } => {
            let value = execute_expressions(expr, env, call_stack)?;

            match *op {
                Operations::Sub => match value {
                    Value::Int(v) => Ok(Value::Int(-v)),
                    Value::Float(v) => Ok(Value::Float(-v)),
                    _ => Err(Error::RuntimeError {
                        message: "Unary '-' is only supported for numbers".to_string(),
                    }),
                },

                Operations::Not => match value {
                    Value::Bool(v) => Ok(Value::Bool(!v)),
                    _ => Err(Error::RuntimeError {
                        message: "'!' only works on booleans".to_string(),
                    }),
                },

                _ => Err(Error::RuntimeError {
                    message: format!("Unsupported unary operation {:?}", op),
                }),
            }
        }
        Expression::Binary { op, lhs, rhs } => {
            let left = execute_expressions(lhs, env, call_stack)?;
            let right = execute_expressions(rhs, env, call_stack)?;

            match op {
                Operations::Add => match (&left, &right) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                    (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
                    (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
                    (Value::Str(a), Value::Int(b)) => Ok(Value::Str(format!("{}{}", a, b))),
                    (Value::Str(a), Value::Float(b)) => Ok(Value::Str(format!("{}{}", a, b))),
                    (Value::Str(a), Value::Bool(b)) => Ok(Value::Str(format!("{}{}", a, b))),
                    (Value::Str(a), Value::Str(b)) => Ok(Value::Str(format!("{}{}", a, b))),
                    _ => Err(Error::RuntimeError {
                        message: format!("'+' not supported between {:?} and {:?}", left, right),
                    }),
                },

                Operations::Sub => match (&left, &right) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                    (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
                    (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
                    _ => Err(Error::RuntimeError {
                        message: format!("'-' not supported between {:?} and {:?}", left, right),
                    }),
                },

                Operations::Mul => match (&left, &right) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                    (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
                    (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),
                    _ => Err(Error::RuntimeError {
                        message: format!("'*' not supported between {:?} and {:?}", left, right),
                    }),
                },

                Operations::Div => match (&left, &right) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
                    (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 / b)),
                    (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / *b as f64)),
                    _ => Err(Error::RuntimeError {
                        message: format!("'/' not supported between {:?} and {:?}", left, right),
                    }),
                },

                Operations::Mod => match (&left, &right) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a % b)),
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
                    (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 % b)),
                    (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a % *b as f64)),
                    _ => Err(Error::RuntimeError {
                        message: format!("'%' not supported between {:?} and {:?}", left, right),
                    }),
                },

                Operations::And => match (&left, &right) {
                    (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a && *b)),
                    _ => Err(Error::RuntimeError {
                        message: "'&' only works on booleans".to_string(),
                    }),
                },

                Operations::Or => match (&left, &right) {
                    (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a || *b)),
                    _ => Err(Error::RuntimeError {
                        message: "'|' only works on booleans".to_string(),
                    }),
                },

                Operations::EqualTo => Ok(Value::Bool(left == right)),
                Operations::NotEqualTo => Ok(Value::Bool(left != right)),

                Operations::GreaterThan => match (&left, &right) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
                    _ => Err(Error::RuntimeError {
                        message: format!("'>' not supported between {:?} and {:?}", left, right),
                    }),
                },

                Operations::LessThan => match (&left, &right) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
                    _ => Err(Error::RuntimeError {
                        message: format!("'<' not supported between {:?} and {:?}", left, right),
                    }),
                },

                Operations::LessOrEquals => match (&left, &right) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
                    _ => Err(Error::RuntimeError {
                        message: format!("'<=' not supported between {:?} and {:?}", left, right),
                    }),
                },

                Operations::GreaterOrEquals => match (&left, &right) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
                    _ => Err(Error::RuntimeError {
                        message: format!("'>=' not supported between {:?} and {:?}", left, right),
                    }),
                },

                unexpected => Err(Error::RuntimeError {
                    message: format!("Unexpected binary operation {:?}", unexpected),
                }),
            }
        }

        Expression::Variable(key) => {
            let value = env.borrow().get(key);
            match value {
                Some(val) => Ok(val.clone()),
                None => Err(Error::RuntimeError {
                    message: format!("Variable '{}' is not defined", key.0),
                }),
            }
        }

        Expression::FnCall(fn_call) => {
            let signal =
                execute_fn_call(&fn_call.callee, &fn_call.args, env, call_stack)?;
            match signal {
                Signal::Return(v) => Ok(v),
                _ => Ok(Value::Int(0)),
            }
        }

        Expression::ArrayLiteral(items) => {
            let values = items
                .iter()
                .map(|item| execute_expressions(item, env, call_stack))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Value::Array(Rc::new(values)))
        }

        Expression::ArrayIndex { target, index } => {
            let arr = match execute_expressions(target, env, call_stack)? {
                Value::Array(v) => v,
                other => {
                    return Err(Error::RuntimeError {
                        message: format!("Cannot index into a non-array value: {:?}", other),
                    })
                }
            };

            let idx = match execute_expressions(index, env, call_stack)? {
                Value::Int(i) => i as usize,
                other => {
                    return Err(Error::RuntimeError {
                        message: format!("Array index must be an integer, got {:?}", other),
                    })
                }
            };

            match arr.get(idx) {
                Some(val) => Ok(val.clone()),
                None => Err(Error::RuntimeError {
                    message: format!("Index {} out of bounds for array of length {}", idx, arr.len()),
                }),
            }
        }

        Expression::EnumCall { name, variant } => {
            let value = env.borrow().get(name);
            let variants = match value {
                Some(val) => val,
                None => return Err(Error::RuntimeError {
                    message: format!("Variable '{}' is not defined", name.0),
                }),
            };

            match variants {
                Value::Enum(v) => {
                    if v.contains(&Value::Str(variant.0.clone())) {
                        Ok(Value::Str(variant.0.clone()))
                    } else {
                        Err(Error::RuntimeError {
                            message: format!("Variant '{}' does not exist in enum '{}'", variant.0, name.0),
                        })
                    }
                }
                unexpected => Err(Error::RuntimeError {
                    message: format!("'{}' is not an enum, got {:?}", name.0, unexpected),
                }),
            }
        }

        Expression::StructCall { target, field } => {
            let struct_value = env
                .borrow()
                .get(target)
                .ok_or_else(|| Error::RuntimeError {
                    message: format!("Variable '{}' is not defined", target.0),
                })?;

            match struct_value {
                Value::Struct(fields) => match fields.get(&field.0) {
                    Some(val) => Ok(val.clone()),
                    None => Err(Error::RuntimeError {
                        message: format!("Struct '{}' has no field '{}'", target.0, field.0),
                    }),
                },
                unexpected => Err(Error::RuntimeError {
                    message: format!("'{}' is not a struct, got {:?}", target.0, unexpected),
                }),
            }
        }

        Expression::StructAssignment {
            target,
            field,
            new_value,
        } => {
            let struct_variable = env
                .borrow()
                .get(target)
                .ok_or_else(|| Error::RuntimeError {
                    message: format!("Variable '{}' is not defined", target.0),
                })?;

            match struct_variable {
                Value::Struct(mut fields) => {
                    if !Rc::make_mut(&mut fields).contains_key(&field.0) {
                        return Err(Error::RuntimeError {
                            message: format!("Struct '{}' has no field '{}'", target.0, field.0),
                        });
                    }
                    let value = execute_expressions(new_value, env, call_stack)?;
                    Rc::make_mut(&mut fields).insert(field.0.clone(), value);
                    env.borrow_mut().set(target, Value::Struct(fields));
                    Ok(Value::Int(0))
                }
                unexpected => Err(Error::RuntimeError {
                    message: format!("'{}' is not a struct, got {:?}", target.0, unexpected),
                }),
            }
        }
    }
}
