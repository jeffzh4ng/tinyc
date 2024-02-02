#[derive(Debug, PartialEq)]
pub enum Expr {
    // introductions (values)
    Num(i128),
    Bool(bool),
    Let {
        identifier: String,
        binding: Box<Expr>,
        body: Box<Expr>,
    },
    Lambda {
        f_param: String,
        body: Box<Expr>,
    },

    // eliminations (operations)
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
    Var(String),
    LambdaApp {
        a_param: Box<Expr>,
        lambda: Box<Expr>, // choice: identifier or expr
    },
}

#[derive(Debug, PartialEq)]
pub enum Val {
    Num(i128),
    Bool(bool),
    Lam { param: String, body: Expr },
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}
