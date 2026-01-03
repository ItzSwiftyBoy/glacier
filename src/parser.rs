use crate::ast::{BinOp, Block, Expr, Function, Item, Parameter, Statement, UnaryOp};
use crate::compiler::Compiler;
use crate::diagnostic::{Diagnostic, DiagnosticKind};
use crate::utils::Span;
use crate::{
    ast::Ast,
    utils::{Token, TokenType as Ty},
};

pub struct Parser<'a> {
    current: usize,
    // source: &'a str,
    compiler: &'a Compiler,
    tokens: Vec<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(compiler: &'a Compiler, tokens: Vec<Token>) -> Self {
        Self {
            current: 0,
            // source: compiler.source,
            compiler,
            tokens,
        }
    }

    pub fn parse(&mut self) -> Ast {
        let mut ast = Ast::new();
        while !self.is_at_end() {
            let item = match self.advance_ty() {
                Ty::KFunction => Item::Func(self.parse_function()),
                _ => {
                    self.error("Unexpected token found.");
                    self.sync();
                    Item::Unknown
                }
            };
            ast.add_item(item);
        }

        ast
    }

    fn parse_function(&mut self) -> Function {
        let name = self.must_consume_ident();
        let mut params: Vec<Parameter> = Vec::new();
        let mut return_ty: Option<Token> = None;
        let mut body: Block = Block::new();

        self.consume(Ty::LParen);
        while !self.is_curr_token(Ty::RParen) {
            params = self.parse_params();
        }
        self.consume(Ty::RParen);

        if self.is_curr_token(Ty::RightArrow) {
            self.consume(Ty::RightArrow);
            return_ty = self.consume_ident();
        }

        self.consume(Ty::LCurly);
        while !self.is_curr_token(Ty::RCurly) {
            body.push_stmt(self.parse_stmt());
        }
        self.consume(Ty::RCurly);

        Function {
            name,
            params,
            body,
            return_ty,
        }
    }

    fn parse_params(&mut self) -> Vec<Parameter> {
        let mut params: Vec<Parameter> = vec![self.parse_param()];
        while self.is_curr_token(Ty::Comma) {
            self.consume(Ty::Comma);
            params.push(self.parse_param());
        }

        params
    }

    fn parse_param(&mut self) -> Parameter {
        let name = self.must_consume_ident();
        self.consume(Ty::Colon);
        let ty = self.must_consume_ident();
        Parameter { name, ty }
    }

    fn parse_stmt(&mut self) -> Statement {
        if self.current().is_none() {
            self.error("Expected a Statement or `}`. Found <EOF>.");
            return Statement::Unknown;
        }

        match self.advance_ty() {
            Ty::KVariable => {
                let name = self.must_consume_ident();
                let mut ty: Option<Token> = None;
                if self.is_curr_token(Ty::Colon) {
                    self.consume(Ty::Colon);
                    ty = self.consume_ident();
                }
                let mut expr: Option<Expr> = None;
                while !self.is_curr_token(Ty::Semicolon) {
                    self.consume(Ty::Eq);
                    expr = Some(self.parse_expr());
                }
                self.consume(Ty::Semicolon);
                Statement::Var { name, ty, expr }
            }
            _ => {
                self.error_on_prev_span("Unexpected token found.");
                self.sync();
                Statement::Unknown
            }
        }
    }

    fn parse_expr(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.is_curr_token(Ty::NotEq) || self.is_curr_token(Ty::DoubleEq) {
            let op = match self.advance_ty() {
                Ty::NotEq => BinOp::NotEq,
                _ => BinOp::Eq,
            };

            let rhs = Box::new(self.comparison());

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs,
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.is_curr_token(Ty::GT)
            || self.is_curr_token(Ty::GTEq)
            || self.is_curr_token(Ty::LT)
            || self.is_curr_token(Ty::LTEq)
        {
            let op = match self.advance_ty() {
                Ty::GT => BinOp::GT,
                Ty::GTEq => BinOp::GTOrEq,
                Ty::LT => BinOp::LT,
                _ => BinOp::LTOrEq,
            };

            let rhs = Box::new(self.term());

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs,
            }
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.is_curr_token(Ty::Plus) || self.is_curr_token(Ty::Minus) {
            let op = match self.advance_ty() {
                Ty::Plus => BinOp::Add,
                _ => BinOp::Subtract,
            };

            let rhs = Box::new(self.term());

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs,
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.is_curr_token(Ty::Asterisk) || self.is_curr_token(Ty::Slash) {
            let op = match self.advance_ty() {
                Ty::Asterisk => BinOp::Multiply,
                _ => BinOp::Divide,
            };

            let rhs = Box::new(self.unary());

            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs,
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.is_curr_token(Ty::Not) || self.is_curr_token(Ty::Minus) {
            let op = match self.advance_ty() {
                Ty::Not => UnaryOp::Not,
                _ => UnaryOp::Negative,
            };

            let rhs = Box::new(self.unary());

            return Expr::Unary { op, rhs };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.is_curr_token_int()
            || self.is_curr_token_float()
            || self.is_curr_token_char()
            || self.is_curr_token_string()
        {
            return Expr::Literal(self.advance());
        }

        if self.is_curr_token(Ty::LParen) {
            self.advance();
            let expr = Box::new(self.parse_expr());
            self.consume(Ty::RParen);
            return Expr::Grouping(expr);
        }

        Expr::Unknown
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.current + offset)
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }

    fn current_ty(&self) -> Ty {
        self.current().unwrap().0.clone()
    }

    fn current_span(&self) -> Span {
        self.current().unwrap().1
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn previous_ty(&self) -> Ty {
        self.previous().unwrap().0.clone()
    }

    fn previous_span(&self) -> Span {
        self.previous().unwrap().1
    }

    fn error(&mut self, message: impl Into<String>) {
        self.compiler.reporter.borrow_mut().add(Diagnostic::new(
            DiagnosticKind::Error,
            message.into(),
            self.current_span(),
        ));
    }

    fn error_on_prev_span(&mut self, message: impl Into<String>) {
        self.compiler.reporter.borrow_mut().add(Diagnostic::new(
            DiagnosticKind::Error,
            message.into(),
            self.previous_span(),
        ));
    }

    fn sync(&mut self) {
        while !self.is_at_end() {
            if self.previous_ty() == Ty::Semicolon {
                break;
            }

            match self.current_ty() {
                Ty::KClass | Ty::KStruct | Ty::KFunction | Ty::KVariable => break,
                _ => {}
            }
            self.advance();
        }
    }

    fn advance(&mut self) -> Token {
        if self.current().is_none() {
            panic!("Cannot Advance........");
        }
        let prev_token = self.current().unwrap().clone();
        self.current += 1;
        prev_token
    }

    fn advance_ty(&mut self) -> Ty {
        self.advance().0
    }

    fn consume(&mut self, token_type: Ty) {
        if self.current().is_none() {
            self.error("Unexpected <EOF>.");
            return;
        }

        if self.current_ty() == token_type {
            self.advance();
        } else {
            self.error(format!("Expected token: `{}`", token_type));
        }
    }

    fn consume_ident(&mut self) -> Option<Token> {
        if self.current().is_none() {
            self.error("Unexpected <EOF>.");
            return None;
        }

        let ident = match self.current_ty() {
            Ty::Identifier(_) => self.current().unwrap().clone(),
            _ => {
                return None;
            }
        };
        self.advance();

        Some(ident)
    }

    fn must_consume_ident(&mut self) -> Token {
        if self.current().is_none() {
            self.error("Unexpected <EOF>.");
            return Token::default();
        }

        let ident = match self.current_ty() {
            Ty::Identifier(_) => self.current().unwrap().clone(),
            _ => {
                self.error("Expected an identifier.");
                return Token::default();
            }
        };
        self.advance();

        ident
    }

    fn is_curr_token(&self, token_type: Ty) -> bool {
        self.current_ty() == token_type
    }

    fn is_curr_token_int(&self) -> bool {
        matches!(self.current_ty(), Ty::Integer(_))
    }

    fn is_curr_token_float(&self) -> bool {
        matches!(self.current_ty(), Ty::Float(_))
    }

    fn is_curr_token_char(&self) -> bool {
        matches!(self.current_ty(), Ty::Char(_))
    }

    fn is_curr_token_string(&self) -> bool {
        matches!(self.current_ty(), Ty::String(_))
    }

    fn is_at_end(&self) -> bool {
        self.current == self.tokens.len()
    }
}
