use std::collections::HashMap;

use crate::ast::{Expr, Op, Val};

type Env = HashMap<String, Val>;

pub struct Interpreter {}

impl Interpreter {
    pub fn eval(&self, e: Expr, nv: Env) -> Val {
        match e {
            Expr::Num(n) => Val::Num(n), // TODO: inheriting host lang's number semantics
            Expr::Bool(b) => Val::Bool(b),
            Expr::Binary { op, l, r } => match op {
                // threading through nv since SMoL has static scope
                Op::Add => self.plus(self.eval(*l, nv), self.eval(*r, nv)),
                Op::Subtract => self.sub(self.eval(*l, nv), self.eval(*r, nv)),
                Op::Multiply => self.mult(self.eval(*l, nv), self.eval(*r, nv)),
                Op::Divide => self.div(self.eval(*l, nv), self.eval(*r, nv)),
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
            Expr::Var(id) => todo!(), // (nv.get(&id).unwrap()),
            Expr::Let {
                identifier,
                binding,
                body,
            } => {
                #[rustfmt::skip]
                let extended_nv = nv
                .into_iter()
                .chain(std::iter::once((identifier, self.eval(*binding, nv))))
                .collect();

                self.eval(*body, extended_nv)
            }
            Expr::Lambda { param, body } => Val::Lam { param, body: *body },
            Expr::LambdaApp { arg, lambda } => {
                // choice: order of f eval
                let arg = self.eval(*arg, nv);
                let lam = self.eval(*lambda, nv);

                // lecture: inherit's host lang (racket's) let semantics

                match lam {
                    Val::Lam { param, body } => {
                        let extended_nv = nv
                            .into_iter()
                            .chain(std::iter::once((param, arg)))
                            .collect();

                        self.eval(body, extended_nv)
                    }
                    _ => {
                        todo!() // error
                    }
                }
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
        let (e, nv) = (Expr::Num(8), HashMap::new());
        let output = interpreter.eval(e, nv);
        assert_eq!(output, Val::Num(8));
    }
}

#[cfg(test)]
mod binary_tests {
    use super::*;

    #[test]
    fn binary_simple() {
        let interpreter = Interpreter {};

        let (e, nv) = (
            Expr::Binary {
                op: Op::Add,
                l: Box::new(Expr::Num(9)),
                r: Box::new(Expr::Num(10)),
            },
            HashMap::new(),
        );

        let output = interpreter.eval(e, nv);
        assert_eq!(output, Val::Num(19));
    }

    #[test]
    fn binary_complex_one() {
        let interpreter = Interpreter {};
        let (e, nv) = (
            Expr::Binary {
                op: Op::Add,
                l: Box::new(Expr::Num(9)),
                r: Box::new(Expr::Binary {
                    op: Op::Add,
                    l: Box::new(Expr::Num(9)),
                    r: Box::new(Expr::Num(10)),
                }),
            },
            HashMap::new(),
        );

        let output = interpreter.eval(e, nv);
        assert_eq!(output, Val::Num(28));
    }

    #[test]
    fn binary_complex_two() {
        let interpreter = Interpreter {};
        let (e, nv) = (
            Expr::Binary {
                op: Op::Add,
                l: Box::new(Expr::Num(9)),
                r: Box::new(Expr::Binary {
                    op: Op::Multiply,
                    l: Box::new(Expr::Num(2)),
                    r: Box::new(Expr::Num(3)),
                }),
            },
            HashMap::new(),
        );

        let output = interpreter.eval(e, nv);
        assert_eq!(output, Val::Num(15));
    }
}

#[cfg(test)]
mod if_tests {
    use super::*;

    #[test]
    fn if_then_simple() {
        let interpreter = Interpreter {};
        let (e, nv) = (
            Expr::If {
                cond: Box::new(Expr::Num(0)),
                then: Box::new(Expr::Num(8)),
                els: Box::new(Expr::Num(88)),
            },
            HashMap::new(),
        );

        let output = interpreter.eval(e, nv);
        // assert_eq!(output, Val::Num(8));
        assert_eq!(output, Val::Num(0));
    }

    #[test]
    fn if_then_complex() {
        let interpreter = Interpreter {};
        let (e, nv) = (
            Expr::If {
                cond: Box::new(Expr::Binary {
                    op: Op::Add,
                    l: Box::new(Expr::Num(10)),
                    r: Box::new(Expr::Num(-10)),
                }),
                then: Box::new(Expr::Num(8)),
                els: Box::new(Expr::Num(88)),
            },
            HashMap::new(),
        );

        let output = interpreter.eval(e, nv);
        // assert_eq!(output, Val::Num(8));
        assert_eq!(output, Val::Num(0));
    }

    #[test]
    fn if_else() {
        let interpreter = Interpreter {};
        let (e, nv) = (
            Expr::If {
                cond: Box::new(Expr::Num(1)),
                then: Box::new(Expr::Num(8)),
                els: Box::new(Expr::Num(88)),
            },
            HashMap::new(),
        );

        let output = interpreter.eval(e, nv);
        // assert_eq!(output, Val::Num(88));
        assert_eq!(output, Val::Num(0));
    }
}
