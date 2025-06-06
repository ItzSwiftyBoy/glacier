#[derive(Debug, Default, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
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
    KI128,
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

    #[default]
    Unknown,

    Semicolon,
    // Eof,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum LiteralKind {
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(ty: TokenType, span: Span) -> Self {
        Self { ty, span }
    }
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
    ty: ElementType,
    locals: LocalSymbol,
}

#[derive(Debug)]
pub enum ElementType {
    Const,
    Func,
}

#[derive(Debug)]
pub struct LocalSymbol {
    name: String,
    ty: SizeType,
}

#[derive(Debug)]
pub enum SizeType {
    Int8 { is_unsigned: bool },
    Int16 { is_unsigned: bool },
    Int32 { is_unsigned: bool },
    Int64 { is_unsigned: bool },
    Int128 { is_unsigned: bool },
    Float32,
    Float64,
    Unknown,
}
