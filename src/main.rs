use std::collections::HashMap;
use std::env;
mod backend;
use backend::config::Config;
use backend::lexer::tokenizer;
use backend::parser::Parser;

use crate::backend::executor::{Value, executor};
use crate::backend::nodes::Identifier;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let init = Config::build(&args);
    let content = init.run();

    let Ok(tokens) = tokenizer(content) else {
        eprintln!("Lexer error");
        std::process::exit(1);
    };

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_code()?;

    let mut variable_hashmap: HashMap<Identifier, Value> = HashMap::new();
    executor(ast, &mut variable_hashmap);

    Ok(())
}
