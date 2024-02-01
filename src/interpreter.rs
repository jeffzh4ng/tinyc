use crate::ir::{Expr, Op};

pub struct Interpreter {}

// implicit for now:
// - left to right order
// - inheriting host language (rust) number semantics
impl Interpreter {
    pub fn eval(e: Expr) -> i128 {
        match e {
            Expr::LiteralNum(n) => n,
            Expr::Binary { op, l, r } => match op {
                Op::Add => Interpreter::eval(*l) + Interpreter::eval(*r),
                Op::Subtract => Interpreter::eval(*l) - Interpreter::eval(*r),
                Op::Multiply => Interpreter::eval(*l) * Interpreter::eval(*r),
                Op::Divide => Interpreter::eval(*l) / Interpreter::eval(*r),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_simple() {
        let input = Expr::LiteralNum(8);
        let output = Interpreter::eval(input);
        assert_eq!(output, 8);
    }

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
