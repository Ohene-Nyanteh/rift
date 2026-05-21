use crate::backend::{
    errors::Error,
    nodes::Expression,
    parser::Parser,
    tokens::{NonAtomic, Tokens},
};

impl Parser {
    pub fn handle_arrays(&mut self) -> Result<Expression, Error> {
        let mut items: Vec<Box<Expression>> = vec![];

        loop {
            let next_token = self.peek().ok_or(Error::UnexpectedEOF)?;

            match next_token.kind.clone() {
                Tokens::NonAtomic(NonAtomic::RSquareBraces) => {
                    self.next();
                    break;
                }
                Tokens::NonAtomic(NonAtomic::Comma) => {
                    self.next();
                }
                _ => {
                    // let parse_expressions consume however many tokens it needs
                    items.push(self.parse_expressions(0)?);
                }
            }
        }
        Ok(Expression::ArrayLiteral(items))
    }
}
