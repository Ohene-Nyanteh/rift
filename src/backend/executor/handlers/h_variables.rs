use std::collections::HashMap;

use crate::backend::{
    executor::{Value, handlers::h_expressions::execute_expressions},
    nodes::{Expression, Identifier, LetDecl},
};

pub fn execute_variables(
    let_decl: Box<LetDecl>,
    variable_hashmap: &mut HashMap<Identifier, Value>,
) {
    let declaration = execute_expressions(
        let_decl.value.expect("Error getting declation"),
        variable_hashmap,
    );

    match declaration {
        Value::Bool(v) => {
            variable_hashmap.insert(let_decl.name, Value::Bool(v));
        }
        Value::Int(v) => {
            variable_hashmap.insert(let_decl.name, Value::Int(v));
        }
        Value::Float(v) => {
            variable_hashmap.insert(let_decl.name, Value::Float(v));
        }
        Value::Str(v) => {
            variable_hashmap.insert(let_decl.name, Value::Str(v));
        }
        Value::Char(v) => {
            variable_hashmap.insert(let_decl.name, Value::Char(v));
        }
        _ => {
            println!("Error parsing var")
        }
    }
}

pub fn execute_update_variable(
    var: Identifier,
    exp: Box<Expression>,
    variable_hashmap: &mut HashMap<Identifier, Value>,
) {
    let exp_result = execute_expressions(exp, variable_hashmap);

    match variable_hashmap.contains_key(&var) {
        true => {
            // update the variable
            variable_hashmap.insert(var, exp_result);
        }
        false => {
            let n = var.0;
            println!("Error: Variable {n} not declared!");
        }
    }
}
