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

main:
";
    let file_end: &str = &format!("    
    mov rax, 60  
    mov rdi, 0   
    syscall

pntfloat:
    .ascii \"%g\\n\\0\"

pntint: 
    .ascii \"%d\\n\\0\"

")[..];

    let text = interpret_to_assembly();

    let file_full = format!("{}{}{}",file_prefix,text,file_end);

    file.write_all(file_full.as_bytes()).unwrap();
}

fn interpret_to_assembly() -> String {
    let file_contents = std::fs::read_to_string("main.ir").expect("failed to read");

    let mut text_contents = String::new();
    let mut registers: HashMap<String, String> = HashMap::new();
    let mut stack: Vec<(String, i64)> = Vec::new();
    
    for line in file_contents.split("\n") {
        let text = process_line(line, &mut registers, &mut stack);
        text_contents.push_str(&text);
    }
    text_contents
}

fn process_line(line: &str, registers: &mut HashMap<String, String>, stack: &mut Vec<(String, i64)>) -> String {
    match to_line_type(line) {
        LineType::EqualsId => {
            process_equals_id(line, registers, stack)
        },
        LineType::EqualsLiteral => {
            process_equals_literal(line, registers, stack)
        },
        LineType::Operation => {
            let to_ret = process_operation(line, registers, stack);
            registers.remove("r12");
            to_ret
        },
        LineType::Print => {
            process_print(line, registers)
        },
        LineType::NonOp => {
            "".to_string()
        }
    }
}

fn process_equals_id(line: &str, registers: &mut HashMap<String, String>, stack: &mut Vec<(String, i64)>) -> String {
    let id1 = line.split(' ').next().unwrap();
    let id2 = line.split(' ').last().unwrap();

    if registers.contains_key("r12") {
        registers.insert("r13".to_owned(), id1.to_string());
        println!("{:?}",registers);
        format!("\tmov r13, [rsp + {}]\n",find_rsp_offset(id2,stack))
    } else {
        println!("{:?}",registers);
        registers.insert("r12".to_owned(),id1.to_string());
        format!("\tmov r12, [rsp + {}]\n",find_rsp_offset(id2,stack))
    }
}

fn find_rsp_offset(id1: &str, stack: &mut Vec<(String, i64)>) -> usize {
    match stack.iter().position(|(id, _)| id.contains(id1)) {
        Some(val) => {
            println!("{val}");
            (stack.len() - val) * 8
        },
        None => panic!("compiler error")
    }
}

fn process_equals_literal(line: &str, registers: &mut HashMap<String, String>, stack: &mut Vec<(String, i64)>) -> String {
    let id = line.split(' ').next().unwrap();
    let value = line.split(' ').last().unwrap();

    stack.push((id.to_owned(),value.parse::<i64>().unwrap()));
    println!("{:?}",&stack);

    format!("\tsub rsp, 8\n\tmov QWORD ptr [rsp + 8], {}\n",value)
}

fn process_operation(line: &str, registers: &mut HashMap<String, String>, stack: &mut Vec<(String, i64)>) -> String {
    let id_assign = line.split(' ').next().unwrap();
    let id_1 = line.split(' ').nth(2).unwrap();
    let operator = line.split(' ').nth(3).unwrap();
    let id_2 = line.split(' ').next_back().unwrap();

    println!("{}{}{}{}",id_assign,id_1,operator,id_2);

    let operator = if operator == "+" {
        "add"
    } else if operator == "-" {
        "sub"
    } else if operator == "*" {
        "imul"
    } else {
        "idiv"
    };

    if (!registers.contains_key("r12") && !registers.contains_key("r13")) {
        
    }

    if operator == "idiv" {
        todo!()
    } else {
        registers.insert("r12".to_owned(), id_assign.to_owned());
        registers.remove("r13");
        format!("\t{} r12, r13", operator)
    }
    
}

fn process_print(line: &str, registers: &mut HashMap<String, String>) -> String {
    "".to_string()
}

fn add_data(name: &str, data: &str) -> String {
    format!("{}:\n  .quad 0x{}",name,data)
}

fn to_line_type(line: &str) -> LineType {
    if line.contains("print") {
        LineType::Print
    } else if line.chars().nth(0) == Some(' ') || line.chars().nth(0) == None {
        LineType::NonOp
    } else if line.contains("+") || line.contains("*") || line.contains("/") || line.contains("-") {
        LineType::Operation
    } else if line[..3] == line[line.len()-3..] {
        LineType::NonOp
    } else if line.contains("= id") {
        LineType::EqualsId
    }else {
        LineType::EqualsLiteral
    }
}
enum LineType {
    EqualsId,
    EqualsLiteral,
    Print,
    Operation,
    NonOp
}
