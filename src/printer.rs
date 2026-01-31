use std::fmt::{write, Display};

use colored::Colorize;

use crate::{
    ast::{BinOp, Block, Expr, Function, Item, Statement},
    utils::{Token, TokenType as Ty},
};

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

pub struct AstPrinter {
    indent: usize,
}

const LEVEL_INDENT: usize = 2;

impl AstPrinter {
    pub fn new() -> Self {
        Self { indent: 0 }
    }

    fn print_with_indent(&self, text: &str) {
        println!("{}{}", " ".repeat(self.indent), text);
    }

    fn incr_indent(&mut self) {
        self.indent += LEVEL_INDENT
    }

    fn decr_indent(&mut self) {
        self.indent -= LEVEL_INDENT
    }
}

impl Visitor for AstPrinter {
    /* fn visit_item(&mut self, item: &Item) {
        self.do_visit_item(item);
        match item {
            Item::Func(func) => {
            }
            Item::Unknown => {
                println!("{}", "Invalid Item!".on_bright_red());
            }
        }
    } */

    fn visit_func(&mut self, function: &Function) {
        self.print_with_indent(&format!("{}: Function {{", function.name));
        self.incr_indent();
        if function.params.len() != 0 {
            self.print_with_indent("params: [");
            self.incr_indent();
            for param in &function.params {
                self.print_with_indent(&format!("{}", param));
            }
            self.decr_indent();
            self.print_with_indent("]");
        } else {
            self.print_with_indent("params: []");
        }
        if let Some(ty) = &function.return_ty {
            self.print_with_indent(&format!("return_type: {}", ty));
        } else {
            self.print_with_indent("return_type: ()");
        }
        self.visit_block(&function.body);
        self.decr_indent();
        self.print_with_indent(&format!("}}"));
    }

    fn visit_block(&mut self, block: &Block) {
        self.print_with_indent("body: {");
        self.incr_indent();
        for stmt in &block.0 {
            self.do_visit_stmt(stmt);
        }
        self.print_with_indent("}");
    }

    fn visit_var_decl(&mut self, name: &Token, ty: &Option<Token>, expr: &Expr) {
        self.print_with_indent("VarDecl: {");
        self.incr_indent();
        self.print_with_indent(&format!("name: {}", name));
        if let Some(t) = ty {
            self.print_with_indent(&format!("ty: {}", t));
        } else {
            self.print_with_indent("ty: ()");
        }
        if *expr != Expr::None {
            self.print_with_indent("expr: {");
            self.visit_expr(expr);
            self.print_with_indent("}");
        } else {
            self.print_with_indent("expr: ()");
        }
        self.decr_indent();
    }

    fn visit_binary_expr(&mut self, lhs: &Box<Expr>, op: &BinOp, rhs: &Box<Expr>) {
        self.print_with_indent("lhs: {");
        self.incr_indent();
        self.visit_expr(lhs);
        self.decr_indent();
        self.print_with_indent("}");
        self.print_with_indent(&format!("op: {}", op));
        self.print_with_indent("rhs: {");
        self.incr_indent();
        self.visit_expr(rhs);
        self.decr_indent();
        self.print_with_indent("}");
    }

    fn visit_ident(&mut self, ident: &Token) {
        self.print_with_indent(&format!("{}", ident));
    }

    fn visit_literal(&mut self, literal: &Token) {
        match &literal.ty {
            Ty::Integer(int) => self.print_with_indent(&format!("{}", int.cyan())),
            Ty::Float(float) => self.print_with_indent(&format!("{}", float.cyan())),
            Ty::String(str) => self.print_with_indent(&format!("\"{}\"", str.green())),
            Ty::Char(ch) => self.print_with_indent(&format!("'{}'", ch).green()),
            _ => unreachable!(),
        }
    }
}
