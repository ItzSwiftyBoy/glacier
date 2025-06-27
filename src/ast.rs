#[derive(Debug)]
pub struct AST {
    elements: Vec<Element>,
}

impl AST {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    // pub fn add_function(&mut self, name: String, param: Vec<Parameter>, body: Vec<Statement>) {
    //     self.add_element(Element::FuncScope { name, param, body });
    // }

    // pub fn add_const_element(&mut self, name: String, ty: Type, expr: Expr) {
    //     self.add_element(Element::Constant { name, ty, expr });
    // }

    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
    }
}

#[derive(Debug)]
pub enum Element {
    FuncScope {
        name: String,
        param: Vec<Parameter>,
        body: Vec<Statement>,
    },
    Constant {
        name: String,
        ty: String,
        expr: Expr,
    },
    Unknown,
}

#[derive(Debug)]
pub enum Statement {
    VarDecl {
        name: String,
        ty: String,
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
pub struct Parameter {
    name: String,
    ty: String,
}
