use crate::backend::{error_parser::Error, nodes::{Expression::Literal, Statement}, parser::{Parser, col_for}, tokens::{Keywords, NonAtomic, Primary, Tokens::{self}}};

impl Parser {
    pub fn handle_imports(&mut self) -> Result<Statement, Error> {

        self.expect(Tokens::NonAtomic(NonAtomic::LCurlyBraces))?;

        loop {
            let children_token = self.next().ok_or(Error::UnexpectedEOF)?;

            match children_token.kind.clone() {
                Tokens::NonAtomic(NonAtomic::RCurlyBraces) => {
                    break;
                }
                Tokens::Variable(..) |  Tokens::NonAtomic(NonAtomic::Comma)=> {
                   continue;
                }

                unexpected =>  { return  Err(Error::UnexpectedToken{
                    expected: Tokens::Primary(Primary::Str("import variable".to_string())),
                    error_line: self.line_text(children_token.span.row),
                    col_start: col_for(children_token.span.start, children_token.span.row, &self.line_starts),
                    col_end: col_for(children_token.span.end, children_token.span.row, &self.line_starts),
                    found: unexpected.clone(),
                    at: children_token.span.clone()
                });
                }
            }
        }



        self.expect(Tokens::Keyword(Keywords::From))?;



        let path_token = self.next().ok_or(Error::UnexpectedEOF)?;
        let path = match path_token.kind.clone() {
            Tokens::Primary(value) => {Literal(value)}
            unexpected => { return  Err(Error::UnexpectedToken{
                expected: Tokens::Primary(Primary::Str("import path".to_string())),
                error_line: self.line_text(path_token.span.row),
                col_start: col_for(path_token.span.start, path_token.span.row, &self.line_starts),
                col_end: col_for(path_token.span.end, path_token.span.row, &self.line_starts),
                found: unexpected.clone(),
                at: path_token.span
            });
            }
        };


        self.expect(Tokens::NonAtomic(NonAtomic::SemiColon))?;

        Ok(Statement::Import { path: Box::new(path) })
    }
}
