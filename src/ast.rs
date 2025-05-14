#[derive(Debug)]
pub struct AST {
    statements: Vec<Statement>,
}

impl AST {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
    pub fn add_stmt(&mut self, stmt: Statement) {
        self.statements.push(stmt);
    }
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expr),
    VarDecl {
        name: String,
        is_mutable: bool,
        ty: Type,
        expr: Vec<Expr>,
    },
    Scope {
        name: String,
        ty: ScopeType,
    },
    Unknown,
}

#[derive(Debug)]
pub enum Expr {
    Number {
        is_float: bool,
        num: String,
    },
    BinaryOp {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Variable(String),
    Unknown,
}

#[derive(Debug)]
pub enum Type {
    Int { is_signed: bool, size: Size },
    Float { is_signed: bool, size: Size },
    Char,
    Bool,
    Unknown,
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Unknown,
}

#[derive(Debug)]
pub enum Size {
    Long = 64,
    Normal = 32,
    Short = 16,
    ShortShort = 8,
}

#[derive(Debug)]
pub enum ScopeType {
    Function {
        parameter: Vec<Parameter>,
        body: Vec<Statement>,
    },
    Class,
    Struct,
    Unknown,
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub ty: Type,
}
