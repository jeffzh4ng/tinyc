#[derive(PartialEq, Debug)]
pub enum Category {
    // single char
    Plus,
    Minus,
    Star,
    Slash,

    // literals
    Int,
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
                        lexeme: f.to_string(),
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
                '+' => {
                    let t = Token {
                        lexeme: "+".to_string(),
                        category: Category::Plus,
                    };

                    std::iter::once(t)
                        .chain(Scanner::scan(r.to_vec()))
                        .collect()
                }
                '-' => {
                    let t = Token {
                        lexeme: "-".to_string(),
                        category: Category::Minus,
                    };

                    std::iter::once(t)
                        .chain(Scanner::scan(r.to_vec()))
                        .collect()
                }
                '*' => {
                    let t = Token {
                        lexeme: "*".to_string(),
                        category: Category::Star,
                    };

                    std::iter::once(t)
                        .chain(Scanner::scan(r.to_vec()))
                        .collect()
                }
                '/' => {
                    let t = Token {
                        lexeme: "/".to_string(),
                        category: Category::Slash,
                    };

                    std::iter::once(t)
                        .chain(Scanner::scan(r.to_vec()))
                        .collect()
                }
                '0'..='9' => Scanner::scan_int(cs),
                _ => {
                    let t = Token {
                        lexeme: "PANIC?".to_string(),
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
            Token { lexeme: "9".to_string(), category: Category::Int },
            Token { lexeme: "+".to_string(), category: Category::Plus },
            Token { lexeme: "8".to_string(), category: Category::Int },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn simple_two() {
        let input = "90 + 80".chars().collect();
        let output = Scanner::scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: "90".to_string(), category: Category::Int },
            Token { lexeme: "+".to_string(), category: Category::Plus },
            Token { lexeme: "80".to_string(), category: Category::Int },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn complex() {
        let input = "2 + 3 * 5 - 8 / 3".chars().collect();
        let output = Scanner::scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: "2".to_string(), category: Category::Int },
            Token { lexeme: "+".to_string(), category: Category::Plus },
            Token { lexeme: "3".to_string(), category: Category::Int },
            Token { lexeme: "*".to_string(), category: Category::Star },
            Token { lexeme: "5".to_string(), category: Category::Int },
            Token { lexeme: "-".to_string(), category: Category::Minus },
            Token { lexeme: "8".to_string(), category: Category::Int },
            Token { lexeme: "/".to_string(), category: Category::Slash },
            Token { lexeme: "3".to_string(), category: Category::Int },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn complex_two() {
        let input = "22 + 33 * 55 - 88 / 33".chars().collect();
        let output = Scanner::scan(input);
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: "22".to_string(), category: Category::Int },
            Token { lexeme: "+".to_string(), category: Category::Plus },
            Token { lexeme: "33".to_string(), category: Category::Int },
            Token { lexeme: "*".to_string(), category: Category::Star },
            Token { lexeme: "55".to_string(), category: Category::Int },
            Token { lexeme: "-".to_string(), category: Category::Minus },
            Token { lexeme: "88".to_string(), category: Category::Int },
            Token { lexeme: "/".to_string(), category: Category::Slash },
            Token { lexeme: "33".to_string(), category: Category::Int },
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
            Token { lexeme: "23".to_string(), category: Category::Int },
            Token { lexeme: "+".to_string(), category: Category::Plus },
            Token { lexeme: "18".to_string(), category: Category::Int },
            Token { lexeme: "-".to_string(), category: Category::Minus },
            Token { lexeme: "45".to_string(), category: Category::Int },
            Token { lexeme: "*".to_string(), category: Category::Star },
            Token { lexeme: "2".to_string(), category: Category::Int },
            Token { lexeme: "/".to_string(), category: Category::Slash },
            Token { lexeme: "18".to_string(), category: Category::Int },
        ];

        assert!(vecs_match(&output, &expected_output))
    }
}
