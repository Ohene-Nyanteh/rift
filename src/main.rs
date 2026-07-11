use std::env;
mod backend;
use backend::config::Config;
use backend::lexer::tokenizer;
use backend::parser::Parser;

use crate::backend::environment::Environment;
use crate::backend::executor::{Value, executor};
use crate::backend::tokens::Token;

#[derive(Debug)]
pub struct StackFrame {
    pub function_name: String,
    pub return_value: Option<Value>,
}


pub struct LexerOutput {
    pub tokens: Vec<Token>,
    pub source_map: Vec<String>,
    pub line_starts: Vec<usize>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let init = Config::build(&args);
    let content = init.run();

    let Ok(lexer_output) = tokenizer(&content) else {
        eprintln!("Lexer error: Couldn't parse the code");
        std::process::exit(1);
    };

    let mut parser = Parser::new(lexer_output);
    let ast = match parser.parse_code() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let mut global_env = Environment::new();
    let mut call_stack: Vec<StackFrame> = vec![];

    if let Err(e) = executor(&ast, &mut global_env, &mut call_stack) {
        eprintln!("{}", e);

        if !call_stack.is_empty() {
            eprintln!("");
            let start = call_stack.len().saturating_sub(5);
            let skipped = start;

            // Show innermost frame first (most recent call)
            for frame in call_stack[start..].iter().rev() {
                eprintln!("  at {}()", frame.function_name);
            }

            if skipped > 0 {
                eprintln!("  ... {} more (use --full-trace to show all)", skipped);
            }
        }

        std::process::exit(1);
    }

    Ok(())
}
