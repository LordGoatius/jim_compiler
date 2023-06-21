use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::os::unix::process;

pub fn assemble(intermediate: &mut File) {
    let mut file = File::create("main.s").unwrap();
    let file_prefix: &str = 
".global _start
.intel_syntax noprefix

_start:";
    let status = 69;
    let file_end: &str = &format!("    
    mov rax, 60  
    mov rdi, {}   
    syscall
"
    ,status)[..];

    let middle = interpret_to_assembly(intermediate);

    let file_full = format!("{}{}{}",file_prefix,middle,file_end);

    file.write_all(file_full.as_bytes()).unwrap();
}

fn interpret_to_assembly(file: &mut File) -> &'static str {
    let file_contents = std::fs::read_to_string("main.ir").expect("failed to read");
    let mut assembly_contents = String::new();
    println!("{:?}",file_contents);
    for chunk in file_contents.split("\n \n") {
        process_chunk(chunk);
    }
    ""
}

fn process_chunk(chunk: &str) -> &str {
    // %rax, %rbx, %rcx, %rdx, %rsi, %rdi, %rsp, %rbp, %r8-r15
    let mut register_map:HashMap<&str, &str> = HashMap::new();
    for token in chunk.split_ascii_whitespace() {
        
    }
    ""
}
