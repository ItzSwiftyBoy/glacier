#![allow(dead_code)]

use std::fmt::Display;

use crate::{printer::AstPrinter, types::Type, utils::Token};

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

#[derive(Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Statement {
    VarDecl {
        name: Token,
        ty: Option<Token>,
        expr: Expr,
    },
    Return(Expr),
    Expression(Expr),
    Block(Block),
    Unknown,
}
/* Constant {
    name: String,
    ty: String,
    expr: Expr,
},
*/

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TypedExpr {
    expr: Expr,
    ty: Type,
}

impl TypedExpr {
    pub fn new(expr: Expr) -> Self {
        Self {
            expr,
            ty: Type::Unknown,
        }
    }

    pub fn ty(&mut self, ty: Type) {
        self.ty = ty
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
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
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Negate,   // !
    Negative, // -

    Unknown,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Parameter {
    pub name: Token,
    pub ty: Token,
}

impl Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}

pub trait Visitor {
    fn do_visit_item(&mut self, item: &Item) {
        match item {
            Item::Func(function) => {
                self.visit_func(function);
            }
            Item::Unknown => unimplemented!(),
        }
    }
    fn visit_item(&mut self, item: &Item) {
        self.do_visit_item(item);
    }
    // fn do_visit_func(&mut self, function: &Function) {
    //     self.visit_func(function);
    // }
    fn visit_func(&mut self, function: &Function);
    fn do_visit_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VarDecl { name, ty, expr } => self.visit_var_decl(name, ty, expr),
            Statement::Expression(expr) => self.visit_expr(expr),
            _ => unimplemented!(),
        }
    }
    fn visit_block(&mut self, block: &Block);
    fn visit_stmt(&mut self, stmt: &Statement) {
        self.do_visit_stmt(stmt);
    }
    fn visit_var_decl(&mut self, name: &Token, ty: &Option<Token>, expr: &Expr);
    fn do_visit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Binary { lhs, op, rhs } => self.visit_binary_expr(lhs, op, rhs),
            Expr::Var(v) => self.visit_ident(v),
            Expr::Literal(literal) => self.visit_literal(literal),
            _ => unimplemented!(),
        }
    }
    fn visit_expr(&mut self, expr: &Expr) {
        self.do_visit_expr(expr);
    }
    fn visit_binary_expr(&mut self, lhs: &Box<Expr>, op: &BinOp, rhs: &Box<Expr>);
    fn visit_ident(&mut self, ident: &Token);
    fn visit_literal(&mut self, literal: &Token);
}
