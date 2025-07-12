use std::env;
use std::f64;
use std::fs;
use std::io;

enum Token {
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Times,
    Divide,
    Integer(i32),
    Float(f64),
    Variable(Box<str>),
    String(Box<str>),
}

enum ScanningOperation {
    None,
    UnknownNumericLiteral,
    FloatLiteral,
    Keyword,
    String,
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)?;

    let mut tokens: Vec<Token> = Vec::new();

    let mut current_token = "".to_string();
    let mut current_scanning_operation = ScanningOperation::None;
    for char in contents.as_str().chars() {
        match current_scanning_operation {
            ScanningOperation::String => {
                if char == '"' {
                    tokens.push(Token::String(current_token.clone().as_str().into()));
                    current_token.clear();
                    current_scanning_operation = ScanningOperation::None;
                } else {
                    current_token.push(char);
                }
            }
            ScanningOperation::Keyword => {
                if char.is_whitespace() {
                    if current_token.len() > 0 {
                        tokens.push(Token::Variable(current_token.clone().as_str().into()));
                    }
                    current_token.clear();
                    current_scanning_operation = ScanningOperation::None;
                } else if char.is_alphanumeric() || char != '_' || char != '-' {
                    current_token.push(char);
                } else {
                    println!("Syntax Error: Invalid keyword character {char}");
                    return Ok(());
                }
            }
            ScanningOperation::FloatLiteral => {
                if char.is_whitespace() {
                    let parsed_float = current_token.clone().parse::<f64>();
                    match parsed_float {
                        Ok(float_value) => tokens.push(Token::Float(float_value)),
                        Err(_) => {
                            println!(
                                "Syntax Error: Could not parse {current_token} as floating point"
                            );
                            return Ok(());
                        }
                    }
                    current_token.clear();
                    current_scanning_operation = ScanningOperation::None;
                } else if char.is_numeric() {
                    current_token.push(char);
                } else {
                    println!("Syntax Error: Could not parse {current_token} as floating point");
                    return Ok(());
                }
            }
            ScanningOperation::UnknownNumericLiteral => {
                if char.is_whitespace() {
                    if current_token.len() > 0 {
                        let parsed_int = current_token.clone().parse::<i32>();
                        tokens.push(Token::Integer(parsed_int.unwrap()));
                    }
                    current_token.clear();
                    current_scanning_operation = ScanningOperation::None;
                } else if char == '.' {
                    current_token.push(char);
                    current_scanning_operation = ScanningOperation::FloatLiteral;
                } else if char.is_numeric() {
                    current_token.push(char);
                } else {
                    println!("Syntax Error: Could not parse {current_token} as integer");
                    return Ok(());
                }
            }
            ScanningOperation::None => {
                if char == '(' {
                    tokens.push(Token::OpenParen);
                } else if char == ')' {
                    tokens.push(Token::CloseParen);
                } else if char == '+' {
                    tokens.push(Token::Plus);
                } else if char == '-' {
                    tokens.push(Token::Minus);
                } else if char == '*' {
                    tokens.push(Token::Times);
                } else if char == '/' {
                    tokens.push(Token::Divide);
                } else if char.is_numeric() {
                    current_token.push(char);
                    current_scanning_operation = ScanningOperation::UnknownNumericLiteral;
                } else if char.is_alphabetic() {
                    current_token.push(char);
                    current_scanning_operation = ScanningOperation::Keyword;
                } else if char == '"' {
                    current_scanning_operation = ScanningOperation::String;
                } else if !char.is_whitespace() {
                    println!("Syntax Error: Variable name cannot start with {char}");
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}
