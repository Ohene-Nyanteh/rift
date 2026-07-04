use crate::backend::{
    error_parser::Error, nodes::Expression, parser::{Parser, col_for}, tokens::{NonAtomic, Primary, Tokens}
};

impl Parser {
    pub fn handle_arrays(&mut self) -> Result<Expression, Error> {
        let mut items: Vec<Box<Expression>> = vec![];
        let mut expect_comma_or_end = false;

        loop {
            let next_token = self.peek().ok_or(Error::UnexpectedEOF)?;

            match next_token.kind.clone() {
                Tokens::NonAtomic(NonAtomic::RSquareBraces) => {
                    self.next();
                    break;
                }
                Tokens::NonAtomic(NonAtomic::Comma) if expect_comma_or_end => {
                    self.next();
                    expect_comma_or_end = false;
                }
                _ if !expect_comma_or_end => {
                    let expected = Tokens::Primary(Primary::Str(vec![",", ";"].join(" or ").to_string()));
                    items.push(self.parse_expressions(0, expected)?);
                    expect_comma_or_end = true;
                }
                t => {
                    // we expected `,` or `]` but got something else (like `;`)
                    return Err(Error::UnexpectedToken {
                        expected: Tokens::NonAtomic(NonAtomic::RSquareBraces),
                        error_line: self.line_text(next_token.span.row),
                        col_start: col_for(next_token.span.start, next_token.span.row, &self.line_starts),
                        col_end: col_for(next_token.span.end, next_token.span.row, &self.line_starts),
                        found: t,
                        at: next_token.span.clone(),
                    });
                }
            }
        }
        Ok(Expression::ArrayLiteral(items))
    }
}
