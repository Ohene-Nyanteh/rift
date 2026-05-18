use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::backend::executor::Value;
use crate::backend::nodes::{FunctionDecl, Identifier};

#[derive(Debug, Clone)]
pub struct Environment {
    pub variables: HashMap<Identifier, Value>,
    pub functions: HashMap<Identifier, FunctionDecl>,
    pub parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: None,
        }))
    }

    pub fn new_child(parent: &Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent: Some(Rc::clone(parent)),
        }))
    }

    pub fn get(&self, key: &Identifier) -> Option<Value> {
        match self.variables.get(key) {
            Some(val) => Some(val.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().get(key),
                None => None,
            },
        }
    }

    pub fn get_fn(&self, key: &Identifier) -> Option<FunctionDecl> {
        match self.functions.get(key) {
            Some(fn_decl) => Some(fn_decl.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().get_fn(key),
                None => None,
            },
        }
    }

    pub fn set(&mut self, key: &Identifier, value: Value) -> bool {
        if self.variables.contains_key(key) {
            self.variables.insert(key.clone(), value);
            true
        } else {
            match &self.parent {
                Some(parent) => parent.borrow_mut().set(key, value),
                None => false,
            }
        }
    }

    pub fn define(&mut self, key: Identifier, value: Value) {
        self.variables.insert(key, value);
    }
}
