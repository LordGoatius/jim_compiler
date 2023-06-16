pub mod ast;
pub mod lexer;
pub mod parser;
pub mod runtime;
pub mod repl;

use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Jimlang REPL v0.1.0");
        repl::start_repl();
    } else {
        let file_path = &args[1];

        let contents = fs::read_to_string(file_path).expect("File read err");
        
        run(contents);
    }
}

fn run(contents: String) {
    let tokens = tokenize(contents);
    let ast = parser::parse(tokens);
    runtime::interpret(ast);
}

fn tokenize(contents: String) -> Vec<lexer::Token>{
    lexer::lex(contents)
}

#[allow(dead_code)]
fn create_xml_file(ast: &ast::Program) -> Result<()>{
    let mut file = File::create("ast.xml")?;
    let ast_str = serde_xml_rs::to_string(&ast).expect("XML parse failed");
    file.write_all(ast_str.as_bytes())?;
    Ok(())
}

#[allow(dead_code)]
fn create_json_file(ast: &ast::Program) -> Result<()>{
    let mut file = File::create("ast.json")?;
    let ast_str = serde_json::to_string(&ast).expect("XML parse failed");
    file.write_all(ast_str.as_bytes())?;
    Ok(())
}