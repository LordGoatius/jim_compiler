use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn assemble(intermediate: File) {
    let mut file = File::create("main.s").unwrap();
    let mut file_start: &str = r#"
    .global	_start

    .text
    _start:"#;
}