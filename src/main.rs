use std::env;
mod backend;
use backend::config::{Config};
use backend::lexer::{tokenizer};



fn main() {
    let args: Vec<String> = env::args().collect();
    let init = Config::build(&args);
    let content = init.run();
    tokenizer(content);
    // println!("{}", content);
    // let _add: Tokens = Tokens::Atomic(Operations::Arithmetic(ArithmeticOp::Add));
    // let _char: Tokens = Tokens::Primary(Primary::Char);
    // let _if_var: Tokens = Tokens::Secondary(Secondary::Keyword(Keywords::If));

}
