use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        error_parser::Error,
        executor::{Value, handlers::h_expressions::execute_expressions},
        nodes::StructDecl,
    },
};

pub fn execute_struct(
    struct_decl: &Box<StructDecl>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Result<(), Error> {
    let fields = struct_decl
        .fields
        .iter()
        .map(|(name, value)| {
            let value = execute_expressions(value, env, call_stack);
            value.map(|v| (name.0.clone(), v))
        })
        .collect::<Result<HashMap<String, Value>, _>>()?;
    env.borrow_mut()
        .define(struct_decl.name.clone(), Value::Struct(Rc::new(fields)));
    Ok(())
}
