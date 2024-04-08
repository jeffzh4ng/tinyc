use super::ir;
use std::fs;

// mod eval;
mod lexer;
mod parser;
mod typer;

pub fn parse(file: &str) -> ir::Program {
    let chars = fs::read(file)
        .expect("Should have been able to read the file")
        .iter()
        .map(|b| *b as char)
        .collect::<Vec<_>>();

    let tokens = lexer::scan(&chars);
    println!("tokens: {:?}", tokens);

    let tree = parser::parse_program(tokens).unwrap();
    println!("tree: {:?}", tree);

    // if !typer::type_program(&tree) {
    //     // return error
    //     todo!()
    // }

    tree
}
