use crate::ir;

// int main() {
//     return 9 * 10 + 11 * 12;
//   }

pub enum Target {
    Rv32i,
    Llvm,
}

pub fn gen(ast: ir::Program, _t: &str) -> Vec<String> {
    let target = Target::Rv32i;

    match target {
        Target::Rv32i => gen_rv32i(ast),
        Target::Llvm => todo!(),
    }
}

fn gen_rv32i(tree: ir::Program) -> Vec<String> {
    let expr_mc = match tree.main_function.statement {
        ir::Statement::Return(e) => gen_expr(e),
    };

    let output: Vec<String> = vec![
        "  .global main".into(),
        "main:".into(),
        expr_mc,
        "  ret".into(),
    ];

    output
}

fn gen_expr(e: ir::Expr) -> String {
    match e {
        ir::Expr::Num(n) => format!("  li a0, {n}"), // TODO: Expr::Num(n) still inherting rust's semantics via 128.
        ir::Expr::String(_) => todo!(),
        ir::Expr::Binary { op: _, l: _, r: _ } => todo!(),
    }
}
