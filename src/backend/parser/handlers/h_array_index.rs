use crate::backend::{
    error_parser::Error, nodes::{Expression, Identifier, Statement}, parser::Parser, tokens::{NonAtomic, Primary, Tokens}
};

impl Parser {
    pub fn parse_array_index_expr(&mut self, name: Identifier) -> Result<Expression, Error> {
        // Skip the [
        self.next();

        let value = self.parse_expressions(0, Tokens::Primary(Primary::Str("Int value".to_string())))?;

        // expect ]
        self.expect(Tokens::NonAtomic(NonAtomic::RSquareBraces))?;

        Ok(Expression::ArrayIndex {
            target: Box::new(Expression::Variable(name)),
            index: value,
        })
    }

    pub fn parse_array_index(&mut self, name: Identifier) -> Result<Statement, Error> {
        // get the current value
        let array_index_exp = self.parse_array_index_expr(name.clone())?;
        // expect semi colon
        self.expect(Tokens::NonAtomic(NonAtomic::SemiColon))?;

        Ok(Statement::Expression(Box::new(array_index_exp)))
    }
}
