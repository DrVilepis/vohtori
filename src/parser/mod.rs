use crate::{
    lexer::{LiteralType, TokenType},
    runtime::operation::Modifier,
};

mod error;
mod tokenstream;
use error::InvalidTokenTypeError;
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
pub enum Expr {
    Binary(Operator, Box<Expr>, Box<Expr>),
    Unary(Operator, Box<Expr>),
    Function(Identifier, Box<Expr>),
    Variable(Identifier),
    Number(Number),
    Array(Vec<Expr>),
}

pub struct Parser<'a> {
    pub token_stream: TokenStream<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(token_stream: TokenStream<'a>) -> Self {
        Self { token_stream }
    }

    fn consume(&mut self) {
        self.token_stream.next();
    }

    pub fn parse_expr(&mut self) -> Expr {
        let lhs = self.parse_term();

        if self.expect(TokenType::Operator) {
            let op = self.parse_operator();
            let rhs = self.parse_expr();
            return Expr::Binary(op, Box::new(lhs), Box::new(rhs));
        }

        if self.expect_with_content(TokenType::SyntaxToken, "\n") {
            return lhs;
        }

        lhs
    }

    fn parse_term(&mut self) -> Expr {
        if self.expect(TokenType::Identifier) {
            return Expr::Variable(Identifier {
                name: self.token_stream.next().unwrap().content,
            });
        } else if self.expect(TokenType::Literal(LiteralType::Number)) {
            return Expr::Number(Number {
                value: self.token_stream.next().unwrap().content.parse().unwrap(),
            });
        } else if self.expect_with_content(TokenType::SyntaxToken, "(") {
            self.consume();
            let expr = self.parse_expr();
            self.consume();
            return expr;
        } else if self.expect_with_content(TokenType::SyntaxToken, "[") {
            self.consume();

            let mut array = vec![];

            while !self.expect_with_content(TokenType::SyntaxToken, "]") {
                array.push(self.parse_term());
            }
            self.consume();

            return Expr::Array(array);
        } else if self.expect(TokenType::Operator) {
            let op = self.parse_operator();
            let expr = self.parse_expr();

            return Expr::Unary(op, Box::new(expr));
        } else if self.expect_with_content(TokenType::SyntaxToken, "$") {
            self.consume();
            let name = self.parse_identifier().unwrap();
            let arg = self.parse_term();
            return Expr::Function(name, Box::new(arg));
        }
        panic!("Valid token not found");
    }

    fn parse_factor(&mut self) -> Result<Expr, InvalidTokenTypeError> {
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
                &[
                    TokenType::Identifier,
                    TokenType::Literal(LiteralType::Number),
                ],
                self.token_stream.next().unwrap().token_type,
            ))
        }
    }

    fn parse_identifier(&mut self) -> Result<Identifier, InvalidTokenTypeError> {
        if self.expect(TokenType::Identifier) {
            Ok(Identifier {
                name: self.token_stream.next().unwrap().content,
            })
        } else {
            Err(InvalidTokenTypeError::new(
                &[TokenType::Identifier],
                self.token_stream.next().unwrap().token_type,
            ))
        }
    }

    fn parse_operator(&mut self) -> Operator {
        if self.expect(TokenType::Operator) {
            let operator = self.token_stream.next().unwrap().content;
            let mut modifiers = Modifier::default();

            if self.expect_with_content(TokenType::SyntaxToken, ":") {
                self.consume();
                modifiers |= parse_modifier(&self.token_stream.next().unwrap().content);
            }

            return Operator {
                name: operator,
                modifiers,
            };
        }

        todo!()
    }

    fn expect(&mut self, tt: TokenType) -> bool {
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
