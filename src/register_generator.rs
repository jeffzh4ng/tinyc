use crate::stack_generator;

pub enum Target {
    Rv32i,
    Llvm,
}

pub fn gen(stack_code: Vec<stack_generator::OpCode>, _t: &str) -> Vec<String> {
    let target = Target::Rv32i;

    match target {
        Target::Rv32i => gen_rv32i(stack_code),
        Target::Llvm => todo!(),
    }
}

fn gen_rv32i(stack_code: Vec<stack_generator::OpCode>) -> Vec<String> {
    #[rustfmt::skip]
    let output: Vec<String> = vec![
        "  .global main".into(),
        "main:".into(),
        "  ret".into(),
    ];

    output
}
