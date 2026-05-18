use std::collections::HashMap;

use crate::backend::{
    executor::{Value, executor, handlers::h_expressions::execute_expressions},
    nodes::{Block, Expression, Identifier, Signal},
};

pub fn execute_if(
    condition: Box<Expression>,
    body: Block,
    elif_branches: Vec<(Box<Expression>, Block)>,
    else_body: Option<Block>,
    variable_hashmap: &mut HashMap<Identifier, Value>,
) -> Signal {
    let condition_result = match execute_expressions(condition, variable_hashmap) {
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
        let signal = executor(body.statements, variable_hashmap);
        if signal != Signal::None {
            return signal;
        }
    } else if !elif_branches.is_empty() {
        for elif_block in elif_branches {
            let signal = execute_if(elif_block.0, elif_block.1, vec![], None, variable_hashmap);
            if signal != Signal::None {
                return signal;
            }
        }
    } else {
        match else_body {
            Some(statement_body) => {
                let signal = executor(statement_body.statements, variable_hashmap);
                if signal != Signal::None {
                    return signal;
                }
            }
            None => (),
        }
    }
    Signal::None
}
