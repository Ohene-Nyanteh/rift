use crate::backend::{error_parser::Error, nodes::Statement, parser::{Parser, col_for}, tokens::{Keywords, Primary, Tokens::{self}}};

impl Parser {
    pub fn handle_exports(&mut self) -> Result<Statement, Error> {
        let next_token = self.peek().ok_or(Error::UnexpectedEOF)?;

        match next_token.kind.clone() {
            Tokens::Keyword(value) => {
                match value {
                    Keywords::Let => {
                        self.next();
                        self.parse_let()
                    }
                    Keywords::Fn => {
                        self.next();
                        self.parse_functions()
                    }
                    Keywords::Struct => {
                        self.next();
                        self.parse_struct()
                    }
                    Keywords::Enum => {
                        self.next();
                        self.parse_enum()
                    }
                    _ => {
                        return Err(Error::UnexpectedToken{
                            expected: Tokens::Primary(Primary::Str("Variable | Struct | Enum | Function".to_string())),
                            found: next_token.kind.clone(),
                            error_line: self.line_text(next_token.span.row),
                            col_start: col_for(next_token.span.start, next_token.span.row, &self.line_starts),
                            col_end: col_for(next_token.span.end, next_token.span.row, &self.line_starts),
                            at: next_token.span.clone()
                        });
                    }

                }
            }
            unexpected => {
                return Err(Error::UnexpectedToken{
                    expected: Tokens::Primary(Primary::Str("Variable | Struct | Enum | Function".to_string())),
                    found: unexpected,
                    error_line: self.line_text(next_token.span.row),
                    col_start: col_for(next_token.span.start, next_token.span.row, &self.line_starts),
                    col_end: col_for(next_token.span.end, next_token.span.row, &self.line_starts),
                    at: next_token.span.clone()
                });
            }
        }
    }
}
