use std::collections::HashMap;

use crate::backend::{
    executor::{Value, executor, handlers::h_expressions::execute_expressions},
    nodes::{Block, Expression, Identifier, Signal},
};

pub fn execute_while(
    condition: Box<Expression>,
    body: Block,
    variable_hashmap: &mut HashMap<Identifier, Value>,
) -> Signal {
    loop {
        let condition_result = match execute_expressions(condition.clone(), variable_hashmap) {
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

        if condition_result == true {
            let signal = executor(body.statements.clone(), variable_hashmap);
            match signal {
                Signal::Break => break,
                Signal::Continue => continue,
                Signal::None => {}
            }
        } else {
            break;
        }
    }
    Signal::None
}
