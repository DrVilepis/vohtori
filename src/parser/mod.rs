use std::io::BufRead;

use crate::{
    lexer::{LiteralType, SourceCursor, TokenType},
    parser::error::{InvalidTokenTypeError, ParserError, UnexpectedEndOfLineError},
    runtime::operation::Modifier,
};

mod error;
mod tokenstream;
pub use tokenstream::*;

#[derive(Debug)]
pub struct Operator {
    pub name: String,
    pub modifiers: Modifier,
}

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Number {
    pub value: isize,
}

#[derive(Debug)]
pub struct Lambda {
    pub body: Box<Expr>,
}

#[derive(Debug)]
pub struct Arg {
    pub index: usize,
}

#[derive(Debug)]
pub enum Expr {
    Binary(Operator, Box<Expr>, Box<Expr>),
    Unary(Operator, Box<Expr>),
    Function(Identifier),
    Variable(Identifier),
    Number(Number),
    Array(Vec<Expr>),
    Lambda(Lambda),
    Call(Box<Expr>, Box<Expr>),
    Argument(Arg),
}

pub struct Parser<T: BufRead> {
    source: SourceCursor<T>,
}

impl<T: BufRead> Parser<T> {
    pub fn new(source: SourceCursor<T>) -> Self {
        Self { source }
    }

    pub fn parse_expr(&mut self) -> Option<Result<Expr, ParserError>> {
        let token_stream = TokenStream::new(self.source.tokenize_line()?);
        let mut line = Line::new(token_stream);

        Some(line.parse_expr())
    }
}

pub struct Line {
    pub token_stream: TokenStream,
}

impl Line {
    pub fn new(token_stream: TokenStream) -> Self {
        Self { token_stream }
    }

    fn consume(&mut self, tt: TokenType) -> bool {
        let matching = self.token_stream.peek().is_some_and(|t| t.token_type == tt);
        if matching {
            self.token_stream.next();
        }

        matching
    }

    fn consume_with_content(&mut self, tt: TokenType, content: &str) -> bool {
        let matching = self
            .token_stream
            .peek()
            .is_some_and(|t| t.token_type == tt && t.content == content);
        if matching {
            self.token_stream.next();
        }

        matching
    }

    pub fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        let lhs = self.parse_term()?;

        if self.expect(TokenType::Operator) {
            let op = self.parse_operator();
            let rhs = self.parse_expr()?;
            return Ok(Expr::Binary(op, Box::new(lhs), Box::new(rhs)));
        }

        Ok(lhs)
    }

    fn parse_term(&mut self) -> Result<Expr, ParserError> {
        if self.expect(TokenType::Identifier) {
            let ident = Identifier {
                name: self.token_stream.next().unwrap().content,
            };
            if self.consume_with_content(TokenType::SyntaxToken, ":") {
                Ok(Expr::Call(
                    Box::new(Expr::Function(ident)),
                    Box::new(self.parse_expr()?),
                ))
            } else {
                Ok(Expr::Variable(ident))
            }
        } else if self.expect(TokenType::Literal(LiteralType::Number)) {
            Ok(Expr::Number(Number {
                value: self.token_stream.next().unwrap().content.parse().unwrap(),
            }))
        } else if self.consume_with_content(TokenType::SyntaxToken, "(") {
            let expr = self.parse_expr();
            self.consume_with_content(TokenType::SyntaxToken, ")");

            expr
        } else if self.consume_with_content(TokenType::SyntaxToken, "[") {
            let mut array = vec![];

            while !self.expect_with_content(TokenType::SyntaxToken, "]") {
                array.push(self.parse_term()?);
            }
            self.consume_with_content(TokenType::SyntaxToken, "]");

            Ok(Expr::Array(array))
        } else if self.expect(TokenType::Operator) {
            let op = self.parse_operator();
            let expr = self.parse_expr()?;

            Ok(Expr::Unary(op, Box::new(expr)))
        } else if self.consume_with_content(TokenType::SyntaxToken, "$") {
            if self.expect(TokenType::Identifier) {
                let ident = self.parse_identifier()?;

                if self.consume_with_content(TokenType::SyntaxToken, ":") {
                    Ok(Expr::Call(
                        Box::new(Expr::Function(ident)),
                        Box::new(self.parse_expr()?),
                    ))
                } else {
                    Ok(Expr::Function(ident))
                }
            } else if self.consume_with_content(TokenType::SyntaxToken, ":") {
                let function = self.parse_term()?;

                if self.consume_with_content(TokenType::SyntaxToken, ":") {
                    let expr = self.parse_expr()?;

                    Ok(Expr::Call(
                        Box::new(Expr::Lambda(Lambda {
                            body: Box::new(function),
                        })),
                        Box::new(expr),
                    ))
                } else {
                    Ok(Expr::Lambda(Lambda {
                        body: Box::new(function),
                    }))
                }
            } else if self.expect(TokenType::Literal(LiteralType::Number)) {
                let num = self.token_stream.next().unwrap().content.parse().unwrap();
                Ok(Expr::Argument(Arg { index: num }))
            } else {
                Err(ParserError::InvalidTokenType(InvalidTokenTypeError::new(
                    vec![TokenType::SyntaxToken],
                    self.token_stream.next().unwrap().token_type,
                )))
            }
        } else {
            Err(ParserError::UnexpectedEndOfLine(UnexpectedEndOfLineError))
        }
    }

    fn parse_factor(&mut self) -> Result<Expr, ParserError> {
        if self.expect(TokenType::Identifier) {
            Ok(Expr::Variable(Identifier {
                name: self.token_stream.next().unwrap().content,
            }))
        } else if self.expect(TokenType::Literal(LiteralType::Number)) {
            Ok(Expr::Number(Number {
                value: self.token_stream.next().unwrap().content.parse().unwrap(),
            }))
        } else {
            Err(InvalidTokenTypeError::new(
                vec![
                    TokenType::Identifier,
                    TokenType::Literal(LiteralType::Number),
                ],
                self.token_stream.next().unwrap().token_type,
            )
            .into())
        }
    }

    fn parse_identifier(&mut self) -> Result<Identifier, InvalidTokenTypeError> {
        if self.expect(TokenType::Identifier) {
            Ok(Identifier {
                name: self.token_stream.next().unwrap().content,
            })
        } else {
            Err(InvalidTokenTypeError::new(
                vec![TokenType::Identifier],
                self.token_stream.next().unwrap().token_type,
            ))
        }
    }

    fn parse_operator(&mut self) -> Operator {
        if self.expect(TokenType::Operator) {
            let operator = self.token_stream.next().unwrap().content;
            let mut modifiers = Modifier::default();

            if self.consume_with_content(TokenType::SyntaxToken, ":") {
                modifiers |= parse_modifier(&self.token_stream.next().unwrap().content);
            }

            return Operator {
                name: operator,
                modifiers,
            };
        }

        todo!()
    }

    fn expect(&self, tt: TokenType) -> bool {
        self.token_stream
            .clone()
            .next()
            .is_some_and(|t| tt == t.token_type)
    }

    fn expect_with_content(&mut self, tt: TokenType, content: &str) -> bool {
        self.token_stream
            .clone()
            .next()
            .is_some_and(|t| tt == t.token_type && content == t.content)
    }
}

fn parse_modifier(m: &str) -> Modifier {
    let mut modifiers = Modifier::default();
    for c in m.chars() {
        match c {
            '*' => modifiers |= Modifier::Table,
            '|' => modifiers |= Modifier::Flip,
            _ => (),
        }
    }
    modifiers
}
