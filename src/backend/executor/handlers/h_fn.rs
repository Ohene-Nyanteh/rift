use std::{cell::RefCell, rc::Rc};

use crate::backend::{
    environment::Environment,
    nodes::FunctionDecl,
};

pub fn execute_fn(fn_decl: &Box<FunctionDecl>, env: &Rc<RefCell<Environment>>) {
    env.borrow_mut()
        .functions
        .insert(fn_decl.name.clone(), *fn_decl.clone());
}
