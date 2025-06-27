use core::str;

use crate::ast::{Element, Expr, Parameter, Statement};
use crate::compiler::Compiler;
use crate::diagnostic::{Diagnostic, DiagnosticKind, DiagnosticReporter};
use crate::utils::{LiteralKind, Span, TokenType as Ty};
use crate::{ast::AST, utils::Token};

pub struct Parser<'a> {
    current: usize,
    source: &'a str,
    reporter: DiagnosticReporter,
    tokens: Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(compiler: &'a Compiler<'a>, tokens: Vec<Token>) -> Self {
        Self {
            current: 0,
            source: compiler.source,
            tokens,
            reporter: DiagnosticReporter::new(),
        }
    }

    pub fn parse(&mut self) -> AST {
        let mut ast = AST::new();
        while !self.is_at_end() {
            let ty = self.current().unwrap().ty.clone();
            if let Some(e) = self.parse_element(ty) {
                ast.add_element(e);
            }

            if self.current().is_none() {
                break;
            }
        }

        if self.reporter.has_error() {
            self.reporter.report(self.source);
        }

        ast
    }

    fn parse_element(&mut self, ty: Ty) -> Option<Element> {
        match ty {
            Ty::KFunction => self.parse_function(),
            _ => {
                self.error("Unexpected token found.");
                Element::Unknown
            }
        };
        None
    }

    fn parse_function(&mut self) -> Element {
        let mut name = String::new();
        let mut param: Vec<Parameter> = Vec::new();
        let mut body: Vec<Statement> = Vec::new();

        self.advance();
        if let Some(t) = self.current() {
            if let Ty::Identifier(n) = &t.ty {
                name = n.to_string();
            }
        } else {
            self.error("Expected an Identifier.");
        }

        self.advance();
        if let Some(t) = self.current() {
            if t.ty == Ty::LParen {
                self.advance();
            } else {
                self.error("Expected Left Parentheses.");
            }
        }
        Element::FuncScope { name, param, body }
    }

    fn parse_stmt(&mut self) -> Option<Element> {
        None
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        None
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.current + offset)
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }

    // fn current_get_ty(&self) -> Ty {
    //     if self.current().is_none() {}
    //     self.current().unwrap().0.clone()
    // }

    // fn current_get_span(&self) -> Span {
    //     self.current().unwrap().1.clone()
    // }

    fn error(&mut self, message: impl Into<String>) {
        self.reporter.add(Diagnostic::new(
            DiagnosticKind::Error,
            message.into(),
            self.current().unwrap().span,
        ));
    }

    fn advance(&mut self) {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
    }

    // fn advance_get_ty(&mut self) -> Ty {
    //     if self.current < self.tokens.len() {
    //         self.current += 1;
    //     }
    //     self.tokens.get(self.current - 1).unwrap().0.clone()
    // }

    fn is_at_end(&self) -> bool {
        self.current == self.tokens.len()
    }
}
