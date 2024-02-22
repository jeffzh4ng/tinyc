use std::collections::HashMap;

use crate::rep::{Expr, Op, Program, Statement, Val};

type Env = HashMap<String, Val>;

pub fn eval_program(p: Program) -> Val {
    match p.main_function.statement {
        Statement::Return(e) => eval_expr(e, &mut HashMap::new()),
    }
}

fn eval_expr(e: Expr, nv: &mut Env) -> Val {
    match e {
        // introductions
        Expr::Num(n) => Val::Num(n), // TODO: inheriting host lang's number semantics
        Expr::String(s) => todo!(),
        // Expr::Bool(b) => Val::Bool(b),
        // Expr::Let {
        //     identifier,
        //     binding,
        //     body,
        // } => {
        //     #[rustfmt::skip]
        //   let extended_nv = nv
        //   .into_iter()
        //   .chain(std::iter::once((identifier, self.eval_expr(*binding, nv))))
        //   .collect();

        //     self.eval_expr(*body, extended_nv)
        // }
        // Expr::Lambda {
        //     f_param: param,
        //     body,
        // } => Val::Lam { param, body: *body },

        // eliminations
        Expr::Binary { op, l, r } => match op {
            // threading through nv since SMoL has static scope
            Op::Add => plus(eval_expr(*l, nv), eval_expr(*r, nv)),
            Op::Sub => sub(eval_expr(*l, nv), eval_expr(*r, nv)),
            Op::Mult => mult(eval_expr(*l, nv), eval_expr(*r, nv)),
            Op::Div => div(eval_expr(*l, nv), eval_expr(*r, nv)),
            Op::AddAdd => todo!(),
            // Op::Subtract => sub(eval_expr(*l, nv), eval_expr(*r, nv)),
            // Op::Multiply => mult(eval_expr(*l, nv), eval_expr(*r, nv)),
            // Op::Divide => div(eval_expr(*l, nv), eval_expr(*r, nv)),
            // Op::AddAdd => todo!(),
        },
        // Expr::If { cond, then, els } => {
        //     Val::Num(0) // for now

        //     // TODO: 0 is the only truthy val for now
        //     // if self.eval(*cond) == 0 {
        //     //     self.eval(*then)
        //     // } else {
        //     //     self.eval(*els)
        //     // }
        // }
        // Expr::Var(id) => todo!(), // (nv.get(&id).unwrap()),
        // Expr::LambdaApp {
        //     a_param: arg,
        //     lambda,
        // } => {
        //     // choice: order of f eval
        //     let arg = self.eval_expr(*arg, nv);
        //     let lam = self.eval_expr(*lambda, nv);

        //     // lecture: inherit's host lang (racket's) let semantics

        //     match lam {
        //         Val::Lam { param, body } => {
        //             let extended_nv = nv
        //                 .into_iter()
        //                 .chain(std::iter::once((param, arg)))
        //                 .collect();

        //             self.eval_expr(body, extended_nv)
        //         }
        //         _ => {
        //             todo!() // error
        //         }
        //     }
        // }
    }
}

fn plus(lv: Val, rv: Val) -> Val {
    match lv {
        Val::Num(l) => match rv {
            Val::Num(r) => Val::Num(l + r),
            Val::Bool(_) => todo!(),
            Val::Lam { param, body } => todo!(),
        },
        Val::Bool(b) => todo!(), // TODO: error, plus has a strict interpretation
        Val::Lam { param, body } => todo!(),
    }
}

fn sub(lv: Val, rv: Val) -> Val {
    match lv {
        Val::Num(l) => match rv {
            Val::Num(r) => Val::Num(l - r),
            Val::Bool(_) => todo!(),
            Val::Lam { param, body } => todo!(),
        },
        Val::Bool(b) => todo!(), // TODO: error, plus has a strict interpretation
        Val::Lam { param, body } => todo!(),
    }
}

fn mult(lv: Val, rv: Val) -> Val {
    match lv {
        Val::Num(l) => match rv {
            Val::Num(r) => Val::Num(l * r),
            Val::Bool(_) => todo!(),
            Val::Lam { param, body } => todo!(),
        },
        Val::Bool(b) => todo!(), // TODO: error, plus has a strict interpretation
        Val::Lam { param, body } => todo!(),
    }
}

fn div(lv: Val, rv: Val) -> Val {
    match lv {
        Val::Num(l) => match rv {
            Val::Num(r) => Val::Num(l / r),
            Val::Bool(_) => todo!(),
            Val::Lam { param, body } => todo!(),
        },
        Val::Bool(b) => todo!(), // TODO: error, plus has a strict interpretation
        Val::Lam { param, body } => todo!(),
    }
}

#[cfg(test)]
mod test_valid_arithmetic {
    use std::fs;

    use crate::{lexer, parser, typer};

    use super::*;

    #[test]
    fn test_valid() {
        #[rustfmt::skip]
        let chars = fs::read("tests/valid/hello.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::scan(&chars);
        let tree = parser::parse_program(tokens).unwrap();
        let judgement = typer::type_program(&tree);
        if !judgement {
            panic!();
        }
        let res = eval_program(tree);
        insta::assert_yaml_snapshot!(res);
    }

    #[test]
    fn test_add() {
        #[rustfmt::skip]
        let chars = fs::read("tests/valid/arithmetic/add.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::scan(&chars);
        let tree = parser::parse_program(tokens).unwrap();
        let judgement = typer::type_program(&tree);
        if !judgement {
            panic!();
        }
        let res = eval_program(tree);
        insta::assert_yaml_snapshot!(res);
    }

    #[test]
    fn test_add_multi() {
        #[rustfmt::skip]
        let chars = fs::read("tests/valid/arithmetic/add_multi.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::scan(&chars);
        let tree = parser::parse_program(tokens).unwrap();
        let judgement = typer::type_program(&tree);
        if !judgement {
            panic!();
        }
        let res = eval_program(tree);
        insta::assert_yaml_snapshot!(res);
    }

    #[test]
    fn test_subtraction() {
        #[rustfmt::skip]
        let chars = fs::read("tests/valid/arithmetic/subtraction.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::scan(&chars);
        let tree = parser::parse_program(tokens).unwrap();
        let judgement = typer::type_program(&tree);
        if !judgement {
            panic!();
        }
        let res = eval_program(tree);
        insta::assert_yaml_snapshot!(res);
    }

    #[test]
    fn test_mult() {
        #[rustfmt::skip]
        let chars = fs::read("tests/valid/arithmetic/mult.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::scan(&chars);
        let tree = parser::parse_program(tokens).unwrap();
        let judgement = typer::type_program(&tree);
        if !judgement {
            panic!();
        }
        let res = eval_program(tree);
        insta::assert_yaml_snapshot!(res);
    }

    #[test]
    fn test_div() {
        #[rustfmt::skip]
        let chars = fs::read("tests/valid/arithmetic/div.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::scan(&chars);
        let tree = parser::parse_program(tokens).unwrap();
        let judgement = typer::type_program(&tree);
        if !judgement {
            panic!();
        }
        let res = eval_program(tree);
        insta::assert_yaml_snapshot!(res);
    }
}

#[cfg(test)]
mod test_valid_arithmetic_precedence {
    use std::fs;

    use crate::{lexer, parser, typer};

    use super::*;

    #[test]
    fn test_add_sub() {
        #[rustfmt::skip]
        let chars = fs::read("tests/valid/arithmetic_precedence/add_sub.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::scan(&chars);
        let tree = parser::parse_program(tokens).unwrap();
        let judgement = typer::type_program(&tree);
        if !judgement {
            panic!();
        }
        let res = eval_program(tree);
        insta::assert_yaml_snapshot!(res);
    }
}
// #[cfg(test)]
// mod literal_tests {
//     use super::*;

//     #[test]
//     fn literal_simple() {
//         let interpreter = Interpreter {};
//         let (e, nv) = (Expr::Num(8), HashMap::new());
//         let output = interpreter.eval_expr(e, nv);
//         assert_eq!(output, Val::Num(8));
//     }
// }

// #[cfg(test)]
// mod binary_tests {
//     use super::*;

//     #[test]
//     fn binary_simple() {
//         let interpreter = Interpreter {};

//         let (e, nv) = (
//             Expr::Binary {
//                 op: Op::Add,
//                 l: Box::new(Expr::Num(9)),
//                 r: Box::new(Expr::Num(10)),
//             },
//             HashMap::new(),
//         );

//         let output = interpreter.eval_expr(e, nv);
//         assert_eq!(output, Val::Num(19));
//     }

//     #[test]
//     fn binary_complex_one() {
//         let interpreter = Interpreter {};
//         let (e, nv) = (
//             Expr::Binary {
//                 op: Op::Add,
//                 l: Box::new(Expr::Num(9)),
//                 r: Box::new(Expr::Binary {
//                     op: Op::Add,
//                     l: Box::new(Expr::Num(9)),
//                     r: Box::new(Expr::Num(10)),
//                 }),
//             },
//             HashMap::new(),
//         );

//         let output = interpreter.eval_expr(e, nv);
//         assert_eq!(output, Val::Num(28));
//     }

//     #[test]
//     fn binary_complex_two() {
//         let interpreter = Interpreter {};
//         let (e, nv) = (
//             Expr::Binary {
//                 op: Op::Add,
//                 l: Box::new(Expr::Num(9)),
//                 r: Box::new(Expr::Binary {
//                     op: Op::Multiply,
//                     l: Box::new(Expr::Num(2)),
//                     r: Box::new(Expr::Num(3)),
//                 }),
//             },
//             HashMap::new(),
//         );

//         let output = interpreter.eval_expr(e, nv);
//         assert_eq!(output, Val::Num(15));
//     }
// }

// #[cfg(test)]
// mod if_tests {
//     use super::*;

//     #[test]
//     fn if_then_simple() {
//         let interpreter = Interpreter {};
//         let (e, nv) = (
//             Expr::If {
//                 cond: Box::new(Expr::Num(0)),
//                 then: Box::new(Expr::Num(8)),
//                 els: Box::new(Expr::Num(88)),
//             },
//             HashMap::new(),
//         );

//         let output = interpreter.eval_expr(e, nv);
//         // assert_eq!(output, Val::Num(8));
//         assert_eq!(output, Val::Num(0));
//     }

//     #[test]
//     fn if_then_complex() {
//         let interpreter = Interpreter {};
//         let (e, nv) = (
//             Expr::If {
//                 cond: Box::new(Expr::Binary {
//                     op: Op::Add,
//                     l: Box::new(Expr::Num(10)),
//                     r: Box::new(Expr::Num(-10)),
//                 }),
//                 then: Box::new(Expr::Num(8)),
//                 els: Box::new(Expr::Num(88)),
//             },
//             HashMap::new(),
//         );

//         let output = interpreter.eval_expr(e, nv);
//         // assert_eq!(output, Val::Num(8));
//         assert_eq!(output, Val::Num(0));
//     }

//     #[test]
//     fn if_else() {
//         let interpreter = Interpreter {};
//         let (e, nv) = (
//             Expr::If {
//                 cond: Box::new(Expr::Num(1)),
//                 then: Box::new(Expr::Num(8)),
//                 els: Box::new(Expr::Num(88)),
//             },
//             HashMap::new(),
//         );

//         let output = interpreter.eval_expr(e, nv);
//         // assert_eq!(output, Val::Num(88));
//         assert_eq!(output, Val::Num(0));
//     }
// }
