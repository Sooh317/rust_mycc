use crate::parser::{NodeKind, Node};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Int, 
    Array,
    Ptr(Box<Type>),
    Init,
}

pub fn type_to_offset(ty : &Type) -> i32 {
    match ty {
        Type::Int => 8,
        Type::Ptr(_) => 8,
        _ => -1
    }
}

pub fn type_to_size(ty : &Type) -> i32 {
    match ty {
        Type::Int => 4,
        Type::Ptr(_) => 8,
        _ => -1
    }
}


pub fn type_of_node(tree : &mut Vec<Node>, index : usize) {
    if index >= tree.len() || tree[index].ty != Type::Init { return; }
    
    let indices = tree[index].indices.clone();
    for child_index in indices {
        type_of_node(tree, child_index);
    }
    
    match tree[index].kind {
        NodeKind::NDAs => {
            if tree[*tree[index].indices.first().unwrap()].ty == tree[*tree[index].indices.last().unwrap()].ty {
                tree[index].ty = tree[*tree[index].indices.first().unwrap()].ty.clone();
            }
            else {
                eprintln!("異なる型の値を代入しています");
                // std::process::exit(1);
            }
        }
        NodeKind::NDAddr => {
            tree[index].ty = Type::Ptr(Box::new(tree[*tree[index].indices.first().unwrap()].ty.clone()));
        }
        NodeKind::NDDeref => {
            match tree[*tree[index].indices.first().unwrap()].ty.clone() {
                Type::Ptr(ty1) => tree[index].ty = *ty1,
                _ => {
                    eprintln!("参照外しができません");
                    std::process::exit(1);
                }
            }
        }
        NodeKind::NDRet => {
            tree[index].ty = tree[*tree[index].indices.first().unwrap()].ty.clone();
        }

        NodeKind::NDMul => {
            match (&tree[*tree[index].indices.first().unwrap()].ty, &tree[*tree[index].indices.last().unwrap()].ty) {
                (Type::Int, Type::Int) => tree[index].ty = Type::Int,
                _ => {
                    eprintln!("未定義の掛け算を行っています");
                    std::process::exit(1);
                }
            }
        }
        NodeKind::NDDiv => {
            match (&tree[*tree[index].indices.first().unwrap()].ty, &tree[*tree[index].indices.last().unwrap()].ty) {
                (Type::Int, Type::Int) => tree[index].ty = Type::Int,
                _ => {
                    eprintln!("未定義の割り算を行っています");
                    std::process::exit(1);
                }
            }
        }
        NodeKind::NDEq => {
            if tree[*tree[index].indices.first().unwrap()].ty == tree[*tree[index].indices.last().unwrap()].ty {
                tree[index].ty = Type::Int;
            }
            else {
                eprintln!("異なる型での比較'='を行っています");
                std::process::exit(1);
            }
        }
        NodeKind::NDNEq => {
            if tree[*tree[index].indices.first().unwrap()].ty == tree[*tree[index].indices.last().unwrap()].ty {
                tree[index].ty = Type::Int;
            }
            else {
                eprintln!("異なる型での比較'!='を行っています");
                std::process::exit(1);
            }
        }
        NodeKind::NDLe => {
            if tree[*tree[index].indices.first().unwrap()].ty == tree[*tree[index].indices.last().unwrap()].ty {
                tree[index].ty = Type::Int;
            }
            else {
                eprintln!("異なる型での比較'<'または'>'を行っています");
                std::process::exit(1);
            }
        }
        NodeKind::NDLeEq => {
            if tree[*tree[index].indices.first().unwrap()].ty == tree[*tree[index].indices.last().unwrap()].ty {
                tree[index].ty = Type::Int;
            }
            else {
                eprintln!("異なる型での比較'<='または'>='を行っています");
                std::process::exit(1);
            }
        }
        _ => {
            return;
        }
    }
}