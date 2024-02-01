use din::scanner;

fn main() {
    let input = "2 + 3 * 5 - 8 / 3".chars().collect();
    let output = scanner::Scanner::scan(input);

    println!("{:?}", output);
}
