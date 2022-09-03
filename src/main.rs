use std::{env, process};
mod tokenizer;
mod parser;

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
    let index = ast_tree.len() - 1;

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    generate_code(&ast_tree, &index);

    println!("  pop rax");
    println!("  ret");

}

fn generate_code(ast_tree : &Vec<parser::Node>, index : &usize) -> () {
    if ast_tree.len() <= *index { return (); }

    let node = &ast_tree[*index];
    // println!("{:?}", node);

    generate_code(ast_tree, &node.left_index);
    generate_code(ast_tree, &node.right_index);

    match node.kind {
        parser::NodeKind::NDAdd => {
            println!("  pop rdi\n  pop rax");
            println!("  add rax, rdi");
            println!("  push rax");
        }
        parser::NodeKind::NDSub => {
            println!("  pop rdi\n  pop rax");
            println!("  sub rax, rdi");
            println!("  push rax");
        }
        parser::NodeKind::NDMul => {
            println!("  pop rdi\n  pop rax");
            println!("  imul rax, rdi");
            println!("  push rax")
        }
        parser::NodeKind::NDDiv => {
            println!("  pop rdi\n  pop rax");
            println!("  cqo");
            println!("  idiv rdi");
            println!("  push rax")
        },
        parser::NodeKind::NDNum(val) => println!("  push {}", val),
    }
}