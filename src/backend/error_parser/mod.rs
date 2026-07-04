use crate::backend::{error_parser::tokens::convert_tokens_to_values, tokens::{Span, Tokens}};

pub mod tokens;

#[derive(Debug, Clone)]
pub enum Error {
    UnexpectedToken { expected: Tokens, found: Tokens, at: Span, error_line: String, col_start: usize, col_end: usize },
    UnexpectedEOF,
    // InvalidCall,
    // InvalidSyntax(String),
    // Custom(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const RED: &str = "\x1b[31;1m";
        const CYAN: &str = "\x1b[36;1m";
        const BOLD: &str = "\x1b[1m";
        const DIM: &str = "\x1b[2m";
        const RESET: &str = "\x1b[0m";

        match self {
            Error::UnexpectedToken { expected, found, at, error_line, col_start, col_end } => {
                let expected_value = convert_tokens_to_values(expected);
                let found_value = convert_tokens_to_values(found);
                let line_num = at.row + 1; // display as 1-indexed

                let gutter_width = line_num.to_string().len().max(3);
                let underline_len = col_end.saturating_sub(*col_start).max(1);

                writeln!(f, "{RED}error{RESET}{BOLD}: Unexpected token{RESET}")?;
                writeln!(
                    f,
                    "{:width$}{CYAN}-->{RESET} line {}, column {}-{}",
                    "", line_num, col_start, col_end, width = gutter_width
                )?;
                writeln!(f, "{:width$} {CYAN}|{RESET}", "", width = gutter_width)?;
                writeln!(
                    f,
                    "{CYAN}{:>width$}{RESET} {CYAN}|{RESET} {}",
                    line_num, error_line, width = gutter_width
                )?;
                writeln!(
                    f,
                    "{:width$} {CYAN}|{RESET} {}{RED}{}{RESET}",
                    "", " ".repeat(*col_start), "^".repeat(underline_len), width = gutter_width
                )?;
                writeln!(f, "{:width$} {CYAN}|{RESET}", "", width = gutter_width)?;
                writeln!(f, "{:width$} {DIM}= expected:{RESET} {}", "", expected_value, width = gutter_width)?;
                write!(f, "{:width$} {DIM}=    found:{RESET} {}", "", found_value, width = gutter_width)
            }
            Error::UnexpectedEOF => write!(f, "{RED}error{RESET}{BOLD}: unexpected end of file{RESET}"),
        }
    }
}


impl std::error::Error for Error {}
