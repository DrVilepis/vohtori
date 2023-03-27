use std::str::Chars;

static SYNTAX_TOKENS: &str = "{}[]();:$";

static OPERATOR_CHARS: &str = "+-*/=@&%^.#|&";

#[derive(Clone)]
pub struct SourceCursor<'a> {
    chars: Chars<'a>,
}

impl<'a> SourceCursor<'a> {
    fn consume(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn first(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn second(&self) -> Option<char> {
        let mut chars = self.chars.clone();

        chars.next();

        chars.next()
    }

    pub fn new(input: &'a str) -> SourceCursor<'a> {
        SourceCursor {
            chars: input.chars(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LiteralType {
    Number,
    String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TokenType {
    Identifier,
    Literal(LiteralType),
    SyntaxToken,
    Operator,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
}

#[derive(PartialEq, Copy, Clone)]
pub enum LexerMode {
    Identifier,
    Literal,
    Number,
    Operator,
}

impl SourceCursor<'_> {
    fn discard_whitespaces(&mut self) {
        while self.first().is_some_and(|c| c.is_whitespace()) {
            self.consume();
        }
    }

    pub fn advance_token(&mut self) -> Option<Token> {
        if self.first().is_some_and(|c| c == '\n') {
            return Some(Token {
                token_type: TokenType::SyntaxToken,
                content: self.consume().unwrap().to_string(),
            });
        }

        self.discard_whitespaces();

        let mut token_content = String::new();

        let mode = match self.first()? {
            c if c.is_alphabetic() => LexerMode::Identifier,
            c if c.is_ascii_digit() => LexerMode::Number,
            c if c == '"' => {
                self.consume();
                LexerMode::Literal
            }
            c if OPERATOR_CHARS.contains(c) => LexerMode::Operator,
            c if SYNTAX_TOKENS.contains(c) => {
                self.consume();

                return Some(Token {
                    token_type: TokenType::SyntaxToken,
                    content: c.to_string(),
                });
            }
            _ => panic!("Invalid character"),
        };

        while let Some(c) = self.first() {
            match mode {
                LexerMode::Identifier => {
                    if c.is_alphabetic() {
                        token_content.push(c);
                        self.consume();
                    } else {
                        return Some(Token {
                            token_type: TokenType::Identifier,
                            content: token_content,
                        });
                    }
                }
                LexerMode::Literal => {
                    if c == '"' {
                        self.consume();
                        return Some(Token {
                            token_type: TokenType::Literal(LiteralType::String),
                            content: token_content,
                        });
                    } else {
                        token_content.push(c);
                        self.consume();
                    }
                }
                LexerMode::Number => {
                    if c.is_ascii_digit() {
                        token_content.push(c);
                        self.consume();
                    } else {
                        return Some(Token {
                            token_type: TokenType::Literal(LiteralType::Number),
                            content: token_content,
                        });
                    }
                }
                LexerMode::Operator => {
                    if OPERATOR_CHARS.contains(c) {
                        token_content.push(c);
                        self.consume();
                    } else {
                        return Some(Token {
                            token_type: TokenType::Operator,
                            content: token_content,
                        });
                    }
                }
            }
        }

        None
    }
}
