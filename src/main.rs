pub mod ast;
pub mod lexer;
pub mod parser;
use std::env;
use std::fs;

fn main() {
    let tokens = tokenize();
    let ast = parser::parse(tokens);
    println!("{:#?}",ast);
}

fn tokenize() -> Vec<lexer::Token>{
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    return lexer::lex(contents);
}
