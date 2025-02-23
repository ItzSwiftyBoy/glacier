use crate::utils::{Span, Token, TokenType as Ty};

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
    pub fn next_token(&'a mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.cursor_index <= self.source.len() {
            tokens.push(self.identify_token());
        }
        tokens
    }
    fn identify_token(&mut self) -> Token {
        if self.peek().is_some() {
            while self.peek().unwrap().is_whitespace() {
                self.advance();
            }
            let ch = self.peek().unwrap();
            let pindex = self.cursor_index;
            if ch == '\n' {
                self.advance();
                Token::new(Ty::Eol, Span::new(pindex, pindex))
            } else if ch == '(' {
                self.advance();
                return Token::new(Ty::LParen, Span::new(pindex, pindex));
            } else if ch == ')' {
                self.advance();
                return Token::new(Ty::RParen, Span::new(pindex, pindex));
            } else if ch == '{' {
                self.advance();
                return Token::new(Ty::LCurly, Span::new(pindex, pindex));
            } else if ch == '}' {
                self.advance();
                return Token::new(Ty::RCurly, Span::new(pindex, pindex));
            } else if ch == '[' {
                self.advance();
                return Token::new(Ty::LBoxed, Span::new(pindex, pindex));
            } else if ch == ']' {
                self.advance();
                return Token::new(Ty::RBoxed, Span::new(pindex, pindex));
            } else if ch == '<' {
                if self.peek_front().is_some_and(|x| x == '=') {
                    self.advance();
                    self.advance();
                    return Token::new(Ty::LessThanEqual, Span::new(pindex, pindex + 1));
                }
                self.advance();
                return Token::new(Ty::LessThan, Span::new(pindex, pindex));
            } else if ch == '>' {
                if self.peek_front().is_some_and(|x| x == '=') {
                    self.advance();
                    self.advance();
                    return Token::new(Ty::GreaterThanEqual, Span::new(pindex, pindex + 1));
                }
                self.advance();
                return Token::new(Ty::GreaterThan, Span::new(pindex, pindex));
            } else if ch == '!' {
                if self.peek_front().is_some_and(|x| x == '=') {
                    self.advance();
                    self.advance();
                    return Token::new(Ty::NotEqual, Span::new(pindex, pindex + 1));
                }
                self.advance();
                return Token::new(Ty::Not, Span::new(pindex, pindex));
            } else if ch == '=' {
                if self.peek_front().is_some() {
                    if self.peek_front() == Some('=') {
                        self.advance();
                        self.advance();
                        return Token::new(Ty::EqualEqual, Span::new(pindex, pindex + 1));
                    } else if self.peek_front() == Some('>') {
                        self.advance();
                        self.advance();
                        return Token::new(Ty::RightFatArrow, Span::new(pindex, pindex + 1));
                    }
                }
                self.advance();
                return Token::new(Ty::Equal, Span::new(pindex, pindex));
            } else if ch.is_alphabetic() || ch == '_' {
                let mut word = String::new();
                let start = self.cursor_index;
                word.push(ch);
                while self
                    .peek_front()
                    .is_some_and(|x| x.is_alphanumeric() || x == '_')
                {
                    self.advance();
                    word.push(self.peek().unwrap());
                }
                let end = self.cursor_index;
                self.advance();
                return Token::new(
                    match word.as_str() {
                        "var" => Ty::KVariable,
                        "mut" => Ty::KMutable,
                        "const" => Ty::KConstant,
                        "struct" => Ty::KStruct,
                        "class" => Ty::KClass,
                        _ => Ty::Identifier(word.to_string()),
                    },
                    Span::new(start, end),
                );
            } else if ch.is_numeric() {
                let mut num = String::new();
                let start = self.cursor_index;
                num.push(ch);
                while self
                    .peek_front()
                    .is_some_and(|x| x.is_numeric() || x == '.')
                {
                    self.advance();
                    num.push(self.peek().unwrap());
                }
                let end = self.cursor_index;
                self.advance();
                return Token::new(Ty::Number(num), Span::new(start, end));
            } else {
                self.advance();
                Token::new(Ty::Unknown(ch), Span::new(pindex, pindex))
            }
        } else {
            self.advance();
            Token::new(
                Ty::Eof,
                Span::new(self.cursor_index - 1, self.cursor_index - 1),
            )
        }
    }
    fn peek(&self) -> Option<char> {
        if self.cursor_index < self.source.len() {
            return self.source.chars().nth(self.cursor_index);
        };
        None
    }
    fn peek_front(&self) -> Option<char> {
        if (self.cursor_index + 1) < self.source.len() {
            return self.source.chars().nth(self.cursor_index + 1);
        };
        None
    }
    fn advance(&mut self) {
        self.cursor_index += 1;
    }
}
