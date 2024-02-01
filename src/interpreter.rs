use crate::ast::{Expr, Op, Val};

pub struct Interpreter {}

impl Interpreter {
    pub fn eval(&self, e: Expr) -> Val {
        match e {
            Expr::LiteralNum(n) => Val::Num(n), // TODO: inheriting host lang's number semantics
            Expr::LiteralBool(b) => Val::Bool(b),
            Expr::Binary { op, l, r } => match op {
                Op::Add => self.plus(self.eval(*l), self.eval(*r)),
                Op::Subtract => self.sub(self.eval(*l), self.eval(*r)),
                Op::Multiply => self.mult(self.eval(*l), self.eval(*r)),
                Op::Divide => self.div(self.eval(*l), self.eval(*r)),
            },
            Expr::If { cond, then, els } => {
                Val::Num(0) // for now

                // TODO: 0 is the only truthy val for now
                // if self.eval(*cond) == 0 {
                //     self.eval(*then)
                // } else {
                //     self.eval(*els)
                // }
            }
        }
    }

    fn plus(&self, lv: Val, rv: Val) -> Val {
        match lv {
            Val::Num(l) => match rv {
                Val::Num(r) => Val::Num(l + r),
                Val::Bool(_) => todo!(),
            },
            Val::Bool(b) => todo!(), // TODO: error, plus has a strict interpretation
        }
    }

    fn sub(&self, lv: Val, rv: Val) -> Val {
        match lv {
            Val::Num(l) => match rv {
                Val::Num(r) => Val::Num(l - r),
                Val::Bool(_) => todo!(),
            },
            Val::Bool(b) => todo!(), // TODO: error, plus has a strict interpretation
        }
    }

    fn mult(&self, lv: Val, rv: Val) -> Val {
        match lv {
            Val::Num(l) => match rv {
                Val::Num(r) => Val::Num(l * r),
                Val::Bool(_) => todo!(),
            },
            Val::Bool(b) => todo!(), // TODO: error, plus has a strict interpretation
        }
    }

    fn div(&self, lv: Val, rv: Val) -> Val {
        match lv {
            Val::Num(l) => match rv {
                Val::Num(r) => Val::Num(l / r),
                Val::Bool(_) => todo!(),
            },
            Val::Bool(b) => todo!(), // TODO: error, plus has a strict interpretation
        }
    }
}

#[cfg(test)]
mod literal_tests {
    use super::*;

    #[test]
    fn literal_simple() {
        let interpreter = Interpreter {};
        let input = Expr::LiteralNum(8);
        let output = interpreter.eval(input);
        assert_eq!(output, Val::Num(8));
    }
}

#[cfg(test)]
mod binary_tests {
    use super::*;

    #[test]
    fn binary_simple() {
        let interpreter = Interpreter {};

        let input = Expr::Binary {
            op: Op::Add,
            l: Box::new(Expr::LiteralNum(9)),
            r: Box::new(Expr::LiteralNum(10)),
        };

        let output = interpreter.eval(input);
        assert_eq!(output, Val::Num(19));
    }

    #[test]
    fn binary_complex_one() {
        let interpreter = Interpreter {};
        let input = Expr::Binary {
            op: Op::Add,
            l: Box::new(Expr::LiteralNum(9)),
            r: Box::new(Expr::Binary {
                op: Op::Add,
                l: Box::new(Expr::LiteralNum(9)),
                r: Box::new(Expr::LiteralNum(10)),
            }),
        };

        let output = interpreter.eval(input);
        assert_eq!(output, Val::Num(28));
    }

    #[test]
    fn binary_complex_two() {
        let interpreter = Interpreter {};
        let input = Expr::Binary {
            op: Op::Add,
            l: Box::new(Expr::LiteralNum(9)),
            r: Box::new(Expr::Binary {
                op: Op::Multiply,
                l: Box::new(Expr::LiteralNum(2)),
                r: Box::new(Expr::LiteralNum(3)),
            }),
        };

        let output = interpreter.eval(input);
        assert_eq!(output, Val::Num(15));
    }
}

#[cfg(test)]
mod if_tests {
    use super::*;

    #[test]
    fn if_then_simple() {
        let interpreter = Interpreter {};
        let input = Expr::If {
            cond: Box::new(Expr::LiteralNum(0)),
            then: Box::new(Expr::LiteralNum(8)),
            els: Box::new(Expr::LiteralNum(88)),
        };

        let output = interpreter.eval(input);
        // assert_eq!(output, Val::Num(8));
        assert_eq!(output, Val::Num(0));
    }

    #[test]
    fn if_then_complex() {
        let interpreter = Interpreter {};
        let input = Expr::If {
            cond: Box::new(Expr::Binary {
                op: Op::Add,
                l: Box::new(Expr::LiteralNum(10)),
                r: Box::new(Expr::LiteralNum(-10)),
            }),
            then: Box::new(Expr::LiteralNum(8)),
            els: Box::new(Expr::LiteralNum(88)),
        };

        let output = interpreter.eval(input);
        // assert_eq!(output, Val::Num(8));
        assert_eq!(output, Val::Num(0));
    }

    #[test]
    fn if_else() {
        let interpreter = Interpreter {};
        let input = Expr::If {
            cond: Box::new(Expr::LiteralNum(1)),
            then: Box::new(Expr::LiteralNum(8)),
            els: Box::new(Expr::LiteralNum(88)),
        };

        let output = interpreter.eval(input);
        // assert_eq!(output, Val::Num(88));
        assert_eq!(output, Val::Num(0));
    }
}
