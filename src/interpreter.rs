use crate::ast::{Expr, Op};

pub struct Interpreter {}

impl Interpreter {
    pub fn eval(e: Expr) -> i128 {
        match e {
            Expr::LiteralNum(n) => n, // TODO: inheriting host lang's number semantics
            Expr::Binary { op, l, r } => match op {
                Op::Add => Interpreter::eval(*l) + Interpreter::eval(*r),
                Op::Subtract => Interpreter::eval(*l) - Interpreter::eval(*r),
                Op::Multiply => Interpreter::eval(*l) * Interpreter::eval(*r),
                Op::Divide => Interpreter::eval(*l) / Interpreter::eval(*r),
            },
            Expr::If { cond, then, els } => {
                // TODO: 0 is the only truthy val for now
                if Interpreter::eval(*cond) == 0 {
                    Interpreter::eval(*then)
                } else {
                    Interpreter::eval(*els)
                }
            }
        }
    }
}

#[cfg(test)]
mod literal_tests {
    use super::*;

    #[test]
    fn literal_simple() {
        let input = Expr::LiteralNum(8);
        let output = Interpreter::eval(input);
        assert_eq!(output, 8);
    }
}

#[cfg(test)]
mod binary_tests {
    use super::*;

    #[test]
    fn binary_simple() {
        let input = Expr::Binary {
            op: Op::Add,
            l: Box::new(Expr::LiteralNum(9)),
            r: Box::new(Expr::LiteralNum(10)),
        };

        let output = Interpreter::eval(input);
        assert_eq!(output, 19);
    }

    #[test]
    fn binary_complex_one() {
        let input = Expr::Binary {
            op: Op::Add,
            l: Box::new(Expr::LiteralNum(9)),
            r: Box::new(Expr::Binary {
                op: Op::Add,
                l: Box::new(Expr::LiteralNum(9)),
                r: Box::new(Expr::LiteralNum(10)),
            }),
        };

        let output = Interpreter::eval(input);
        assert_eq!(output, 28);
    }

    #[test]
    fn binary_complex_two() {
        let input = Expr::Binary {
            op: Op::Add,
            l: Box::new(Expr::LiteralNum(9)),
            r: Box::new(Expr::Binary {
                op: Op::Multiply,
                l: Box::new(Expr::LiteralNum(2)),
                r: Box::new(Expr::LiteralNum(3)),
            }),
        };

        let output = Interpreter::eval(input);
        assert_eq!(output, 15);
    }
}

#[cfg(test)]
mod if_tests {
    use super::*;

    #[test]
    fn if_then_simple() {
        let input = Expr::If {
            cond: Box::new(Expr::LiteralNum(0)),
            then: Box::new(Expr::LiteralNum(8)),
            els: Box::new(Expr::LiteralNum(88)),
        };

        let output = Interpreter::eval(input);
        assert_eq!(output, 8);
    }

    #[test]
    fn if_then_complex() {
        let input = Expr::If {
            cond: Box::new(Expr::Binary {
                op: Op::Add,
                l: Box::new(Expr::LiteralNum(10)),
                r: Box::new(Expr::LiteralNum(-10)),
            }),
            then: Box::new(Expr::LiteralNum(8)),
            els: Box::new(Expr::LiteralNum(88)),
        };

        let output = Interpreter::eval(input);
        assert_eq!(output, 8);
    }

    #[test]
    fn if_else() {
        let input = Expr::If {
            cond: Box::new(Expr::LiteralNum(1)),
            then: Box::new(Expr::LiteralNum(8)),
            els: Box::new(Expr::LiteralNum(88)),
        };

        let output = Interpreter::eval(input);
        assert_eq!(output, 88);
    }
}
