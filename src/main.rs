use din::{lexer, parser};
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
    let chars = fs::read("tests/valid/hello.c")
        .expect("Should have been able to read the file")
        .iter()
        .map(|b| *b as char)
        .collect();
    let tokens = lexer::scan(chars);
    let tree = parser::parse_program(tokens).unwrap();
    println!("program: {:?}", tree);
}
