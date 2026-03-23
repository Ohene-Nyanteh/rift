use crate::backend::tokens::Tokens;

#[derive(Debug, Clone)]
pub enum Error {
    UnexpectedToken {
        expected: Tokens,
        found: Tokens,
    },
    UnexpectedEOF,
    InvalidCall,
    InvalidSyntax(String),
    Custom(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnexpectedToken { expected, found } => {
                write!(f, "Unexpected token. Expected: {:?}, found: {:?}", expected, found)
            }
            Error::UnexpectedEOF => write!(f, "Unexpected end of file"),
            Error::InvalidCall => write!(f, "Invalid Function Call"),
            Error::InvalidSyntax(msg) => write!(f, "{}",msg),
            Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}
