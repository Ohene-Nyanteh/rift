use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        executor::{Value, handlers::h_expressions::execute_expressions},
        nodes::{Expression, Identifier, LetDecl},
    },
};

pub fn execute_variables(
    let_decl: Box<LetDecl>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) {
    let declaration = execute_expressions(
        let_decl.value.expect("Error getting declation"),
        env,
        call_stack,
    );

    match declaration {
        Value::Bool(v) => {
            env.borrow_mut().define(let_decl.name, Value::Bool(v));
        }
        Value::Int(v) => {
            env.borrow_mut().define(let_decl.name, Value::Int(v));
        }
        Value::Float(v) => {
            env.borrow_mut().define(let_decl.name, Value::Float(v));
        }
        Value::Str(v) => {
            env.borrow_mut().define(let_decl.name, Value::Str(v));
        }
    }
}

pub fn execute_update_variable(
    var: Identifier,
    exp: Box<Expression>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) {
    let exp_result = execute_expressions(exp, env, call_stack);
    if !env.borrow_mut().set(&var, exp_result) {
        println!("Error: Variable {} not declared!", var.0);
    }
}
