use crate::ast::{BinOp, Expr};
use crate::utils::{Token, TokenType as Ty};

struct Parser {
    tokens: Vec<Token>,
    curr_token: usize,
}
impl Parser {
    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_multiplicative()?;

        while self.peek().is_some() {
            match self.peek().unwrap() {
                Token {
                    ty: Ty::Plus,
                    span: _,
                }
                | Token {
                    ty: Ty::Minus,
                    span: _,
                } => {
                    self.advance();
                    let rhs = self.parse_multiplicative()?;
                    expr = Expr::BinaryOp {
                        op: match self.peek().unwrap() {
                            Token {
                                ty: Ty::Plus,
                                span: _,
                            } => BinOp::Add,
                            Token {
                                ty: Ty::Minus,
                                span: _,
                            } => BinOp::Subtract,
                            _ => unreachable!(),
                        },
                        lhs: Box::new(expr),
                        rhs: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        while self.peek().is_some() {
            match self.peek().unwrap() {
                Token {
                    ty: Ty::Asterisk,
                    span: _,
                }
                | Token {
                    ty: Ty::Slash,
                    span: _,
                } => {
                    self.advance();
                    let rhs = self.parse_primary()?;
                    expr = Expr::BinaryOp {
                        op: match self.peek().unwrap() {
                            Token {
                                ty: Ty::Asterisk,
                                span: _,
                            } => BinOp::Multiply,
                            Token {
                                ty: Ty::Slash,
                                span: _,
                            } => BinOp::Divide,
                            _ => unreachable!(),
                        },
                        lhs: Box::new(expr),
                        rhs: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Some(Token {
                ty: Ty::Number(n),
                span: _,
            }) => Ok(Expr::Number(*n)),
            Some(Token {
                ty: Ty::Identifier(name),
                span: _,
            }) => Ok(Expr::Variable(name.clone())),
            Some(Token {
                ty: Ty::LParen,
                span: _,
            }) => {
                let expr = self.parse_expr()?;
                match self.advance() {
                    Some(Token {
                        ty: Ty::RParen,
                        span: _,
                    }) => Ok(expr),
                    _ => Err("Expected ')' after expression".to_string()),
                }
            }
            _ => Err("Expected a number, variable, or parenthesized expression".to_string()),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.curr_token)
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.curr_token < self.tokens.len() {
            self.curr_token += 1;
        }
        self.tokens.get(self.curr_token - 1)
    }

    fn is_at_end(&self) -> bool {
        self.curr_token >= self.tokens.len()
    }
}
