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
    variable: &Box<Expression>,
    body: &Block,
    value: &Box<Expression>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) {
    let iter_value = match execute_expressions(&value, env, call_stack) {
        Value::Int(v) => v,
        _ => 0,
    };

    let variable_name = match variable.as_ref() {
        Expression::Variable(v) => v,
        unexpected => panic!("Expected variable name, got {unexpected:?}"),
    };

    env.borrow_mut()
        .define(variable_name.clone(), Value::Int(iter_value));

    loop {
        let loop_env = Environment::new_child(env);
        let signal = executor(&body.statements.clone(), &loop_env, call_stack);

        if signal == Signal::Break {
            break;
        }

        let current = match env.borrow().get(&variable_name) {
            Some(Value::Int(v)) => v,
            _ => break,
        };

        env.borrow_mut()
            .define(variable_name.clone(), Value::Int(current + 1));
    }
}
