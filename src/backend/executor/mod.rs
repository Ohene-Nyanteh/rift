use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        executor::handlers::{
            h_call::execute_fn_call,
            h_enums::execute_enums,
            h_expressions::execute_expressions,
            h_fn::execute_fn,
            h_for::execute_for,
            h_if::execute_if,
            h_loop::execute_loop,
            h_match::execute_match,
            h_print::execute_print,
            h_struct::execute_struct,
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
    Array(Rc<Vec<Value>>),
    Enum(Rc<Vec<Value>>), // Str(Variant)
    Struct(Rc<HashMap<String, Value>>),
}

use crate::backend::error_parser::Error;

/// Determines the truthiness of a Rift value in a conditional context.
pub fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Bool(v) => *v,
        Value::Int(v) => *v > 0,
        Value::Float(v) => *v > 0.0,
        Value::Str(v) => !v.is_empty(),
        Value::Array(v) => !v.is_empty(),
        Value::Struct(_) | Value::Enum(_) => true,
    }
}

pub fn executor(
    ast: &Vec<Statement>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Result<Signal, Error> {
    for statement in ast {
        match statement {
            Statement::Print(exp) => {
                let value = execute_expressions(exp, env, call_stack)?;
                execute_print(&value);
            }
            Statement::Let(let_decl) => {
                execute_variables(let_decl, env, call_stack)?;
            }
            Statement::Expression(exp) => {
                execute_expressions(exp, env, call_stack)?;
            }
            Statement::If {
                body,
                condition,
                elif_branches,
                else_body,
            } => {
                let signal = execute_if(condition, body, elif_branches, else_body, env, call_stack)?;
                if signal != Signal::None {
                    return Ok(signal);
                }
            }
            Statement::While { condition, body } => {
                let signal = execute_while(condition, body, env, call_stack)?;
                if signal != Signal::None {
                    return Ok(signal);
                }
            }
            Statement::VariableAssignment { var, exp } => {
                execute_update_variable(var, exp, env, call_stack)?;
            }
            Statement::Loop {
                variable,
                body,
                value,
            } => {
                let signal = execute_loop(variable, body, value, env, call_stack)?;
                if signal != Signal::None {
                    return Ok(signal);
                }
            }
            Statement::Function(fn_decl) => {
                execute_fn(fn_decl, env);
            }
            Statement::FnCall(fn_call) => {
                let signal = execute_fn_call(&fn_call.callee, &fn_call.args, env, call_stack)?;
                if signal != Signal::None {
                    return Ok(signal);
                }
            }
            Statement::Enum(enum_decl) => {
                execute_enums(enum_decl, env, call_stack)?;
            }
            Statement::Return(exp) => {
                let value = match exp {
                    Some(expression) => execute_expressions(expression, env, call_stack)?,
                    None => Value::Int(0),
                };
                return Ok(Signal::Return(value));
            }
            Statement::Break => return Ok(Signal::Break),
            Statement::Continue => return Ok(Signal::Continue),
            Statement::For {
                var,
                iterable,
                body,
            } => {
                let signal = execute_for(var, iterable, body, env, call_stack)?;
                if signal != Signal::None {
                    return Ok(signal);
                }
            }
            Statement::Match { value, arms } => {
                execute_match(value, arms, env, call_stack)?;
            }
            Statement::Struct(struct_decl) => {
                execute_struct(struct_decl, env, call_stack)?;
            }
        }
    }
    Ok(Signal::None)
}
