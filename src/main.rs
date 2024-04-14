use din::{generator, lexer, parser};
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

    println!("Compiling source: tests/fixtures/din/legal/arithmetic/add_multi.c");
    let src = "tests/fixtures/din/legal/arithmetic/add_multi.c";
    let dest = "./tmp.s";

    let chars = fs::read(src)
        .expect("Should have been able to read the file")
        .iter()
        .map(|b| *b as char)
        .collect::<Vec<_>>();

    let tokens = lexer::scan(&chars);
    let tree = parser::parse(tokens).unwrap();
    let assembly = generator::gen(tree);

    println!("Generating target: {dest}");
    let mut f = fs::File::create(dest).expect("Unable to create file");
    f.write_all(assembly.join("\n").as_bytes())
        .expect("Unable to write data");
}
