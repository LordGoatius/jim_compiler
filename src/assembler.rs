use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::os::unix::process;

pub fn assemble(intermediate: &mut File) {
    let mut file = File::create("main.s").unwrap();
    let file_prefix: &str = 
".global main
.extern printf
.intel_syntax noprefix

main:";
    let status = 69;
    let file_end: &str = &format!("    
    mov rax, 60  
    mov rdi, {}   
    syscall
"
    ,status)[..];

    let (text, data) = interpret_to_assembly(intermediate);

    let file_full = format!("{}{}{}{}",file_prefix,text,file_end,data);

    file.write_all(file_full.as_bytes()).unwrap();
}

fn interpret_to_assembly(file: &mut File) -> (&'static str, &'static str) {
    let file_contents = std::fs::read_to_string("main.ir").expect("failed to read");
    let mut text_contents = String::new();
    let mut data_contents = String::new();
    for chunk in file_contents.split("\n \n") {
        let (text,data) = process_chunk(chunk);
    }

    ("","")
}

fn process_chunk(chunk: &str) -> (&str, &str) {
    // %rax, %rbx, %rcx, %rdx, %rsi, %rdi, %rsp, %rbp, %r8-r15
    let registers = ["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10", "r11"];
    let mut register_map:HashMap<&str, &str> = HashMap::new();
    for line in chunk.split("\n").filter(|line| !line.is_empty()) {
        println!("{}",line);
        match to_line_type(line) {
            LineType::Equals => {
                println!("Equals");
            },
            LineType::Operation => {
                println!("Operation");
            },
            LineType::Print => {
                println!("Print");
            },
        }
    }
    ("","")
}

fn add_data(name: &str, data: &str) -> String {
    format!("{}:\n  .quad 0x{}",name,data)
}

fn to_line_type(line: &str) -> LineType {
    if line.contains("print") {
        LineType::Print
    } else if line.contains("+") || line.contains("*") || line.contains("/") || line.contains("-") {
        LineType::Operation
    } else {
        LineType::Equals
    }
}
enum LineType {
    Equals,
    Print,
    Operation
}