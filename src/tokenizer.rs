#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub enum TokenKind<'a> {
    TKReserved(&'a str), 
    TKIdent(&'a str),
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
                    true
                }
                else {
                    false
                }
            }
            _ => false
        }
    }

    pub fn expect(s : &str, token : &Token, index : &mut usize, op : &str) {
        if let TokenKind::TKReserved(sig) = token.kind {
            if sig == op {
                *index += 1;
                return;
            }
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

    pub fn error_msg(s : &str, pos : usize, msg : &str) {
        eprintln!("{}", s);
        eprintln!("{}^{}", " ".repeat(pos), msg);
        std::process::exit(1);
    }

    pub fn at_eof(token : &Token) -> bool { token.kind == TokenKind::TKEof }

    pub fn tokenize(s : &'a str) -> Vec<Token<'a>> {
        let mut sequence : Vec<Token<'a>> = Vec::new();
        let mut next = 0;
        for (i, c) in s.chars().enumerate() {
            if next > i || char::is_whitespace(c) {
                continue;
            }
            else if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')' || c == '[' || c == ']' || c == ';' || c == '{' || c == '}' || c == ',' || c == '&' {
                sequence.push(Token::new(TokenKind::TKReserved(&s[i..i+1]), i, i + 1));
            }
            else if c == '>' || c == '<' || c == '=' || c == '!' {
                if i + 1 >= s.len() { Token::error_msg(s, i+1, "式になっていません"); }
                if &s[i+1..i+2] == "=" {
                    sequence.push(Token::new(TokenKind::TKReserved(&s[i..i+2]), i, i + 2));
                    next = i + 2;
                }
                else {
                    sequence.push(Token::new(TokenKind::TKReserved(&s[i..i+1]), i, i + 1));
                }
            }
            else if c.is_ascii_alphabetic() {
                for (j, d) in (&s[i+1..]).chars().enumerate() {
                    if d.is_ascii_alphabetic() || d.is_ascii_digit() || d == '_' { continue;}
                    next = i + 1 + j;
                    break;
                }
                if &s[i..next] == "sizeof" || &s[i..next] == "return" || &s[i..next] == "if" || &s[i..next] == "else" || &s[i..next] == "while" || &s[i..next] == "for" || &s[i..next] == "int" {
                    sequence.push(Token::new(TokenKind::TKReserved(&s[i..next]), i, next));
                }
                else {
                    sequence.push(Token::new(TokenKind::TKIdent(&s[i..next]), i, next));
                }
            }
            else if c.is_ascii_digit() {
                let result = strtol(&s[i..]);
                next = i + result.len();
                sequence.push(Token::new(TokenKind::TKNum(result.parse().unwrap()), i, next));
            }
        }
        sequence.push(Token::new(TokenKind::TKEof, s.len(), s.len()));
        sequence
    }

}


pub fn strtol(s : &str) -> &str {
    for (i, c) in s.chars().enumerate() {
        if !c.is_ascii_digit() {
            return &s[..i]
        }
    }
    s
}