use crate::backend::{
    errors::Error,
    nodes::{Expression, Identifier},
    parser::Parser,
    tokens::{NonAtomic, Tokens},
};

impl Parser {
    pub fn parse_struct_call(&mut self, name: Identifier) -> Result<Box<Expression>, Error> {
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
        let statement_exp = Box::new(Expression::StructCall {
            target: name,
            field: field,
        });

        Ok(statement_exp)
    }
}
