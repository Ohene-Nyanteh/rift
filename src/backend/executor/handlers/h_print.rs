use crate::backend::executor::Value;

pub fn execute_print(value: Value) {
    match value {
        Value::Bool(v) => println!("{}", v),
        Value::Int(v) => println!("{}", v),
        Value::Float(v) => println!("{}", v),
        Value::Str(v) => println!("{}", v),
    }
}
