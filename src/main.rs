use std::env;
mod backend;
use backend::config::Config;

use crate::backend::environment::Environment;
use crate::backend::executor::{Value, executor};
use crate::backend::module_system::ModuleGraph;
use crate::backend::tokens::Token;

#[derive(Debug)]
pub struct StackFrame {
    pub function_name: String,
    pub return_value: Option<Value>,
}

#[derive(Debug)]
pub struct LexerOutput {
    pub tokens: Vec<Token>,
    pub source_map: Vec<String>,
    pub line_starts: Vec<usize>
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let init = Config::build(&args);

    let mut module_system = ModuleGraph::new(init);
    if let Err(e) = module_system.walk_modules() {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    let mut global_env = Environment::new();
    let mut call_stack: Vec<StackFrame> = vec![];


    for node_index in module_system.order {
        if let Err(e) = executor(&module_system.nodes[node_index].ast, &mut global_env, &mut call_stack) {
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
    }
    Ok(())
}
