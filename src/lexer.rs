use core::str;

use crate::{
    compiler::Compiler,
    diagnostic::{Diagnostic, DiagnosticLevel, DiagnosticReporter},
    utils::{LiteralKind, Span, Token, TokenType as Ty},
};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Lexer<'a> {
    index: usize,
    source: &'a [u8],
    reporter: DiagnosticReporter,
}

impl<'a> Lexer<'a> {
    pub fn new(compiler: &'a Compiler) -> Self {
        Self {
            index: 0,
            source: compiler.source,
            reporter: DiagnosticReporter::new(),
        }
    }

    pub fn identify_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
                continue;
            }

            let start = self.index;
            self.advance();
            let token = match ch {
                ';' => Ty::Semicolon,
                '(' => Ty::LParen,
                ')' => Ty::RParen,
                '{' => Ty::LCurly,
                '}' => Ty::RCurly,
                '[' => Ty::LBoxed,
                ']' => Ty::RBoxed,

                '<' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Ty::LTEq
                    }
                    _ => Ty::LT,
                },

                '>' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Ty::GTEq
                    }
                    _ => Ty::GT,
                },

                '!' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Ty::NotEq
                    }
                    _ => Ty::Not,
                },

                '=' => match self.peek() {
                    Some('=') => {
                        self.advance();
                        Ty::DoubleEq
                    }
                    Some('>') => {
                        self.advance();
                        Ty::RightFatArrow
                    }
                    _ => Ty::Eq,
                },

                '+' => Ty::Plus,
                '-' => Ty::Minus,
                '*' => Ty::Asterisk,
                '/' => Ty::Slash,
                ':' => Ty::Colon,

                '\'' => match self.peek() {
                    Some(c) if c != '\\' => {
                        self.advance();
                        match self.peek() {
                            Some('\'') => {
                                self.advance();
                                Ty::Literal(LiteralKind::Char(c))
                            }
                            _ => {
                                self.error(
                                    "Expected end of char quote.",
                                    Span {
                                        start,
                                        end: self.index,
                                    },
                                );
                                Ty::Unknown
                            }
                        }
                    }
                    _ => Ty::Unknown,
                },

                '"' => {
                    // TODO: Implement string literal parsing
                    todo!()
                }

                '_' | 'a'..='z' | 'A'..='Z' => self.identify_keyword_or_id(start),
                '0'..='9' => self.identify_number(start),

                _ => {
                    self.error("Unknown token used: '{ch}'", Span { start, end: start });
                    Ty::Unknown
                }
            };

            tokens.push(Token::new(
                token,
                Span {
                    start,
                    end: self.index - 1,
                },
            ));
        }

        if self.reporter.has_error() {
            self.reporter
                .report(str::from_utf8(&self.source).ok().unwrap());
        }
        tokens
    }

    fn identify_keyword_or_id(&mut self, start: usize) -> Ty {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        match str::from_utf8(&self.source[start..self.index]).unwrap() {
            "isize" => Ty::KISIZE,
            "i128" => Ty::KI128,
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
        let mut has_dot = false;

        while let Some(ch) = self.peek() {
            match ch {
                '0'..='9' => self.advance(),
                '.' if !has_dot => {
                    if self.peek_front(0) == Some('.') {
                        break;
                    }
                    has_dot = true;
                    self.advance();
                }
                _ => break,
            }
        }

        let num = String::from_utf8_lossy(&self.source[start..self.index]).to_string();

        if has_dot {
            let num: f64 = num.parse().ok().unwrap_or_default();
            Ty::Literal(LiteralKind::Float(num))
        } else {
            let num: i64 = num.parse().ok().unwrap_or_default();
            Ty::Literal(LiteralKind::Integer(num))
        }
    }

    fn peek(&self) -> Option<char> {
        str::from_utf8(&self.source[self.index..])
            .ok()
            .and_then(|s| s.chars().next())
    }

    fn peek_front(&self, offset: usize) -> Option<char> {
        str::from_utf8(&self.source[self.index..])
            .ok()
            .and_then(|s| s.chars().nth(offset + 1))
    }

    fn advance(&mut self) {
        if let Some(ch) = self.peek() {
            self.index += ch.len_utf8();
        }
    }

    fn error(&mut self, message: impl Into<String>, span: Span) {
        self.reporter.add(Diagnostic::new(
            DiagnosticLevel::Error,
            message.into(),
            span,
        ));
    }
}
