use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        executor::{Value, executor, handlers::h_expressions::execute_expressions},
        nodes::{Block, Expression, Signal},
    },
};

pub fn execute_loop(
    variable: Box<Expression>,
    body: Block,
    value: Box<Expression>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) {
    let iter_value = match execute_expressions(value, env, call_stack) {
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
    let variable_hashmap = &mut env.borrow_mut().variables;
    variable_hashmap.insert(variable_name.clone(), Value::Int(iter_value));

    loop {
        let mut loop_env = Environment::new_child(env);
        let signal = executor(body.statements.clone(), &mut loop_env, call_stack);
        if signal == Signal::Break {
            break;
        }
        // increment counter
        if let Some(value) = loop_env.borrow_mut().variables.get_mut(&variable_name) {
            if let Value::Int(v) = value {
                *v += 1;
            }
        }
    }
}
