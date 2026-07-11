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

pub fn execute_if(
    condition: &Box<Expression>,
    body: &Block,
    elif_branches: &Vec<(Box<Expression>, Block)>,
    else_body: &Option<Block>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Result<Signal, Error> {
    let condition_value = execute_expressions(condition, env, call_stack)?;

    if is_truthy(&condition_value) {
        let if_env = Environment::new_child(env);
        let signal = executor(&body.statements, &if_env, call_stack)?;
        if signal != Signal::None {
            return Ok(signal);
        }
        return Ok(Signal::None);
    }

    // check elif branches
    for (elif_cond, elif_body) in elif_branches {
        let elif_value = execute_expressions(elif_cond, env, call_stack)?;
        if is_truthy(&elif_value) {
            let elif_env = Environment::new_child(env);
            let signal = executor(&elif_body.statements, &elif_env, call_stack)?;
            if signal != Signal::None {
                return Ok(signal);
            }
            return Ok(Signal::None);
        }
    }

    // fall through to else
    if let Some(else_body) = else_body {
        let else_env = Environment::new_child(env);
        let signal = executor(&else_body.statements, &else_env, call_stack)?;
        if signal != Signal::None {
            return Ok(signal);
        }
    }

    Ok(Signal::None)
}
