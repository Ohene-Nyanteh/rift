use std::collections::HashMap;

use crate::backend::{
    executor::{Value, executor, handlers::h_expressions::execute_expressions},
    nodes::{Block, Expression, Identifier, Signal},
};

pub fn execute_loop(
    variable: Box<Expression>,
    body: Block,
    value: Box<Expression>,
    variable_hashmap: &mut HashMap<Identifier, Value>,
) {
    let iter_value = match execute_expressions(value, variable_hashmap) {
        Value::Int(v) => v,
        Value::Bool(_) => 0,
        Value::Str(_) => 0,
        Value::Float(_) => 0,
    };

    // retrieve variable name
    let variable_name = match *variable {
        Expression::Variable(v) => v,
        unexpected => {
            panic!("Unexpected variable name type, expected a variable name, got {unexpected:?}")
        }
    };

    // store variable in hashmap
    variable_hashmap.insert(variable_name.clone(), Value::Int(iter_value));

    loop {
        let signal = executor(body.statements.clone(), variable_hashmap);
        if signal == Signal::Break {
            break;
        }
        // increment counter
        if let Some(value) = variable_hashmap.get_mut(&variable_name) {
            if let Value::Int(v) = value {
                *v += 1;
            }
        }
    }
}
