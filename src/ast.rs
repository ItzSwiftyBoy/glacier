#[derive(Debug)]
pub enum Expr {
    Number(i64),
    BinaryOp {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Variable(String),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    VarAssign(String, Expr),
}

#[derive(Debug)]
pub struct Program {
    statements: Vec<Stmt>,
}
