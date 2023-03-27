use std::fmt::Display;

use crate::lexer::TokenType;

#[derive(Debug)]
pub enum ParserError {
    InvalidTokenType(InvalidTokenTypeError),
    MissingToken(MissingTokenError),
    UnexpectedEndOfLine(UnexpectedEndOfLineError),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::InvalidTokenType(err) => {
                write!(f, "Expected one of [")?;
                let len = err.expected.len();

                for (i, tt) in err.expected.iter().enumerate() {
                    write!(f, "{:?}", tt)?;

                    if i < (len - 1) {
                        write!(f, ", ")?;
                    }
                }

                write!(f, "], found {:?}", err.found)?;
            }
            ParserError::MissingToken(_) => todo!(),
            ParserError::UnexpectedEndOfLine(_) => {
                write!(f, "Unexpected EOL")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct InvalidTokenTypeError {
    pub expected: Vec<TokenType>,
    pub found: TokenType,
}

impl InvalidTokenTypeError {
    pub fn new(expected: Vec<TokenType>, found: TokenType) -> Self {
        Self { expected, found }
    }
}

impl From<InvalidTokenTypeError> for ParserError {
    fn from(value: InvalidTokenTypeError) -> Self {
        Self::InvalidTokenType(value)
    }
}

#[derive(Debug)]
pub struct MissingTokenError {
    pub expected: Vec<TokenType>,
}

impl MissingTokenError {
    pub fn new(expected: Vec<TokenType>) -> Self {
        Self { expected }
    }
}

#[derive(Debug)]
pub struct UnexpectedEndOfLineError;
