use std::{env, process};
mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません\n");
        process::exit(1);
    }

    let expression = &args[1];
    let tokens = tokenizer::Token::tokenize(expression);
    let mut index = 0;

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", tokenizer::Token::expect_number(&expression, &tokens[index], &mut index));

    while !tokenizer::Token::at_eof(&tokens[index]) {

        if tokenizer::Token::consume(&expression, &tokens[index], &mut index, '+') {
            println!("  add rax, {}", tokenizer::Token::expect_number(&expression, &tokens[index], &mut index)); 
            continue;   
        }

        tokenizer::Token::expect(&expression, &tokens[index], &mut index, '-');
        println!("  sub rax, {}", tokenizer::Token::expect_number(&expression, &tokens[index], &mut index));
    }
    
    println!("  ret");

}
