use crate::parser;

pub enum OpCode {
    ADD,
    SUB,
    MULT,
    DIV,
}

pub fn gen(tree: parser::Program) -> Vec<OpCode> {
    let expr_mc = match tree.main_function.statement {
        parser::Statement::Return(e) => gen_expr(e),
    };

    expr_mc
}

fn gen_expr(e: parser::Expr) -> Vec<OpCode> {
    let mut output = Vec::new();

    match e {
        parser::Expr::Num(n) => output.push(format!("  li a0, {n}")),
        parser::Expr::String(_) => todo!(),
        parser::Expr::Binary { op, l, r } => {
            let left_register = gen_expr(*l);
            let right_register = gen_expr(*r);

            // output.push(format!("li t0 {l_mc}"));
            // output.push(format!("addi a0 {r_mc}"))
        }
    }

    todo!()

    // output.join("\n")
}
