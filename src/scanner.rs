use std::f64;

pub enum Token {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Integer(i32),
    Float(f64),
    Char(char),
    String(Box<str>),
    Identifier(Box<str>),
}

enum ScanningOperation {
    None,
    NumericLiteral,
    FloatLiteral,
    Identifier,
    String,
    Char,
}

pub struct SyntaxError {
    position: usize,
}

pub fn scan(input: String) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut current_token = "".to_string();
    let mut scanned_char: Option<char> = None;
    let mut current_scanning_operation = ScanningOperation::None;
    for (idx, char) in input.as_str().chars().enumerate() {
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
            ScanningOperation::Char => {
                if char == '\'' {
                    match scanned_char {
                        Some(ch) => {
                            tokens.push(Token::Char(ch));
                            current_scanning_operation = ScanningOperation::None;
                        }
                        None => return Err(SyntaxError { position: idx }),
                    }
                } else {
                    match scanned_char {
                        Some(_ch) => return Err(SyntaxError { position: idx }),
                        None => scanned_char = Some(char),
                    }
                }
            }
            ScanningOperation::Identifier => {
                if char.is_whitespace() {
                    if current_token.len() > 0 {
                        tokens.push(Token::Identifier(current_token.clone().as_str().into()));
                    }
                    current_token.clear();
                    current_scanning_operation = ScanningOperation::None;
                } else if char.is_alphanumeric() || char != '_' || char != '-' {
                    current_token.push(char);
                } else {
                    return Err(SyntaxError { position: idx });
                }
            }
            ScanningOperation::FloatLiteral => {
                if char.is_whitespace() {
                    let parsed_float = current_token.clone().parse::<f64>();
                    match parsed_float {
                        Ok(float_value) => tokens.push(Token::Float(float_value)),
                        Err(_) => return Err(SyntaxError { position: idx }),
                    }
                    current_token.clear();
                    current_scanning_operation = ScanningOperation::None;
                } else if char.is_numeric() {
                    current_token.push(char);
                } else {
                    return Err(SyntaxError { position: idx });
                }
            }
            ScanningOperation::NumericLiteral => {
                if char.is_whitespace() {
                    let parsed_int = current_token.clone().parse::<i32>().unwrap();
                    tokens.push(Token::Integer(parsed_int));
                    current_token.clear();
                    current_scanning_operation = ScanningOperation::None;
                } else if char == '.' {
                    current_token.push(char);
                    current_scanning_operation = ScanningOperation::FloatLiteral;
                } else if char.is_numeric() {
                    current_token.push(char);
                } else {
                    return Err(SyntaxError { position: idx });
                }
            }
            ScanningOperation::None => {
                if char == '(' {
                    tokens.push(Token::OpenParen);
                } else if char == ')' {
                    tokens.push(Token::CloseParen);
                } else if char == '[' {
                    tokens.push(Token::OpenBracket);
                } else if char == ']' {
                    tokens.push(Token::CloseBracket);
                } else if char == '+' {
                    tokens.push(Token::Plus);
                } else if char == '-' {
                    tokens.push(Token::Minus);
                } else if char == '*' {
                    tokens.push(Token::Times);
                } else if char == '/' {
                    tokens.push(Token::Divide);
                } else if char == '%' {
                    tokens.push(Token::Modulo);
                } else if char.is_numeric() {
                    current_token.push(char);
                    current_scanning_operation = ScanningOperation::NumericLiteral;
                } else if char.is_alphabetic() {
                    current_token.push(char);
                    current_scanning_operation = ScanningOperation::Identifier;
                } else if char == '"' {
                    current_scanning_operation = ScanningOperation::String;
                } else if char == '\'' {
                    current_scanning_operation = ScanningOperation::Char;
                } else if !char.is_whitespace() {
                    return Err(SyntaxError { position: idx });
                }
            }
        }
    }

    Ok(tokens)
}
