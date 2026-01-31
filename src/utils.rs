use std::{fmt::Display, path::Path};

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

    pub fn get_filename(&self, compiler: &'a Compiler) -> &'a Path {
        compiler.get_module_filepath(self.file_id)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}->{}]", self.start, self.end)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub enum TokenType {
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

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenType::LParen => "(",
                TokenType::RParen => ")",
                TokenType::LCurly => "{",
                TokenType::RCurly => "}",
                TokenType::LBoxed => "[",
                TokenType::RBoxed => "]",
                TokenType::Dot => ".",
                TokenType::DoubleDot => "..",
                TokenType::LT => "<",
                TokenType::GT => ">",
                TokenType::Eq => "=",
                TokenType::DoubleEq => "==",
                TokenType::Not => "!",
                TokenType::NotEq => "!=",
                TokenType::LTEq => "<=",
                TokenType::GTEq => ">=",
                TokenType::RightFatArrow => "=>",
                TokenType::Plus => "+",
                TokenType::Minus => "-",
                TokenType::Asterisk => "*",
                TokenType::Slash => "/",
                TokenType::Colon => ":",
                TokenType::Comma => ",",
                TokenType::KVariable => "var",
                TokenType::KMutable => "mut",
                TokenType::KConstant => "const",
                TokenType::KFunction => "func",
                TokenType::KStruct => "struct",
                TokenType::KClass => "class",
                TokenType::Integer(int) => int,
                TokenType::Float(float) => float,
                TokenType::Char(ch) => ch,
                TokenType::String(string) => string,
                TokenType::Semicolon => ";",
                TokenType::Eof => "<EOF>",
                TokenType::RightArrow => "=>",
                TokenType::KReturn => "return",
                TokenType::Identifier(ident) => ident,
                TokenType::Unknown => "<UNKNOWN>",
            }
        )
    }
}

impl PartialEq<&TokenType> for TokenType {
    fn eq(&self, other: &&TokenType) -> bool {
        *self == **other
    }
}

impl PartialEq<&Token> for TokenType {
    fn eq(&self, other: &&Token) -> bool {
        *self == other.ty
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct Token {
    pub ty: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(ty: TokenType, span: Span) -> Self {
        Self { ty, span }
    }

    pub fn is_eof(&self) -> bool {
        self.ty == TokenType::Eof
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.ty, self.span)
    }
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        self.ty == *other
    }
}

/* #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Symbol {
    Function {
        name: String,
        params: Vec<Parameter>,
        return_ty: Type,
        span: Span,
    },
}

#[derive(Debug)]
pub struct Project {
    symbols: Vec<Symbol>,
}

impl Project {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }

    pub fn find_func(&self, name: String) -> Option<&Symbol> {
        self.symbols.iter().find()
    }
} */
