use std::io;

use crate::lexer::{Token, TokenType};
use crate::rep::{Expr, MainFunction, Op, Program, Statement};

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
    // parse_expr(tokens); // infinite recursion if parsing left-associative grammar;
    // println!("moose: {:?}", tokens);
    match tokens {
        // expensive: cloning b/c of recursive call below
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TokenType::LiteralInt => {
                let mut root = if let Ok((op, _)) = parse_binop(r) {
                    Expr::Binary {
                        op,
                        l: Box::new(Expr::Num(f.lexeme.parse().unwrap())), // TODO: unwrapping
                        r: Box::new(Expr::Num(-1)),                        // TODO??
                    }
                } else {
                    Expr::Num(f.lexeme.parse().unwrap())
                };

                // TODO: assuming only plus for now.
                // let mut root = Expr::Binary {
                //     op: Op::Add,
                //     l: Box::new(Expr::Num(f.lexeme.parse().unwrap())), // TODO: unwrapping
                //     r: Box::new(Expr::Num(-1)),                        // TODO??
                // };

                let mut cur_node = &mut root;
                let mut r_tokens = r;

                while let Ok((op, r)) = parse_binop(r_tokens) {
                    let (f, r) = mtch(r, TokenType::LiteralInt)?;
                    let lit = f.lexeme.parse::<i128>().unwrap();

                    if let Expr::Binary {
                        r: ref mut right_child,
                        ..
                    } = cur_node
                    {
                        if let Ok((op, r)) = parse_binop(r) {
                            *right_child = Box::new(Expr::Binary {
                                op,
                                l: Box::new(Expr::Num(lit)), // inheriting rust's i128 for now
                                r: Box::new(Expr::Num(-1)),  // TODO??
                            });
                            cur_node = right_child;
                        } else {
                            *right_child = Box::new(Expr::Num(lit));
                            cur_node = right_child;
                        }
                    }

                    r_tokens = r;
                }

                Ok((root, r_tokens))
            }
            TokenType::PuncLeftParen => todo!(),
            _ => todo!(), // panic?
        },
    }
}

fn parse_binop(tokens: &[Token]) -> Result<(Op, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TokenType::Plus => Ok((Op::Add, r)),
            TokenType::Minus => Ok((Op::Sub, r)),
            TokenType::Star => Ok((Op::Mult, r)),
            TokenType::Slash => Ok((Op::Div, r)),
            _ => Err(io::Error::new(io::ErrorKind::Other, "bla")),
        },
    }
}

fn check(tokens: Vec<Token>, tt: TokenType) {
    todo!()
}

fn mtch<'a>(tokens: &'a [Token], tt: TokenType) -> Result<(&'a Token, &'a [Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => {
            if f.typ == tt {
                // Use an if-guard to compare values
                Ok((f, r))
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "bla"))
            }
        }
    }
}

#[cfg(test)]
mod test_valid {
    use super::*;
    use crate::lexer;
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
    fn test_arithmetic_addition() {
        #[rustfmt::skip]
        let chars = fs::read("tests/valid/arithmetic/addition.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::scan(&chars);
        let tree = parse_program(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn test_arithmetic_addition_multi() {
        #[rustfmt::skip]
        let chars = fs::read("tests/valid/arithmetic/addition_multi.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::scan(&chars);
        let tree = parse_program(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn test_arithmetic_subtraction() {
        #[rustfmt::skip]
        let chars = fs::read("tests/valid/arithmetic/subtraction.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::scan(&chars);
        let tree = parse_program(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree);
    }

    #[test]
    fn test_arithmetic_mult() {
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

    #[test]
    fn test_arithmetic_div() {
        #[rustfmt::skip]
        let chars = fs::read("tests/valid/arithmetic/div.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::scan(&chars);
        let tree = parse_program(tokens).unwrap();
        insta::assert_yaml_snapshot!(tree);
    }
}
