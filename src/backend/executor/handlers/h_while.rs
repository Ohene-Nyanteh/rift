use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        executor::{Value, executor, handlers::h_expressions::execute_expressions},
        nodes::{Block, Expression, Signal},
    },
};

pub fn execute_while(
    condition: Box<Expression>,
    body: Block,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Signal {
    loop {
        let condition_result = match execute_expressions(condition.clone(), env, call_stack) {
            Value::Bool(v) => v,
            Value::Float(v) => {
                if v > 0.0 {
                    true
                } else {
                    false
                }
            }
            Value::Int(v) => {
                if v > 0 {
                    true
                } else {
                    false
                }
            }
            Value::Str(v) => {
                if v.len() != 0 {
                    true
                } else {
                    false
                }
            }
        };

        if !condition_result {
            break;
        }
        let mut while_env = Environment::new_child(env);
        let signal = executor(body.statements.clone(), &mut while_env, call_stack);
        match &signal {
            Signal::Break => break,
            Signal::Continue => continue,
            Signal::Return(_) => return signal,
            Signal::None => {}
        }
    }
    Signal::None
}
