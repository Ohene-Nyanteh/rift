use crate::backend::tokens::{Tokens};
use super::nodes::{Node};
use super::errors::{Errors};


pub struct Parser {
    pos: usize,
    expected: Vec<Token>
}






impl Parser {
    fn new(tokens: Vec<Tokens>) -> self {
        self
    }

    fn peek(&self) -> &Tokens {

    }

    fn next(&mut self) -> Token {

    }

    fn expect(&mut self, token: Token) -> Token {
        // returns a token if the correct token is matched
    }
}
