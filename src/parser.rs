// use proptest::prelude::*;

use crate::lexer::{Token, TokenType};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Program {
    pub main_function: MainFunction,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MainFunction {
    pub statement: Statement,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum Statement {
    Return(Expr),
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum Expr {
    // introductions (values)
    Num(i128),
    String(String),
    // Bool(bool),
    // Let {
    //     identifier: String,
    //     binding: Box<Expr>,
    //     body: Box<Expr>,
    // },
    // Lambda {
    //     f_param: String,
    //     body: Box<Expr>,
    // },

    // eliminations (operations)
    Binary { op: Op, l: Box<Expr>, r: Box<Expr> },
    // If {
    //     cond: Box<Expr>,
    //     then: Box<Expr>,
    //     els: Box<Expr>,
    // },
    // Var(String),
    // LambdaApp {
    //     a_param: Box<Expr>,
    //     lambda: Box<Expr>, // choice: identifier or expr
    // },
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum Val {
    Num(i128),
    Bool(bool),
    Lam { param: String, body: Expr },
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum Op {
    Add,
    Sub,
    Mult,
    Div,
    AddAdd, // works on strings
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, io::Error> {
    let main_function = parse_function(tokens)?;
    Ok(Program { main_function })
}

fn parse_function(tokens: Vec<Token>) -> Result<MainFunction, io::Error> {
    let (_, r) = mtch(&tokens, TokenType::KeywordTypeInt)?;
    let (_, r) = mtch(r, TokenType::KeywordMain)?;
    let (_, r) = mtch(r, TokenType::PuncLeftParen)?;
    let (_, r) = mtch(r, TokenType::PuncRightParen)?;
    let (_, r) = mtch(r, TokenType::PuncLeftBrace)?;
    let (statement, r) = parse_statement(r)?;
    let (_, r) = mtch(r, TokenType::PuncRightBrace)?;

    if !r.is_empty() {
        // panic?
    }

    Ok(MainFunction { statement })
}

fn parse_statement(tokens: &[Token]) -> Result<(Statement, &[Token]), io::Error> {
    let (_, r) = mtch(tokens, TokenType::StatementReturn)?;
    let (expr, r) = parse_expr(r)?;
    let (_, r) = mtch(r, TokenType::PuncSemiColon)?;
    Ok((Statement::Return(expr), r))
}

fn parse_expr(tokens: &[Token]) -> Result<(Expr, &[Token]), io::Error> {
    parse_term(tokens)
}

fn parse_term(tokens: &[Token]) -> Result<(Expr, &[Token]), io::Error> {
    let (left, r) = parse_factor(tokens)?;

    match r {
        [] => Ok((left, r)),
        r => {
            let mut root = left;
            let mut r_tokens = r;

            while let Ok((op, r)) = parse_term_op(r_tokens) {
                let (right, r) = parse_factor(r)?;

                root = Expr::Binary {
                    op,
                    l: Box::new(root),
                    r: Box::new(right),
                };

                r_tokens = r;
            }

            Ok((root, r_tokens))
        }
    }
}

fn parse_term_op(tokens: &[Token]) -> Result<(Op, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TokenType::Plus => Ok((Op::Add, r)),
            TokenType::Minus => Ok((Op::Sub, r)),
            foo => {
                println!("{:?}", foo);
                Err(io::Error::new(io::ErrorKind::Other, "bla")) // MOOSE. KEEP STRONG. DON'T GET DISTRACTED!!!
            }
        },
    }
}

fn parse_factor(tokens: &[Token]) -> Result<(Expr, &[Token]), io::Error> {
    let (left, r) = parse_atom(tokens)?;

    match r {
        [] => Ok((left, r)),
        r => {
            let mut root = left;
            let mut r_tokens = r;

            while let Ok((op, r)) = parse_factor_op(r_tokens) {
                let (right, r) = parse_atom(r)?;

                root = Expr::Binary {
                    op,
                    l: Box::new(root),
                    r: Box::new(right),
                };
                println!("wolf {:?}", root);

                r_tokens = r;
            }

            Ok((root, r_tokens))
        }
    }
}

fn parse_factor_op(tokens: &[Token]) -> Result<(Op, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TokenType::Star => Ok((Op::Mult, r)),
            TokenType::Slash => Ok((Op::Div, r)),
            _ => {
                // println!("{:?}", f);
                Err(io::Error::new(io::ErrorKind::Other, "bla"))
            }
        },
    }
}

fn parse_atom(tokens: &[Token]) -> Result<(Expr, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => Ok((Expr::Num(f.lexeme.parse().unwrap()), r)),
    }
}

// fn parse_expr(tokens: &[Token]) -> Result<(Expr, &[Token]), io::Error> {
//     // parse_expr(tokens); // infinite recursion if parsing left-associative grammar;

//     match tokens {
//         [] => todo!(),
//         [f, r @ ..] => match f.typ {
//             TokenType::LiteralInt => {
//                 let mut root = if let Ok((op, _)) = parse_binop(r) {
//                     Expr::Binary {
//                         op,
//                         l: Box::new(Expr::Num(f.lexeme.parse().unwrap())), // TODO: unwrapping
//                         r: Box::new(Expr::Num(-1)),                        // TODO??
//                     }
//                 } else {
//                     Expr::Num(f.lexeme.parse().unwrap())
//                 };

//                 let mut cur_node = &mut root;
//                 let mut r_tokens = r;

//                 while let Ok((_, r)) = parse_binop(r_tokens) {
//                     let (f, r) = mtch(r, TokenType::LiteralInt)?;
//                     let lit = f.lexeme.parse::<i128>().unwrap();

//                     if let Expr::Binary {
//                         r: ref mut right_child,
//                         ..
//                     } = cur_node
//                     {
//                         if let Ok((op, r)) = parse_binop(r) {
//                             *right_child = Box::new(Expr::Binary {
//                                 op,
//                                 l: Box::new(Expr::Num(lit)), // inheriting rust's i128 for now
//                                 r: Box::new(Expr::Num(-1)),  // TODO??
//                             });
//                             cur_node = right_child;
//                         } else {
//                             *right_child = Box::new(Expr::Num(lit));
//                             cur_node = right_child;
//                         }
//                     }

//                     r_tokens = r;
//                 }

//                 Ok((root, r_tokens))
//             }
//             TokenType::PuncLeftParen => {
//                 let (expr, r) = parse_expr(r)?;
//                 println!("waz {:?}", expr);
//                 println!("bla {:?}", r);
//                 let (_, r) = mtch(r, TokenType::PuncRightParen)?;
//                 Ok((expr, r))
//             }
//             _ => todo!(), // panic?
//         },
//     }
// }

// fn parse_binop(tokens: &[Token]) -> Result<(Op, &[Token]), io::Error> {
//     match tokens {
//         [] => todo!(),
//         [f, r @ ..] => match f.typ {
//             TokenType::Plus => Ok((Op::Add, r)),
//             TokenType::Minus => Ok((Op::Sub, r)),
//             TokenType::Star => Ok((Op::Mult, r)),
//             TokenType::Slash => Ok((Op::Div, r)),
//             _ => {
//                 // println!("{:?}", f);
//                 Err(io::Error::new(io::ErrorKind::Other, "bla"))
//             }
//         },
//     }
// }

fn mtch(tokens: &[Token], tt: TokenType) -> Result<(&Token, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => {
            if f.typ == tt {
                // Use an if-guard to compare values
                Ok((f, r))
            } else {
                println!("expected: {:?} got: {:?}", tt, f);
                Err(io::Error::new(io::ErrorKind::Other, "bla"))
            }
        }
    }
}

#[cfg(test)]
mod test_legal_arithmetic {
    use crate::lexer;
    use std::fs;

    const TEST_DIR: &str = "tests/fixtures/din/legal/arithmetic";

    #[test]
    fn lit() {
        let chars = fs::read(format!("{TEST_DIR}/lit.c"))
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars);
        let tree = super::parse(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        main_function:
          statement:
            Return:
              Num: 8
        "###);
    }

    #[test]
    fn add() {
        let chars = fs::read(format!("{TEST_DIR}/add.c"))
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars);
        let tree = super::parse(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        main_function:
          statement:
            Return:
              Binary:
                op: Add
                l:
                  Num: 9
                r:
                  Num: 10
        "###);
    }

    #[test]
    fn add_multi() {
        let chars = fs::read(format!("{TEST_DIR}/add_multi.c"))
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars);
        let tree = super::parse(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        main_function:
          statement:
            Return:
              Binary:
                op: Add
                l:
                  Binary:
                    op: Add
                    l:
                      Num: 9
                    r:
                      Num: 10
                r:
                  Num: 11
        "###);
    }

    #[test]
    fn sub() {
        #[rustfmt::skip]
        let chars = fs::read(format!("{TEST_DIR}/sub.c"))
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars);
        let tree = super::parse(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        main_function:
          statement:
            Return:
              Binary:
                op: Sub
                l:
                  Num: 88
                r:
                  Num: 32
        "###);
    }

    #[test]
    fn mult() {
        #[rustfmt::skip]
        let chars = fs::read(format!("{TEST_DIR}/mult.c"))
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars);
        let tree = super::parse(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        main_function:
          statement:
            Return:
              Binary:
                op: Mult
                l:
                  Num: 9
                r:
                  Num: 10
        "###);
    }

    #[test]
    fn div() {
        #[rustfmt::skip]
        let chars = fs::read(format!("{TEST_DIR}/div.c"))
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars);
        let tree = super::parse(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        main_function:
          statement:
            Return:
              Binary:
                op: Div
                l:
                  Num: 100
                r:
                  Num: 9
        "###);
    }
}

#[cfg(test)]
mod test_legal_arithmetic_precedence {
    use crate::lexer;
    use std::fs;

    const TEST_DIR: &str = "tests/fixtures/din/legal/arithmetic_precedence";

    #[test]
    fn add_associative() {
        let chars = fs::read(format!("{TEST_DIR}/add_associative.c"))
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars);
        let tree = super::parse(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        main_function:
          statement:
            Return:
              Binary:
                op: Add
                l:
                  Binary:
                    op: Add
                    l:
                      Num: 9
                    r:
                      Num: 10
                r:
                  Num: 11
        "###);
    }

    #[test]
    fn sub_associative() {
        let chars = fs::read(format!("{TEST_DIR}/sub_associative.c"))
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars);
        let tree = super::parse(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        main_function:
          statement:
            Return:
              Binary:
                op: Sub
                l:
                  Binary:
                    op: Sub
                    l:
                      Num: 30
                    r:
                      Num: 9
                r:
                  Num: 10
        "###);
    }
}

// proptest! {
//     #[test]
//     fn doesnt_crash(s in "\\PC*") {
//         let t = Token{ lexeme: s, typ: TokenType::Identifier };
//         let tokens = vec![t];

//         let _ = parse_program(tokens);
//     }
// }
