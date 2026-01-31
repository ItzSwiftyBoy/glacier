use crate::ast::{BinOp, Block, Expr, Function, Item, Parameter, Statement, UnaryOp};
use crate::compiler::Compiler;
use crate::diag;
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
            ast.add_item(self.parse_item());
        }

        ast
    }

    fn parse_item(&mut self) -> Item {
        match self.advance_ty() {
            Ty::KFunction => Item::Func(self.parse_function()),
            _ => {
                self.error_on_prev_span("Unexpected token found.");
                self.sync(false);
                Item::Unknown
            }
        }
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
        if self.current().is_eof() {
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
                let mut expr = Expr::None;
                if self.is_curr_token(Ty::Eq) {
                    self.advance();
                    expr = self.parse_expr();
                    if expr == Expr::None {
                        self.error_with_diag(diag!(
                            "Unexpected '=' without expression.",
                            "Provide an expression.",
                            self.previous_span()
                        ));
                    }
                } else {
                    self.terminate();
                }
                Statement::VarDecl { name, ty, expr }
            }
            Ty::KReturn => Statement::Return(self.parse_expr()),
            _ => {
                self.current -= 1;
                let expr = self.parse_expr();
                self.sync(true);
                if expr == Expr::Unknown {
                    self.error_on_prev_span("Unexpected token found.");
                    Statement::Unknown
                } else {
                    Statement::Expression(expr)
                }
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
                Ty::Not => UnaryOp::Negate,
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
            return Expr::Literal(self.advance().clone());
        } else if self.is_curr_token_ident() {
            return Expr::Var(self.advance().clone());
        } else if self.is_curr_token(Ty::LParen) {
            self.advance();
            let expr = Box::new(self.parse_expr());
            self.consume(Ty::RParen);
            return Expr::Grouping(expr);
        } else if self.is_curr_token(Ty::Semicolon) {
            self.terminate();
            return Expr::None;
        } else {
            self.error("Expected an expression.");
            return Expr::Unknown;
        }
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.current + offset)
    }

    fn current(&self) -> &Token {
        self.peek(0).unwrap()
    }

    fn current_ty(&self) -> &Ty {
        &self.current().ty
    }

    fn current_span(&self) -> Span {
        self.current().span
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn previous_ty(&self) -> &Ty {
        &self.previous().ty
    }

    fn previous_span(&self) -> Span {
        self.previous().span
    }

    fn error_with_diag(&mut self, diagnostic: Diagnostic) {
        self.compiler.reporter.borrow_mut().add(diagnostic);
    }

    fn error(&mut self, message: impl Into<String>) {
        self.error_with_diag(diag!(message.into(), self.current_span()));
    }

    fn error_on_prev_span(&mut self, message: impl Into<String>) {
        self.compiler
            .reporter
            .borrow_mut()
            .add(diag!(message.into(), self.previous_span()))
    }

    fn sync(&mut self, sync_with_semicolon: bool) {
        while !self.is_at_end() {
            if Ty::Semicolon == self.previous_ty() && sync_with_semicolon {
                break;
            }

            match self.current_ty() {
                Ty::KClass | Ty::KStruct | Ty::KFunction => break,
                _ => {}
            }
            self.advance();
        }
    }

    fn terminate(&mut self) {
        self.consume(Ty::Semicolon);
    }

    fn advance(&mut self) -> &Token {
        self.current += 1;
        self.previous()
    }

    fn advance_ty(&mut self) -> &Ty {
        &self.advance().ty
    }

    fn consume(&mut self, token_type: Ty) {
        if self.current().is_eof() {
            self.error("Unexpected <EOF>.");
            return;
        }

        if *self.advance() == token_type {
        } else {
            self.error_with_diag(diag!(
                format!("Expected token: `{}`", token_type),
                format!("Put {} here.", token_type),
                self.current_span()
            ));
        }
    }

    fn consume_ident(&mut self) -> Option<Token> {
        if self.current().is_eof() {
            self.error("Unexpected <EOF>.");
            return None;
        }

        let ident = match self.current_ty() {
            Ty::Identifier(_) => Some(self.current().clone()),
            _ => {
                return None;
            }
        };
        self.advance();

        ident
    }

    fn must_consume_ident(&mut self) -> Token {
        if self.current().is_eof() {
            self.error("Unexpected <EOF>.");
            return Token::default();
        }

        let ident = match self.current_ty() {
            Ty::Identifier(_) => self.current().clone(),
            _ => {
                self.error("Expected an identifier.");
                return Token::default();
            }
        };
        self.advance();

        ident
    }

    fn is_curr_token(&self, token_type: Ty) -> bool {
        token_type == self.current_ty()
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

    fn is_curr_token_ident(&self) -> bool {
        matches!(self.current_ty(), Ty::Identifier(_))
    }

    fn is_at_end(&self) -> bool {
        self.current == self.tokens.len()
    }
}
