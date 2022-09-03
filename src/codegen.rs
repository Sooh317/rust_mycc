use crate::parser;

pub fn generate_code(ast_tree : &Vec<parser::Node>, index : &usize) -> () {
    if ast_tree.len() <= *index { return (); }

    let node = &ast_tree[*index];
    // println!("{:?}", node);

    generate_code(ast_tree, &node.left_index);
    generate_code(ast_tree, &node.right_index);

    match node.kind {
        parser::NodeKind::NDAdd => {
            println!("  pop rdi\n  pop rax");
            println!("  add rax, rdi");
            println!("  push rax");
        }
        parser::NodeKind::NDSub => {
            println!("  pop rdi\n  pop rax");
            println!("  sub rax, rdi");
            println!("  push rax");
        }
        parser::NodeKind::NDMul => {
            println!("  pop rdi\n  pop rax");
            println!("  imul rax, rdi");
            println!("  push rax")
        }
        parser::NodeKind::NDDiv => {
            println!("  pop rdi\n  pop rax");
            println!("  cqo");
            println!("  idiv rdi");
            println!("  push rax")
        }
        parser::NodeKind::NDEq => {
            println!("  pop rdi\n  pop rax");
            println!("  cmp rax, rdi");
            println!("  sete al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        parser::NodeKind::NDNEq => {
            println!("  pop rdi\n  pop rax");
            println!("  cmp rax, rdi");
            println!("  setne al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        parser::NodeKind::NDLeEq => {
            println!("  pop rdi\n  pop rax");
            println!("  cmp rax, rdi");
            println!("  setle al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        parser::NodeKind::NDLe => {
            println!("  pop rdi\n  pop rax");
            println!("  cmp rax, rdi");
            println!("  setl al");
            println!("  movzb rax, al");
            println!("  push rax");
        }
        parser::NodeKind::NDNum(val) => println!("  push {}", val),
    }
}