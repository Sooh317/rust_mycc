use std::{env, process};
mod tokenizer;
mod parser;
mod codegen;
use codegen::Register;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません\n");
        process::exit(1);
    }

    let expression = &args[1];
    let tokens = tokenizer::Token::tokenize(expression);
    // println!("{:?}", tokens);
    let (ast_trees, lvar_num) = parser::Node::parse(expression, &tokens);
    // println!("{:?}", ast_trees);

    let mut regs = Register::new();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    regs.push("rbp");
    regs.mov_rsp_to_rbp();
    regs.sub_rsp(lvar_num * 8);
    // println!("  push rbp");
    // println!("  mov rbp, rsp");
    // println!("  sub rsp, {}", lvar_num * 8);

    let mut branch_num = 0;
    for ast_tree in ast_trees {
        let index = ast_tree.len() - 1;
        codegen::generate_code(&ast_tree, &index, &mut branch_num, &mut regs);
        regs.pop("rax");
        // println!("  pop rax");
    }
    regs.mov_rbp_to_rsp();
    regs.pop("rbp");
    // println!("  mov rsp, rbp");
    // println!("  pop rbp");
    println!("  ret");

}
