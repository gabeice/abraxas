use std::env;
use std::fs;
use std::io;

pub mod scanner;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)?;

    let tokens = scanner::scan(contents);

    Ok(())
}
