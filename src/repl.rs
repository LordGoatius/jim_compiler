use crate::lexer;
use crate::lexer::*;
use crate::parser;
use crate::runtime;

use std::io;
use std::collections::HashMap;
use std::io::Write;

pub fn start_repl() {
    let mut variable_map: HashMap<String, Number> = HashMap::new();
    io::stdout().flush().unwrap();
    'repl_loop: loop {
        let mut input = String::new();
        print!(r#"> "#);
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if (input.contains("exit\r")) | (input.contains("quit\r")) {
            break 'repl_loop;
        }

        let tokens = lexer::lex(input.clone());
        let ast = parser::parse(tokens);

        if let Option::Some(string) = runtime::execute(ast.statements[0].clone(), &mut variable_map) {
            println!("{}",string);
        }
    }
}