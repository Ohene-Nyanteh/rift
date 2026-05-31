use crate::backend::{
    errors::Error,
    nodes::{
        Expression, Identifier,
        Statement::{self},
    },
    parser::Parser,
    tokens::Tokens,
};

impl Parser {
    pub fn parse_enum_calls(&mut self, name: Identifier) -> Result<Expression, Error> {
        // skip the first colon
        self.next().ok_or(Error::UnexpectedEOF)?;
        // expect another colon
        self.expect(Tokens::NonAtomic(crate::backend::tokens::NonAtomic::Colon))?;

        let variant_token = self.next().ok_or(Error::UnexpectedEOF)?;

        let variant = match variant_token.kind {
            Tokens::Variable(v) => Identifier(v.clone()),
            unexpected => {
                return Err(Error::UnexpectedToken {
                    expected: Tokens::Variable("Variant Name".to_string()),
                    found: unexpected,
                });
            }
        };

        Ok(Expression::EnumCall { name, variant })
    }
}
