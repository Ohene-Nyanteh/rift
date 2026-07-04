use crate::backend::{
    error_parser::Error,
    nodes::{Expression, Identifier},
    parser::{Parser, col_for},
    tokens::{NonAtomic, Primary, Tokens},
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
            unexpected =>  Err(Error::UnexpectedToken{
                expected: Tokens::Primary(Primary::Str("struct variant name".to_string())),
                error_line: self.line_text(field_token.span.row),
                col_start: col_for(field_token.span.start, field_token.span.row, &self.line_starts),
                col_end: col_for(field_token.span.end, field_token.span.row, &self.line_starts),
                found: unexpected,
                at: field_token.span
            })?
        };

        // check the next token to parse it as assignment or just access
        let next_token = self.peek().ok_or(Error::UnexpectedEOF)?;
        if next_token.kind == Tokens::NonAtomic(NonAtomic::Assignment) {
            // skip the =
            self.next().ok_or(Error::UnexpectedEOF)?;
            let expected = Tokens::Primary(Primary::Str(vec!["value", "enum variant", "struct variant"].join(" or ").to_string()));
            let expression = self.parse_expressions(0, expected)?;

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
