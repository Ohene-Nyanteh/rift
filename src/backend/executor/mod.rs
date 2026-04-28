use std::collections::HashMap;

use crate::backend::{
    executor::handlers::{
        h_expressions::execute_expressions,
        h_if::execute_if,
        h_print::execute_print,
        h_variables::{execute_update_variable, execute_variables},
        h_while::execute_while,
    },
    nodes::{Identifier, Statement},
};

pub mod handlers;
pub mod helpers;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Char(char),
}

pub fn executor(ast: Vec<Statement>, variable_hashmap: &mut HashMap<Identifier, Value>) {
    for statement in ast {
        match statement {
            Statement::Print(exp) => {
                let value = execute_expressions(exp, variable_hashmap);
                execute_print(value);
            }
            Statement::Let(let_decl) => {
                execute_variables(let_decl, variable_hashmap);
            }
            Statement::Expression(exp) => {
                execute_expressions(exp, variable_hashmap);
            }
            Statement::If {
                body,
                condition,
                elif_branches,
                else_body,
            } => execute_if(condition, body, elif_branches, else_body, variable_hashmap),
            Statement::While { condition, body } => {
                execute_while(condition, body, variable_hashmap);
            }
            Statement::VariableAssignment { var, exp } => {
                execute_update_variable(var, exp, variable_hashmap);
            }
            _ => {
                print!("Error!");
            }
        }
    }
}
