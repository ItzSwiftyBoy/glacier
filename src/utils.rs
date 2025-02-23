#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Span {
    start: usize,
    end: usize,
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
    LessThan,
    GreaterThan,
    Equal,
    EqualEqual,
    Not,
    NotEqual,
    LessThanEqual,
    GreaterThanEqual,
    RightFatArrow,

    KVariable,
    KMutable,
    KConstant,
    KStruct,
    KClass,

    Number(String), // TODO: Make String -> i64

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
