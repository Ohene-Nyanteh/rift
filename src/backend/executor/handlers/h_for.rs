use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        executor::{Value, executor, handlers::h_expressions::execute_expressions},
        nodes::{Block, Expression, Identifier, Signal},
    },
};

pub fn execute_for(
    var: &Identifier,
    iterable: &Box<Expression>,
    body: &Block,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Signal {
    // create a new environment
    let child_env = Environment::new_child(env);
    let array = execute_expressions(&iterable, &child_env, call_stack);

    match &array {
        Value::Array(v) => {
            // initialize Value and check if array isnt empty
            child_env.borrow_mut().define(var.clone(), v[0].clone());

            for value in v {
                // store value as current_value
                child_env.borrow_mut().set(&var, value.clone());
                let signal = executor(&body.statements, &child_env, call_stack);
                match &signal {
                    Signal::Break => break,
                    Signal::Continue => continue,
                    Signal::Return(_) => return signal,
                    Signal::None => {}
                }
            }
        }
        _ => panic!("Variable passed in for loop must be an Array"),
    };

    Signal::None
}
