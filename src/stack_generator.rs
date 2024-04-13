use crate::parser;
use serde::{Deserialize, Serialize};

// Load and store architecture encoded via 1AC linear representation
// The 1 address is implicitly at the top of the stack, and so all
// arithemtic ops perform load operations implicitly
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum OpCode {
    // ARITHMETIC
    ADD,
    SUB,
    MULT,
    DIV,
    // STORE
    PUSH(i128),
}

pub fn gen(tree: parser::Program) -> Vec<OpCode> {
    let expr_mc = match tree.main_function.statement {
        parser::Statement::Return(e) => gen_expr(e),
    };

    for x in &expr_mc {
        println!("{:?}", x);
    }

    expr_mc
}

fn gen_expr(e: parser::Expr) -> Vec<OpCode> {
    match e {
        parser::Expr::Num(n) => vec![OpCode::PUSH(n)], // i128?
        parser::Expr::String(_) => todo!(),
        parser::Expr::Binary { op, l, r } => {
            let left_expr = gen_expr(*l);
            let right_expr = gen_expr(*r);
            let op_code = match op {
                parser::Op::Add => OpCode::ADD,
                parser::Op::Sub => OpCode::SUB,
                parser::Op::Mult => OpCode::MULT,
                parser::Op::Div => OpCode::DIV,
                parser::Op::AddAdd => todo!(),
            };

            let mut output = Vec::with_capacity(left_expr.len() + right_expr.len() + 1);
            output.extend(left_expr);
            output.extend(right_expr);
            output.push(op_code);
            output
        }
    }
}

#[cfg(test)]
mod test_legal_arithmetic {
    use crate::{lexer, parser};
    use std::fs;

    const TEST_DIR: &str = "tests/fixtures/din/legal/arithmetic";

    #[test]
    fn lit() {
        #[rustfmt::skip]
        let chars = fs::read(format!("{TEST_DIR}/lit.c"))
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::scan(&chars);
        let tree = parser::parse_program(tokens).unwrap();
        let stack_code = super::gen(tree);
        insta::assert_yaml_snapshot!(stack_code, @r###"
        ---
        - PUSH: 8
        "###);
    }

    #[test]
    fn test_add() {
        #[rustfmt::skip]
        let chars = fs::read(format!("{TEST_DIR}/add.c"))
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::scan(&chars);
        let tree = parser::parse_program(tokens).unwrap();
        let stack_code = super::gen(tree);
        insta::assert_yaml_snapshot!(stack_code, @r###"
        ---
        - PUSH: 9
        - PUSH: 10
        - ADD
        "###);
    }

    #[test]
    fn test_add_multi() {
        #[rustfmt::skip]
        let chars = fs::read(format!("{TEST_DIR}/add_multi.c"))
            .expect("Should have been able to read the file")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::scan(&chars);
        let tree = parser::parse_program(tokens).unwrap();
        let stack_code = super::gen(tree);
        insta::assert_yaml_snapshot!(stack_code, @r###"
        ---
        - PUSH: 9
        - PUSH: 10
        - ADD
        - PUSH: 11
        - ADD
        "###);
    }
}
