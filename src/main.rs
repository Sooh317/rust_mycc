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
    let ast_tree = parser::Node::parse(expression, &tokens);
    // println!("{:?}", ast_tree);
    let index = ast_tree.len() - 1;

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    codegen::generate_code(&ast_tree, &index);

    println!("  pop rax");
    println!("  ret");

}
