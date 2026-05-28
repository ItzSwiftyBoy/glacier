use crate::{
    compiler::Compiler,
    diagnostic::Diagnostic,
    errors::{error, CompilerError},
    utils::{Span, Token, TokenKind as Ty, TokenStream},
};

pub struct Lexer<'a> {
    index: usize,
    source: &'a str,
    compiler: &'a Compiler,
    match_brackets: Vec<(Ty, usize)>,
}

impl<'a> Lexer<'a> {
    pub fn new(compiler: &'a Compiler) -> Self {
        Self {
            index: 0,
            source: &compiler.curr_source_content,
            compiler,
            match_brackets: vec![],
        }
    }

    pub fn identify_tokens(mut self) -> Option<TokenStream> {
        let mut stream = TokenStream::new();
        while let Some(v) = self.next() {
            match v {
                Ok(token) => stream.push(token),
                Err(diagnostic) => self.push_diag(diagnostic),
            }
        }

        self.check_unmatched_brackets();

        if self.compiler.reporter.borrow().has_error() {
            None
        } else {
            Some(stream)
        }
    }

    fn start_paren(&mut self) -> Ty {
        self.match_brackets.push((Ty::LParen, self.index - 1));
        Ty::LParen
    }

    fn start_curly(&mut self) -> Ty {
        self.match_brackets.push((Ty::LCurly, self.index - 1));
        Ty::LCurly
    }

    fn start_boxed(&mut self) -> Ty {
        self.match_brackets.push((Ty::LBoxed, self.index - 1));
        Ty::LBoxed
    }

    /// `match_paren()` is a function that will do the check if the parentheses have matched properly or is there any parentheses extra.
    fn match_paren(&mut self) -> Ty {
        let poped_bracket = self.match_brackets.pop();
        if poped_bracket.is_none() {
            self.push_diag(error(
                CompilerError::ExtraParen,
                self.span(self.index - 1, self.index - 1),
            ));
        } else if poped_bracket.as_ref().unwrap().0 != Ty::LParen {
            let expected_bracket = poped_bracket.unwrap().0.get_opposite_bracket();
            self.push_diag(error(
                CompilerError::UnexpectedParen(expected_bracket),
                self.span(self.index - 1, self.index - 1),
            ));
        }
        Ty::RParen
    }

    /// `match_curly()` is a function that will do the check if the curly brackets have matched properly or is there any curly bracket extra.
    fn match_curly(&mut self) -> Ty {
        let poped_bracket = self.match_brackets.pop();
        if let Some(bracket) = poped_bracket {
            if bracket.0 != Ty::LCurly {
                let expected_bracket = bracket.0.get_opposite_bracket();
                self.push_diag(error(
                    CompilerError::UnexpectedCurly(expected_bracket),
                    self.span(self.index - 1, self.index - 1),
                ));
            }
        } else {
            self.push_diag(error(
                CompilerError::ExtraCurly,
                self.span(self.index - 1, self.index - 1),
            ));
        }
        Ty::RCurly
    }

    /// `match_boxed()` is a function that will do the check if the square brackets have matched properly or is there any square bracket extra.
    fn match_boxed(&mut self) -> Ty {
        let poped_bracket = self.match_brackets.pop();
        if let Some(bracket) = poped_bracket {
            if bracket.0 != Ty::LBoxed {
                let expected_bracket = bracket.0.get_opposite_bracket();
                self.push_diag(error(
                    CompilerError::UnexpectedBoxed(expected_bracket),
                    self.span(self.index - 1, self.index - 1),
                ));
            }
        } else {
            self.push_diag(error(
                CompilerError::ExtraBoxed,
                self.span(self.index - 1, self.index - 1),
            ));
        }
        Ty::RBoxed
    }

    /// `check_unmatched_brackets()` does the checking of unmatched parentheses, curly, and square brackets. If found it will throw an error.
    fn check_unmatched_brackets(&mut self) {
        while let Some(bracket) = self.match_brackets.pop() {
            let ty = bracket.0;
            let span = bracket.1;
            if ty == Ty::LParen {
                self.push_diag(error(CompilerError::UnmatchedParen, self.span(span, span)))
            } else if ty == Ty::LCurly {
                self.push_diag(error(CompilerError::UnmatchedCurly, self.span(span, span)))
            } else if ty == Ty::LBoxed {
                self.push_diag(error(CompilerError::UnmatchedBoxed, self.span(span, span)))
            }
        }
    }

    fn span(&self, start: usize, end: usize) -> Span {
        Span::new(start, end)
    }

    /// Checks if the source has any keyword or `IDENT`.
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
            "return" => Ty::KReturn,
            "func" => Ty::KFunction,
            "struct" => Ty::KStruct,
            "class" => Ty::KClass,

            "int8" => Ty::KInt(8),
            "int16" => Ty::KInt(16),
            "int32" => Ty::KInt(32),
            "int64" => Ty::KInt(64),
            "int128" => Ty::KInt(128),
            "isize" => Ty::KISize,

            "uint8" => Ty::KUInt(8),
            "uint16" => Ty::KUInt(16),
            "uint32" => Ty::KUInt(32),
            "uint64" => Ty::KUInt(64),
            "uint128" => Ty::KUInt(128),
            "usize" => Ty::KUSize,

            "f16" => Ty::KFloat(16),
            "f32" => Ty::KFloat(32),
            "f64" => Ty::KFloat(64),

            "mod" => Ty::KMod,

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
                                return Err(error(
                                    CompilerError::InvalidHexEsc,
                                    self.span(start, self.index),
                                ));
                            }
                        } else {
                            return Err(error(
                                CompilerError::IncompleteHexEsc,
                                self.span(start, self.index),
                            ));
                        }
                    }
                    Some('u') => {
                        // Unicode escapes (example): \u{1F600}
                        if self.peek() != Some('{') {
                            return Err(error(
                                CompilerError::ExpectedLCurlyAfterUniEsc,
                                self.span(start, self.index),
                            ));
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
                                return Err(error(
                                    CompilerError::InvalidUniCodePoint,
                                    self.span(start, self.index),
                                ));
                            }
                        } else {
                            return Err(error(
                                CompilerError::InvalidUniEsc,
                                self.span(start, self.index),
                            ));
                        }
                    }
                    Some(c) => {
                        return Err(error(
                            CompilerError::UnknownEsc(c),
                            self.span(start, self.index),
                        ))
                    }
                    None => {
                        return Err(error(
                            CompilerError::UnexpectedEOF,
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

    fn push_diag(&mut self, diagnostic: Diagnostic) {
        self.compiler
            .reporter
            .borrow_mut()
            .add(diagnostic, self.compiler.get_curr_file_id());
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Diagnostic>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index == self.source.len() {
                self.advance();
                return Some(Ok(Token::new(
                    Ty::Eof,
                    self.span(self.index - 1, self.index - 1),
                )));
            }
            if let Some(c) = self.peek() {
                if c == '\n' {
                    self.advance();
                    return Some(Ok(Token::new(
                        Ty::Newline,
                        self.span(self.index - 1, self.index - 1),
                    )));
                } else if c.is_whitespace() {
                    self.advance();
                    continue;
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
                '(' => self.start_paren(),
                ')' => self.match_paren(),
                '{' => self.start_curly(),
                '}' => self.match_curly(),
                '[' => self.start_boxed(),
                ']' => self.match_boxed(),

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
                    _ => Ty::Bang,
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
                                Ty::Char(c.to_string())
                            }
                            _ => {
                                error(
                                    CompilerError::UnmatchedSingleQuote,
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
                    return Some(Err(error(
                        CompilerError::UnknownChar(c),
                        self.span(start, start),
                    )))
                }
            };
        }
        let end = self.index - 1;
        Some(Ok(Token::new(ty, self.span(start, end))))
    }
}
