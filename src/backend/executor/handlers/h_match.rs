use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        executor::{Value, executor, handlers::h_expressions::execute_expressions},
        nodes::{Block, Expression},
    },
};

pub fn execute_match(
    value: &Box<Expression>,
    arms: &Vec<(Box<Expression>, Block)>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) {
    let value = execute_expressions(value, env, call_stack);

    for arm in arms {
        let arm_value: Option<Value>;
        match *arm.0.clone() {
            Expression::Variable(_) => {
                executor(&arm.1.statements, env, call_stack);
            }
            _ => {
                arm_value = Some(execute_expressions(&arm.0, env, call_stack));
                let arm_v = arm_value.expect("Error: Couldnt parse match arm value");
                if value == arm_v {
                    executor(&arm.1.statements, env, call_stack);
                    break;
                }
            }
        }
    }
}
