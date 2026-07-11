use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        error_parser::Error,
        executor::handlers::h_expressions::execute_expressions,
        nodes::{Expression, Identifier, LetDecl},
    },
};

pub fn execute_variables(
    let_decl: &LetDecl,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Result<(), Error> {
    let value = match &let_decl.value {
        Some(expr) => execute_expressions(expr, env, call_stack)?,
        None => {
            return Err(Error::RuntimeError {
                message: format!("'let' declaration for '{}' requires a value", let_decl.name.0),
                })
        }
    };

    env.borrow_mut().define(let_decl.name.clone(), value);
    Ok(())
}

pub fn execute_update_variable(
    var: &Identifier,
    exp: &Box<Expression>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Result<(), Error> {
    let value = execute_expressions(exp, env, call_stack)?;
    if !env.borrow_mut().set(var, value) {
        return Err(Error::RuntimeError {
            message: format!("Variable '{}' is not defined", var.0),
        });
    }
    Ok(())
}
