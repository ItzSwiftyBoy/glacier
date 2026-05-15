use std::{
    fmt::Display,
    ops::{AddAssign, SubAssign},
    path::Path,
};

use crate::compiler::Compiler;

pub type FileId = usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub file_id: FileId,
}

impl<'a> Span {
    pub fn new(start: usize, end: usize, file_id: FileId) -> Self {
        Self {
            start,
            end,
            file_id,
        }
    }

    pub fn len(&self) -> usize {
        (self.end - self.start) + 1
    }

    pub fn get_filename(&self, compiler: &'a Compiler) -> &'a Path {
        compiler.get_module_filepath(self.file_id)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len() == 1 {
            write!(f, "{}", self.start)
        } else {
            write!(f, "{}-{}", self.start, self.end)
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub enum TokenKind {
    // Punctuation Marks.
    LParen,
    RParen,
    LCurly,
    RCurly,
    LBoxed,
    RBoxed,
    Dot,
    DoubleDot,
    LT,
    GT,
    Eq,
    DoubleEq,
    Not,
    NotEq,
    LTEq,
    GTEq,
    RightFatArrow,
    RightArrow,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Colon,
    Comma,
    Bang,

    Semicolon,

    // Keywords.
    KVariable,
    KMutable,
    KConstant,
    KReturn,
    KFunction,
    KStruct,
    KClass,

    // Type Keywords. `usize` here should be the types size (8, 16, 32, 64, 128).
    KInt(usize),
    KISize,
    KUInt(usize),
    KUSize,
    KFloat(usize), // The size should be only 16, 32 or 64.

    // Literals.
    Integer(String),
    Float(String),
    Char(String),
    String(String),
    Identifier(String),

    // Specials.
    Unknown,
    Newline,
    #[default]
    Eof,
}

impl TokenKind {
    pub fn get_opposite_bracket(&self) -> char {
        match self {
            Self::LParen => ')',
            Self::LCurly => '}',
            Self::LBoxed => ']',
            Self::RParen => '(',
            Self::RCurly => '{',
            Self::RBoxed => '{',
            _ => unreachable!(),
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::LParen => "(",
                Self::RParen => ")",
                Self::LCurly => "{",
                Self::RCurly => "}",
                Self::LBoxed => "[",
                Self::RBoxed => "]",
                Self::Dot => ".",
                Self::DoubleDot => "..",
                Self::LT => "<",
                Self::GT => ">",
                Self::Eq => "=",
                Self::DoubleEq => "==",
                Self::Not => "!",
                Self::NotEq => "!=",
                Self::LTEq => "<=",
                Self::GTEq => ">=",
                Self::RightFatArrow => "=>",
                Self::Plus => "+",
                Self::Minus => "-",
                Self::Asterisk => "*",
                Self::Slash => "/",
                Self::Colon => ":",
                Self::Comma => ",",
                Self::Newline => "<NEWLINE>",
                Self::KVariable => "var",
                Self::KMutable => "mut",
                Self::KConstant => "const",
                Self::KFunction => "func",
                Self::KStruct => "struct",
                Self::KClass => "class",
                Self::KReturn => "return",
                Self::Integer(int) => int,
                Self::Float(float) => float,
                Self::Char(ch) => ch,
                Self::String(string) => string,
                Self::Semicolon => ";",
                Self::Eof => "<EOF>",
                Self::RightArrow => "=>",
                Self::Identifier(ident) => ident,
                Self::Unknown => "<UNKNOWN>",
                _ => unreachable!(),
            }
        )
    }
}

impl PartialEq<&TokenKind> for TokenKind {
    fn eq(&self, other: &&TokenKind) -> bool {
        *self == **other
    }
}

impl PartialEq<&Token> for TokenKind {
    fn eq(&self, other: &&Token) -> bool {
        *self == other.0
    }
}

impl PartialEq<Option<&Token>> for TokenKind {
    fn eq(&self, other: &Option<&Token>) -> bool {
        if let Some(token) = *other {
            *self == token.0
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct Token(pub TokenKind, pub Span);

impl Token {
    pub fn new(ty: TokenKind, span: Span) -> Self {
        Self(ty, span)
    }

    pub fn is_eof(&self) -> bool {
        self.0 == TokenKind::Eof
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl PartialEq<TokenKind> for Token {
    fn eq(&self, other: &TokenKind) -> bool {
        self.0 == *other
    }
}

pub struct TokenStream {
    stream: Vec<Token>,
    current: usize,
}

impl TokenStream {
    pub fn new() -> Self {
        Self {
            stream: vec![],
            current: 0,
        }
    }

    pub fn push(&mut self, token: Token) {
        self.stream.push(token);
    }

    pub fn peek(&self, offset: isize) -> &Token {
        &self.stream[(self.current as isize + offset) as usize]
    }

    pub fn previous(&self) -> &Token {
        self.peek(-1)
    }

    pub fn previous_ty(&self) -> &TokenKind {
        &self.previous().0
    }

    pub fn previous_span(&self) -> Span {
        self.previous().1
    }

    pub fn current(&self) -> &Token {
        self.peek(0)
    }

    pub fn current_ty(&self) -> &TokenKind {
        &self.current().0
    }

    pub fn current_span(&self) -> Span {
        self.current().1
    }

    pub fn advance(&mut self) -> &Token {
        self.current += 1;
        self.previous()
    }

    pub fn advance_ty(&mut self) -> &TokenKind {
        &self.advance().0
    }

    pub fn is_curr_token(&mut self, token_type: TokenKind) -> bool {
        token_type == self.current()
    }

    pub fn is_curr_token_int(&mut self) -> bool {
        matches!(self.current_ty(), TokenKind::Integer(_))
    }

    pub fn is_curr_token_float(&mut self) -> bool {
        matches!(self.current_ty(), TokenKind::Float(_))
    }

    pub fn is_curr_token_char(&mut self) -> bool {
        matches!(self.current_ty(), TokenKind::Char(_))
    }

    pub fn is_curr_token_string(&mut self) -> bool {
        matches!(self.current_ty(), TokenKind::String(_))
    }

    pub fn is_curr_token_ident(&mut self) -> bool {
        matches!(self.current_ty(), TokenKind::Identifier(_))
    }

    pub fn ignore_newline(&mut self) {
        while self.is_curr_token(TokenKind::Newline) {
            self.advance();
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current().is_eof()
    }
    /* pub fn from(tokens: Vec<Token>) -> Self {
        Self {
            stream: tokens,
            current: 0
        }
    } */
}

impl AddAssign<usize> for TokenStream {
    fn add_assign(&mut self, rhs: usize) {
        self.current += rhs;
    }
}

impl SubAssign<usize> for TokenStream {
    fn sub_assign(&mut self, rhs: usize) {
        self.current -= rhs;
    }
}
