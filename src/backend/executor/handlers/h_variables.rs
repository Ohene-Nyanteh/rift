use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        executor::handlers::h_expressions::execute_expressions,
        nodes::{Expression, Identifier, LetDecl},
    },
};

pub fn execute_variables(
    let_decl: &LetDecl,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) {
    let declaration = execute_expressions(
        let_decl
            .value
            .as_ref()
            .expect("Error getting variable declaration"),
        env,
        call_stack,
    );

    env.borrow_mut().define(let_decl.name.clone(), declaration);
}

pub fn execute_update_variable(
    var: &Identifier,
    exp: &Box<Expression>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) {
    let exp_result = execute_expressions(&exp, env, call_stack);
    if !env.borrow_mut().set(&var, exp_result) {
        println!("Error: Variable {} not declared!", var.0);
    }
}
