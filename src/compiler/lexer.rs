use serde::{Deserialize, Serialize};

// non-tokens:
// - comments
// - preprocessor directives
// - macros
// - whitespace: spaces, tabs, newlines

// see: ARCHITECTURE.md for a more specified lexical grammar

// note: variations are explicitly typed. Collapsing categories like keywords
//       into one variant while outsourcing variation to lexeme field on Token
//       will produce more work for syntactic analysis, since lexeme : String
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    // introductions (values)
    LiteralInt, // RE: [0-9]+
    Identifier, // RE: [a−zA−Z][a−zA−Z0−9]*

    // keywords (subset of identifiers)
    KeywordTypeInt,
    KeywordMain,
    KeywordVoid,

    // statements
    StatementReturn,

    // eliminations (operations)
    Plus,
    Minus,
    Star,
    Slash,

    // punctuation
    PuncLeftParen,
    PuncRightParen,
    PuncLeftBrace,
    PuncRightBrace,
    PuncSemiColon,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub lexeme: String,
    pub typ: TokenType,
}

// TODO: keep track of file and (col, row) for error reporting
// struct Position {}

// TODO: just filter out whitespace instead of having a helper function
pub fn scan(input: &[char]) -> Vec<Token> {
    let cs = skip_whitespace(input);

    // literals and identifiers have arbitrary length
    // operations and punctuations are single ASCII characters
    match cs {
        [] => vec![],
        [f, r @ ..] => match f {
            '0'..='9' => scan_int(cs),
            'a'..='z' | 'A'..='Z' => scan_id(cs),
            '+' => {
                let t = Token {
                    lexeme: String::from("+"),
                    typ: TokenType::Plus,
                };

                std::iter::once(t).chain(scan(r)).collect()
            }
            '-' => {
                let t = Token {
                    lexeme: String::from("-"),
                    typ: TokenType::Minus,
                };

                std::iter::once(t).chain(scan(r)).collect()
            }
            '*' => {
                let t = Token {
                    lexeme: String::from("*"),
                    typ: TokenType::Star,
                };

                std::iter::once(t).chain(scan(r)).collect()
            }
            '/' => {
                let t = Token {
                    lexeme: String::from("/"),
                    typ: TokenType::Slash,
                };

                std::iter::once(t).chain(scan(r)).collect()
            }
            '(' => {
                let t = Token {
                    lexeme: String::from("("),
                    typ: TokenType::PuncLeftParen,
                };

                std::iter::once(t).chain(scan(r)).collect()
            }
            ')' => {
                let t = Token {
                    lexeme: String::from(")"),
                    typ: TokenType::PuncRightParen,
                };

                std::iter::once(t).chain(scan(r)).collect()
            }
            '{' => {
                let t = Token {
                    lexeme: String::from("{"),
                    typ: TokenType::PuncLeftBrace,
                };

                std::iter::once(t).chain(scan(r)).collect()
            }
            '}' => {
                let t = Token {
                    lexeme: String::from("}"),
                    typ: TokenType::PuncRightBrace,
                };

                std::iter::once(t).chain(scan(r)).collect()
            }
            ';' => {
                let t = Token {
                    lexeme: String::from(";"),
                    typ: TokenType::PuncSemiColon,
                };

                std::iter::once(t).chain(scan(r)).collect()
            }
            _ => {
                let t = Token {
                    lexeme: String::from("PANIC?"),
                    typ: TokenType::Plus,
                };

                std::iter::once(t).chain(scan(r)).collect()
            }
        },
    }
}

fn scan_int(input: &[char]) -> Vec<Token> {
    // scan_int calls skip_whitespace too to remain idempotent
    let cs = skip_whitespace(input);

    match cs {
        [] => vec![],
        [f, r @ ..] => match f {
            '0'..='9' => {
                #[rustfmt::skip]
                let i = r
                    .iter()
                    .take_while(|&&c| c.is_numeric())
                    .count();

                #[rustfmt::skip]
                let f = cs[..=i]
                    .iter()
                    .collect::<String>();
                let new_r = &cs[i + 1..];

                let t = Token {
                    lexeme: f,
                    typ: TokenType::LiteralInt,
                };

                std::iter::once(t).chain(scan(new_r)).collect()
            }
            _ => {
                // panic
                todo!()
            }
        },
    }
}

// TODO: support identifiers with alpha*numeric* characters after first alphabetic
fn scan_id(input: &[char]) -> Vec<Token> {
    // scan_id calls skip_whitespace too to remain idempotent
    let cs = skip_whitespace(input);

    match cs {
        [] => vec![],
        [f, r @ ..] => match f {
            'a'..='z' => {
                // Find the index where the alphabetic characters end
                #[rustfmt::skip]
                let i = r
                    .iter()
                    .take_while(|&&c| c.is_alphabetic())
                    .count();

                let f = (cs[..=i].iter()).collect::<String>();
                let new_r = &cs[i + 1..];

                let keyword = match f.as_str() {
                    "int" => Some(Token {
                        lexeme: String::from("int"),
                        typ: TokenType::KeywordTypeInt,
                    }),
                    "main" => Some(Token {
                        lexeme: String::from("main"),
                        typ: TokenType::KeywordMain,
                    }),
                    "return" => Some(Token {
                        lexeme: String::from("return"),
                        typ: TokenType::StatementReturn,
                    }),
                    _ => None,
                };

                let t = match keyword {
                    Some(k) => k,
                    None => Token {
                        lexeme: f,
                        typ: TokenType::Identifier,
                    },
                };

                std::iter::once(t).chain(scan(new_r)).collect()
            }
            _ => {
                // panic
                todo!()
            }
        },
    }
}

fn skip_whitespace(input: &[char]) -> &[char] {
    match input {
        [] => input,
        [f, r @ ..] => {
            if f.is_whitespace() {
                skip_whitespace(r)
            } else {
                input
            }
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
    use insta;
    use std::fs;

    use super::*;

    #[test]
    fn hello() {
        #[rustfmt::skip]
        let input = fs::read("tests/valid/hello.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let output = scan(input.as_slice());
        insta::assert_yaml_snapshot!(output);
    }

    #[test]
    fn arithmetic_add() {
        #[rustfmt::skip]
        let input = fs::read("tests/valid/arithmetic/add.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let output = scan(input.as_slice());
        insta::assert_yaml_snapshot!(output);
    }

    #[test]
    fn arithmetic_add_multi() {
        #[rustfmt::skip]
        let input = fs::read("tests/valid/arithmetic/add_multi.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let output = scan(input.as_slice());
        insta::assert_yaml_snapshot!(output);
    }

    #[test]
    fn arithmetic_sub() {
        #[rustfmt::skip]
        let input = fs::read("tests/valid/arithmetic/sub.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let output = scan(input.as_slice());
        insta::assert_yaml_snapshot!(output);
    }

    #[test]
    fn arithmetic_mult() {
        #[rustfmt::skip]
        let input = fs::read("tests/valid/arithmetic/mult.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let output = scan(input.as_slice());
        insta::assert_yaml_snapshot!(output);
    }

    #[test]
    fn arithmetic_div() {
        #[rustfmt::skip]
        let input = fs::read("tests/valid/arithmetic/div.c")
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let output = scan(input.as_slice());
        insta::assert_yaml_snapshot!(output);
    }
}

#[cfg(test)]
mod test_invalid {}

#[cfg(test)]
mod test_skip_whitespace {
    use super::*;

    #[test]
    fn skip_space() {
        let input = "    7".chars().collect::<Vec<_>>();
        let output = skip_whitespace(input.as_slice());
        let expected_output = "7".chars().collect::<Vec<_>>();

        assert!(vecs_match(&output.to_vec(), &expected_output))
    }

    #[test]
    fn skip_newline() {
        let input = r#"




        7"#
        .chars()
        .collect::<Vec<_>>();
        let output = skip_whitespace(input.as_slice());
        let expected_output = "7".chars().collect::<Vec<_>>();

        assert!(vecs_match(&output.to_vec(), &expected_output))
    }
}

#[cfg(test)]
mod test_arithmetic {
    use super::*;

    #[test]
    fn simple() {
        let input = "9 + 8".chars().collect::<Vec<_>>();
        let output = scan(input.as_slice());
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: String::from("9"), typ: TokenType::LiteralInt },
            Token { lexeme: String::from("+"), typ: TokenType::Plus },
            Token { lexeme: String::from("8"), typ: TokenType::LiteralInt },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn simple_two() {
        let input = "90 + 80".chars().collect::<Vec<_>>();
        let output = scan(input.as_slice());
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: String::from("90"), typ: TokenType::LiteralInt },
            Token { lexeme: String::from("+"), typ: TokenType::Plus },
            Token { lexeme: String::from("80"), typ: TokenType::LiteralInt },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn complex() {
        let input = "2 + 3 * 5 - 8 / 3".chars().collect::<Vec<_>>();
        let output = scan(input.as_slice());
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: String::from("2"), typ: TokenType::LiteralInt },
            Token { lexeme: String::from("+"), typ: TokenType::Plus },
            Token { lexeme: String::from("3"), typ: TokenType::LiteralInt },
            Token { lexeme: String::from("*"), typ: TokenType::Star },
            Token { lexeme: String::from("5"), typ: TokenType::LiteralInt },
            Token { lexeme: String::from("-"), typ: TokenType::Minus },
            Token { lexeme: String::from("8"), typ: TokenType::LiteralInt },
            Token { lexeme: String::from("/"), typ: TokenType::Slash },
            Token { lexeme: String::from("3"), typ: TokenType::LiteralInt },
        ];

        assert!(vecs_match(&output, &expected_output))
    }

    #[test]
    fn complex_two() {
        let input = "22 + 33 * 55 - 88 / 33".chars().collect::<Vec<_>>();
        let output = scan(input.as_slice());
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: String::from("22"), typ: TokenType::LiteralInt },
            Token { lexeme: String::from("+"), typ: TokenType::Plus },
            Token { lexeme: String::from("33"), typ: TokenType::LiteralInt },
            Token { lexeme: String::from("*"), typ: TokenType::Star },
            Token { lexeme: String::from("55"), typ: TokenType::LiteralInt },
            Token { lexeme: String::from("-"), typ: TokenType::Minus },
            Token { lexeme: String::from("88"), typ: TokenType::LiteralInt },
            Token { lexeme: String::from("/"), typ: TokenType::Slash },
            Token { lexeme: String::from("33"), typ: TokenType::LiteralInt },
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
        .collect::<Vec<_>>();
        let output = scan(input.as_slice());
        #[rustfmt::skip]
        let expected_output = vec![
            Token { lexeme: String::from("23"), typ: TokenType::LiteralInt },
            Token { lexeme: String::from("+"), typ: TokenType::Plus },
            Token { lexeme: String::from("18"), typ: TokenType::LiteralInt },
            Token { lexeme: String::from("-"), typ: TokenType::Minus },
            Token { lexeme: String::from("45"), typ: TokenType::LiteralInt },
            Token { lexeme: String::from("*"), typ: TokenType::Star },
            Token { lexeme: String::from("2"), typ: TokenType::LiteralInt },
            Token { lexeme: String::from("/"), typ: TokenType::Slash },
            Token { lexeme: String::from("18"), typ: TokenType::LiteralInt },
        ];

        assert!(vecs_match(&output, &expected_output))
    }
}
