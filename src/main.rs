use std::env;
use std::fs;

use crate::parser::SyntaxError;

pub mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let contents = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to read file {file_path}: {e}");
            return;
        }
    };
    let expressions = match parser::parse(contents) {
        Ok(t) => t,
        Err(SyntaxError { position }) => {
            println!("Syntax error at position {position}");
            return;
        }
    };

    print!("{:#?}", expressions);
}
