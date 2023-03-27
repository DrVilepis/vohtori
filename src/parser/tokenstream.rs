use crate::lexer::{SourceCursor, Token};

#[derive(Clone)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new(line: Vec<Token>) -> Self {
        TokenStream { tokens: line }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.last()
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.pop()
    }
}
