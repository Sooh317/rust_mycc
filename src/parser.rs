use crate::tokenizer::{Token, TokenKind};
use std::{collections::HashMap, vec};
use crate::ty::Type;
use crate::ty;

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
    NDFnDef(&'a str, Vec<&'a str>), // (func name, argument lists)
    NDAddr, 
    NDDeref,
    NDNum(i32),
}

#[derive(Debug)]
pub struct Node<'a> {
    pub kind : NodeKind<'a>,
    pub indices : Vec<usize>,
    pub ty : Type,
}

#[derive(Debug)]
pub struct Ast<'a> {
    pub tree : Vec<Node<'a>>,
    pub map : HashMap<&'a str, VarInfo>,
    pub region : i32
}

#[derive(Debug)]
pub struct VarInfo {
    pub ty : Type,
    pub offset : i32
}


impl<'a> Node<'a> {
    fn new(kind: NodeKind<'a>, indices : Vec<usize>, ty : Type) -> Node<'a> {
        Node {
            kind, 
            indices,
            ty,
        }
    }
    fn new_init(kind: NodeKind<'a>, indices : Vec<usize>) -> Node<'a> {
        Node::new(kind, indices, Type::Init)
    }
    fn new_num(val : i32, ty : Type) -> Node<'a> {
        Node::new(NodeKind::NDNum(val), Vec::new(), ty)
    }
    fn new_lvar(name : &'a str, ty : Type) -> Node<'a> {
        Node::new(NodeKind::NDLVa(name), Vec::new(), ty)
    }
    fn new_ret(left_index : usize) -> Node<'a> {
        let vec = vec![left_index];
        Node::new_init(NodeKind::NDRet, vec)
    }
    fn new_add(tree : &mut Vec<Node<'a>>, lval_index : usize, rval_index : usize, vec : Vec<usize>) -> Node<'a> {
        ty::type_of_node(tree, lval_index);
        ty::type_of_node(tree, rval_index);

        match (tree[lval_index].ty.clone(), tree[rval_index].ty.clone()) {
            (Type::Int, Type::Int) => Node::new(NodeKind::NDAdd, vec, Type::Int),
            (Type::Ptr(_), Type::Ptr(_)) => {
                eprintln!("ポインタ同士を足しています");
                std::process::exit(1);
            }
            (Type::Ptr(ty1), Type::Int) => {
                tree.push(Node::new_num(8, Type::Int));
                tree.push(Node::new(NodeKind::NDMul, vec![rval_index, tree.len() - 1], Type::Int));
                Node::new(NodeKind::NDAdd, vec![lval_index, tree.len() - 1], Type::Ptr(ty1.clone()).clone())
            }
            (Type::Int, Type::Ptr(ty1)) => {
                tree.push(Node::new_num(8, Type::Int));
                tree.push(Node::new(NodeKind::NDMul, vec![lval_index, tree.len() - 1], Type::Int));
                Node::new(NodeKind::NDAdd, vec![rval_index, tree.len() - 1], Type::Ptr(ty1.clone()).clone())
            }
            (_, _) => {
                eprintln!("違法な足し算です");
                std::process::exit(1);
            }
        }

    }

    fn new_sub(tree : &mut Vec<Node<'a>>, lval_index : usize, rval_index : usize, vec : Vec<usize>) -> Node<'a> {
        ty::type_of_node(tree, lval_index);
        ty::type_of_node(tree, rval_index);

        match (tree[lval_index].ty.clone(), tree[rval_index].ty.clone()) {
            (Type::Int, Type::Int) => Node::new(NodeKind::NDSub, vec, Type::Int),
            (Type::Ptr(_), Type::Ptr(_)) => {
                tree.push(Node::new(NodeKind::NDSub, vec, Type::Int));
                let tmp = tree.len() - 1;
                tree.push(Node::new_num(8, Type::Int));
                Node::new(NodeKind::NDDiv, vec![tmp, tree.len() - 1], Type::Int)
            }
            (Type::Ptr(ty1), Type::Int) => {
                tree.push(Node::new_num(8, Type::Int));
                tree.push(Node::new(NodeKind::NDMul, vec![rval_index, tree.len() - 1], Type::Int));
                Node::new(NodeKind::NDSub, vec![lval_index, tree.len() - 1], Type::Ptr(ty1.clone()).clone())
            }
            (Type::Int, Type::Ptr(ty1)) => {
                tree.push(Node::new_num(8, Type::Int));
                tree.push(Node::new(NodeKind::NDMul, vec![lval_index, tree.len() - 1], Type::Int));
                Node::new(NodeKind::NDSub, vec![lval_index, tree.len() - 1], Type::Ptr(ty1.clone()).clone())
            }
            (_, _) => {
                eprintln!("違法な引き算です");
                std::process::exit(1);
            }
        }

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

    fn init_map(map : &mut HashMap<&'a str, VarInfo>, code : &Vec<Ast<'a>>) {
        for ast in code.iter() {
            let node = &ast.tree.last().unwrap();
            match node.kind {
                NodeKind::NDFnDef(func_name, _) => {
                    map.insert(func_name.clone(), VarInfo { ty: node.ty.clone(), offset: -1});
                }
                _ => (),
            }
        }
    }


    fn program(s : &str, tokens : &'a Vec<Token>, index : &mut usize) -> Vec<Ast<'a>> {
        let mut code : Vec<Ast<'a>> = Vec::new();
        while !Token::at_eof(&tokens[*index]) {
            let mut tree : Vec<Node> = Vec::new();
            let mut region = 0;
            let mut map : HashMap<&'a str, VarInfo> = HashMap::new();
            Node::init_map(&mut map, &code);
            Node::definition(s, tokens, index, &mut tree, &mut map, &mut region);
            region = (region + 15) / 16 * 16;
            code.push(Ast { tree, map, region});
        }
        code
    }

    fn definition(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>, map : &mut HashMap<&'a str, VarInfo>, region : &mut i32) {
        let func_type = Node::find_type(s, tokens, index);        
        let token = &tokens[*index];
        match token.kind { // 関数名と引数を記録
            TokenKind::TKIdent(func_name) => { //関数名
                *index += 1;
                Token::expect(s, &tokens[*index], index, "(");
                let mut arguments : Vec<&'a str> = Vec::new();
                while !Token::consume(s, &tokens[*index], index, ")") {
                    let arg_type = Node::find_type(s, tokens, index);

                    let token = &tokens[*index];
                    *index += 1;
                    match token.kind {
                        TokenKind::TKIdent(arg) => { // 引数名
                            arguments.push(arg);
                            if map.get(arg).is_some() {
                                Token::error_msg(s, token.index, "同じ名前の引数が使われています");
                            }
                            *region += ty::type_to_offset(&arg_type);
                            map.insert(arg, VarInfo { ty: arg_type, offset: *region});
                        }
                        _ => Token::error_msg(s, token.index, "変数ではありません"),
                    }
                    Token::consume(s, &tokens[*index], index, ",");
                }
                map.insert(func_name, VarInfo { ty: func_type.clone(), offset: -1 });
                Token::expect(s, &tokens[*index], index, "{"); // 関数本体の処理が始まる
                let mut func_code : Vec<usize> = Vec::new();
                while !Token::consume(s, &tokens[*index], index, "}") {
                    func_code.push(Node::stmt(s, tokens, index, tree, map, region)); // statementごとにパース
                    ty::type_of_node(tree, tree.len() - 1); // nodeに型情報を付加しておく
                }
                tree.push(Node::new(NodeKind::NDFnDef(func_name, arguments), func_code, func_type));
            }
            _ => {
                Token::error_msg(s, token.index, "関数定義ではありません");
            }
        }
    }


    fn stmt(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>, map : &mut HashMap<&'a str, VarInfo>, region : &mut i32) -> usize {
        let token = &tokens[*index];
        if Token::consume(s, token, index, "{") { // ブロック {}
            let mut vec : Vec<usize> = Vec::new();
            while !Token::consume(s, &tokens[*index], index, "}") {
                let index = Node::stmt(s, tokens, index, tree, map, region);
                vec.push(index);
            }
            tree.push(Node::new_init(NodeKind::NDBlock, vec));
        }
        else if Token::consume(s, token, index, "return") { // return文
            let left_index = Node::expr(s, tokens, index, tree, map, region);
            tree.push(Node::new_ret(left_index));
            Token::expect(s, &tokens[*index], index, ";");
        }
        else if Token::consume(s, token, index, "if") { // if文
            Token::expect(s, &tokens[*index], index, "(");
            let cond_index = Node::expr(s, tokens, index, tree, map, region);
            Token::expect(s, &tokens[*index], index, ")");
            let stmt_if = Node::stmt(s, tokens, index, tree, map, region);
            let mut stmt_else = std::usize::MAX;
            if Token::consume(s, &tokens[*index], index, "else") {
                stmt_else = Node::stmt(s, tokens, index, tree, map, region);
            }
            let vec = vec![cond_index, stmt_if, stmt_else];
            tree.push(Node::new_init(NodeKind::NDIf, vec));
        }
        else if Token::consume(s, token, index, "while") { // while文
            Token::expect(s, &tokens[*index], index, "(");
            let cond_index = Node::expr(s, tokens, index, tree, map, region);
            Token::expect(s, &tokens[*index], index, ")");
            let stmt_wh = Node::stmt(s, tokens, index, tree, map, region);
            let vec = vec![cond_index, stmt_wh];
            tree.push(Node::new_init(NodeKind::NDWh, vec));
        }
        else if Token::consume(s, token, index, "for") { // for文
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
            let node = Node::new_init(NodeKind::NDFor, vec);
            tree.push(node);
        }   
        else { // それ以外の文
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
        if Token::consume(s, token, index, "=") { // 代入文
            let val_index = Node::assign(s, tokens, index, tree, map, region);
            let vec = vec![left_index, val_index];
            tree.push(Node::new_init(NodeKind::NDAs, vec));
        }
        tree.len() - 1
    }

    fn equality(s : &str, tokens : &'a Vec<Token>, index : &mut usize, tree : &mut Vec<Node<'a>>, map : &mut HashMap<&'a str, VarInfo>, region : &mut i32) -> usize {
        let mut lval_index = Node::relational(s, tokens, index, tree, map, region);
        loop {
            if Token::consume(s, &tokens[*index], index, "==") {
                let rval_index = Node::relational(s, tokens, index, tree, map, region);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new_init(NodeKind::NDEq, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, "!=") {
                let rval_index = Node::relational(s, tokens, index, tree, map, region);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new_init(NodeKind::NDNEq, vec));
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
                tree.push(Node::new_init(NodeKind::NDLeEq, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, "<") {
                let rval_index = Node::add(s, tokens, index, tree, map, region);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new_init(NodeKind::NDLe, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, ">=") {
                let rval_index = Node::add(s, tokens, index, tree, map, region);
                let vec = vec![rval_index, lval_index];
                tree.push(Node::new_init(NodeKind::NDLeEq, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, ">") {
                let rval_index = Node::add(s, tokens, index, tree, map, region);
                let vec = vec![rval_index, lval_index];
                tree.push(Node::new_init(NodeKind::NDLe, vec));
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
                // オーバーロードに対応
                let node = Node::new_add(tree, lval_index, rval_index, vec);
                tree.push(node);
            }   
            else if Token::consume(s, &tokens[*index], index, "-") {
                let rval_index = Node::mul(s, tokens, index, tree, map, region);
                let vec = vec![lval_index, rval_index];
                // オーバーロードに対応
                let node = Node::new_sub(tree, lval_index, rval_index, vec);
                tree.push(node);
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
                tree.push(Node::new_init(NodeKind::NDMul, vec));
            }   
            else if Token::consume(s, &tokens[*index], index, "/") {
                let rval_index = Node::unary(s, tokens, index, tree, map, region);
                let vec = vec![lval_index, rval_index];
                tree.push(Node::new_init(NodeKind::NDDiv, vec));
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
            tree.push(Node::new_init(NodeKind::NDDeref, addr_index));
        }
        else if Token::consume(s, token, index, "&") {
            let var_index = vec![Node::unary(s, tokens, index, tree, map, region)];
            tree.push(Node::new_init(NodeKind::NDAddr, var_index));
        }
        // -x = 0 - x
        else if Token::consume(s, token, index, "-") {
            let lnode = Node::new_num(0, Type::Int); // we only have int type now
            let lval_index = tree.len();
            tree.push(lnode);
            let rval_index = Node::primary(s, tokens, index, tree, map, region);
            let vec = vec![lval_index, rval_index];
            tree.push(Node::new(NodeKind::NDSub, vec, Type::Int));
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
            let mut ty = Type::Int;
            *region += 8;
            while Token::consume(s, &tokens[*index], index, "*") {
                ty = Type::Ptr(Box::new(ty));
            }
            let token = &tokens[*index];
            match token.kind {
                TokenKind::TKIdent(lvar_name) => {
                    if map.get(lvar_name).is_some() {
                        Token::error_msg(s, token.index, "既に宣言された変数です");    
                    }

                    map.insert(lvar_name, VarInfo { ty : ty.clone() , offset: *region});
                    tree.push(Node::new_lvar(lvar_name, ty));
                    *index += 1;
                }
                _ => {
                    Token::error_msg(s, token.index, "変数ではありません");
                }
            }
        }
        else{
            match token.kind { 
                TokenKind::TKIdent(lvar_name) => { // function call
                    *index += 1;
                    if Token::consume(s, &tokens[*index], index, "(") {
                        let mut vec : Vec<usize> = Vec::new();
                        while !Token::consume(s, &tokens[*index], index, ")") {
                            vec.push(Node::expr(s, tokens, index, tree, map, region));
                            Token::consume(s, &tokens[*index], index, ",");
                        }
                        let var_info = map.entry(lvar_name).or_insert(VarInfo {ty : Type::Init, offset : -1});
                        tree.push(Node::new(NodeKind::NDFnCall(lvar_name), vec, var_info.ty.clone()));
                    }
                    else{
                        if map.get(lvar_name).is_none() {
                            Token::error_msg(s, token.index, "宣言されていない変数です");
                        }
                        tree.push(Node::new_lvar(lvar_name,  map.get(lvar_name).unwrap().ty.clone()));
                    }
                }
                _ => {
                    if true { // if int 
                        tree.push(Node::new_num(Token::expect_number(s, token, index), Type::Int));
                    }
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