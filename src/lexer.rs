use crate::{
    diagnostic::{Diagnostic, DiagnosticLevel, DiagnosticReporter},
    utils::{Span, Token, TokenType as Ty},
};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Lexer<'a> {
    source: &'a str,
    cursor_index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            cursor_index: 0,
        }
    }

    pub fn identify_tokens(mut self) -> (Vec<Token>, DiagnosticReporter) {
        let mut tokens: Vec<Token> = Vec::new();
        let mut diagnostics = DiagnosticReporter::new();
        while self.cursor_index <= self.source.len() {
            if self.peek().is_none() {
                self.advance();
                tokens.push(Token::new(
                    Ty::Eof,
                    Span::new(self.cursor_index - 1, self.cursor_index - 1),
                ));
                continue;
            };
            while self.peek().unwrap().is_whitespace() {
                self.advance();
            }
            let ch = self.peek().unwrap();
            let start = self.cursor_index;
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
                    let ty: Ty;
                    if self.peek().is_some_and(|x| x == '=') {
                        self.advance();
                        ty = Ty::LessThanEqual;
                    } else {
                        ty = Ty::LessThan;
                    }
                    ty
                }
                '>' => {
                    let ty: Ty;
                    if self.peek().is_some_and(|x| x == '=') {
                        self.advance();
                        ty = Ty::GreaterThanEqual;
                    } else {
                        ty = Ty::GreaterThan;
                    }
                    ty
                }
                '!' => {
                    let ty: Ty;
                    if self.peek().is_some_and(|x| x == '=') {
                        self.advance();
                        ty = Ty::NotEqual;
                    } else {
                        ty = Ty::Not;
                    }
                    ty
                }
                '=' => {
                    let mut ty: Ty = Ty::Unknown('\0');
                    if self.peek().is_some() {
                        if self.peek() == Some('=') {
                            self.advance();
                            ty = Ty::EqualEqual;
                        } else if self.peek() == Some('>') {
                            self.advance();
                            ty = Ty::RightFatArrow;
                        } else {
                            ty = Ty::Equal;
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
                    match &self.source[start..self.cursor_index] {
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
                    let num = &self.source[start..=self.cursor_index];
                    let num_as_i64 = num.parse::<i64>().unwrap_or_else(|_| {
                        panic!("Failed to parse '{}' as an i64", num);
                    });
                    Ty::Number(num_as_i64)
                }
                _ => {
                    diagnostics.add(Diagnostic::new(
                        DiagnosticLevel::Error,
                        format!("Unknown token used: '{}'", ch),
                        Span::new(self.cursor_index - 1, self.cursor_index - 1),
                    ));
                    Ty::Unknown(ch)
                }
            };
            let end = self.cursor_index;
            tokens.push(Token::new(token, Span::new(start, end)));
        }
        (tokens, diagnostics)
    }

    fn peek(&self) -> Option<char> {
        if self.cursor_index < self.source.len() {
            return self.source.chars().nth(self.cursor_index);
        };
        None
    }

    fn advance(&mut self) {
        self.cursor_index += 1;
    }
}
