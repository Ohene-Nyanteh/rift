use std::{cell::RefCell, rc::Rc};

use crate::backend::{
    environment::Environment,
    executor::{Value, executor, handlers::h_expressions::execute_expressions},
    nodes::{Block, Expression, Signal},
};

pub fn execute_if(
    condition: Box<Expression>,
    body: Block,
    elif_branches: Vec<(Box<Expression>, Block)>,
    else_body: Option<Block>,
    env: &Rc<RefCell<Environment>>,
) -> Signal {
    let condition_result = match execute_expressions(condition, env) {
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

    // handle condition
    if condition_result {
        let mut if_env = Environment::new_child(env);
        let signal = executor(body.statements, &mut if_env);
        if signal != Signal::None {
            return signal;
        }
    } else if !elif_branches.is_empty() {
        for elif_block in elif_branches {
            let mut elif_env = Environment::new_child(env);
            let signal = execute_if(elif_block.0, elif_block.1, vec![], None, &mut elif_env);
            if signal != Signal::None {
                return signal;
            }
        }
    } else {
        match else_body {
            Some(statement_body) => {
                let mut else_env = Environment::new_child(env);
                let signal = executor(statement_body.statements, &mut else_env);
                if signal != Signal::None {
                    return signal;
                }
            }
            None => (),
        }
    }
    Signal::None
}
