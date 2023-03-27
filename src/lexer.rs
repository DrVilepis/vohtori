use std::{
    io::{BufRead, Lines, Stdin, StdinLock},
    str::Chars,
};

static SYNTAX_TOKENS: &str = "{}[]();:$";

static OPERATOR_CHARS: &str = "+-*/=@&%^.#|&";

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

pub struct SourceCursor<T: BufRead> {
    lines: Lines<T>,
}

pub struct Line<'a> {
    chars: Chars<'a>,
}

impl<T: BufRead> SourceCursor<T> {
    pub fn new(input: T) -> Self {
        Self {
            lines: input.lines(),
        }
    }

    pub fn tokenize_line(&mut self) -> Option<Vec<Token>> {
        self.lines.next().map(|l| {
            let str = l.unwrap();
            let mut line = Line::new(&str);

            let mut tokens: Vec<Token> = std::iter::from_fn(move || line.advance_token()).collect();
            tokens.reverse();

            tokens
        })
    }
}

impl<'a> Line<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
        }
    }

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

    fn discard_whitespaces(&mut self) {
        while self.first().is_some_and(|c| c.is_whitespace()) {
            self.consume();
        }
    }

    pub fn advance_token(&mut self) -> Option<Token> {
        self.discard_whitespaces();

        let mut token_content = String::new();

        let mode = match self.first()? {
            c if c.is_alphabetic() || c == '_' => LexerMode::Identifier,
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
                    if c.is_alphabetic() || c == '_' {
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
        let token_type = match mode {
            LexerMode::Identifier => TokenType::Identifier,
            LexerMode::Literal => TokenType::Literal(LiteralType::String),
            LexerMode::Number => TokenType::Literal(LiteralType::Number),
            LexerMode::Operator => TokenType::Operator,
        };

        Some(Token {
            token_type,
            content: token_content,
        })
    }
}
