use std::{cell::RefCell, rc::Rc};

use crate::{
    StackFrame,
    backend::{
        environment::Environment,
        executor::{executor, handlers::h_expressions::execute_expressions},
        nodes::{Expression, Identifier, Signal},
    },
};

pub fn execute_fn_call(
    callee: &Identifier,
    args: &Vec<Expression>,
    env: &Rc<RefCell<Environment>>,
    call_stack: &mut Vec<StackFrame>,
) -> Signal {
    // push the function unto the call Stack
    let fn_frame = StackFrame {
        function_name: callee.0.clone(),
        return_value: None,
    };
    // create a new Environment
    let fn_env = Environment::new_child(env);

    // push fn into stack frame
    call_stack.push(fn_frame);

    // get function body next
    let function = match env.borrow().get_fn(&callee) {
        Some(f) => f,
        None => panic!("Error: Couldn't find function"),
    };

    // map the values of the args with the args name and store it in the new Environment
    for (index, expression) in args.iter().enumerate() {
        // parse the expression and get the value
        let value = execute_expressions(expression, env, call_stack);
        let key = match function.args.get(index) {
            Some(v) => v,
            None => panic!(
                "Error: Functions accept {:?} params, got {:?} params",
                function.args.len(),
                args.len()
            ),
        };
        // inserting into variables needs borrow_mut
        let _ = fn_env.borrow_mut().variables.insert(key.clone(), value);
    }

    // now execute the body
    let signal = executor(&function.body.statements, &fn_env, call_stack);

    // remove the function from the stack
    call_stack.pop(); // clean up
    match signal {
        Signal::Return(value) => return Signal::Return(value),
        Signal::None => {}
        _ => panic!("Unexpected signal in function body"),
    }
    Signal::None
}
