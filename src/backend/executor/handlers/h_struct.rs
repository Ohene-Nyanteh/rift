use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        executor::{Value, handlers::h_expressions::execute_expressions},
        nodes::StructDecl,
    },
};

pub fn execute_struct(
    struct_decl: &Box<StructDecl>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) {
    // parse all fields and names into values
    let fields = struct_decl
        .fields
        .iter()
        .map(|(name, value)| {
            let value = execute_expressions(value, env, call_stack);
            (name.0.clone(), value)
        })
        .collect::<HashMap<String, Value>>();
    env.borrow_mut()
        .define(struct_decl.name.clone(), Value::Struct(fields));
}
