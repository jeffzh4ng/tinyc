#[derive(PartialEq, Debug)]

pub enum Category {
    // introductions (values)
    LiteralInt,
    Identifier,

    // eliminations (operations)
    Plus,
    Minus,
    Star,
    Slash,

    // punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SemiColon,

    // keywords
    ReturnInt,
    Void,
    Return,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub lexeme: Option<String>,
    pub category: Category,
}

#[derive(Default)]
pub struct Scanner {}

impl Scanner {
    fn skip_whitespace(input: Vec<char>) -> Vec<char> {
        match input.as_slice() {
            [] => vec![],
            [f, r @ ..] => {
                if f.is_whitespace() {
                    Scanner::skip_whitespace(r.to_vec())
                } else {
                    input
                }
            }
        }
    }

    fn scan_int(input: Vec<char>) -> Vec<Token> {
        // scan_int calls skip_whitespace too to remain idempotent
        let cs: Vec<char> = Scanner::skip_whitespace(input);
        match cs.as_slice() {
            [] => vec![],
            [f, _r @ ..] => match f {
                '0'..='9' => {
                    #[rustfmt::skip]
                    let f = cs
                        .iter()
                        .take_while(|&&c| c.is_numeric())
                        .collect::<String>();

                    #[rustfmt::skip]
                    let r = cs
                        .into_iter()
                        .skip_while(|&c| c.is_numeric())
                        .collect::<Vec<_>>();

                    let t = Token {
                        lexeme: Some(String::from(f)),
                        category: Category::LiteralInt,
                    };

                    std::iter::once(t)
                        .chain(Scanner::scan(r.to_vec()))
                        .collect()
                }
                _ => {
                    // panic
                    todo!()
                }
            },
        }
    }

    pub fn scan(input: Vec<char>) -> Vec<Token> {
        let cs = Scanner::skip_whitespace(input);
        match cs.as_slice() {
            [] => vec![],
            [f, r @ ..] => match f {
                '0'..='9' => Scanner::scan_int(cs),
                '+' => {
                    let t = Token {
                        lexeme: Some(String::from("+")),
                        category: Category::Plus,
                    };

                    std::iter::once(t)
                        .chain(Scanner::scan(r.to_vec()))
                        .collect()
                }
                '-' => {
                    let t = Token {
                        lexeme: Some(String::from("-")),
                        category: Category::Minus,
                    };

                    std::iter::once(t)
                        .chain(Scanner::scan(r.to_vec()))
                        .collect()
                }
                '*' => {
                    let t = Token {
                        lexeme: Some(String::from("*")),
                        category: Category::Star,
                    };

                    std::iter::once(t)
                        .chain(Scanner::scan(r.to_vec()))
                        .collect()
                }
                '/' => {
                    let t = Token {
                        lexeme: Some(String::from("/")),
                        category: Category::Slash,
                    };

                    std::iter::once(t)
                        .chain(Scanner::scan(r.to_vec()))
                        .collect()
                }
                _ => {
                    let t = Token {
                        lexeme: Some(String::from("PANIC?")),
                        category: Category::Plus,
                    };

                    std::iter::once(t)
                        .chain(Scanner::scan(r.to_vec()))
                        .collect()
                }
            },
        }
    }
}

#[cfg(test)]
fn vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    #[rustfmt::skip]
    let matching = a
        .iter()
        .zip(b.iter())
        .filter(|&(a, b)| a == b)
        .count();

    matching == a.len() && matching == b.len()
}

#[cfg(test)]
mod test_valid {
    use std::fs;

    use super::*;

    #[test]
    fn hello() {
        #[rustfmt::skip]
        let input = fs::read("tests/valid/hello.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect();

        println!("{:?}", input);
        let output = Scanner::scan(input);
        let expected_output = todo!();
        println!("{:?}", output);
    }
}

#[cfg(test)]
mod test_invalid {}

#[cfg(test)]
mod test_skip_whitespace {
    use super::*;

    #[test]
    fn skip_space() {
        let input = "    7".chars().collect();
        let output = Scanner::skip_whitespace(input);
        let expected_output = "7".chars().collect();

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn skip_newline() {
        let input = r#"




        7"#
        .chars()
        .collect();
        let output = Scanner::skip_whitespace(input);
        let expected_output = "7".chars().collect();

        assert!(vecs_match(&output, &expected_output))
    }
}

#[cfg(test)]
mod test_arithmetic {
    use super::*;

    #[test]
    fn simple() {
        let input = "9 + 8".chars().collect();
        let output = Scanner::scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: Some(String::from("9")), category: Category::LiteralInt },
            Token { lexeme: Some(String::from("+")), category: Category::Plus },
            Token { lexeme: Some(String::from("8")), category: Category::LiteralInt },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn simple_two() {
        let input = "90 + 80".chars().collect();
        let output = Scanner::scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: Some(String::from("90")), category: Category::LiteralInt },
            Token { lexeme: Some(String::from("+")), category: Category::Plus },
            Token { lexeme: Some(String::from("80")), category: Category::LiteralInt },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn complex() {
        let input = "2 + 3 * 5 - 8 / 3".chars().collect();
        let output = Scanner::scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: Some(String::from("2")), category: Category::LiteralInt },
            Token { lexeme: Some(String::from("+")), category: Category::Plus },
            Token { lexeme: Some(String::from("3")), category: Category::LiteralInt },
            Token { lexeme: Some(String::from("*")), category: Category::Star },
            Token { lexeme: Some(String::from("5")), category: Category::LiteralInt },
            Token { lexeme: Some(String::from("-")), category: Category::Minus },
            Token { lexeme: Some(String::from("8")), category: Category::LiteralInt },
            Token { lexeme: Some(String::from("/")), category: Category::Slash },
            Token { lexeme: Some(String::from("3")), category: Category::LiteralInt },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn complex_two() {
        let input = "22 + 33 * 55 - 88 / 33".chars().collect();
        let output = Scanner::scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: Some(String::from("22")), category: Category::LiteralInt },
            Token { lexeme: Some(String::from("+")), category: Category::Plus },
            Token { lexeme: Some(String::from("33")), category: Category::LiteralInt },
            Token { lexeme: Some(String::from("*")), category: Category::Star },
            Token { lexeme: Some(String::from("55")), category: Category::LiteralInt },
            Token { lexeme: Some(String::from("-")), category: Category::Minus },
            Token { lexeme: Some(String::from("88")), category: Category::LiteralInt },
            Token { lexeme: Some(String::from("/")), category: Category::Slash },
            Token { lexeme: Some(String::from("33")), category: Category::LiteralInt },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn complex_three() {
        let input = r#"
        23 +
        18 -
        45 * 2
        / 18
        "#
        .chars()
        .collect();
        let output = Scanner::scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: Some(String::from("23")), category: Category::LiteralInt },
            Token { lexeme: Some(String::from("+")), category: Category::Plus },
            Token { lexeme: Some(String::from("18")), category: Category::LiteralInt },
            Token { lexeme: Some(String::from("-")), category: Category::Minus },
            Token { lexeme: Some(String::from("45")), category: Category::LiteralInt },
            Token { lexeme: Some(String::from("*")), category: Category::Star },
            Token { lexeme: Some(String::from("2")), category: Category::LiteralInt },
            Token { lexeme: Some(String::from("/")), category: Category::Slash },
            Token { lexeme: Some(String::from("18")), category: Category::LiteralInt },
        ];

        assert!(vecs_match(&output, &expected_output))
    }
}
