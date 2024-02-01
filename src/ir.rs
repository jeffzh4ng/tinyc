pub enum Expr {
    LiteralNum(i128),
    Binary { op: Op, l: Box<Expr>, r: Box<Expr> },
}

pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}
