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
    KFunction,
    KStruct,
    KClass,

    Integer(String),
    Float(String),
    Char(char),
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
                // Token::Integer(int, _) => int,
                // Token::Float(float, _) => float,
                // Token::Char(ch, _) => ch,
                // Token::String(string, _) => string,
                TokenType::Semicolon => ";",
                TokenType::Eof => "<EOF>",
                _ => unreachable!(),
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct Token(pub TokenType, pub Span);

impl Token {
    pub fn new(ty: TokenType, span: Span) -> Self {
        Self(ty, span)
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
