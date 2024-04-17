use crate::lexer::{Token, TokenType};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Program {
    pub main_function: MainFunction,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MainFunction {
    pub statement: Stmt,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum Stmt {
    If {
        cond: Box<Expr>,
        then: Box<Stmt>,
        els: Box<Stmt>,
    },
    Return(Expr),
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum Expr {
    // introductions (values)
    Num(i128),
    String(String),

    // eliminations (operations)
    BinE {
        op: BinOp,
        l: Box<Expr>,
        r: Box<Expr>,
    },
    RelE {
        op: RelOp,
        l: Box<Expr>,
        r: Box<Expr>,
    },
    LogE {
        op: LogOp,
        l: Box<Expr>,
        r: Box<Expr>,
    },
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum RelOp {
    Eq,
    Neq,
    Lteq,
    Lt,
    Gteq,
    Gt,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum LogOp {
    And,
    Or,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mult,
    Div,
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
    let (statement, r) = parse_stmt(r)?;
    let (_, r) = mtch(r, TokenType::PuncRightBrace)?;

    if !r.is_empty() {
        // panic?
    }

    Ok(MainFunction { statement })
}

fn parse_stmt(tokens: &[Token]) -> Result<(Stmt, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TokenType::KeywordIf => {
                let (_, r) = mtch(r, TokenType::PuncLeftParen)?;
                let (cond, r) = parse_rel_expr(r)?;
                let (_, r) = mtch(r, TokenType::PuncRightParen)?;
                let (_, r) = mtch(r, TokenType::PuncLeftBrace)?;
                let (then, r) = parse_stmt(r)?;
                let (_, r) = mtch(r, TokenType::PuncRightBrace)?;
                let (_, r) = mtch(r, TokenType::KeywordEls)?;
                let (_, r) = mtch(r, TokenType::PuncLeftBrace)?;
                let (els, r) = parse_stmt(r)?;
                let (_, r) = mtch(r, TokenType::PuncRightBrace)?;

                Ok((
                    Stmt::If {
                        cond: Box::new(cond),
                        then: Box::new(then),
                        els: Box::new(els),
                    },
                    r,
                ))
            }
            TokenType::KeywordRet => {
                let (expr, r) = parse_rel_expr(r)?;
                let (_, r) = mtch(r, TokenType::PuncSemiColon)?;
                Ok((Stmt::Return(expr), r))
            }
            t => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("token not recognizable {:?}", t),
            )),
        },
    }
}

fn parse_rel_expr(tokens: &[Token]) -> Result<(Expr, &[Token]), io::Error> {
    let (left, r) = parse_term(tokens)?;

    match r {
        [] => Ok((left, r)),
        r => {
            let mut cur_node = left;
            let mut r = r;

            while let Ok((op, r_temp)) = parse_rel_op(r) {
                let (right, r_temp) = parse_term(r_temp)?;

                cur_node = Expr::RelE {
                    op,
                    l: Box::new(cur_node),
                    r: Box::new(right),
                };

                r = r_temp;
            }

            Ok((cur_node, r))
        }
    }
}

fn parse_term(tokens: &[Token]) -> Result<(Expr, &[Token]), io::Error> {
    let (left, r) = parse_factor(tokens)?;

    match r {
        [] => Ok((left, r)),
        r => {
            let mut cur_node = left;
            let mut r = r;

            while let Ok((op, r_temp)) = parse_term_op(r) {
                let (right, r_temp) = parse_factor(r_temp)?;

                cur_node = Expr::BinE {
                    op,
                    l: Box::new(cur_node),
                    r: Box::new(right),
                };

                r = r_temp;
            }

            Ok((cur_node, r))
        }
    }
}

fn parse_factor(tokens: &[Token]) -> Result<(Expr, &[Token]), io::Error> {
    let (left, r) = parse_atom(tokens)?;

    match r {
        [] => Ok((left, r)),
        r => {
            let mut cur_node = left;
            let mut r = r;

            while let Ok((op, r_temp)) = parse_factor_op(r) {
                let (right, r_temp) = parse_atom(r_temp)?;

                cur_node = Expr::BinE {
                    op,
                    l: Box::new(cur_node),
                    r: Box::new(right),
                };

                r = r_temp;
            }

            Ok((cur_node, r))
        }
    }
}

fn parse_atom(tokens: &[Token]) -> Result<(Expr, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => Ok((Expr::Num(f.lexeme.parse().unwrap()), r)),
    }
}

fn parse_rel_op(tokens: &[Token]) -> Result<(RelOp, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TokenType::LeftAngleBracket => match r {
                [] => todo!(),
                [s, r @ ..] => match s.typ {
                    TokenType::Equals => Ok((RelOp::Lteq, r)),
                    _ => Ok((RelOp::Lt, &tokens[1..])), // include s
                },
            },
            TokenType::RightAngleBracket => match r {
                [] => todo!(),
                [s, r @ ..] => match s.typ {
                    TokenType::Equals => Ok((RelOp::Gteq, r)),
                    _ => Ok((RelOp::Gt, &tokens[1..])), // include s
                },
            },
            t => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("token not recognizable {:?}", t),
            )),
        },
    }
}

fn parse_term_op(tokens: &[Token]) -> Result<(BinOp, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TokenType::Plus => Ok((BinOp::Add, r)),
            TokenType::Minus => Ok((BinOp::Sub, r)),
            t => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("token not recognizable {:?}", t),
            )),
        },
    }
}

fn parse_factor_op(tokens: &[Token]) -> Result<(BinOp, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TokenType::Star => Ok((BinOp::Mult, r)),
            TokenType::Slash => Ok((BinOp::Div, r)),
            t => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("token not recognizable {:?}", t),
            )),
        },
    }
}

fn mtch(tokens: &[Token], tt: TokenType) -> Result<(&Token, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => {
            if f.typ == tt {
                // Use an if-guard to compare values
                Ok((f, r))
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("expected: {:?} got: {:?}", tt, f),
                ))
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
              BinE:
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
              BinE:
                op: Add
                l:
                  BinE:
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
              BinE:
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
              BinE:
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
              BinE:
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
              BinE:
                op: Add
                l:
                  BinE:
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
              BinE:
                op: Sub
                l:
                  BinE:
                    op: Sub
                    l:
                      Num: 30
                    r:
                      Num: 9
                r:
                  Num: 10
        "###);
    }

    #[test]
    fn mult_add_precedence() {
        let chars = fs::read(format!("{TEST_DIR}/mult_add_precedence.c"))
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
              BinE:
                op: Add
                l:
                  BinE:
                    op: Mult
                    l:
                      Num: 9
                    r:
                      Num: 10
                r:
                  Num: 11
        "###);
    }

    #[test]
    fn mult_add_precedence_multi() {
        let chars = fs::read(format!("{TEST_DIR}/mult_add_precedence_multi.c"))
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
              BinE:
                op: Add
                l:
                  BinE:
                    op: Mult
                    l:
                      Num: 9
                    r:
                      Num: 10
                r:
                  BinE:
                    op: Mult
                    l:
                      Num: 11
                    r:
                      Num: 12
        "###);
    }
}

#[cfg(test)]
mod test_legal_control_flow {
    use crate::lexer;
    use std::fs;

    const TEST_DIR: &str = "tests/fixtures/din/legal/control_flow";

    #[test]
    fn lt() {
        let chars = fs::read(format!("{TEST_DIR}/lt_true.c"))
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
              RelE:
                op: Lt
                l:
                  Num: 9
                r:
                  Num: 10
        "###);
    }

    #[test]
    fn gt() {
        let chars = fs::read(format!("{TEST_DIR}/gt_true.c"))
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
              RelE:
                op: Gt
                l:
                  Num: 10
                r:
                  Num: 9
        "###);
    }

    #[test]
    fn ifels_lt() {
        let chars = fs::read(format!("{TEST_DIR}/ifels_then.c"))
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
            If:
              cond:
                RelE:
                  op: Lt
                  l:
                    Num: 9
                  r:
                    Num: 10
              then:
                Return:
                  Num: 0
              els:
                Return:
                  Num: 1
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
