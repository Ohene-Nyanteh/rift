use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        executor::{Value, executor, handlers::h_expressions::execute_expressions},
        nodes::{Block, Expression, Signal},
    },
};

pub fn execute_if(
    condition: Box<Expression>,
    body: Block,
    elif_branches: Vec<(Box<Expression>, Block)>,
    else_body: Option<Block>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Signal {
    let condition_result = match execute_expressions(condition, env, call_stack) {
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

        Value::Array(v) => {
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
        let signal = executor(body.statements, &mut if_env, call_stack);
        if signal != Signal::None {
            return signal;
        }
    } else if !elif_branches.is_empty() {
        for elif_block in elif_branches {
            let condition_result = match execute_expressions(elif_block.0, env, call_stack) {
                Value::Bool(v) => v,
                Value::Float(v) => v > 0.0,
                Value::Int(v) => v > 0,
                Value::Str(v) => v.len() != 0,
                Value::Array(v) => v.len() != 0,
            };

            if condition_result {
                let mut elif_env = Environment::new_child(env);
                let signal = executor(elif_block.1.statements, &mut elif_env, call_stack);
                if signal != Signal::None {
                    return signal;
                }
                return Signal::None; // matched and ran — stop here
            }
        }

        // no elif matched, run else if present
        if let Some(statement_body) = else_body {
            let mut else_env = Environment::new_child(env);
            let signal = executor(statement_body.statements, &mut else_env, call_stack);
            if signal != Signal::None {
                return signal;
            }
        }
    } else {
        match else_body {
            Some(statement_body) => {
                let mut else_env = Environment::new_child(env);
                let signal = executor(statement_body.statements, &mut else_env, call_stack);
                if signal != Signal::None {
                    return signal;
                }
            }
            None => (),
        }
    }
    Signal::None
}
