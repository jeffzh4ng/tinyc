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

    let src = std::env::args()
        .nth(1)
        .expect("error: no source file given");
    println!("Compiling source: {src}");

    let chars = fs::read(src)
        .expect("Should have been able to read the file")
        .iter()
        .map(|b| *b as char)
        .collect::<Vec<_>>();

    let tokens = lexer::scan(&chars);
    let tree = parser::parse(tokens).unwrap();
    let assembly = generator::gen(tree);

    let dest = "./tmp.s";
    println!("Generating target: {dest}");
    let mut f = fs::File::create(dest).expect("Unable to create file");
    f.write_all(assembly.join("\n").as_bytes())
        .expect("Unable to write data");
}
