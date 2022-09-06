use crate::parser::{NodeKind, Ast, Type};

pub fn generate_code(ast : &Ast, index : &usize, branch_num : &mut i32) {
    if ast.tree.len() <= *index { return; }
    
    let node = &ast.tree[*index];
    // println!("{:?}", node);

    match &node.kind {
        NodeKind::NDRet => {
            generate_code(ast, node.indices.first().unwrap(), branch_num);
            println!("  pop rax");
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
            return;
        }
        NodeKind::NDLVa(_) => { // When variable occurs in the context of expressions, the value is stored in the stack.
            generate_lval(ast, index, branch_num);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            return;
        }
        NodeKind::NDAs => {
            generate_lval(ast, node.indices.first().unwrap(), branch_num); // -> rax
            generate_code(ast, node.indices.get(1).unwrap(), branch_num); // -> rdi
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
            generate_code(ast, node.indices.first().unwrap(), branch_num);
            let use_num = *branch_num;
            *branch_num += 1;
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je  .Lelse{}", use_num);
            generate_code(ast, node.indices.get(1).unwrap(), branch_num);
            println!("  jmp .Lend{}", use_num);
            println!(".Lelse{}:", use_num);
            generate_code(ast, node.indices.get(2).unwrap(), branch_num);
            println!(".Lend{}:", use_num);
            return;
        }
        NodeKind::NDWh => {
            let use_num = *branch_num;
            *branch_num += 1;
            println!(".Lbegin{}:", use_num);
            generate_code(ast, node.indices.first().unwrap(), branch_num);
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je  .Lend{}", use_num);
            generate_code(ast, node.indices.get(1).unwrap(), branch_num);
            println!("  jmp .Lbegin{}", use_num);
            println!(".Lend{}:", use_num);
            return;
        }
        NodeKind::NDFor => {
            let use_num = *branch_num;
            *branch_num += 1;
            generate_code(ast, node.indices.first().unwrap(), branch_num);
            println!(".Lbegin{}:", use_num);
            generate_code(ast, node.indices.get(1).unwrap(), branch_num);
            if node.indices.get(1).unwrap() < &ast.tree.len() {
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je  .Lend{}", use_num);
            }
            generate_code(ast, node.indices.get(3).unwrap(), branch_num);
            generate_code(ast, node.indices.get(2).unwrap(), branch_num);
            println!("  jmp .Lbegin{}", use_num);
            println!(".Lend{}:", use_num);
            return;
        }
        NodeKind::NDBlock => {
            for index in &node.indices {
                generate_code(ast, index, branch_num);
                println!("  pop rax");
            }
            println!("  push rax");
            return;
        }
        NodeKind::NDFnCall(func) => {
            if node.indices.len() <= 6 {
                for i in 0..node.indices.len() {
                    generate_code(ast, node.indices.get(i).unwrap(), branch_num);
                }
                for i in (0..node.indices.len()).rev() {
                    match i {
                        0 => println!("  pop rdi"),
                        1 => println!("  pop rsi"),
                        2 => println!("  pop rdx"),
                        3 => println!("  pop rcx"),
                        4 => println!("  pop r8"),
                        5 => println!("  pop r9"),
                        _ => std::process::exit(1),
                    }
                }

            }
            println!("  call {}", func);
            println!("  push rax");
            return;
        }
        NodeKind::NDFnDef(func_name, arg_names) => {
            println!("{}:", func_name);
            println!("  push rbp");
            println!("  mov rbp, rsp");
            println!("  sub rsp, {}", ast.region); // lvar_num is a multiple of 16
            for (i, arg) in arg_names.iter().enumerate() {
                println!("  mov rax, rbp");
                println!("  sub rax, {}", ast.map.get(arg).unwrap().offset);
                match i {
                    0 => println!("  mov [rax], rdi"),
                    1 => println!("  mov [rax], rsi"),
                    2 => println!("  mov [rax], rdx"),
                    3 => println!("  mov [rax], rcx"),
                    4 => println!("  mov [rax], r8"),
                    5 => println!("  mov [rax], r9"),
                    _ => std::process::exit(1),
                }
            }
            for i in 0..node.indices.len() {
                generate_code(ast, node.indices.get(i).unwrap(), branch_num);
                println!("  pop rax");
            }
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
            return;
        }
        NodeKind::NDAddr => {
            generate_lval(ast, node.indices.first().unwrap(), branch_num);
            return;
        }
        NodeKind::NDDeref => {
            generate_code(ast, node.indices.first().unwrap(), branch_num);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            return;
        }
        _ => (),
    }

    generate_code(ast, node.indices.first().unwrap(), branch_num);
    generate_code(ast, node.indices.get(1).unwrap(), branch_num);

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
fn generate_lval(ast : &Ast, index : &usize, branch_num : &mut i32) {
    let node = &ast.tree[*index];
    match node.kind {
        NodeKind::NDLVa(name) => {
            println!("  mov rax, rbp");
            println!("  sub rax, {}", ast.map.get(name).unwrap().offset);
            println!("  push rax");
        }
        NodeKind::NDDeref => {
            generate_code(ast, node.indices.first().unwrap(), branch_num);
        }
        _ => {
            eprintln!("代入の左辺値が変数ではありません");
            std::process::exit(1);
        }
    }
}

