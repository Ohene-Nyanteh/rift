use std::env;
mod backend;
use backend::config::Config;
use backend::lexer::tokenizer;
use backend::parser::Parser;

use crate::backend::environment::Environment;
use crate::backend::executor::executor;

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
    executor(ast, &mut global_env);

    Ok(())
}
