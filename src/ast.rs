#![allow(dead_code)]

use std::fmt::Display;

use crate::{
    printer::{AstPrinter, Visitor},
    utils::Token,
};

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

    pub fn visit(&self, visitor: &mut dyn Visitor) {
        for item in &self.items {
            visitor.visit_item(item);
        }
    }

    pub fn dump(&self) {
        let mut printer = AstPrinter::new();
        self.visit(&mut printer);
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

// impl Display for Function {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let return_ty = format!(
//             "{}",
//             match self.return_ty {
//                 Some(token) => &format!("{}", token),
//                 None => "None",
//             }
//         );
//         write!(
//             f,
//             "{}({:?}) -> {} {{{}}}",
//             self.name, self.params, return_ty, self.body
//         )
//     }
// }

#[derive(Debug)]
pub struct Block(pub Vec<Statement>);

impl Block {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push_stmt(&mut self, stmt: Statement) {
        self.0.push(stmt);
    }
}

// impl Iterator for Block {
//     type Item = Statement;
//     fn next(&mut self) -> Option<Self::Item> {
//         for stmt in self.0.iter().clone() {
//             return Some(stmt);
//         }
//         None
//     }
// }

#[derive(Debug)]
pub enum Statement {
    VarDecl {
        name: Token,
        ty: Option<Token>,
        expr: Expr,
    },
    Return(Expr),
    Expression(Expr),
    Unknown,
}
/* Constant {
    name: String,
    ty: String,
    expr: Expr,
},
*/

#[derive(Debug, PartialEq)]
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
    Var(Token),
    Grouping(Box<Expr>),
    None,
    Unknown,
}

#[derive(Debug, PartialEq)]
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

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BinOp::Add => "+",
                BinOp::Subtract => "-",
                BinOp::Multiply => "*",
                BinOp::Divide => "/",
                BinOp::Eq => "==",
                BinOp::NotEq => "!=",
                BinOp::GTOrEq => ">=",
                BinOp::GT => ">",
                BinOp::LTOrEq => "<=",
                BinOp::LT => "<",
            }
        )
    }
}

/// Binary Unary Operations
#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Negate,   // !
    Negative, // -

    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Parameter {
    pub name: Token,
    pub ty: Token,
}

impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}
