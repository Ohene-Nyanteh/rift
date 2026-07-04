use crate::backend::{
    error_parser::Error,
    nodes::Statement,
    parser::{Parser, col_for},
    tokens::{NonAtomic, Primary, Tokens},
};

impl Parser {
    pub fn parse_print(&mut self) -> Result<Statement, Error> {
        // expect the next (
        self.expect(Tokens::NonAtomic(NonAtomic::LParen))?;

        // read the expression value
        let value = match self.peek() {
            None => return Err(Error::UnexpectedEOF)?,
            Some(v) => match v.clone().kind {
                Tokens::Atomic(_) | Tokens::Variable(_) | Tokens::Primary(_) => {
                    let expected = Tokens::Primary(Primary::Str(vec!["Expression(+, -, /, %,) etc..", "variable", "value"].join(" or ").to_string()));
                    let exp = self.parse_expressions(0, expected)?;
                    exp
                }
                unexpected => {
                    return Err(Error::UnexpectedToken {
                        expected: Tokens::Primary(Primary::Str("'value'".to_string())),
                        error_line: self.line_text(v.span.row),
                        col_start: col_for(v.span.start, v.span.row, &self.line_starts),
                        col_end: col_for(v.span.end, v.span.row, &self.line_starts),
                        found: unexpected,
                        at: v.span.clone()
                    })?;
                }
            },
        };

        // expect and consume } and ;
        self.expect(Tokens::NonAtomic(NonAtomic::RParen))?;
        self.expect(Tokens::NonAtomic(NonAtomic::SemiColon))?;

        Ok(Statement::Print(value))
    }
}
