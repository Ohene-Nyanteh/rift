use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        error_parser::Error,
        executor::{executor, handlers::h_expressions::execute_expressions},
        nodes::{Block, Expression},
    },
};

pub fn execute_match(
    value: &Box<Expression>,
    arms: &Vec<(Box<Expression>, Block)>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Result<(), Error> {
    let match_value = execute_expressions(value, env, call_stack)?;

    for arm in arms {
        match arm.0.as_ref() {
            // `default` arm matches anything (parsed as a variable)
            Expression::Variable(_) => {
                executor(&arm.1.statements, env, call_stack)?;
                return Ok(());
            }
            arm_expr => {
                let arm_value = execute_expressions(arm_expr, env, call_stack)?;
                if match_value == arm_value {
                    executor(&arm.1.statements, env, call_stack)?;
                    return Ok(());
                }
            }
        }
    }

    // No arm matched, just fall through
    Ok(())
}
