

#[derive(PartialEq)]
#[derive(Debug)]
pub enum TokenKind<'a> {
    TKReserved(&'a str), 
    TKNum(i32), 
    TKEof, 
}

#[derive(Debug)]
pub struct Token<'a>{
    pub index : usize,
    pub next_index : usize, 
    pub kind : TokenKind<'a>,
}

impl<'a> Token<'a>{
    pub fn new(kind : TokenKind, index : usize, next_index : usize) -> Token{
        Token {
            kind, 
            index, 
            next_index,
        }
    }

    pub fn consume(_s : &str, token : &Token, index : &mut usize, op : &str) -> bool {
        match token.kind {
            TokenKind::TKReserved(sig) => {
                if sig == op {
                    *index += 1;
                    return true
                }
                else {
                    false
                }
            }
            _ => false
        }
    }

    pub fn expect(s : &str, token : &Token, index : &mut usize, op : &str) -> () {
        match token.kind {
            TokenKind::TKReserved(sig) => {
                if sig == op {
                    *index += 1;
                    return ();
                }
            }
            _ => ()
        }
        eprintln!("{}", s);
        eprintln!("{}^{}ではありません", " ".repeat(token.index), op);
        std::process::exit(1);
    }

    pub fn expect_number(s : &str, token : &Token, index : &mut usize) -> i32 {
        *index += 1;
        match token.kind {
            TokenKind::TKNum(val) => val,
            _ => {
                eprintln!("{}", s);
                eprintln!("{}^数ではありません", " ".repeat(token.index));
                std::process::exit(1);
            }
        }
    }

    pub fn at_eof(token : &Token) -> bool { token.kind == TokenKind::TKEof }

    pub fn tokenize(s : &'a str) -> Vec<Token<'a>> {
        let mut sequence : Vec<Token<'a>> = Vec::new();
        let mut next = 0;
        for (i, c) in s.chars().enumerate() {
            if next > i || char::is_whitespace(c) {
                continue;
            }
            if c == '+' || c == '-'{
                sequence.push(Token::new(TokenKind::TKReserved(&s[i..i+1]), i, i + 1));
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