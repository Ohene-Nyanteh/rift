use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        executor::{Value, handlers::h_expressions::execute_expressions},
        nodes::{EnumDecl, Expression},
        tokens::Primary,
    },
};

pub fn execute_enums(
    enum_decl: Box<EnumDecl>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) {
    let values: Vec<Value> = enum_decl
        .variants
        .iter()
        .map(|v| {
            let exp = Box::new(Expression::Literal(Primary::Str(v.0.clone())));
            execute_expressions(exp, env, call_stack)
        })
        .collect();
    env.borrow_mut().define(enum_decl.name, Value::Enum(values));
}
