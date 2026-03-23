use std::env;
mod backend;
use backend::config::Config;
use backend::lexer::tokenizer;
use backend::parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let init = Config::build(&args);
    let content = init.run();
    let Ok(tokens) = tokenizer(content) else {
        todo!()
    };
    let mut parser = Parser::new(tokens);
    let code = parser.parse_code()?;
    println!("{code:#?}");
    Ok(())
}
