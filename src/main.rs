use std::env;
use std::fs;

use crate::scanner::SyntaxError;

pub mod scanner;

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
    let tokens = match scanner::scan(contents) {
        Ok(t) => t,
        Err(SyntaxError { position }) => {
            println!("Syntax error at position {position}");
            return;
        }
    };
}
