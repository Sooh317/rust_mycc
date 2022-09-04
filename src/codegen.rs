use crate::parser::{NodeKind, Node};

pub fn generate_code(ast_tree : &Vec<Node>, index : &usize) -> () {
    if ast_tree.len() <= *index { return (); }
    
    let node = &ast_tree[*index];
    // println!("{:?}", node);

    match node.kind {
        NodeKind::NDLVa(_, _) => { // When variable occurs in the context of expressions, the value is stored in the stack.
            generate_lval(ast_tree, index);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            return ();
        }
        NodeKind::NDAs => {
            generate_lval(ast_tree, &node.left_index); // -> rax
            generate_code(ast_tree, &node.right_index); // -> rdi
            println!("  pop rdi\n  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
            return ();
        }
        NodeKind::NDNum(val) => {
            println!("  push {}", val);
            return ();
        }

        _ => (),
    }

    generate_code(ast_tree, &node.left_index);
    generate_code(ast_tree, &node.right_index);

    println!("  pop rdi\n  pop rax");

    match node.kind {
        NodeKind::NDAdd => {
            println!("  add rax, rdi");
        }
        NodeKind::NDSub => {
            println!("  sub rax, rdi");
        }
        NodeKind::NDMul => {
            println!("  imul rax, rdi");
        }
        NodeKind::NDDiv => {
            println!("  cqo");
            println!("  idiv rdi");
        }
        NodeKind::NDEq => {
            println!("  cmp rax, rdi");
            println!("  sete al");
            println!("  movzb rax, al");
        }
        NodeKind::NDNEq => {
            println!("  cmp rax, rdi");
            println!("  setne al");
            println!("  movzb rax, al");
        }
        NodeKind::NDLeEq => {
            println!("  cmp rax, rdi");
            println!("  setle al");
            println!("  movzb rax, al");
        }
        NodeKind::NDLe => {
            println!("  cmp rax, rdi");
            println!("  setl al");
            println!("  movzb rax, al");
        }
        _ => (),
    }
    println!("  push rax");
}

// push the address of a variable on the stack
fn generate_lval(ast_tree : &Vec<Node>, index : &usize) -> () {
    let node = &ast_tree[*index];
    match node.kind {
        NodeKind::NDLVa(_, offset) => {
            println!("  mov rax, rbp");
            println!("  sub rax, {}", offset);
            println!("  push rax");
        }
        _ => {
            eprintln!("代入の左辺値が変数ではありません");
        }
    }
}