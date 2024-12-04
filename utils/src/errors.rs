use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    EndOfInput(&'static str),
    ParseError(&'static str, String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EndOfInput(var) => {
                write!(f, "Unexpected end of input while parsing `{}`", var)
            }
            ParseError::ParseError(var, message) => {
                write!(f, "Error while parsing `{}`: {}", var, message)
            }
        }
    }
}
