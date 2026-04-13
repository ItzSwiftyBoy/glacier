use crate::ast::{BinOp, Block, Expr, Function, Item, Parameter, Statement, UnaryOp};
use crate::compiler::Compiler;
use crate::diag;
use crate::diagnostic::{Diagnostic, DiagnosticKind};
use crate::utils::TokenStream;
use crate::{
    ast::Ast,
    utils::{Token, TokenKind as Ty},
};

pub struct Parser<'a> {
    stream: TokenStream,
    compiler: &'a Compiler,
}

impl<'a> Parser<'a> {
    pub fn new(compiler: &'a Compiler, stream: TokenStream) -> Self {
        Self { stream, compiler }
    }

    pub fn parse(&mut self) -> Ast {
        let mut ast = Ast::new();
        while let Some(item) = self.parse_item() {
            ast.add_item(item);
        }

        ast
    }

    fn parse_item(&mut self) -> Option<Item> {
        self.stream.ignore_newline();

        if let Some(ty) = self.stream.advance_ty() {
            Some(match ty {
                Ty::KFunction => Item::Func(self.parse_function()),
                _ => {
                    self.error_on_prev_span("Unexpected token found.");
                    self.sync(true, false);
                    self.stream.ignore_newline();
                    Item::Unknown
                }
            })
        } else {
            None
        }
    }

    fn parse_function(&mut self) -> Function {
        let name = self.must_consume_ident();
        let mut params: Vec<Parameter> = Vec::new();
        let mut return_ty: Option<Token> = None;

        self.consume(Ty::LParen);
        while !self.stream.is_curr_token(Ty::RParen) {
            params = self.parse_params();
        }
        self.consume(Ty::RParen);

        if self.stream.is_curr_token(Ty::RightArrow) {
            self.consume(Ty::RightArrow);
            return_ty = self.consume_ident().cloned();
        }

        let body = self.parse_block();

        let func = Function {
            name,
            params,
            body,
            return_ty,
        };

        self.compiler.project.add_module(module_name);

        func
    }

    fn parse_params(&mut self) -> Vec<Parameter> {
        let mut params: Vec<Parameter> = vec![self.parse_param()];
        while self.stream.is_curr_token(Ty::Comma) {
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

    fn parse_block(&mut self) -> Block {
        let mut body = Block::new();
        while let Some(stmt) = self.parse_stmt() {
            body.push_stmt(stmt);
        }
        body
    }

    fn parse_stmt(&mut self) -> Option<Statement> {
        if self.stream.is_curr_token(Ty::LCurly) {
            self.stream.advance();
        }

        self.stream.ignore_newline();

        if self.stream.is_curr_token(Ty::RCurly) {
            self.stream.advance();
            return None;
        }

        if self.stream.is_at_end() {
            self.diagnostic(diag!(
                "Unexpected <EOF>.",
                "Expected a Statement, an Expression, or `}`.",
                self.stream.current_span()
            ));
            self.diagnostic(diag!(
                DiagnosticKind::Hint("}".to_string()),
                "Put a `}` to end the block.",
                "",
                self.stream.current_span()
            ));
            return None;
        }

        Some(match self.stream.advance_ty().unwrap() {
            Ty::KVariable => {
                let name = self.must_consume_ident();
                let mut ty: Option<Token> = None;
                if self.stream.is_curr_token(Ty::Colon) {
                    self.consume(Ty::Colon);
                    ty = self.consume_ident().cloned();
                }
                let mut expr = Expr::None;
                if self.stream.is_curr_token(Ty::Eq) {
                    self.consume(Ty::Eq);
                    expr = self.parse_expr();
                    if expr == Expr::None {
                        self.diagnostic(diag!(
                            "Unexpected '=' without expression.",
                            "Provide an expression.",
                            self.stream.previous_span()
                        ));
                    }
                }
                Statement::VarDecl { name, ty, expr }
            }
            Ty::KReturn => Statement::Return(self.parse_expr()),
            Ty::LCurly => Statement::Block(self.parse_block()),
            _ => {
                self.stream -= 1;
                let expr = self.parse_expr();
                if self.stream.is_curr_token(Ty::RCurly) {
                    return Some(Statement::Expression(expr));
                }
                self.sync(false, true);
                if expr == Expr::Unknown {
                    self.error_on_prev_span("Unexpected token found.");
                    Statement::Unknown
                } else {
                    Statement::Expression(expr)
                }
            }
        })
    }

    fn parse_expr(&mut self) -> Expr {
        if self.stream.is_curr_token(Ty::Semicolon) {
            self.terminate();
            return Expr::None;
        }
        let expr = self.equality();
        self.terminate();
        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.stream.is_curr_token(Ty::NotEq) || self.stream.is_curr_token(Ty::DoubleEq) {
            let op = match self.stream.advance_ty().unwrap() {
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

        while self.stream.is_curr_token(Ty::GT)
            || self.stream.is_curr_token(Ty::GTEq)
            || self.stream.is_curr_token(Ty::LT)
            || self.stream.is_curr_token(Ty::LTEq)
        {
            let op = match self.stream.advance_ty().unwrap() {
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

        while self.stream.is_curr_token(Ty::Plus) || self.stream.is_curr_token(Ty::Minus) {
            let op = match self.stream.advance_ty().unwrap() {
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

        while self.stream.is_curr_token(Ty::Asterisk) || self.stream.is_curr_token(Ty::Slash) {
            let op = match self.stream.advance_ty().unwrap() {
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
        if self.stream.is_curr_token(Ty::Not) || self.stream.is_curr_token(Ty::Minus) {
            let op = match self.stream.advance_ty().unwrap() {
                Ty::Not => UnaryOp::Negate,
                _ => UnaryOp::Negative,
            };

            let rhs = Box::new(self.unary());

            return Expr::Unary { op, rhs };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.stream.is_curr_token_int()
            || self.stream.is_curr_token_float()
            || self.stream.is_curr_token_char()
            || self.stream.is_curr_token_string()
        {
            Expr::Literal(self.stream.advance().unwrap().clone())
        } else if self.stream.is_curr_token_ident() {
            Expr::Var(self.stream.advance().unwrap().clone())
        } else if self.stream.is_curr_token(Ty::LParen) {
            self.stream.advance();
            let expr = Box::new(self.parse_expr());
            self.consume(Ty::RParen);
            Expr::Grouping(expr)
        } else {
            self.error("Expected an expression.");
            Expr::Unknown
        }
    }

    fn diagnostic(&mut self, diagnostic: Diagnostic) {
        self.compiler.reporter.borrow_mut().add(diagnostic);
    }

    fn error(&mut self, message: impl Into<String>) {
        self.diagnostic(diag!(message.into(), self.stream.current_span()));
    }

    fn error_on_prev_span(&mut self, message: impl Into<String>) {
        self.diagnostic(diag!(message.into(), self.stream.previous_span()))
    }

    fn terminate(&mut self) -> bool {
        if self.consume(Ty::Semicolon).is_none() {
            self.diagnostic(diag!(
                DiagnosticKind::Hint(";".to_string()),
                format!("Terminate the statement with a `;`."),
                "",
                self.stream.previous_span()
            ));
            self.stream.ignore_newline();
            false
        // } else if terminate_by_comma && self.consume(Ty::Comma).is_none() {
        //     self.diagnostic(diag!(
        //         DiagnosticKind::Hint(",".to_string()),
        //         format!("Put a `,` here."),
        //         "",
        //         self.tokens.get(self.current - 1).unwrap().span
        //     ));
        //     self.ignore_newline();
        //     false
        // } else if (terminate_by_comma && self.consume(Ty::Comma).is_some())
        //     || (!terminate_by_comma && self.consume(Ty::Semicolon).is_some())
        // {
        //     self.advance();
        //     self.ignore_newline();
        //     true
        } else {
            self.stream.ignore_newline();
            true
        }
    }

    fn sync(&mut self, skip_semicolon: bool, skip_item_keywords: bool) {
        while !self.stream.is_at_end() {
            if Ty::Semicolon == self.stream.previous_ty() && !skip_semicolon {
                break;
            }

            if skip_item_keywords {
                match self.stream.current_ty() {
                    Ty::KVariable => break,
                    _ => {}
                }
            } else {
                match self.stream.current_ty() {
                    Ty::KClass | Ty::KStruct | Ty::KFunction => break,
                    _ => {}
                }
            }
            self.stream.advance();
        }
    }

    fn consume(&mut self, token_type: Ty) -> Option<&Token> {
        if self.stream.is_at_end() {
            self.diagnostic(diag!(
                format!("Expected token: `{}`", token_type),
                format!("Unexpected <EOF>."),
                self.stream.current_span()
            ));
            None
        } else if token_type == self.stream.advance() {
            Some(self.stream.previous())
        } else {
            self.diagnostic(diag!(
                format!("Expected token: `{}`", token_type),
                format!("Put {} here.", token_type),
                self.stream.previous_span()
            ));
            None
        }
    }

    fn consume_ident(&mut self) -> Option<&Token> {
        if self.stream.is_at_end() {
            self.error("Unexpected <EOF>.");
            return None;
        }

        self.stream.advance();
        let ident = if self.stream.is_curr_token_ident() {
            Some(self.stream.previous())
        } else {
            None
        };

        ident
    }

    fn must_consume_ident(&mut self) -> Token {
        if self.stream.is_at_end() {
            self.diagnostic(diag!(
                "Unexpected <EOF>.",
                "Expected an identifier",
                self.stream.current_span()
            ));
            return Token::default();
        }

        let ident = match self.stream.advance_ty() {
            Some(&Ty::Identifier(_)) => self.stream.previous().clone(),
            _ => {
                self.error_on_prev_span("Expected an identifier.");
                return Token::default();
            }
        };

        ident
    }
}
