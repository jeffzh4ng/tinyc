use crate::parser;

pub fn gen(tree: parser::Program) -> Vec<String> {
    println!("{:?}", tree.main_function.statement);
    let expr = match tree.main_function.statement {
        parser::Stmt::Return(e) => gen_expr(e),
        parser::Stmt::If { cond, then, els } => {
            // check if cond is truthy
            // if truthy, gen instructions for then
            // if not truthy, gen instructions for els
            todo!()
        }
    };

    let output: Vec<String> = vec![
        ".text".to_owned(),
        ".globl main".to_owned(),
        ".section .text".to_owned(),
        "main:".to_owned(),
        expr.iter()
            .map(|line| format!("  {line}"))
            .collect::<Vec<_>>()
            .join("\n"),
        "  lw a0,0(sp)".to_owned(),
        "  addi sp,sp,8".to_owned(),
        "  ret".to_owned(),
        "".to_owned(),
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
            output.push("sw t1,0(sp)".to_owned()); // i128?
            output.push(
                "##############################################################################"
                    .to_owned(),
            );

            output
        }
        parser::Expr::String(_) => todo!(),
        parser::Expr::BinE { op, l, r } => {
            let left_expr = gen_expr(*l);
            let right_expr = gen_expr(*r);

            let mut output = Vec::with_capacity(left_expr.len() + right_expr.len() + 8);
            output.extend(left_expr);
            output.extend(right_expr);

            // emulating stack machine's push/pop 1AC with register machine's load/store 3AC
            // 1. pop the operands
            output.push("# 1. pop the operands".to_owned());
            output.push("lw t1,0(sp)".to_owned());
            output.push("addi sp,sp,8".to_owned());
            output.push("lw t2,0(sp)".to_owned());
            output.push("addi sp,sp,8".to_owned());
            output.push("".to_owned());

            // 2. operate on the operands
            let instr = match op {
                parser::BinOp::Add => "add t3,t1,t2".to_owned(),
                parser::BinOp::Sub => "sub t3,t2,t1".to_owned(),
                parser::BinOp::Mult => "mul t3,t1,t2".to_owned(),
                parser::BinOp::Div => "div t3,t2,t1".to_owned(),
            };
            output.push("# 2. operate on the operands".to_owned());
            output.push(instr);
            output.push("".to_owned());

            // 3. push the operands
            output.push("# 3. push the operands".to_owned());
            output.push("addi sp,sp,-8".to_owned());
            output.push("sw t3,0(sp)".to_owned());
            output.push(
                "##############################################################################"
                    .to_owned(),
            );

            output
        }
        parser::Expr::RelE { op, l, r } => todo!(),
        parser::Expr::LogE { op, l, r } => todo!(),
    }
}
