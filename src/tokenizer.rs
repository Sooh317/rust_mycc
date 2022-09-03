

#[derive(PartialEq)]
#[derive(Debug)]
pub enum TokenKind {
    TKReserved, 
    TKNum(i32), 
    TKEof, 
}

#[derive(Debug)]
pub struct Token {
    pub index : usize,
    pub next_index : usize, 
    pub kind : TokenKind,
}

impl Token {
    pub fn new(kind : TokenKind, index : usize, next_index : usize) -> Token{
        Token {
            kind, 
            index, 
            next_index,
        }
    }

    pub fn consume(s : &str, token : &Token, index : &mut usize, op : char) -> bool {
        // println!("kind -> {:?}, s[index] -> {}", token.kind, s.as_bytes()[token.index]);
        if token.kind != TokenKind::TKReserved || s.as_bytes()[token.index] != op as u8 {
            return false;
        }
        *index += 1;
        true
    }

    pub fn expect(s : &str, token : &Token, index : &mut usize, op : char) -> () {
        *index += 1;
        if token.kind != TokenKind::TKReserved || s.as_bytes()[token.index] != op as u8 {
            eprintln!("{}ではありません", op);
            std::process::exit(1);
        }
    }

    pub fn expect_number(token : &Token, index : &mut usize) -> i32 {
        *index += 1;
        match token.kind {
            TokenKind::TKNum(val) => val,
            _ => {
                eprintln!("数ではありません");
                std::process::exit(1);
            }
        }
    }

    pub fn at_eof(token : &Token) -> bool { token.kind == TokenKind::TKEof }

    pub fn tokenize(s : &str) -> Vec<Token> {
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


pub fn strtol(s : &str) -> &str {
    for (i, c) in s.chars().enumerate() {
        if !char::is_digit(c, 10) {
            return &s[..i]
        }
    }
    &s
}