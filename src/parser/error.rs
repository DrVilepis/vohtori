use crate::lexer::TokenType;

#[derive(Debug)]
pub struct InvalidTokenTypeError<'a> {
    pub expected: &'a [TokenType],
    pub found: TokenType,
}

impl<'a> InvalidTokenTypeError<'a> {
    pub fn new(expected: &'a [TokenType], found: TokenType) -> Self {
        Self { expected, found }
    }
}


#[derive(Debug)]
pub struct MissingTokenError<'a> {
    pub expected: &'a [TokenType],
}

impl<'a> MissingTokenError<'a> {
    pub fn new(expected: &'a [TokenType]) -> Self {
        Self { expected }
    }
}
