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
    let asts = parser::Node::parse(expression, &tokens);
    // println!("{:?}", asts);

    println!(".intel_syntax noprefix");
    println!(".globl main");

    let mut branch_num = 0;
    for ast in asts {
        let index = ast.tree.len() - 1;
        codegen::generate_code(&ast, &index, &mut branch_num);
        println!("  pop rax"); // pop return value of the previously executed function
    }

}
