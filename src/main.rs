use std::{env, process, char};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません\n");
        process::exit(1);
    }

    let mut index = 0;
    let expression = &args[1];

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let result = strtol(&expression);
    index += result.len();
    println!("  mov rax, {}", result);

    while index < expression.len() {
        if expression.as_bytes()[index] == '+' as u8 {
            index += 1;
            let result = strtol(&expression[index..]);
            index += result.len();
            println!("  add rax, {}", result);
            continue;
        }
        if expression.as_bytes()[index] == '-' as u8 {
            index += 1;
            let result = strtol(&expression[index..]);
            index += result.len();
            println!("  sub rax, {}", result);
            continue;
        }

        eprintln!("予期しない文字です: {}", expression.as_bytes()[index]);
        process::exit(1);

    }

    println!("  ret");
}

fn strtol(s : &str) -> &str {
    for (i, c) in s.chars().enumerate() {
        if !char::is_digit(c, 10) {
            return &s[..i]
        }
    }
    &s
}