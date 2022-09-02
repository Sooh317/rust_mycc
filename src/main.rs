use std::{env, process, char};

#[derive(PartialEq)]
#[derive(Debug)]
enum TokenKind {
    TKReserved, 
    TKNum(i32), 
    TKEof, 
}

#[derive(Debug)]
struct Token {
    index : usize,
    next_index : usize, 
    kind : TokenKind,
}

impl Token {
    fn new(kind : TokenKind, index : usize, next_index : usize) -> Token{
        Token {
            kind, 
            index, 
            next_index,
        }
    }

    fn consume(s : &str, token : &Token, index : &mut usize, op : char) -> bool {
        // println!("kind -> {:?}, s[index] -> {}", token.kind, s.as_bytes()[token.index]);
        if token.kind != TokenKind::TKReserved || s.as_bytes()[token.index] != op as u8 {
            return false;
        }
        *index += 1;
        true
    }
    
    fn expect(s : &str, token : &Token, index : &mut usize, op : char) -> () {
        *index += 1;
        if token.kind != TokenKind::TKReserved || s.as_bytes()[token.index] != op as u8 {
            eprintln!("{}ではありません", op);
            process::exit(1);
        }
    }
    
    fn expect_number(token : &Token, index : &mut usize) -> i32 {
        *index += 1;
        match token.kind {
            TokenKind::TKNum(val) => val,
            _ => {
                eprintln!("数ではありません");
                process::exit(1);
            }
        }
    }
    
    fn at_eof(token : &Token) -> bool { token.kind == TokenKind::TKEof }

    fn tokenize(s : &str) -> Vec<Token> {
        let mut sequence : Vec<Token> = Vec::new();
        let mut next = 0;
        for (i, c) in s.chars().enumerate() {
            if next > i || char::is_whitespace(c) {
                continue;
            }
            if c == '+' || c == '-' {
                sequence.push(Token::new(TokenKind::TKReserved, i, i + 1));
                continue;
            }
            if char::is_digit(c, 10) {
                let result = strtol(&s[i..]);
                next = i + result.len();
                sequence.push(Token::new(TokenKind::TKNum(result.parse().unwrap()), i, next));
                continue;
            }
        }
        sequence.push(Token::new(TokenKind::TKEof, s.len(), s.len()));
        sequence
    }

}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません\n");
        process::exit(1);
    }

    let expression = &args[1];
    let tokens = Token::tokenize(expression);
    let mut index = 0;

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", Token::expect_number(&tokens[index], &mut index));

    while !Token::at_eof(&tokens[index]) {

        if Token::consume(&expression, &tokens[index], &mut index, '+') {
            println!("  add rax, {}", Token::expect_number(&tokens[index], &mut index)); 
            continue;   
        }

        Token::expect(&expression, &tokens[index], &mut index, '-');
        println!("  sub rax, {}", Token::expect_number(&tokens[index], &mut index));
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