use std::{env, process};
mod tokenizer;
mod parser;
mod codegen;

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

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, {}", lvar_num); // lvar_num is a multiple of 16

    let mut branch_num = 0;
    for ast_tree in ast_trees {
        let index = ast_tree.len() - 1;
        codegen::generate_code(&ast_tree, &index, &mut branch_num);
        println!("  pop rax");
    }
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");

}
