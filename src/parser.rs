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

    if !tokens.is_empty() {
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
    match tokens {
        // expensive: cloning b/c of recursive call below
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TokenType::LiteralInt => {
                // TODO: assuming only plus for now.
                let mut parent = Expr::Binary {
                    op: Op::Add,
                    l: Box::new(Expr::Num(f.lexeme.parse().unwrap())), // TODO: unwrapping
                    r: Box::new(Expr::Num(-1)),                        // TODO??
                };
                let mut r_expr = r;

                while let Ok((f, r)) = mtch(r_expr, TokenType::Plus) {
                    let (f, r) = mtch(r, TokenType::LiteralInt)?;
                    let new_parent = Box::new(Expr::Binary {
                        op: Op::Add,
                        l: Box::new(Expr::Num(f.lexeme.parse::<i128>().unwrap())), // inheriting rust's i128 for now
                        r: Box::new(Expr::Num(-1)),                                // TODO??
                    });

                    parent = match parent {
                        Expr::Binary { op, l, r } => Expr::Binary {
                            op,
                            l,
                            r: new_parent,
                        },
                        _ => todo!(),
                    };

                    r_expr = r;
                }

                Ok((parent, r_expr))
            }
            TokenType::PuncLeftParen => todo!(),
            _ => todo!(), // panic?
        },
    }
}

fn parse_binop(tokens: Vec<Token>) -> Expr {
    todo!()
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
    fn test_valid() {
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
}
