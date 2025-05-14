use core::str;
use std::{cell::RefCell, rc::Rc};

use crate::{
    compiler::Compiler,
    diagnostic::{Diagnostic, DiagnosticLevel},
    utils::{LiteralKind, Span, Token, TokenType as Ty},
};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Lexer<'a> {
    index: usize,
    source: &'a [u8],
    compiler: Rc<RefCell<Compiler<'a>>>,
}

impl<'a> Lexer<'a> {
    pub fn new(compiler: Rc<RefCell<Compiler<'a>>>) -> Self {
        Self {
            index: 0,
            source: compiler.borrow().source,
            compiler: compiler.clone(),
        }
    }

    pub fn identify_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            if self.peek().is_none() {
                break;
            };
            while self.peek().unwrap().is_whitespace() {
                self.advance();
            }
            let ch = self.peek().unwrap();
            let start = self.index;
            self.advance();
            let token: Ty = match ch {
                ';' => Ty::Semicolon,
                '(' => Ty::LParen,
                ')' => Ty::RParen,
                '{' => Ty::LCurly,
                '}' => Ty::RCurly,
                '[' => Ty::LBoxed,
                ']' => Ty::RBoxed,
                '<' => {
                    if self.peek().is_some_and(|x| x == '=') {
                        self.advance();
                        Ty::LTEq
                    } else {
                        Ty::LT
                    }
                }
                '>' => {
                    if self.peek().is_some_and(|x| x == '=') {
                        self.advance();
                        Ty::GTEq
                    } else {
                        Ty::GT
                    }
                }
                '!' => {
                    if self.peek().is_some_and(|x| x == '=') {
                        self.advance();
                        Ty::NotEq
                    } else {
                        Ty::Not
                    }
                }
                '=' => {
                    let mut ty: Ty = Ty::Unknown;
                    if self.peek().is_some() {
                        if self.peek() == Some('=') {
                            self.advance();
                            ty = Ty::DoubleEq;
                        } else if self.peek() == Some('>') {
                            self.advance();
                            ty = Ty::RightFatArrow;
                        } else {
                            ty = Ty::Eq;
                        }
                    }
                    ty
                }
                '+' => Ty::Plus,
                '-' => Ty::Minus,
                '*' => Ty::Asterisk,
                '/' => Ty::Slash,
                ':' => Ty::Colon,
                '\'' => {
                    if self.peek().is_some_and(|x| x != '\\') {
                        let c = self.peek().unwrap();
                        self.advance();
                        if self.peek().is_some_and(|x| x != '\'') {
                            self.error(
                                "Expected the end of char quote.".to_string(),
                                Span {
                                    start: self.index,
                                    end: self.index,
                                },
                            );
                        }
                        self.advance();
                        Ty::Literal(LiteralKind::Char(c))
                    } else {
                        Ty::Unknown
                    }
                }
                '"' => {
                    todo!()
                }
                '_' | 'a'..='z' | 'A'..='Z' => self.identify_keyword_or_id(start),
                '0'..='9' => self.identify_number(start),
                _ => {
                    self.error(
                        format!("Unknown token used: '{}'", ch),
                        Span {
                            start: self.index - 1,
                            end: self.index - 1,
                        },
                    );
                    Ty::Unknown
                }
            };
            let end = self.index - 1;
            tokens.push(Token::new(token, Span { start, end }));
        }
        tokens
    }

    fn identify_keyword_or_id(&mut self, start: usize) -> Ty {
        while self.peek().is_some_and(|x| x.is_alphanumeric() || x == '_') {
            self.advance();
        }
        match str::from_utf8(&self.source[start..self.index]).unwrap() {
            "isize" => Ty::KISIZE,
            "i64" => Ty::KI64,
            "i32" => Ty::KI32,
            "i16" => Ty::KI16,
            "i8" => Ty::KI8,
            "var" => Ty::KVariable,
            "mut" => Ty::KMutable,
            "const" => Ty::KConstant,
            "func" => Ty::KFunction,
            "struct" => Ty::KStruct,
            "class" => Ty::KClass,
            id => Ty::Identifier(id.to_string()),
        }
    }

    fn identify_number(&mut self, start: usize) -> Ty {
        let mut has_point = false;
        while self.peek().is_some_and(|x| x.is_numeric() || x == '.') {
            if self.peek() != Some('.') {
                self.advance();
                continue;
            }
            if self.peek_front(0) == Some('.') && !has_point {
                return Ty::Literal(LiteralKind::Integer(
                    String::from_utf8_lossy(&self.source[start..self.index]).to_string(),
                ));
            } else if !has_point {
                has_point = true;
            } else if has_point {
                self.error(
                    "Can't parse number properly.".to_string(),
                    Span {
                        start,
                        end: self.index,
                    },
                );
            }
            self.advance();
        }
        let num = String::from_utf8_lossy(&self.source[start..self.index]).to_string();
        if has_point {
            Ty::Literal(LiteralKind::Float(num))
        } else {
            Ty::Literal(LiteralKind::Integer(num))
        }
    }

    fn peek(&self) -> Option<char> {
        if self.index < self.source.len() {
            return str::from_utf8(self.source).unwrap().chars().nth(self.index);
        };
        None
    }

    fn peek_front(&self, offset: usize) -> Option<char> {
        if self.index < self.source.len() {
            return str::from_utf8(self.source)
                .unwrap()
                .chars()
                .nth(self.index + offset + 1);
        };
        None
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn error(&mut self, message: String, span: Span) {
        self.compiler.borrow_mut().reporter.add(Diagnostic::new(
            DiagnosticLevel::Error,
            message,
            span,
        ));
    }
}
