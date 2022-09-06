use crate::tokenizer::{Token, TokenKind};
use std::{collections::HashMap, vec};

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
    NDFor,
    NDBlock, // code block
    NDFnCall(&'a str),
    NDFnDef(&'a str, Vec<&'a str>, Vec<i32>, i32), // (func name, argument lists, argument offsets, necessary memory for local variables)
    NDNum(i32),
}

#[derive(Debug)]
pub struct Node<'a> {
    pub kind : NodeKind<'a>,
    pub indices : Vec<usize>
}

impl<'a> Node<'a> {
    fn new(kind: NodeKind<'a>, indices : Vec<usize>) -> Node<'a> {
        Node {
            kind, 
            indices,
        }
    }
    fn new_num(val : i32) -> Node<'a> {
        Node::new(NodeKind::NDNum(val), Vec::new())
    }
    fn new_lvar(name : &'a str) -> Node<'a> {
        Node::new(NodeKind::NDLVa(name, -1), Vec::new())
    }
    fn new_ret(left_index : usize) -> Node<'a> {
        let vec = vec![left_index];
        Node::new(NodeKind::NDRet, vec)
    }

    fn program(s : &str, tokens : &'a Vec<Token>, index : &mut usize) -> Vec<Vec<Node<'a>>> {
        let mut code : Vec<Vec<Node>> = Vec::new();
        while !Token::at_eof(&tokens[*index]) {
            let mut tree : Vec<Node> = Vec::new();
            Node::definition(s, tokens, index, &mut tree);
            code.push(tree)
        }
        code
    }

    fn definition(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>) -> usize {
        let token = &tokens[*index];
        match token.kind {
            TokenKind::TKIdent(func_name) => {
                *index += 1;
                Token::expect(s, &tokens[*index], index, "(");
                let mut arguments : Vec<&'a str> = Vec::new();
                while !Token::consume(s, &tokens[*index], index, ")") {
                    let token = &tokens[*index];
                    *index += 1;
                    match token.kind {
                        TokenKind::TKIdent(arg) =>  arguments.push(arg),
                        _ => Token::error_msg(s, token.index, "変数ではありません"),
                    }
                    Token::consume(s, &tokens[*index], index, ",");
                }
                Token::expect(s, &tokens[*index], index, "{");
                let mut func_code : Vec<usize> = Vec::new();
                while !Token::consume(s, &tokens[*index], index, "}") {
                    func_code.push(Node::stmt(s, tokens, index, tree));
                }
                tree.push(Node::new(NodeKind::NDFnDef(func_name, arguments, Vec::new(), -1), func_code));
            }
            _ => {
                Token::error_msg(s, token.index, "関数定義ではありません");
            }
        }
        tree.len() - 1
    }


    fn stmt(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>) -> usize {
        let token = &tokens[*index];
        if Token::consume(s, token, index, "{") {
            let mut vec : Vec<usize> = Vec::new();
            while !Token::consume(s, &tokens[*index], index, "}") {
                let index = Node::stmt(s, tokens, index, tree);
                vec.push(index);
            }
            tree.push(Node::new(NodeKind::NDBlock, vec));
        }
        else if Token::consume(s, token, index, "return") {
            let left_index = Node::expr(s, tokens, index, tree);
            tree.push(Node::new_ret(left_index));
            Token::expect(s, &tokens[*index], index, ";");
        }
        else if Token::consume(s, token, index, "if") {
            Token::expect(s, &tokens[*index], index, "(");
            let cond_index = Node::expr(s, tokens, index, tree);
            Token::expect(s, &tokens[*index], index, ")");
            let stmt_if = Node::stmt(s, tokens, index, tree);
            let mut stmt_else = std::usize::MAX;
            if Token::consume(s, &tokens[*index], index, "else") {
                stmt_else = Node::stmt(s, tokens, index, tree);
            }
            let vec = vec![cond_index, stmt_if, stmt_else];
            tree.push(Node::new(NodeKind::NDIf, vec));
        }
        else if Token::consume(s, token, index, "while") {
            Token::expect(s, &tokens[*index], index, "(");
            let cond_index = Node::expr(s, tokens, index, tree);
            Token::expect(s, &tokens[*index], index, ")");
            let stmt_wh = Node::stmt(s, tokens, index, tree);
            let vec = vec![cond_index, stmt_wh];
            tree.push(Node::new(NodeKind::NDWh, vec));
        }
        else if Token::consume(s, token, index, "for") {
            let mut decl_index = std::usize::MAX;
            let mut manip_index = std::usize::MAX;
            let mut cond_index = std::usize::MAX;

            Token::expect(s, &tokens[*index], index, "(");
            if !Token::consume(s, &tokens[*index], index, ";") {
                decl_index = Node::expr(s, tokens, index, tree);
                Token::expect(s, &tokens[*index], index, ";");
            }

            if !Token::consume(s, &tokens[*index], index, ";") {
                cond_index = Node::expr(s, tokens, index, tree);
                Token::expect(s, &tokens[*index], index, ";");
            }

            if !Token::consume(s, &tokens[*index], index, ")") {
                manip_index = Node::expr(s, tokens, index, tree);
                Token::expect(s, &tokens[*index], index, ")");
            }

            let stmt_index = Node::stmt(s, tokens, index, tree);
            let vec = vec![decl_index, cond_index, manip_index, stmt_index];
            let node = Node::new(NodeKind::NDFor, vec);
            tree.push(node);
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
            let val_index = Node::assign(s, tokens, index, tree);
            let vec = vec![left_index, val_index];
            tree.push(Node::new(NodeKind::NDAs, vec));
        }
        tree.len() - 1
    }

    fn equality(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>) -> usize {
        let mut lval_index = Node::relational(s, tokens, index, tree);
        loop {
            if Token::consume(s, &tokens[*index], index, "==") {
                let rval_index = Node::relational(s, tokens, index, tree);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new(NodeKind::NDEq, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, "!=") {
                let rval_index = Node::relational(s, tokens, index, tree);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new(NodeKind::NDNEq, vec));
            }   
            else {
                return tree.len() - 1;
            }
            lval_index = tree.len() - 1;
        }
    }

    fn relational(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>) -> usize {
        let mut lval_index = Node::add(s, tokens, index, tree);
        loop {
            if Token::consume(s, &tokens[*index], index, "<=") {
                let rval_index = Node::add(s, tokens, index, tree);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new(NodeKind::NDLeEq, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, "<") {
                let rval_index = Node::add(s, tokens, index, tree);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new(NodeKind::NDLe, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, ">=") {
                let rval_index = Node::add(s, tokens, index, tree);
                let vec = vec![rval_index, lval_index];
                tree.push(Node::new(NodeKind::NDLeEq, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, ">") {
                let rval_index = Node::add(s, tokens, index, tree);
                let vec = vec![rval_index, lval_index];
                tree.push(Node::new(NodeKind::NDLe, vec));
            }   
            else {
                return tree.len() - 1;
            }
            lval_index = tree.len() - 1;
        }
    }

    fn add(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>) -> usize {
        let mut lval_index = Node::mul(s, tokens, index, tree);
        loop {
            if Token::consume(s, &tokens[*index], index, "+") {
                let rval_index = Node::mul(s, tokens, index, tree);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new(NodeKind::NDAdd, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, "-") {
                let rval_index = Node::mul(s, tokens, index, tree);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new(NodeKind::NDSub, vec));
            }   
            else {
                return tree.len() - 1;
            }
            lval_index = tree.len() - 1;
        }
    }

    fn mul(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>) -> usize {
        let mut lval_index = Node::unary(s, tokens, index, tree);
        loop {
            if Token::consume(s, &tokens[*index], index, "*") {
                let rval_index = Node::unary(s, tokens, index, tree);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new(NodeKind::NDMul, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, "/") {
                let rval_index = Node::unary(s, tokens, index, tree);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new(NodeKind::NDDiv, vec));
            }   
            else {
                return tree.len() - 1;
            }
            lval_index = tree.len() - 1;
        }
    }

    fn unary(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>) -> usize {
        let token = &tokens[*index];
        // -x = 0 - x
        if Token::consume(s, token, index, "-") {
            let lnode = Node::new_num(0);
            let lval_index = tree.len();
            tree.push(lnode);
            let rval_index = Node::primary(s, tokens, index, tree);
            let vec = vec![lval_index, rval_index];
            tree.push(Node::new(NodeKind::NDSub, vec));
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
                    if Token::consume(s, &tokens[*index], index, "(") {
                        let mut vec : Vec<usize> = Vec::new();
                        while !Token::consume(s, &tokens[*index], index, ")") {
                            vec.push(Node::expr(s, tokens, index, tree));
                            Token::consume(s, &tokens[*index], index, ",");
                        }
                        tree.push(Node::new(NodeKind::NDFnCall(lvar_name), vec));
                    }
                    else {
                        tree.push(Node::new_lvar(lvar_name));
                    }
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
        let mut functions = Node::program(s, tokens, &mut index);

        Node::offset_calculation(&mut functions);
        // let mut maps = Node::process_argument_offset(&functions);
        // let length = functions.len();
        // for i in 0..length {
        //     Node::assign_offset(&mut functions[i], &mut maps[i]);
        // }
        functions
    }

    fn offset_calculation(functions : &mut Vec<Vec<Node>>) {
        let (mut maps, offsets) = Node::process_argument_offset(functions);
        let length = functions.len();
        for i in 0..length {
            let region = Node::assign_offset(&mut functions[i], &mut maps[i]);
            let last_index = functions[i].len() - 1;
            if let NodeKind::NDFnDef(name, _, _, _) = functions[i][last_index].kind {
                functions[i][last_index].kind = NodeKind::NDFnDef(name, Vec::new(), offsets[i].clone(), region);
            }
        }

    }

    fn process_argument_offset(functions : &Vec<Vec<Node<'a>>>) -> (Vec<HashMap<&'a str, i32>>, Vec<Vec<i32>>) {
        let mut vec_map : Vec<HashMap<&'a str, i32>> = Vec::new();
        let mut vec_offset : Vec<Vec<i32>> = Vec::new();
        for function in functions {
            let mut map = HashMap::new();
            let mut offset = Vec::new();
            let argument = &function.last().unwrap().kind;
            match &argument {
                NodeKind::NDFnDef(_, args, _, _) => {
                    for arg in args {
                        let leng = map.len() as i32;
                        map.entry(*arg).or_insert((leng + 1) * 8);
                        offset.push((leng + 1) * 8);
                    }
                }
                _ => std::process::exit(1),
            }
            vec_map.push(map);
            vec_offset.push(offset);
        }
        (vec_map, vec_offset)
    }

    fn assign_offset(functions : &mut Vec<Node<'a>>, map : &mut HashMap<&'a str, i32>) -> i32 {
        for node in functions {
            match node.kind {
                NodeKind::NDLVa(s, _) => {
                    let leng = map.len() as i32;
                    map.entry(s).or_insert((leng + 1) * 8);
                    node.kind = NodeKind::NDLVa(s, *map.get(s).unwrap());
                }
                _ => continue,
            }
        }
        let region = map.len() as i32 * 8;
        (region + 15) / 16 * 16
    }
}