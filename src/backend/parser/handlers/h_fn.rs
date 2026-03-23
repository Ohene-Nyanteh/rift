use crate::backend::parser::{Parser};
use crate::backend::errors::{Error};
use crate::backend::nodes::{Statement, LetDecl, Identifier, Expression};
use crate::backend::tokens::{Primary, Tokens, NonAtomic, Token};


impl Parser {
    pub fn parse_functions(&mut self) -> Result<Statement, Error> {
        /*
         * fn main(parse_args) {
         *  create a block here and call parse_statements
         * }
         */


         let fn_name_token = self.next();
         let fn_name = match &fn_name_token.kind {
             Tokens::Variable(val) => Identifier(val.to_string()),
             _ => return Err(Error::InvalidSyntax("Expected a fn name ".to_string())),
         };

         // skip the first open tag
         let open_tag = self.next();
         if open_tag.kind != Tokens::LOpenBraces
    }
}
