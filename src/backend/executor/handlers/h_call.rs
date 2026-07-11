use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        error_parser::Error,
        executor::{executor, handlers::h_expressions::execute_expressions},
        nodes::{Expression, Identifier, Signal},
    },
};

pub fn execute_fn_call(
    callee: &Identifier,
    args: &Vec<Expression>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Result<Signal, Error> {
    // push the function onto the call stack
    let fn_frame = StackFrame {
        function_name: callee.0.clone(),
        return_value: None,
    };
    let fn_env = Environment::new_child(env);

    call_stack.push(fn_frame);

    // get function declaration
    let function = env
        .borrow()
        .get_fn(callee)
        .ok_or_else(|| Error::RuntimeError {
            message: format!("Function '{}' is not defined", callee.0),
        })?;

    // validate argument count
    if args.len() != function.args.len() {
        return Err(Error::RuntimeError {
            message: format!(
                "Function '{}' expects {} argument(s), but got {}",
                callee.0,
                function.args.len(),
                args.len()
            ),
        });
    }

    // map argument values to parameter names in the new environment
    for (index, expression) in args.iter().enumerate() {
        let value = execute_expressions(expression, env, call_stack)?;
        let key = &function.args[index];
        fn_env.borrow_mut().variables.insert(key.clone(), value);
    }

    // execute the body
    let signal = executor(&function.body.statements, &fn_env, call_stack)?;

    // clean up call stack
    call_stack.pop();

    match signal {
        Signal::Return(value) => Ok(Signal::Return(value)),
        Signal::None => Ok(Signal::None),
        _ => Err(Error::RuntimeError {
            message: format!("Unexpected control flow signal in function '{}'", callee.0),
        }),
    }
}
