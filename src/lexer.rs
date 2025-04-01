use core::str;
use std::{cell::RefCell, rc::Rc};

use crate::{
    compiler::Compiler,
    diagnostic::{Diagnostic, DiagnosticLevel},
    utils::{Span, Token, TokenType as Ty},
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
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

    pub fn identify_tokens(mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.index <= self.source.len() {
            if self.peek().is_none() {
                self.advance();
                tokens.push(Token::new(
                    Ty::Eof,
                    Span::new(self.index - 1, self.index - 1),
                ));
                continue;
            };
            while self.peek().unwrap().is_whitespace() {
                self.advance();
            }
            let ch = self.peek().unwrap();
            let start = self.index;
            self.advance();
            let token: Ty = match ch {
                '\n' => Ty::Eol,
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
                    let mut ty: Ty = Ty::Unknown(ch);
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
                '_' | 'a'..='z' | 'A'..='Z' => {
                    while self.peek().is_some_and(|x| x.is_alphanumeric() || x == '_') {
                        self.advance();
                    }
                    match str::from_utf8(&self.source[start..self.index]).unwrap() {
                        "var" => Ty::KVariable,
                        "mut" => Ty::KMutable,
                        "const" => Ty::KConstant,
                        "struct" => Ty::KStruct,
                        "class" => Ty::KClass,
                        id => Ty::Identifier(id.to_string()),
                    }
                }
                '.' | '0'..='9' => {
                    while self.peek().is_some_and(|x| x.is_numeric() || x == '.') {
                        self.advance();
                    }
                    let num = str::from_utf8(&self.source[start..self.index]).unwrap();
                    let num_as_i64 = num.parse::<i64>().unwrap_or_else(|_| {
                        panic!("Failed to parse '{}' as an i64", num);
                    });
                    Ty::Number(num_as_i64)
                }
                _ => {
                    self.error(
                        format!("Unknown token used: '{}'", ch),
                        Span::new(self.index - 1, self.index - 1),
                    );
                    Ty::Unknown(ch)
                }
            };
            let end = self.index - 1;
            tokens.push(Token::new(token, Span::new(start, end)));
        }
        tokens
    }

    fn peek(&self) -> Option<char> {
        if self.index < self.source.len() {
            return str::from_utf8(self.source).unwrap().chars().nth(self.index);
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
