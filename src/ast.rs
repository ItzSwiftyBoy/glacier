#[derive(Debug)]
pub struct AST<'a> {
    elements: Vec<Element<'a>>,
}

impl<'a> AST<'a> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn add_function(&mut self, name: String, param: Vec<Parameter>, body: Vec<Statement<'a>>) {
        self.add_element(Element::FuncScope { name, param, body });
    }

    pub fn add_const_element(&mut self, name: &'a String, ty: Type, expr: Expr) {
        self.add_element(Element::Constant { name, ty, expr });
    }

    pub fn add_element(&mut self, stmt: Element<'a>) {
        self.elements.push(stmt);
    }
}

#[derive(Debug)]
pub enum Element<'a> {
    FuncScope {
        name: String,
        param: Vec<Parameter>,
        body: Vec<Statement<'a>>,
    },
    Constant {
        name: &'a String,
        ty: Type,
        expr: Expr,
    },
    Unknown,
}

#[derive(Debug)]
pub enum Statement<'a> {
    VarDecl {
        name: &'a String,
        ty: Type,
        expr: Option<Expr>,
    },
}

#[derive(Debug)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Expression {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Var(String),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Subtract,
    Multiplicate,
    Divide,
}

#[derive(Debug)]
pub enum Type {
    Int8 { is_unsigned: bool },
    Int16 { is_unsigned: bool },
    Int32 { is_unsigned: bool },
    Int64 { is_unsigned: bool },
    Int128 { is_unsigned: bool },
    Float32,
    Float64,
    Unknown,
}

/* #[derive(Debug)]
pub enum Size {
    TooShort = 8,
    Short = 16,
    Normal = 32,
    Long = 64,
    TooLong = 128,
    TooMuchLong = 256,
} */

#[derive(Debug)]
pub struct Parameter {
    name: String,
    ty: Type,
}
