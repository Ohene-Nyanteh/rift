use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        error_parser::Error,
        executor::{Value, handlers::h_expressions::execute_expressions},
        nodes::{EnumDecl, Expression},
        tokens::Primary,
    },
};

pub fn execute_enums(
    enum_decl: &Box<EnumDecl>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Result<(), Error> {
    let values: Vec<Value> = enum_decl
        .variants
        .iter()
        .map(|v| {
            let exp = Box::new(Expression::Literal(Primary::Str(v.0.clone())));
            execute_expressions(&exp, env, call_stack)
        })
        .collect::<Result<Vec<_>, _>>()?;
    env.borrow_mut()
        .define(enum_decl.name.clone(), Value::Enum(Rc::new(values)));
    Ok(())
}
