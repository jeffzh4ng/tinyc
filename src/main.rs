use din::{gen, lexer, parser};
use std::{fs, io::Write};

fn main() {
    println!(
        "
    ⠀⠀⠀⠀⠀⣼⣧⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⣼⣿⣿⣧⠀⠀⠀⠀
    ⠀⠀⠀⠾⠿⠿⠿⠿⠷⠀⠀⠀
    ⠀⠀⣼⣆⠀⠀⠀⠀⣰⣧⠀⠀
    ⠀⣼⣿⣿⣆⠀⠀⣰⣿⣿⣧⠀
    ⠾⠟⠿⠿⠿⠧⠼⠿⠿⠿⠻⠷
    din: C89/90 -> RV32I
    "
    );

    let src = "tests/fixtures/din/legal/arithmetic/lit.c";
    let trgt = "rv32i";
    let dest = "./tmp.s";

    let chars = fs::read(src)
        .expect("Should have been able to read the file")
        .iter()
        .map(|b| *b as char)
        .collect::<Vec<_>>();

    let tokens = lexer::scan(&chars);
    let tree = parser::parse_program(tokens).unwrap();
    let mc = gen::gen(tree, trgt).join("\n");

    let mut f = fs::File::create(dest).expect("Unable to create file");
    f.write_all(mc.as_bytes()).expect("Unable to write data");

    // gcc to assemble and link
}
