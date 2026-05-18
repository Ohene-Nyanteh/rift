use std::{cell::RefCell, rc::Rc};

use crate::backend::{
    environment::Environment,
    executor::handlers::{
        h_expressions::execute_expressions,
        h_if::execute_if,
        h_loop::execute_loop,
        h_print::execute_print,
        h_variables::{execute_update_variable, execute_variables},
        h_while::execute_while,
    },
    nodes::{Signal, Statement},
};

pub mod handlers;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

pub fn executor(ast: Vec<Statement>, env: &Rc<RefCell<Environment>>) -> Signal {
    for statement in ast {
        match statement {
            Statement::Print(exp) => {
                let value = execute_expressions(exp, env);
                execute_print(value);
            }
            Statement::Let(let_decl) => {
                execute_variables(let_decl, env);
            }
            Statement::Expression(exp) => {
                execute_expressions(exp, env);
            }
            Statement::If {
                body,
                condition,
                elif_branches,
                else_body,
            } => {
                let signal = execute_if(condition, body, elif_branches, else_body, env);
                if signal != Signal::None {
                    return signal;
                }
            }
            Statement::While { condition, body } => {
                execute_while(condition, body, env);
            }
            Statement::VariableAssignment { var, exp } => {
                execute_update_variable(var, exp, env);
            }
            Statement::Loop {
                variable,
                body,
                value,
            } => {
                execute_loop(variable, body, value, env);
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
