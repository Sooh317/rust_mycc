use crate::tokenizer::{Token, TokenKind};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
pub enum NodeKind<'a> {
    NDAdd, 
    NDSub, 
    NDMul, 
    NDDiv, 
    NDLe,
    NDLeEq,
    NDEq,
    NDAs, // assign
    NDLVa(&'a str, i32), // local variable(offset from RBP)
    NDNEq,
    NDRet,
    NDIf, 
    NDWh, 
    NDNum(i32),
}

#[derive(Debug)]
pub struct Node<'a> {
    pub kind : NodeKind<'a>,
    pub left_index : usize,
    pub right_index : usize,
    pub cond_index : usize,
}

impl<'a> Node<'a> {
    fn new(kind: NodeKind<'a>, left_index: usize, right_index: usize, cond_index : usize) -> Node<'a> {
        Node {
            kind, 
            left_index, 
            right_index,
            cond_index,
        }
    }
    fn new_num(val : i32) -> Node<'a> {
        Node::new(NodeKind::NDNum(val), std::usize::MAX, std::usize::MAX, std::usize::MAX)
    }
    fn new_lvar(name : &'a str) -> Node<'a> {
        Node::new(NodeKind::NDLVa(name, -1), std::usize::MAX, std::usize::MAX, std::usize::MAX)
    }
    fn new_ret(left_index : usize) -> Node<'a> {
        Node::new(NodeKind::NDRet, left_index, std::usize::MAX, std::usize::MAX)
    }

    fn program(s : &str, tokens : &'a Vec<Token>, index : &mut usize) -> Vec<Vec<Node<'a>>> {
        let mut code : Vec<Vec<Node>> = Vec::new();
        while !Token::at_eof(&tokens[*index]) {
            let mut tree : Vec<Node> = Vec::new();
            Node::stmt(s, tokens, index, &mut tree);
            code.push(tree)
        }
        code
    }

    fn stmt(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>) -> usize {
        let token = &tokens[*index];
        if Token::consume(s, token, index, "return") {
            let left_index = Node::expr(s, tokens, index, tree);
            tree.push(Node::new_ret(left_index));
            Token::expect(s, &tokens[*index], index, ";");
        }
        else if Token::consume(s, token, index, "if") {
            Token::expect(s, &tokens[*index], index, "(");
            let cond_index = Node::expr(s, tokens, index, tree);
            Token::expect(s, &tokens[*index], index, ")");
            let left_index = Node::stmt(s, tokens, index, tree);
            let mut right_index = std::usize::MAX;
            if Token::consume(s, &tokens[*index], index, "else") {
                right_index = Node::stmt(s, tokens, index, tree);
            }
            tree.push(Node::new(NodeKind::NDIf, left_index, right_index, cond_index));
        }
        else if Token::consume(s, token, index, "while") {
            Token::expect(s, &tokens[*index], index, "(");
            let cond_index = Node::expr(s, tokens, index, tree);
            Token::expect(s, &tokens[*index], index, ")");
            let left_index = Node::stmt(s, tokens, index, tree);
            let right_index = std::usize::MAX;
            tree.push(Node::new(NodeKind::NDWh, left_index, right_index, cond_index));
        }
        else {
            Node::expr(s, tokens, index, tree);
            Token::expect(s, &tokens[*index], index, ";");
        }
        tree.len() - 1
    }

    fn expr(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>) -> usize {
        Node::assign(s, tokens, index, tree)
    }

    fn assign(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>) -> usize {
        let left_index = Node::equality(s, tokens, index, tree);
        let token = &tokens[*index];
        if Token::consume(s, token, index, "=") {
            let right_index = Node::assign(s, tokens, index, tree);
            tree.push(Node::new(NodeKind::NDAs, left_index, right_index, std::usize::MAX));
        }
        tree.len() - 1
    }

    fn equality(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>) -> usize {
        let mut left_index = Node::relational(s, tokens, index, tree);
        loop {
            if Token::consume(s, &tokens[*index], index, "==") {
                let right_index = Node::relational(s, tokens, index, tree);
                tree.push(Node::new(NodeKind::NDEq, left_index, right_index, std::usize::MAX));
            }   
            else if Token::consume(s, &tokens[*index], index, "!=") {
                let right_index = Node::relational(s, tokens, index, tree);
                tree.push(Node::new(NodeKind::NDNEq, left_index, right_index, std::usize::MAX));
            }   
            else {
                return tree.len() - 1;
            }
            left_index = tree.len() - 1;
        }
    }

    fn relational(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>) -> usize {
        let mut left_index = Node::add(s, tokens, index, tree);
        loop {
            if Token::consume(s, &tokens[*index], index, "<=") {
                let right_index = Node::add(s, tokens, index, tree);
                tree.push(Node::new(NodeKind::NDLeEq, left_index, right_index, std::usize::MAX));
            }   
            else if Token::consume(s, &tokens[*index], index, "<") {
                let right_index = Node::add(s, tokens, index, tree);
                tree.push(Node::new(NodeKind::NDLe, left_index, right_index, std::usize::MAX));
            }   
            else if Token::consume(s, &tokens[*index], index, ">=") {
                let right_index = Node::add(s, tokens, index, tree);
                tree.push(Node::new(NodeKind::NDLeEq, right_index, left_index, std::usize::MAX));
            }   
            else if Token::consume(s, &tokens[*index], index, ">") {
                let right_index = Node::add(s, tokens, index, tree);
                tree.push(Node::new(NodeKind::NDLe, right_index, left_index, std::usize::MAX));
            }   
            else {
                return tree.len() - 1;
            }
            left_index = tree.len() - 1;
        }
    }

    fn add(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>) -> usize {
        let mut left_index = Node::mul(s, tokens, index, tree);
        loop {
            if Token::consume(s, &tokens[*index], index, "+") {
                let right_index = Node::mul(s, tokens, index, tree);
                tree.push(Node::new(NodeKind::NDAdd, left_index, right_index, std::usize::MAX));
            }   
            else if Token::consume(s, &tokens[*index], index, "-") {
                let right_index = Node::mul(s, tokens, index, tree);
                tree.push(Node::new(NodeKind::NDSub, left_index, right_index, std::usize::MAX));
            }   
            else {
                return tree.len() - 1;
            }
            left_index = tree.len() - 1;
        }
    }

    fn mul(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>) -> usize {
        let mut left_index = Node::unary(s, tokens, index, tree);
        loop {
            if Token::consume(s, &tokens[*index], index, "*") {
                let right_index = Node::unary(s, tokens, index, tree);
                tree.push(Node::new(NodeKind::NDMul, left_index, right_index, std::usize::MAX));
            }   
            else if Token::consume(s, &tokens[*index], index, "/") {
                let right_index = Node::unary(s, tokens, index, tree);
                tree.push(Node::new(NodeKind::NDDiv, left_index, right_index, std::usize::MAX));
            }   
            else {
                return tree.len() - 1;
            }
            left_index = tree.len() - 1;
        }
    }

    fn unary(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>) -> usize {
        let token = &tokens[*index];
        // -x = 0 - x
        if Token::consume(s, token, index, "-") {
            let lnode = Node::new_num(0);
            let left_index = tree.len();
            tree.push(lnode);
            let right_index = Node::primary(s, tokens, index, tree);
            tree.push(Node::new(NodeKind::NDSub, left_index, right_index, std::usize::MAX));
            tree.len() - 1
        }
        else {
            Token::consume(s, token, index, "+");
            Node::primary(s, tokens, index, tree)
        }
    }

    fn primary(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>) -> usize {
        let token = &tokens[*index];
        if Token::consume(s, token, index, "(") {
            let id = Node::expr(s, tokens, index, tree);

            let token = &tokens[*index];
            Token::expect(s, token, index, ")");
            
            return id;
        }
        else{
            match token.kind {
                TokenKind::TKIdent(lvar_name) => {
                    *index += 1;
                    tree.push(Node::new_lvar(lvar_name));
                }
                _ => {
                    tree.push(Node::new_num(Token::expect_number(s, token, index)));
                }
            }
        }
        tree.len() - 1
    }

    pub fn parse(s : &str, tokens : &'a Vec<Token>) -> Vec<Vec<Node<'a>>> {
        let mut index = 0;
        let mut tree = Node::program(s, tokens, &mut index);
        Node::assign_offset(&mut tree);
        tree
    }

    fn assign_offset(trees : &mut Vec<Vec<Node>>) {
        let mut map = HashMap::new();
        for tree in trees {
            for node in tree {
                match node.kind {
                    NodeKind::NDLVa(s, _) => {
                        let leng = map.len() as i32;
                        map.entry(s).or_insert(8 * (leng + 1) as i32);
                        node.kind = NodeKind::NDLVa(s, *map.get(s).unwrap());
                    }
                    _ => continue,
                }
            }
        }
    }
}