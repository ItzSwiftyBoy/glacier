use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    ffi::OsString,
    fmt::Display,
    ops::{AddAssign, SubAssign},
    path::Path,
};

use crate::{
    ast::{Function, Parameter},
    compiler::Compiler,
    types::Type,
};

pub type FileId = usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Default)]
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
        self.end - self.start
    }

    pub fn get_filename(&self, compiler: &'a Compiler) -> &'a Path {
        compiler.get_module_filepath(self.file_id)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}->{}]", self.start, self.end)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub enum TokenKind {
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
    Newline,

    KVariable,
    KMutable,
    KConstant,
    KReturn,
    KFunction,
    KStruct,
    KClass,

    Integer(String),
    Float(String),
    Char(String),
    String(String),

    Identifier(String),

    Unknown,

    Semicolon,
    #[default]
    Eof,
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
        *self == other.ty
    }
}

impl PartialEq<Option<&Token>> for TokenKind {
    fn eq(&self, other: &Option<&Token>) -> bool {
        if let Some(token) = *other {
            *self == token.ty
        } else {
            false
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub struct Token {
    pub ty: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(ty: TokenKind, span: Span) -> Self {
        Self { ty, span }
    }

    pub fn is_eof(&self) -> bool {
        self.ty == TokenKind::Eof
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.ty, self.span)
    }
}

impl PartialEq<TokenKind> for Token {
    fn eq(&self, other: &TokenKind) -> bool {
        self.ty == *other
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

    pub fn peek(&self, offset: isize) -> Option<&Token> {
        self.stream.get((self.current as isize + offset) as usize)
    }

    pub fn previous(&self) -> &Token {
        self.peek(-1).unwrap()
    }

    pub fn previous_ty(&self) -> &TokenKind {
        &self.previous().ty
    }

    pub fn previous_span(&self) -> Span {
        self.previous().span
    }

    pub fn current(&self) -> &Token {
        self.peek(0).unwrap()
    }

    pub fn current_ty(&self) -> &TokenKind {
        &self.current().ty
    }

    pub fn current_span(&self) -> Span {
        self.current().span
    }

    pub fn advance(&mut self) -> Option<&Token> {
        if self.is_at_end() {
            return None;
        }
        self.current += 1;
        Some(self.previous())
    }

    pub fn advance_ty(&mut self) -> Option<&TokenKind> {
        if let Some(token) = self.advance() {
            Some(&token.ty)
        } else {
            None
        }
    }

    pub fn is_curr_token(&self, token_type: TokenKind) -> bool {
        token_type == self.current()
    }

    pub fn is_curr_token_int(&self) -> bool {
        matches!(self.current_ty(), TokenKind::Integer(_))
    }

    pub fn is_curr_token_float(&self) -> bool {
        matches!(self.current_ty(), TokenKind::Float(_))
    }

    pub fn is_curr_token_char(&self) -> bool {
        matches!(self.current_ty(), TokenKind::Char(_))
    }

    pub fn is_curr_token_string(&self) -> bool {
        matches!(self.current_ty(), TokenKind::String(_))
    }

    pub fn is_curr_token_ident(&self) -> bool {
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
