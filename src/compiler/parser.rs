use proptest::prelude::*;

use std::io;

use crate::compiler::lexer::{Token, TokenType};
use crate::compiler::rep::{Expr, MainFunction, Op, Program, Statement};

pub fn parse_program(tokens: Vec<Token>) -> Result<Program, io::Error> {
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
    println!("moose {:?}", left);

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
            _ => {
                // println!("{:?}", f);
                Err(io::Error::new(io::ErrorKind::Other, "bla"))
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
                println!("moose: {:?} {:?}", f, tt);
                Err(io::Error::new(io::ErrorKind::Other, "bla"))
            }
        }
    }
}

#[cfg(test)]
mod test_valid_arithmetic {
    use super::*;
    use crate::compiler::lexer;
    use insta;
    use std::fs;

    #[test]
    fn test_hello() {
        #[rustfmt::skip]
        let chars = fs::read("tests/valid/hello.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::scan(&chars);
        let tree = parse_program(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree);
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
        let tree = parse_program(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree);
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
        let tree = parse_program(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn test_sub() {
        #[rustfmt::skip]
        let chars = fs::read("tests/valid/arithmetic/sub.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::scan(&chars);
        let tree = parse_program(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree);
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
        let tree = parse_program(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree);
    }
}

#[cfg(test)]
mod test_valid_arithmetic_precedence {
    use std::fs;

    use super::*;
    use crate::compiler::lexer;

    #[test]
    fn test_mult_add() {
        #[rustfmt::skip]
        let chars = fs::read("tests/valid/arithmetic_precedence/mult_add_precedence.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let scan = lexer::scan(&chars);
        let tokens = scan;
        let tree = parse_program(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree);
    }
}

proptest! {
    #[test]
    fn doesnt_crash(s in "\\PC*") {
        let t = Token{ lexeme: s, typ: TokenType::Identifier };
        let tokens = vec![t];

        let _ = parse_program(tokens);
    }
}
