use crate::backend::{
    errors::Error::{self},
    nodes::{Expression, Identifier},
    parser::Parser,
    tokens::{NonAtomic, Tokens},
};

impl Parser {
    pub fn parse_struct_call(&mut self, name: Identifier) -> Result<Box<Expression>, Error> {
        let statement_exp: Box<Expression>;

        // skip the dot
        self.expect(Tokens::NonAtomic(NonAtomic::Dot))?;

        // check the value expected
        let field_token = self.next().ok_or(Error::UnexpectedEOF)?;
        let field = match field_token.kind {
            Tokens::Variable(v) => Identifier(v),
            unexpected => Err(Error::InvalidSyntax(String::from(format!(
                "Invalid Syntax: Expected a Variant, got {unexpected:?}"
            ))))?,
        };

        // check the next token to parse it as assignment or just access
        let next_token = self.peek().ok_or(Error::UnexpectedEOF)?;
        if next_token.kind == Tokens::NonAtomic(NonAtomic::Assignment) {
            // skip the =
            self.next().ok_or(Error::UnexpectedEOF)?;

            let expression = self.parse_expressions(0)?;

            statement_exp = Box::new(Expression::StructAssignment {
                target: name.clone(),
                field: field.clone(),
                new_value: expression,
            });
        } else {
            statement_exp = Box::new(Expression::StructCall {
                target: name,
                field: field,
            });
        };

        Ok(statement_exp)
    }
}
