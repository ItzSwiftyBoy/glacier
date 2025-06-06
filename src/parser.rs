use core::str;

use crate::ast::{Element, Expr, Parameter, Type};
use crate::compiler::Compiler;
use crate::diagnostic::{Diagnostic, DiagnosticLevel, DiagnosticReporter};
use crate::utils::{LiteralKind, Span, TokenType as Ty};
use crate::{ast::AST, utils::Token};

pub struct Parser<'a> {
    current: usize,
    source: &'a [u8],
    reporter: DiagnosticReporter,
    tokens: &'a Vec<Token>,
}
impl<'a> Parser<'a> {
    pub fn new(compiler: &'a Compiler<'a>, tokens: &'a Vec<Token>) -> Self {
        Self {
            current: 0,
            source: compiler.source,
            tokens,
            reporter: DiagnosticReporter::new(),
        }
    }

    pub fn parse(&mut self) -> Option<AST> {
        let mut ast = AST::new();
        while !self.is_at_end() {}
        if self.reporter.has_error() {
            self.reporter
                .report(str::from_utf8(&self.source).ok().unwrap());
            return None;
        }
        Some(ast)
    }

    fn parse_element(&mut self) -> Element {}

    fn parse_stmt(&mut self) -> Option<Statement> {
        None
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        None
    }

    fn peek(&self, offset: usize) -> &Token {
        self.tokens.get(self.current + offset).unwrap()
    }

    fn current(&self) -> &Token {
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
            DiagnosticLevel::Error,
            message.into(),
            self.current().span,
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
