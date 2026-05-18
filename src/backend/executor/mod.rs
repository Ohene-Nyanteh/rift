use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        executor::handlers::{
            h_call::execute_fn_call,
            h_expressions::execute_expressions,
            h_fn::execute_fn,
            h_if::execute_if,
            h_loop::execute_loop,
            h_print::execute_print,
            h_variables::{execute_update_variable, execute_variables},
            h_while::execute_while,
        },
        nodes::{Signal, Statement},
    },
};

pub mod handlers;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

pub fn executor(
    ast: Vec<Statement>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Signal {
    for statement in ast {
        match statement {
            Statement::Print(exp) => {
                let value = execute_expressions(exp, env, call_stack);
                execute_print(value);
            }
            Statement::Let(let_decl) => {
                execute_variables(let_decl, env, call_stack);
            }
            Statement::Expression(exp) => {
                execute_expressions(exp, env, call_stack);
            }
            Statement::If {
                body,
                condition,
                elif_branches,
                else_body,
            } => {
                let signal = execute_if(condition, body, elif_branches, else_body, env, call_stack);
                if signal != Signal::None {
                    return signal;
                }
            }
            Statement::While { condition, body } => {
                execute_while(condition, body, env, call_stack);
            }
            Statement::VariableAssignment { var, exp } => {
                execute_update_variable(var, exp, env, call_stack);
            }
            Statement::Loop {
                variable,
                body,
                value,
            } => {
                execute_loop(variable, body, value, env, call_stack);
            }
            Statement::Function(fn_decl) => {
                execute_fn(fn_decl, env);
            }
            Statement::FnCall(fn_call) => {
                let signal = execute_fn_call(fn_call.callee, fn_call.args, env, call_stack);
                if signal != Signal::None {
                    return signal;
                }
            }
            Statement::Return(exp) => match exp {
                Some(expression) => {
                    let value = execute_expressions(expression, env, call_stack);
                    return Signal::Return(value);
                }
                None => return Signal::Return(Value::Int(0)),
            },
            Statement::Break => return Signal::Break,
            Statement::Continue => return Signal::Continue,
            _ => {
                print!("Error!");
            }
        }
    }
    Signal::None
}
