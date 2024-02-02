#[derive(PartialEq, Debug)]
pub enum Category {
    // literals
    Int,

    // single char
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub lexeme: String,
    pub category: Category,
    // line: u32,
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
                        lexeme: String::from(f),
                        category: Category::Int,
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
                        lexeme: String::from("+"),
                        category: Category::Plus,
                    };

                    std::iter::once(t)
                        .chain(Scanner::scan(r.to_vec()))
                        .collect()
                }
                '-' => {
                    let t = Token {
                        lexeme: String::from("-"),
                        category: Category::Minus,
                    };

                    std::iter::once(t)
                        .chain(Scanner::scan(r.to_vec()))
                        .collect()
                }
                '*' => {
                    let t = Token {
                        lexeme: String::from("*"),
                        category: Category::Star,
                    };

                    std::iter::once(t)
                        .chain(Scanner::scan(r.to_vec()))
                        .collect()
                }
                '/' => {
                    let t = Token {
                        lexeme: String::from("/"),
                        category: Category::Slash,
                    };

                    std::iter::once(t)
                        .chain(Scanner::scan(r.to_vec()))
                        .collect()
                }
                _ => {
                    let t = Token {
                        lexeme: String::from("PANIC?"),
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
mod test_scan {
    use super::*;

    #[test]
    fn simple() {
        let input = "9 + 8".chars().collect();
        let output = Scanner::scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: String::from("9"), category: Category::Int },
            Token { lexeme: String::from("+"), category: Category::Plus },
            Token { lexeme: String::from("8"), category: Category::Int },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn simple_two() {
        let input = "90 + 80".chars().collect();
        let output = Scanner::scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: String::from("90"), category: Category::Int },
            Token { lexeme: String::from("+"), category: Category::Plus },
            Token { lexeme: String::from("80"), category: Category::Int },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn complex() {
        let input = "2 + 3 * 5 - 8 / 3".chars().collect();
        let output = Scanner::scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: String::from("2"), category: Category::Int },
            Token { lexeme: String::from("+"), category: Category::Plus },
            Token { lexeme: String::from("3"), category: Category::Int },
            Token { lexeme: String::from("*"), category: Category::Star },
            Token { lexeme: String::from("5"), category: Category::Int },
            Token { lexeme: String::from("-"), category: Category::Minus },
            Token { lexeme: String::from("8"), category: Category::Int },
            Token { lexeme: String::from("/"), category: Category::Slash },
            Token { lexeme: String::from("3"), category: Category::Int },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn complex_two() {
        let input = "22 + 33 * 55 - 88 / 33".chars().collect();
        let output = Scanner::scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: String::from("22"), category: Category::Int },
            Token { lexeme: String::from("+"), category: Category::Plus },
            Token { lexeme: String::from("33"), category: Category::Int },
            Token { lexeme: String::from("*"), category: Category::Star },
            Token { lexeme: String::from("55"), category: Category::Int },
            Token { lexeme: String::from("-"), category: Category::Minus },
            Token { lexeme: String::from("88"), category: Category::Int },
            Token { lexeme: String::from("/"), category: Category::Slash },
            Token { lexeme: String::from("33"), category: Category::Int },
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
            Token { lexeme: String::from("23"), category: Category::Int },
            Token { lexeme: String::from("+"), category: Category::Plus },
            Token { lexeme: String::from("18"), category: Category::Int },
            Token { lexeme: String::from("-"), category: Category::Minus },
            Token { lexeme: String::from("45"), category: Category::Int },
            Token { lexeme: String::from("*"), category: Category::Star },
            Token { lexeme: String::from("2"), category: Category::Int },
            Token { lexeme: String::from("/"), category: Category::Slash },
            Token { lexeme: String::from("18"), category: Category::Int },
        ];

        assert!(vecs_match(&output, &expected_output))
    }
}
