use crate::parser;

pub fn gen(tree: parser::Program) -> Vec<String> {
    let expr = match tree.main_function.statement {
        parser::Statement::Return(e) => gen_expr(e),
    };

    #[rustfmt::skip]
    let output: Vec<String> = vec![
        ".text".to_owned(),
        ".globl main".to_owned(),
        ".section .text".to_owned(),
        "main:".to_owned(),
        expr.iter().map(|line| format!("  {line}")).collect::<Vec<_>>().join("\n"),
        "  lw a0,0(sp)".to_owned(),
        "  addi sp,sp,8".to_owned(),
        "  ret".to_owned(),
    ];

    output
}

fn gen_expr(e: parser::Expr) -> Vec<String> {
    match e {
        parser::Expr::Num(n) => {
            let mut output = Vec::new();
            // 1. load the immediate
            output.push("# 1. load the immediate".to_owned());
            output.push(format!("li t1,{n}"));
            output.push("".to_owned());

            // 2. push the immediate
            output.push("# 2. push the immediate".to_owned());
            output.push("addi sp,sp,-8".to_owned());
            output.push(format!("sw t1,0(sp)")); // i128?
            output.push("".to_owned());

            output
        }
        parser::Expr::String(_) => todo!(),
        parser::Expr::Binary { op, l, r } => {
            let left_expr = gen_expr(*l);
            let right_expr = gen_expr(*r);

            let mut output = Vec::with_capacity(left_expr.len() + right_expr.len() + 16);
            output.extend(left_expr);
            output.extend(right_expr);

            // emulating stack machine's push/pop 1AC with register machine's load/store 3AC
            // 1. pop the operands
            output.push("# 1. pop the operands".to_owned());
            output.push("lw t1,0(sp)".to_owned());
            output.push("addi sp,sp,8".to_owned());
            output.push("lw t2,0(sp)".to_owned());
            output.push("addi sp,sp,8".to_owned());
            output.push("addi sp,sp,8".to_owned());
            output.push("".to_owned());

            // 2. operate on the operands (1 instruction)
            let instr = match op {
                parser::Op::Add => "add t3,t1,t2".to_owned(),
                parser::Op::Sub => "sub t3,t1,t2".to_owned(),
                parser::Op::Mult => "mult t3,t1,t2".to_owned(),
                parser::Op::Div => "div t3,t1,t2".to_owned(),
                parser::Op::AddAdd => todo!(),
            };
            output.push("# 2. operate on the operands".to_owned());
            output.push(instr);
            output.push("".to_owned());

            // 3. push the operands (2 instruction)
            output.push("# 3. push the operands".to_owned());
            output.push("addi sp,sp,-8".to_owned());
            output.push("sd t3,0(sp)".to_owned());
            output.push("".to_owned());

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
        let tree = parser::parse(tokens).unwrap();
        let stack_code = super::gen(tree);
        insta::assert_yaml_snapshot!(stack_code, @r###"
        ---
        - ".text"
        - ".globl main"
        - ".section .text"
        - "main:"
        - "  # 1. load the immediate\n  li t1,8\n  \n  # 2. push the immediate\n  addi sp,sp,-8\n  sw t1,0(sp)\n  "
        - "  lw a0,0(sp)"
        - "  addi sp,sp,8"
        - "  ret"
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
        let tree = parser::parse(tokens).unwrap();
        let stack_code = super::gen(tree);
        insta::assert_yaml_snapshot!(stack_code, @r###"
        ---
        - ".text"
        - ".globl main"
        - ".section .text"
        - "main:"
        - "  # 1. load the immediate\n  li t1,9\n  \n  # 2. push the immediate\n  addi sp,sp,-8\n  sw t1,0(sp)\n  \n  # 1. load the immediate\n  li t1,10\n  \n  # 2. push the immediate\n  addi sp,sp,-8\n  sw t1,0(sp)\n  \n  # 1. pop the operands\n  lw t1,0(sp)\n  addi sp,sp,8\n  lw t2,0(sp)\n  addi sp,sp,8\n  addi sp,sp,8\n  \n  # 2. operate on the operands\n  add t3,t1,t2\n  \n  # 3. push the operands\n  addi sp,sp,-8\n  sd t3,0(sp)\n  "
        - "  lw a0,0(sp)"
        - "  addi sp,sp,8"
        - "  ret"
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
        let tree = parser::parse(tokens).unwrap();
        let stack_code = super::gen(tree);
        insta::assert_yaml_snapshot!(stack_code, @r###"
        ---
        - ".text"
        - ".globl main"
        - ".section .text"
        - "main:"
        - "  # 1. load the immediate\n  li t1,9\n  \n  # 2. push the immediate\n  addi sp,sp,-8\n  sw t1,0(sp)\n  \n  # 1. load the immediate\n  li t1,10\n  \n  # 2. push the immediate\n  addi sp,sp,-8\n  sw t1,0(sp)\n  \n  # 1. pop the operands\n  lw t1,0(sp)\n  addi sp,sp,8\n  lw t2,0(sp)\n  addi sp,sp,8\n  addi sp,sp,8\n  \n  # 2. operate on the operands\n  add t3,t1,t2\n  \n  # 3. push the operands\n  addi sp,sp,-8\n  sd t3,0(sp)\n  \n  # 1. load the immediate\n  li t1,11\n  \n  # 2. push the immediate\n  addi sp,sp,-8\n  sw t1,0(sp)\n  \n  # 1. pop the operands\n  lw t1,0(sp)\n  addi sp,sp,8\n  lw t2,0(sp)\n  addi sp,sp,8\n  addi sp,sp,8\n  \n  # 2. operate on the operands\n  add t3,t1,t2\n  \n  # 3. push the operands\n  addi sp,sp,-8\n  sd t3,0(sp)\n  "
        - "  lw a0,0(sp)"
        - "  addi sp,sp,8"
        - "  ret"
        "###);
    }
}
