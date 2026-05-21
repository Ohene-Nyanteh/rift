use std::env;
mod backend;
use backend::config::Config;
use backend::lexer::tokenizer;
use backend::parser::Parser;

use crate::backend::environment::Environment;
use crate::backend::executor::{Value, executor};

#[derive(Debug)]
pub struct StackFrame {
    pub function_name: String,
    pub return_value: Option<Value>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let init = Config::build(&args);
    let content = init.run();

    let Ok(tokens) = tokenizer(content) else {
        eprintln!("Lexer error: Couldn't parse the code");
        std::process::exit(1);
    };

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_code()?;

    let mut global_env = Environment::new();
    let mut call_stack: Vec<StackFrame> = vec![];
    executor(ast, &mut global_env, &mut call_stack);

    Ok(())
}
