#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum TokenType {
    LParen,
    RParen,
    LCurly,
    RCurly,
    LBoxed,
    RBoxed,
    LT,
    GT,
    Eq,
    DoubleEq,
    Not,
    NotEq,
    LTEq,
    GTEq,
    RightFatArrow,
    Plus,
    Minus,
    Asterisk,
    Slash,

    KVariable,
    KMutable,
    KConstant,
    KStruct,
    KClass,

    Number(i64), // TODO: Have full `Literal` implementation.

    Identifier(String),

    Unknown(char),

    Eol,
    Eof,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Token {
    pub ty: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(ty: TokenType, span: Span) -> Self {
        Self { ty, span }
    }
}
