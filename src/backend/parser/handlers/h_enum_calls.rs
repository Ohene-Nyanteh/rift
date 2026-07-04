use crate::backend::{
    error_parser::Error,
    nodes::{Expression, Identifier},
    parser::{Parser, col_for},
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
                    error_line: self.line_text(variant_token.span.row),
                    col_start: col_for(variant_token.span.start, variant_token.span.row, &self.line_starts),
                    col_end: col_for(variant_token.span.end, variant_token.span.row, &self.line_starts),
                    at: variant_token.span
                });
            }
        };

        Ok(Expression::EnumCall { name, variant })
    }
}
