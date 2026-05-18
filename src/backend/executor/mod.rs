use std::collections::HashMap;

use crate::backend::{
    executor::handlers::{
        h_expressions::execute_expressions,
        h_if::execute_if,
        h_loop::execute_loop,
        h_print::execute_print,
        h_variables::{execute_update_variable, execute_variables},
        h_while::execute_while,
    },
    nodes::{Identifier, Signal, Statement},
};

pub mod handlers;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

pub fn executor(ast: Vec<Statement>, variable_hashmap: &mut HashMap<Identifier, Value>) -> Signal {
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
            } => {
                let signal =
                    execute_if(condition, body, elif_branches, else_body, variable_hashmap);
                if signal != Signal::None {
                    return signal;
                }
            }
            Statement::While { condition, body } => {
                execute_while(condition, body, variable_hashmap);
            }
            Statement::VariableAssignment { var, exp } => {
                execute_update_variable(var, exp, variable_hashmap);
            }
            Statement::Loop {
                variable,
                body,
                value,
            } => {
                execute_loop(variable, body, value, variable_hashmap);
            }
            Statement::Break => return Signal::Break,
            Statement::Continue => return Signal::Continue,
            _ => {
                print!("Error!");
            }
        }
    }
    Signal::None
}
