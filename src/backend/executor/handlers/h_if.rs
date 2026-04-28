use std::collections::HashMap;

use crate::backend::{
    executor::{Value, executor, handlers::h_expressions::execute_expressions},
    nodes::{Block, Expression, Identifier},
};

pub fn execute_if(
    condition: Box<Expression>,
    body: Block,
    elif_branches: Vec<(Box<Expression>, Block)>,
    else_body: Option<Block>,
    variable_hashmap: &mut HashMap<Identifier, Value>,
) {
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
        _ => true,
    };

    // handle condition
    if condition_result {
        executor(body.statements, variable_hashmap)
    } else if !elif_branches.is_empty() {
        for elif_block in elif_branches {
            execute_if(elif_block.0, elif_block.1, vec![], None, variable_hashmap);
        }
    } else {
        match else_body {
            Some(statement_body) => executor(statement_body.statements, variable_hashmap),
            None => (),
        }
    }
}
