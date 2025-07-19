use std::env;
use std::fs;

pub mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let contents =
        fs::read_to_string(file_path).unwrap_or_else(|e| format!("Error reading {file_path}: {e}"));

    let tokens = scanner::scan(contents);
}
