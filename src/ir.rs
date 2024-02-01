pub enum Expr {
    LiteralNum(i128),
    Binary {
        op: Op,
        l: Box<Expr>,
        r: Box<Expr>,
    },
    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        els: Box<Expr>,
    },
}

pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}
