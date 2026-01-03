#![allow(dead_code)]

use crate::utils::Token;

#[derive(Debug)]
pub struct Ast {
    items: Vec<Item>,
}

impl Ast {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    // pub fn add_function(&mut self, name: String, param: Vec<Parameter>, body: Vec<Statement>) {
    //     self.add_element(Element::FuncScope { name, param, body });
    // }

    // pub fn add_const_element(&mut self, name: String, ty: Type, expr: Expr) {
    //     self.add_element(Element::Constant { name, ty, expr });
    // }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }
}

#[derive(Debug)]
pub enum Item {
    Func(Function),
    Unknown,
}
#[derive(Debug)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Parameter>,
    pub return_ty: Option<Token>,
    pub body: Block,
}

#[derive(Debug)]
pub struct Block {
    statements: Vec<Statement>,
    expressions: Vec<Expr>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
            expressions: Vec::new(),
        }
    }

    pub fn push_stmt(&mut self, stmt: Statement) {
        self.statements.push(stmt);
    }

    pub fn push_expr(&mut self, expr: Expr) {
        self.expressions.push(expr);
    }
}

#[derive(Debug)]
pub enum Statement {
    Var {
        name: Token,
        ty: Option<Token>,
        expr: Option<Expr>,
    },
    Unknown,
}
/* Constant {
    name: String,
    ty: String,
    expr: Expr,
},
VarDecl {
    name: String,
    ty: Option<Token>,
    expr: Option<Expr>,
},
Unknown */

#[derive(Debug)]
pub enum Expr {
    Binary {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        rhs: Box<Expr>,
    },
    Literal(Token),
    Grouping(Box<Expr>),
    Unknown,
}

#[derive(Debug)]
pub enum BinOp {
    // Main Binary Operations
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /

    // Binary Comparison Operations
    Eq,     // ==
    NotEq,  // !=
    GTOrEq, // >=
    LTOrEq, // <=
    GT,     // >
    LT,     // <
}

/// Binary Unary Operations
#[derive(Debug)]
pub enum UnaryOp {
    Not,      // !
    Negative, // -

    Unknown,
}

#[derive(Debug)]
pub struct Parameter {
    pub name: Token,
    pub ty: Token,
}
