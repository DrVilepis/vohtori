use crate::lexer::{SourceCursor, Token};

#[derive(Clone)]
pub struct TokenStream<'a> {
    source: SourceCursor<'a>,
}

impl<'a> TokenStream<'a> {
    pub fn new(source: SourceCursor<'a>) -> Self {
        TokenStream { source }
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.advance_token()
    }
}
