use std::f64;

enum ScannerState {
    ExpectingExpressionStart,
    ExpectingOperand,
    ExpectingArg,
    ScanningOperand,
    ScanningNumber,
    ScanningFloat,
    ScanningChar,
}

#[derive(Clone)]
enum Operand {
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    And,
    Or,
    If,
    Equal,
    LessThan,
    GreaterThan,
}

#[derive(Clone)]
enum ExpressionArg {
    Int(i32),
    Float(f64),
    Char(char),
    NestedExpression(Expression),
}

#[derive(Clone)]
pub struct Expression {
    operand: Operand,
    arguments: Vec<ExpressionArg>,
}

pub struct SyntaxError {
    pub position: usize,
}

fn get_operand(token: &mut String, position: usize) -> Result<Operand, SyntaxError> {
    match token.as_str() {
        "+" => Ok(Operand::Add),
        "-" => Ok(Operand::Subtract),
        "*" => Ok(Operand::Multiply),
        "/" => Ok(Operand::Divide),
        "not" => Ok(Operand::Not),
        "and" => Ok(Operand::And),
        "or" => Ok(Operand::Or),
        "if" => Ok(Operand::If),
        "=" => Ok(Operand::Equal),
        "<" => Ok(Operand::LessThan),
        ">" => Ok(Operand::GreaterThan),
        _ => Err(SyntaxError { position }),
    }
}

pub fn scan(input: String) -> Result<Vec<Expression>, SyntaxError> {
    let mut complete_expressions: Vec<Expression> = Vec::new();
    let mut open_expressions: Vec<Expression> = Vec::new();

    let mut current_operand = None;
    let mut current_arg_list = Vec::new();

    let mut scanner_state = ScannerState::ExpectingExpressionStart;
    let mut current_token = "".to_string();
    let mut scanned_char: Option<char> = None;
    let mut num_open_parentheses = 0;
    for (idx, char) in input.as_str().chars().enumerate() {
        match scanner_state {
            ScannerState::ExpectingExpressionStart => {
                if char == '(' {
                    scanner_state = ScannerState::ExpectingOperand;
                    num_open_parentheses += 1;
                } else if !char.is_whitespace() {
                    return Err(SyntaxError { position: idx });
                }
            }
            ScannerState::ExpectingOperand => {
                if char.is_alphabetic() || ['+', '-', '*', '/', '=', '<', '>'].contains(&char) {
                    scanner_state = ScannerState::ScanningOperand;
                    current_token.push(char);
                } else if !char.is_whitespace() {
                    return Err(SyntaxError { position: idx });
                }
            }
            ScannerState::ScanningOperand => {
                if char.is_whitespace() {
                    let operand = get_operand(&mut current_token, idx)?;
                    current_operand = Some(operand);
                    current_token.clear();
                    scanner_state = ScannerState::ExpectingArg;
                } else if char.is_alphanumeric() || char == '_' || char == '-' {
                    current_token.push(char);
                } else {
                    return Err(SyntaxError { position: idx });
                }
                // Check close paren
            }
            ScannerState::ScanningChar => {
                if char == '\'' {
                    match scanned_char {
                        Some(ch) => {
                            current_arg_list.push(ExpressionArg::Char(ch));
                            scanner_state = ScannerState::ExpectingArg;
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
            ScannerState::ScanningFloat => {
                if char.is_whitespace() {
                    let parsed_float = current_token.clone().parse::<f64>();
                    match parsed_float {
                        Ok(float_value) => {
                            current_arg_list.push(ExpressionArg::Float(float_value));
                        }
                        Err(_) => return Err(SyntaxError { position: idx }),
                    }
                    current_token.clear();
                    scanner_state = ScannerState::ExpectingArg;
                } else if char.is_numeric() {
                    current_token.push(char);
                } else {
                    return Err(SyntaxError { position: idx });
                }
                // Check close paren
            }
            ScannerState::ScanningNumber => {
                if char.is_whitespace() {
                    let parsed_int = current_token.clone().parse::<i32>().unwrap();
                    current_arg_list.push(ExpressionArg::Int(parsed_int));
                    current_token.clear();
                    scanner_state = ScannerState::ExpectingArg;
                } else if char == '.' {
                    current_token.push(char);
                    scanner_state = ScannerState::ScanningFloat;
                } else if char.is_numeric() {
                    current_token.push(char);
                } else {
                    return Err(SyntaxError { position: idx });
                }
                // Check close paren
            }
            ScannerState::ExpectingArg => {
                if char == '(' {
                    scanner_state = ScannerState::ExpectingOperand;
                    num_open_parentheses += 1;
                    match current_operand {
                        Some(op) => open_expressions.push(Expression {
                            operand: op,
                            arguments: current_arg_list.clone(),
                        }),
                        None => return Err(SyntaxError { position: idx }),
                    }
                    current_operand = None;
                    current_arg_list.clear();
                } else if char == ')' {
                    num_open_parentheses -= 1;
                    if num_open_parentheses < 0 {
                        return Err(SyntaxError { position: idx });
                    }
                    let expression = match current_operand {
                        Some(op) => Expression {
                            operand: op.clone(),
                            arguments: current_arg_list.clone(),
                        },
                        None => return Err(SyntaxError { position: idx }),
                    };
                    match open_expressions.pop() {
                        Some(mut open_expression) => {
                            current_operand = Some(open_expression.operand);
                            open_expression
                                .arguments
                                .push(ExpressionArg::NestedExpression(expression));
                            current_arg_list = open_expression.arguments;
                        }
                        None => {
                            complete_expressions.push(expression);
                            current_operand = None;
                            current_arg_list.clear();
                        }
                    }
                } else if char.is_numeric() {
                    current_token.push(char);
                    scanner_state = ScannerState::ScanningNumber;
                } else if char == '\'' {
                    scanner_state = ScannerState::ScanningChar;
                } else if !char.is_whitespace() {
                    return Err(SyntaxError { position: idx });
                }
            }
        }
    }

    Ok(complete_expressions)
}
