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
    NDLVa(&'a str), // local variable(offset from RBP)
    NDNEq,
    NDRet,
    NDIf, 
    NDWh, 
    NDFor,
    NDBlock, // code block
    NDFnCall(&'a str),
    NDFnDef(&'a str, Vec<&'a str>), // (func name, argument lists, argument offsets, necessary memory for local variables)
    NDAddr, 
    NDDeref,
    NDNum(i32),
}

#[derive(Debug)]
pub struct Node<'a> {
    pub kind : NodeKind<'a>,
    pub indices : Vec<usize>
}

#[derive(Debug)]
pub struct Ast<'a> {
    pub tree : Vec<Node<'a>>,
    pub map : HashMap<&'a str, VarInfo>,
    pub region : i32
}

#[derive(Debug)]
pub enum Type {
    Int, 
    // Ptr(Box<Type>),
    Init,
}

#[derive(Debug)]
pub struct VarInfo {
    pub ty : Type,
    pub offset : i32
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
        Node::new(NodeKind::NDLVa(name), Vec::new())
    }
    fn new_ret(left_index : usize) -> Node<'a> {
        let vec = vec![left_index];
        Node::new(NodeKind::NDRet, vec)
    }

    fn find_type(s : &str, tokens : &'a Vec<Token>, index : &mut usize) -> Type {
        let mut func_type = Type::Init;
        let token = &tokens[*index];
        if Token::consume(s, token, index, "int") {
            func_type = Type::Int;
        }
        else {
            Token::error_msg(s, token.index, "正しい型を使用してください")
        }
        func_type
    }

    fn type_to_offset(ty : &Type) -> i32 {
        match ty {
            Type::Int => 8,
            _ => -1
        }
    }

    fn program(s : &str, tokens : &'a Vec<Token>, index : &mut usize) -> Vec<Ast<'a>> {
        let mut code : Vec<Ast<'a>> = Vec::new();
        while !Token::at_eof(&tokens[*index]) {
            let mut tree : Vec<Node> = Vec::new();
            let mut map : HashMap<&'a str, VarInfo> = HashMap::new();
            let mut region = 0;
            Node::definition(s, tokens, index, &mut tree, &mut map, &mut region);
            region = (region + 15) / 16 * 16;
            code.push(Ast { tree, map, region});
        }
        code
    }

    fn definition(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>, map : &mut HashMap<&'a str, VarInfo>, region : &mut i32) -> usize {
        let func_type = Node::find_type(s, tokens, index);        
        let token = &tokens[*index];
        match token.kind {
            TokenKind::TKIdent(func_name) => {
                *index += 1;
                Token::expect(s, &tokens[*index], index, "(");
                let mut arguments : Vec<&'a str> = Vec::new();
                while !Token::consume(s, &tokens[*index], index, ")") {
                    let arg_type = Node::find_type(s, tokens, index);

                    let token = &tokens[*index];
                    *index += 1;
                    match token.kind {
                        TokenKind::TKIdent(arg) => {
                            arguments.push(arg);
                            if map.get(arg).is_some() {
                                Token::error_msg(s, token.index, "同じ名前の引数が使われています");
                            }
                            *region += Node::type_to_offset(&arg_type);
                            map.insert(arg, VarInfo { ty: arg_type, offset: *region});
                        }
                        _ => Token::error_msg(s, token.index, "変数ではありません"),
                    }
                    Token::consume(s, &tokens[*index], index, ",");
                }
                map.insert(func_name, VarInfo { ty: func_type, offset: -1 });

                Token::expect(s, &tokens[*index], index, "{");
                let mut func_code : Vec<usize> = Vec::new();
                while !Token::consume(s, &tokens[*index], index, "}") {
                    func_code.push(Node::stmt(s, tokens, index, tree, map, region));
                }
                tree.push(Node::new(NodeKind::NDFnDef(func_name, arguments), func_code));
            }
            _ => {
                Token::error_msg(s, token.index, "関数定義ではありません");
            }
        }
        tree.len() - 1
    }


    fn stmt(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>, map : &mut HashMap<&'a str, VarInfo>, region : &mut i32) -> usize {
        let token = &tokens[*index];
        if Token::consume(s, token, index, "{") {
            let mut vec : Vec<usize> = Vec::new();
            while !Token::consume(s, &tokens[*index], index, "}") {
                let index = Node::stmt(s, tokens, index, tree, map, region);
                vec.push(index);
            }
            tree.push(Node::new(NodeKind::NDBlock, vec));
        }
        else if Token::consume(s, token, index, "return") {
            let left_index = Node::expr(s, tokens, index, tree, map, region);
            tree.push(Node::new_ret(left_index));
            Token::expect(s, &tokens[*index], index, ";");
        }
        else if Token::consume(s, token, index, "if") {
            Token::expect(s, &tokens[*index], index, "(");
            let cond_index = Node::expr(s, tokens, index, tree, map, region);
            Token::expect(s, &tokens[*index], index, ")");
            let stmt_if = Node::stmt(s, tokens, index, tree, map, region);
            let mut stmt_else = std::usize::MAX;
            if Token::consume(s, &tokens[*index], index, "else") {
                stmt_else = Node::stmt(s, tokens, index, tree, map, region);
            }
            let vec = vec![cond_index, stmt_if, stmt_else];
            tree.push(Node::new(NodeKind::NDIf, vec));
        }
        else if Token::consume(s, token, index, "while") {
            Token::expect(s, &tokens[*index], index, "(");
            let cond_index = Node::expr(s, tokens, index, tree, map, region);
            Token::expect(s, &tokens[*index], index, ")");
            let stmt_wh = Node::stmt(s, tokens, index, tree, map, region);
            let vec = vec![cond_index, stmt_wh];
            tree.push(Node::new(NodeKind::NDWh, vec));
        }
        else if Token::consume(s, token, index, "for") {
            let mut decl_index = std::usize::MAX;
            let mut manip_index = std::usize::MAX;
            let mut cond_index = std::usize::MAX;

            Token::expect(s, &tokens[*index], index, "(");
            if !Token::consume(s, &tokens[*index], index, ";") {
                decl_index = Node::expr(s, tokens, index, tree, map, region);
                Token::expect(s, &tokens[*index], index, ";");
            }

            if !Token::consume(s, &tokens[*index], index, ";") {
                cond_index = Node::expr(s, tokens, index, tree, map, region);
                Token::expect(s, &tokens[*index], index, ";");
            }

            if !Token::consume(s, &tokens[*index], index, ")") {
                manip_index = Node::expr(s, tokens, index, tree, map, region);
                Token::expect(s, &tokens[*index], index, ")");
            }

            let stmt_index = Node::stmt(s, tokens, index, tree, map, region);
            let vec = vec![decl_index, cond_index, manip_index, stmt_index];
            let node = Node::new(NodeKind::NDFor, vec);
            tree.push(node);
        }   
        else {
            Node::expr(s, tokens, index, tree, map, region);
            Token::expect(s, &tokens[*index], index, ";");
        }
        tree.len() - 1
    }

    fn expr(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>, map : &mut HashMap<&'a str, VarInfo>, region : &mut i32) -> usize {
        Node::assign(s, tokens, index, tree, map, region)
    }

    fn assign(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>, map : &mut HashMap<&'a str, VarInfo>, region : &mut i32) -> usize {
        let left_index = Node::equality(s, tokens, index, tree, map, region);
        let token = &tokens[*index];
        if Token::consume(s, token, index, "=") {
            let val_index = Node::assign(s, tokens, index, tree, map, region);
            let vec = vec![left_index, val_index];
            tree.push(Node::new(NodeKind::NDAs, vec));
        }
        tree.len() - 1
    }

    fn equality(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>, map : &mut HashMap<&'a str, VarInfo>, region : &mut i32) -> usize {
        let mut lval_index = Node::relational(s, tokens, index, tree, map, region);
        loop {
            if Token::consume(s, &tokens[*index], index, "==") {
                let rval_index = Node::relational(s, tokens, index, tree, map, region);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new(NodeKind::NDEq, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, "!=") {
                let rval_index = Node::relational(s, tokens, index, tree, map, region);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new(NodeKind::NDNEq, vec));
            }   
            else {
                return tree.len() - 1;
            }
            lval_index = tree.len() - 1;
        }
    }

    fn relational(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>, map : &mut HashMap<&'a str, VarInfo>, region : &mut i32) -> usize {
        let mut lval_index = Node::add(s, tokens, index, tree, map, region);
        loop {
            if Token::consume(s, &tokens[*index], index, "<=") {
                let rval_index = Node::add(s, tokens, index, tree, map, region);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new(NodeKind::NDLeEq, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, "<") {
                let rval_index = Node::add(s, tokens, index, tree, map, region);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new(NodeKind::NDLe, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, ">=") {
                let rval_index = Node::add(s, tokens, index, tree, map, region);
                let vec = vec![rval_index, lval_index];
                tree.push(Node::new(NodeKind::NDLeEq, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, ">") {
                let rval_index = Node::add(s, tokens, index, tree, map, region);
                let vec = vec![rval_index, lval_index];
                tree.push(Node::new(NodeKind::NDLe, vec));
            }   
            else {
                return tree.len() - 1;
            }
            lval_index = tree.len() - 1;
        }
    }

    fn add(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>, map : &mut HashMap<&'a str, VarInfo>, region : &mut i32) -> usize {
        let mut lval_index = Node::mul(s, tokens, index, tree, map, region);
        loop {
            if Token::consume(s, &tokens[*index], index, "+") {
                let rval_index = Node::mul(s, tokens, index, tree, map, region);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new(NodeKind::NDAdd, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, "-") {
                let rval_index = Node::mul(s, tokens, index, tree, map, region);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new(NodeKind::NDSub, vec));
            }   
            else {
                return tree.len() - 1;
            }
            lval_index = tree.len() - 1;
        }
    }

    fn mul(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>, map : &mut HashMap<&'a str, VarInfo>, region : &mut i32) -> usize {
        let mut lval_index = Node::unary(s, tokens, index, tree, map, region);
        loop {
            if Token::consume(s, &tokens[*index], index, "*") {
                let rval_index = Node::unary(s, tokens, index, tree, map, region);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new(NodeKind::NDMul, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, "/") {
                let rval_index = Node::unary(s, tokens, index, tree, map, region);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new(NodeKind::NDDiv, vec));
            }   
            else {
                return tree.len() - 1;
            }
            lval_index = tree.len() - 1;
        }
    }

    fn unary(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>, map : &mut HashMap<&'a str, VarInfo>, region : &mut i32) -> usize {
        let token = &tokens[*index];
        if Token::consume(s, token, index, "*") {
            let addr_index = vec![Node::unary(s, tokens, index, tree, map, region)];
            tree.push(Node::new(NodeKind::NDDeref, addr_index));
        }
        else if Token::consume(s, token, index, "&") {
            let var_index = vec![Node::unary(s, tokens, index, tree, map, region)];
            tree.push(Node::new(NodeKind::NDAddr, var_index));
        }
        // -x = 0 - x
        else if Token::consume(s, token, index, "-") {
            let lnode = Node::new_num(0);
            let lval_index = tree.len();
            tree.push(lnode);
            let rval_index = Node::primary(s, tokens, index, tree, map, region);
            let vec = vec![lval_index, rval_index];
            tree.push(Node::new(NodeKind::NDSub, vec));
        }
        else if Token::consume(s, token, index, "+") {
            Node::primary(s, tokens, index, tree, map, region);
        }
        else {
            Node::primary(s, tokens, index, tree, map, region);
        }

        tree.len() - 1
    }

    fn primary(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>, map : &mut HashMap<&'a str, VarInfo>, region : &mut i32) -> usize {
        let token = &tokens[*index];
        if Token::consume(s, token, index, "(") {
            let id = Node::expr(s, tokens, index, tree, map, region);
            let token = &tokens[*index];
            Token::expect(s, token, index, ")");
            
            return id;
        }
        else if Token::consume(s, token, index, "int") {
            let ty = Type::Int;
            *region += 8;
            let token = &tokens[*index];
            match token.kind {
                TokenKind::TKIdent(lvar_name) => {
                    if map.get(lvar_name).is_some() {
                        Token::error_msg(s, token.index, "既に宣言された変数です");    
                    }

                    tree.push(Node::new_lvar(lvar_name));
                    map.insert(lvar_name, VarInfo { ty, offset: *region});
                    *index += 1;
                }
                _ => {
                    Token::error_msg(s, token.index, "変数ではありません");
                }
            }
        }
        else{
            match token.kind {
                TokenKind::TKIdent(lvar_name) => {
                    *index += 1;
                    if Token::consume(s, &tokens[*index], index, "(") {
                        let mut vec : Vec<usize> = Vec::new();
                        while !Token::consume(s, &tokens[*index], index, ")") {
                            vec.push(Node::expr(s, tokens, index, tree, map, region));
                            Token::consume(s, &tokens[*index], index, ",");
                        }
                        tree.push(Node::new(NodeKind::NDFnCall(lvar_name), vec));
                    }
                    else{
                        if map.get(lvar_name).is_none() {
                            Token::error_msg(s, token.index, "宣言されていない変数です");
                        }
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

    pub fn parse(s : &str, tokens : &'a Vec<Token>) -> Vec<Ast<'a>> {
        let mut index = 0;
        let functions = Node::program(s, tokens, &mut index);
        functions
    }
}