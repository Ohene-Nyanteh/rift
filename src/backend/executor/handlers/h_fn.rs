use std::{cell::RefCell, rc::Rc};

use crate::backend::{
    environment::Environment,
    nodes::{FunctionDecl, Signal},
};

pub fn execute_fn(fn_decl: Box<FunctionDecl>, env: &Rc<RefCell<Environment>>) -> Signal {
    env.borrow_mut()
        .functions
        .insert(fn_decl.name.clone(), *fn_decl);
    Signal::None
}
