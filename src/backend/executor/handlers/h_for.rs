use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        error_parser::Error,
        executor::{executor, Value, handlers::h_expressions::execute_expressions},
        nodes::{Block, Expression, Identifier, Signal},
    },
};

pub fn execute_for(
    var: &Identifier,
    iterable: &Box<Expression>,
    body: &Block,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Result<Signal, Error> {
    let child_env = Environment::new_child(env);
    let array = execute_expressions(iterable, &child_env, call_stack)?;

    let items = match &array {
        Value::Array(v) => Rc::clone(v),
        _ => {
            return Err(Error::RuntimeError {
                message: format!("for-in requires an array, got {:?}", array),
                })
        }
    };

    for value in items.iter() {
        child_env.borrow_mut().define(var.clone(), value.clone());
        let signal = executor(&body.statements, &child_env, call_stack)?;
        match &signal {
            Signal::Break => break,
            Signal::Continue => continue,
            Signal::Return(_) => return Ok(signal),
            Signal::None => {}
        }
    }

    Ok(Signal::None)
}
