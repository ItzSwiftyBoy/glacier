#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
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
    Plus,
    Minus,
    Asterisk,
    Slash,
    Colon,

    KISIZE,
    KI64,
    KI32,
    KI16,
    KI8,
    KVariable,
    KMutable,
    KConstant,
    KFunction,
    KStruct,
    KClass,

    Literal(LiteralKind), // TODO: Have full `Literal` implementation.

    Identifier(String),

    Unknown,

    Semicolon,
    // Eof,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum LiteralKind {
    Integer(String),
    Float(String),
    Char(char),
    String(String),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Token(pub TokenType, pub Span);

impl Token {
    pub fn new(ty: TokenType, span: Span) -> Self {
        Self(ty, span)
    }
}
