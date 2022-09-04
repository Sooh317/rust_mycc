use crate::parser::{NodeKind, Node};

pub fn generate_code(ast_tree : &Vec<Node>, index : &usize, branch_num : &mut i32) {
    if ast_tree.len() <= *index { return; }
    
    let node = &ast_tree[*index];
    // println!("{:?}", node);

    match node.kind {
        NodeKind::NDRet => {
            generate_code(ast_tree, &node.indices.get(0).unwrap(), branch_num);
            println!("  pop rax");
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
            return;
        }
        NodeKind::NDLVa(_, _) => { // When variable occurs in the context of expressions, the value is stored in the stack.
            generate_lval(ast_tree, index);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            return;
        }
        NodeKind::NDAs => {
            generate_lval(ast_tree, &node.indices.get(0).unwrap()); // -> rax
            generate_code(ast_tree, &node.indices.get(1).unwrap(), branch_num); // -> rdi
            println!("  pop rdi\n  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
            return;
        }
        NodeKind::NDNum(val) => {
            println!("  push {}", val);
            return;
        }
        NodeKind::NDIf => {
            generate_code(ast_tree, &node.indices.get(0).unwrap(), branch_num);
            let use_num = *branch_num;
            *branch_num += 1;
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je  .Lelse{}", use_num);
            generate_code(ast_tree, &node.indices.get(1).unwrap(), branch_num);
            println!("  jmp .Lend{}", use_num);
            println!(".Lelse{}:", use_num);
            generate_code(ast_tree, &node.indices.get(2).unwrap(), branch_num);
            println!(".Lend{}:", use_num);
            return;
        }
        NodeKind::NDWh => {
            let use_num = *branch_num;
            *branch_num += 1;
            println!(".Lbegin{}:", use_num);
            generate_code(ast_tree, &node.indices.get(0).unwrap(), branch_num);
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je  .Lend{}", use_num);
            generate_code(ast_tree, &node.indices.get(1).unwrap(), branch_num);
            println!("  jmp .Lbegin{}", use_num);
            println!(".Lend{}:", use_num);
            return;
        }
        NodeKind::NDFor => {
            let use_num = *branch_num;
            *branch_num += 1;
            generate_code(ast_tree, &node.indices.get(0).unwrap(), branch_num);
            println!(".Lbegin{}:", use_num);
            generate_code(ast_tree, &node.indices.get(1).unwrap(), branch_num);
            if node.indices.get(1).unwrap() < &ast_tree.len() {
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je  .Lend{}", use_num);
            }
            generate_code(ast_tree, &node.indices.get(3).unwrap(), branch_num);
            generate_code(ast_tree, &node.indices.get(2).unwrap(), branch_num);
            println!("  jmp .Lbegin{}", use_num);
            println!(".Lend{}:", use_num);
            return;
        }
        _ => (),
    }

    generate_code(ast_tree, &node.indices.get(0).unwrap(), branch_num);
    generate_code(ast_tree, &node.indices.get(1).unwrap(), branch_num);

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
fn generate_lval(ast_tree : &Vec<Node>, index : &usize) {
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