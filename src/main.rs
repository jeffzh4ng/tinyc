use din::{eval, lexer, parser, typer};
use std::fs;

fn main() {
    println!(
        "
    ⠀⠀⠀⠀⠀⣼⣧⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⣼⣿⣿⣧⠀⠀⠀⠀
    ⠀⠀⠀⠾⠿⠿⠿⠿⠷⠀⠀⠀
    ⠀⠀⣼⣆⠀⠀⠀⠀⣰⣧⠀⠀
    ⠀⣼⣿⣿⣆⠀⠀⣰⣿⣿⣧⠀
    ⠾⠟⠿⠿⠿⠧⠼⠿⠿⠿⠻⠷
    din: C89/90 -> RISC V
    "
    );

    let chars = fs::read("tests/valid/arithmetic/addition_multi.c")
        .expect("Should have been able to read the file")
        .iter()
        .map(|b| *b as char)
        .collect::<Vec<_>>();

    let tokens = lexer::scan(&chars);
    println!("tokens: {:?}", tokens);

    let tree = parser::parse_program(tokens).unwrap();
    println!("tree: {:?}", tree);

    if !typer::type_program(&tree) {
        // return error
        todo!()
    }

    let res = eval::eval_program(tree);
    println!("result: {:?}", res);
}
