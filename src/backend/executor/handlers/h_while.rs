use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        error_parser::Error,
        executor::{executor, is_truthy, handlers::h_expressions::execute_expressions},
        nodes::{Block, Expression, Signal},
    },
};

pub fn execute_while(
    condition: &Box<Expression>,
    body: &Block,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Result<Signal, Error> {
    loop {
        let condition_value = execute_expressions(condition, env, call_stack)?;

        if !is_truthy(&condition_value) {
            break;
        }

        let while_env = Environment::new_child(env);
        let signal = executor(&body.statements, &while_env, call_stack)?;
        match &signal {
            Signal::Break => break,
            Signal::Continue => continue,
            Signal::Return(_) => return Ok(signal),
            Signal::None => {}
        }
    }
    Ok(Signal::None)
}
