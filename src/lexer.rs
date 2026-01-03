use crate::{
    compiler::Compiler,
    diagnostic::{Diagnostic, DiagnosticKind},
    utils::{Span, Token, TokenType as Ty},
};

#[derive(Debug)]
pub struct Lexer<'a> {
    index: usize,
    source: &'a str,
    compiler: &'a Compiler,
}

impl<'a> Lexer<'a> {
    pub fn new(compiler: &'a Compiler) -> Self {
        Self {
            index: 0,
            source: &compiler.curr_source,
            compiler,
        }
    }

    pub fn identify_tokens(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(v) = self.next() {
            match v {
                Ok(token) => tokens.push(token),
                Err(diagnostic) => self.compiler.reporter.borrow_mut().add(diagnostic),
            }
        }

        tokens
    }

    fn span(&self, start: usize, end: usize) -> Span {
        Span::new(start, end, self.compiler.get_curr_file_id())
    }

    fn identify_keyword_or_id(&mut self, start: usize) -> Ty {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        match &self.source[start..self.index] {
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
                    self.advance();
                    if self.peek() == Some('.') {
                        self.advance();
                        return Ty::DoubleDot;
                    }
                    has_dot = true;
                    self.advance();
                }
                _ => break,
            }
        }

        if has_dot {
            Ty::Float(self.source[start..self.index].to_string())
        } else {
            Ty::Integer(self.source[start..self.index].to_string())
        }
    }

    fn identify_string_literal(&mut self) -> Result<String, Diagnostic> {
        let mut result = String::new();

        while let Some(c) = self.peek() {
            let start = self.index;
            if c == '\\' {
                self.advance();
                match self.peek() {
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some('x') => {
                        // e.g., \x41 => 'A'
                        let hex1 = self.peek();
                        let hex2 = self.peek();
                        if let (Some(h1), Some(h2)) = (hex1, hex2) {
                            let hex_str = format!("{}{}", h1, h2);
                            if let Ok(byte) = u8::from_str_radix(&hex_str, 16) {
                                result.push(byte as char);
                            } else {
                                return Err(
                                    self.error("Invalid hex escape", self.span(start, self.index))
                                );
                            }
                        } else {
                            return Err(
                                self.error("Incomplete hex escape", self.span(start, self.index))
                            );
                        }
                    }
                    Some('u') => {
                        // Unicode escapes: \u{1F600}
                        if self.peek() != Some('{') {
                            return Err(
                                self.error("Expected '{' after \\u", self.span(start, self.index))
                            );
                        }
                        let mut unicode = String::new();
                        while let Some(next) = self.peek() {
                            if next == '}' {
                                self.peek();
                                break;
                            }
                            unicode.push(next);
                            self.peek();
                        }
                        if let Ok(code_point) = u32::from_str_radix(&unicode, 16) {
                            if let Some(c) = char::from_u32(code_point) {
                                result.push(c);
                            } else {
                                return Err(self.error(
                                    "Invalid Unicode code point",
                                    self.span(start, self.index),
                                ));
                            }
                        } else {
                            return Err(
                                self.error("Invalid Unicode escape", self.span(start, self.index))
                            );
                        }
                    }
                    Some(c) => {
                        return Err(self.error(
                            format!("Unknown escape: \\{}", c),
                            self.span(start, self.index),
                        ))
                    }
                    None => {
                        return Err(self.error(
                            "Unexpected end of input after \\",
                            self.span(start, self.index),
                        ))
                    }
                }
                self.advance();
            } else if c == '"' {
                self.advance();
                break;
            } else {
                self.advance();
                result.push(c);
            }
        }

        Ok(result)
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.index)
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn error(&mut self, message: impl Into<String>, span: Span) -> Diagnostic {
        Diagnostic::new(DiagnosticKind::Error, message.into(), span)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Diagnostic>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(c) = self.peek() {
                if c.is_whitespace() {
                    self.advance();
                    continue;
                } else if c == '\0' {
                    self.advance();
                    return Some(Ok(Token::new(
                        Ty::Eof,
                        self.span(self.index - 1, self.index - 1),
                    )));
                }
                break;
            } else {
                return None;
            }
        }

        let start = self.index;
        let mut ty: Ty = Ty::Unknown;
        if let Some(c) = self.peek() {
            self.advance();
            ty = match c {
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

                '.' => match self.peek() {
                    Some('.') => {
                        self.advance();
                        Ty::DoubleDot
                    }
                    _ => Ty::Dot,
                },

                '+' => Ty::Plus,
                '-' => match self.peek() {
                    Some('>') => {
                        self.advance();
                        Ty::RightArrow
                    }
                    _ => Ty::Minus,
                },
                '*' => Ty::Asterisk,
                '/' => Ty::Slash,
                ':' => Ty::Colon,
                ',' => Ty::Comma,

                '\'' => match self.peek() {
                    Some(c) if c != '\\' => {
                        self.advance();
                        match self.peek() {
                            Some('\'') => {
                                self.advance();
                                Ty::Char(c)
                            }
                            _ => {
                                self.error(
                                    "Expected end of char quote.",
                                    self.span(start, self.index),
                                );
                                Ty::Unknown
                            }
                        }
                    }
                    _ => Ty::Unknown,
                },

                '"' => match self.identify_string_literal() {
                    Ok(string) => {
                        return Some(Ok(Token::new(
                            Ty::String(string),
                            self.span(start, self.index - 3),
                        )))
                    }
                    Err(e) => return Some(Err(e)),
                },

                '_' | 'a'..='z' | 'A'..='Z' => self.identify_keyword_or_id(start),
                '0'..='9' => self.identify_number(start),

                _ => {
                    return Some(Err(self.error(
                        format!("{}: '{}'", "Unknown token used", c),
                        self.span(start, start),
                    )))
                }
            };
        }
        let end = self.index - 1;
        Some(Ok(Token::new(ty, self.span(start, end))))
    }
}
