use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{BinOp, Expr, Parameter, ScopeType, Size, Statement, Type, AST},
    compiler::Compiler,
    diagnostic::{Diagnostic, DiagnosticLevel},
    utils::{LiteralKind, Span, Token, TokenType as Ty},
};

pub struct Parser<'a> {
    current: usize,
    tokens: Vec<Token>,
    compiler: Rc<RefCell<Compiler<'a>>>,
}
impl<'a> Parser<'a> {
    pub fn new(compiler: Rc<RefCell<Compiler<'a>>>, tokens: Vec<Token>) -> Self {
        Self {
            current: 0,
            tokens,
            compiler,
        }
    }

    pub fn parse(&mut self) -> AST {
        let mut ast = AST::new();
        loop {
            if self.is_at_end() {
                break;
            }
            match self.advance_get_ty() {
                Ty::KFunction => {
                    let name = if let Ty::Identifier(x) = self.current_get_ty() {
                        x
                    } else {
                        self.error("Expected an identifier.".to_string());
                        String::new()
                    };
                    let mut parameter: Vec<Parameter> = Vec::new();
                    if self.current_get_ty() == Ty::LParen {
                        while self.advance_get_ty() != Ty::RParen {
                            let mut name = String::new();
                            let mut ty = Type::Unknown;
                            if let Ty::Identifier(x) = self.advance_get_ty() {
                                name = x;
                                if self.current_get_ty() == Ty::Colon {
                                    ty = match self.advance_get_ty() {
                                        Ty::KI64 => Type::Int {
                                            is_signed: true,
                                            size: Size::Long,
                                        },
                                        Ty::KI32 => Type::Int {
                                            is_signed: true,
                                            size: Size::Normal,
                                        },
                                        Ty::KI16 => Type::Int {
                                            is_signed: true,
                                            size: Size::Short,
                                        },
                                        Ty::KI8 => Type::Int {
                                            is_signed: true,
                                            size: Size::ShortShort,
                                        },
                                        _ => {
                                            self.error(
                                                "Expected builtin type or class or struct."
                                                    .to_string(),
                                            );
                                            Type::Unknown
                                        }
                                    }
                                }
                            }
                            parameter.push(Parameter { name, ty });
                        }
                    } else {
                        self.error("Expected LParen.".to_string());
                        parameter.push(Parameter {
                            name: String::new(),
                            ty: Type::Unknown,
                        });
                    }
                    let body = self.parse_body();
                    ast.add_stmt(Statement::Scope {
                        name,
                        ty: ScopeType::Function { parameter, body },
                    });
                }
                _ => self.error("Expected an item.".to_string()),
            }
        }
        ast
    }

    fn parse_stmt(&mut self) -> Statement {
        match self.advance_get_ty() {
            Ty::KVariable => {
                let mut name = String::new();
                let mut is_mutable = false;
                if self.current_get_ty() == Ty::KMutable {
                    is_mutable = true;
                }
                if let Ty::Identifier(x) = self.current_get_ty() {
                    name = x;
                } else {
                    self.error("Expected an identifier or `mut`.".to_string());
                };
                let mut ty = Type::Unknown;
                let mut expr: Vec<Expr> = Vec::new();

                if self.advance_get_ty() == Ty::Colon {
                    ty = match self.advance_get_ty() {
                        Ty::KI64 => Type::Int {
                            is_signed: true,
                            size: Size::Long,
                        },
                        Ty::KI32 => Type::Int {
                            is_signed: true,
                            size: Size::Normal,
                        },
                        Ty::KI16 => Type::Int {
                            is_signed: true,
                            size: Size::Short,
                        },
                        Ty::KI8 => Type::Int {
                            is_signed: true,
                            size: Size::ShortShort,
                        },
                        _ => {
                            self.error("Expected builtin type or class or struct.".to_string());
                            Type::Unknown
                        }
                    }
                }
                if self.advance_get_ty() == Ty::Eq {
                    while self.current_get_ty() != Ty::Semicolon {
                        if let Some(e) = self.parse_expr() {
                            expr.push(e);
                        } else {
                            self.error("Expected an expression.".to_string());
                        }
                    }
                }
                Statement::VarDecl {
                    name,
                    is_mutable,
                    ty,
                    expr,
                }
            }
            _ => todo!(),
        }
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        if let Ty::Literal(LiteralKind::Integer(x)) = self.current_get_ty() {
            let lhs = Box::new(Expr::Number {
                is_float: false,
                num: x,
            });
            self.advance();
            let op = match self.advance_get_ty() {
                Ty::Plus => BinOp::Add,
                Ty::Minus => BinOp::Subtract,
                Ty::Asterisk => BinOp::Multiply,
                Ty::Slash => BinOp::Divide,
                _ => {
                    self.error("Unexpected Binary Operation in this expression.".to_string());
                    BinOp::Unknown
                }
            };
            let mut rhs = Expr::Unknown;
            if let Ty::Literal(LiteralKind::Integer(y)) = self.current_get_ty() {
                rhs = Expr::Number {
                    is_float: false,
                    num: y,
                };
                self.advance();
            }
            Some(Expr::BinaryOp {
                op,
                lhs,
                rhs: Box::new(rhs),
            })
        } else {
            None
        }
    }

    fn parse_body(&mut self) -> Vec<Statement> {
        let mut stmts: Vec<Statement> = Vec::new();
        if self.advance_get_ty() != Ty::LCurly {
            self.error("Expected Left Brace which marks the begining of the body.".to_string());
            self.advance();
            return stmts;
        }
        self.advance();
        while self.current_get_ty() != Ty::RCurly {
            stmts.push(self.parse_stmt());
        }
        self.advance();
        stmts
    }

    fn error(&self, message: String) {
        self.compiler.borrow_mut().reporter.add(Diagnostic::new(
            DiagnosticLevel::Error,
            message,
            self.current_get_span(),
        ));
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.current + offset)
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }

    fn current_get_ty(&self) -> Ty {
        if self.current().is_none() {}
        self.current().unwrap().0.clone()
    }

    fn current_get_span(&self) -> Span {
        self.current().unwrap().1.clone()
    }

    fn advance(&mut self) {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
    }

    fn advance_get_ty(&mut self) -> Ty {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        self.tokens.get(self.current - 1).unwrap().0.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current == self.tokens.len()
    }
}
