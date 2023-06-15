pub mod ast;
pub mod lexer;
pub mod parser;

use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

fn main() {
    let tokens = tokenize();
    let ast = parser::parse(tokens);
    println!("{:#?}",ast);
    
    create_xml_file(&ast).expect("XML creation failed");
    create_json_file(&ast).expect("JSON creation failed");
}

fn tokenize() -> Vec<lexer::Token>{
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    return lexer::lex(contents);
}

fn create_xml_file(ast: &ast::Program) -> Result<()>{
    let mut file = File::create("ast.xml")?;
    let ast_str = serde_xml_rs::to_string(&ast).expect("XML parse failed");
    file.write_all(ast_str.as_bytes())?;
    Ok(())
}

fn create_json_file(ast: &ast::Program) -> Result<()>{
    let mut file = File::create("ast.json")?;
    let ast_str = serde_json::to_string(&ast).expect("XML parse failed");
    file.write_all(ast_str.as_bytes())?;
    Ok(())
}