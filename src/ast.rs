pub enum Expr {
    LiteralNum(i128),
    LiteralBool(bool),
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

#[derive(Debug, PartialEq)]
pub enum Val {
    Num(i128),
    Bool(bool),
}

pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}
