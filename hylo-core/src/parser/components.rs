#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub enum Literal {
    Int(i64, Span),
    Float(f64, Span),
    String(String, Span),
    Bool(bool, Span),
    Word(String, Span)
}

#[derive(Debug)]
pub enum UnaryOp {
    Not(Span),
    Negative(Span),
}

#[derive(Debug)]
pub enum BinaryOp {
    Add(Span),
    Sub(Span),
    Mul(Span),
    Div(Span),
    
    Greater(Span),
    Less(Span),
    GreaterEqual(Span),
    LessEqual(Span),
    
    IsEqual(Span),
    IsNotEqual(Span),
    And(Span),
    Or(Span),
    Dot(Span)
}

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    
    Unary {
        op: UnaryOp,
        expr: Box<Expr>
    },
    
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>
    },
    
    Call {
        callee: Box<Expr>,
        lparen: Span,
        args: Vec<Expr>,
        rparen: Span
    }
}

pub enum Stmt {
    ExprStmt {
        expr: Expr,
        semicolon: Span
    }
}