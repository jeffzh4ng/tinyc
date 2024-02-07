use std::io;

use crate::lexer::{Token, TokenType};
use crate::rep::{Expr, MainFunction, Program, Statement};

pub fn parse_program(tokens: Vec<Token>) -> Result<Program, io::Error> {
    let main_function = parse_function(tokens)?;
    Ok(Program { main_function })
}

fn parse_function(tokens: Vec<Token>) -> Result<MainFunction, io::Error> {
    let tokens = mtch(tokens, TokenType::KeywordTypeInt)?;
    let tokens = mtch(tokens, TokenType::KeywordMain)?;
    let tokens = mtch(tokens, TokenType::PuncLeftParen)?;
    let tokens = mtch(tokens, TokenType::PuncRightParen)?;
    let tokens = mtch(tokens, TokenType::PuncLeftBrace)?;
    let (statement, tokens) = parse_statement(tokens)?;
    let tokens = mtch(tokens, TokenType::PuncRightBrace)?;

    if !tokens.is_empty() {
        // panic?
    }

    Ok(MainFunction { statement })
}

fn parse_statement(tokens: Vec<Token>) -> Result<(Statement, Vec<Token>), io::Error> {
    match tokens.as_slice() {
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            // Category::StatementIf => todo!(),
            // Category::StatementWhile => todo!(),
            // Category::StatementFor => todo!(),
            TokenType::StatementReturn => {
                let tokens = mtch(tokens, TokenType::StatementReturn)?;
                let (expr, tokens) = parse_expr(tokens)?;
                let tokens = mtch(tokens, TokenType::PuncSemiColon)?;
                Ok((Statement::Return(expr), tokens))
            }
            _ => todo!(), // panic?
        },
    }
}

fn parse_expr(tokens: Vec<Token>) -> Result<(Expr, Vec<Token>), io::Error> {
    // parse_expr(tokens); // infinite recursion; stack overflow

    match tokens.clone().as_slice() {
        // expensive: cloning b/c of recursive call below
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TokenType::LiteralInt => {
                let tokens = mtch(tokens, TokenType::LiteralInt)?;
                // while parse_binop:
                //      ...
                Ok((Expr::Num(f.lexeme.parse().unwrap()), r.to_vec()))
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

fn mtch(tokens: Vec<Token>, tt: TokenType) -> Result<Vec<Token>, io::Error> {
    match tokens.as_slice() {
        [] => todo!(),
        [f, r @ ..] => match &f.typ {
            tt => Ok(r.to_vec()), // TODO: expensive bc pure recursive
            _ => todo!(),         // panic
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn arithmetic() {
    //     let input = vec![Token {
    //         lexeme: String::from("8"),
    //         category: Category::LiteralInt,
    //     }];
    //     let output = Parser::parse(input);
    //     let expected_output = Expr::Num(8);

    //     assert_eq!(output, expected_output);
    // }
}
