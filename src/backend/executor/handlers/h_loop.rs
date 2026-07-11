use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        error_parser::Error,
        executor::{executor, Value, handlers::h_expressions::execute_expressions},
        nodes::{Block, Expression, Signal},
    },
};

pub fn execute_loop(
    variable: &Box<Expression>,
    body: &Block,
    value: &Box<Expression>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Result<Signal, Error> {
    let start_value = match execute_expressions(value, env, call_stack)? {
        Value::Int(v) => v,
        _ => {
            return Err(Error::RuntimeError {
                message: "'loop from' requires an integer start value".to_string(),
            })
        }
    };

    let variable_name = match variable.as_ref() {
        Expression::Variable(v) => v,
        unexpected => {
            return Err(Error::RuntimeError {
                message: format!("Expected variable name in 'loop from', got {:?}", unexpected),
            })
        }
    };

    env.borrow_mut()
        .define(variable_name.clone(), Value::Int(start_value));

    loop {
        let loop_env = Environment::new_child(env);
        let signal = executor(&body.statements, &loop_env, call_stack)?;

        if signal == Signal::Break {
            break;
        }
        // continue is handled by the inner executor returning to us

        let current = match env.borrow().get(variable_name) {
            Some(Value::Int(v)) => v,
            _ => break,
        };

        env.borrow_mut()
            .define(variable_name.clone(), Value::Int(current + 1));
    }

    Ok(Signal::None)
}
