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

#[derive(Clone, Debug)]
enum Operand {
    Add,
    Subtract,
    Multiply,
    Divide,
    AbsoluteValue,
    Not,
    And,
    Or,
    If,
    Equal,
    LessThan,
    GreaterThan,
}

#[derive(Clone, Debug)]
enum ExpressionArg {
    Int(i32),
    Float(f64),
    Char(char),
    NestedExpression(Expression),
}

#[derive(Clone, Debug)]
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
        "abs" => Ok(Operand::AbsoluteValue),
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

pub fn parse(input: String) -> Result<Vec<Expression>, SyntaxError> {
    let mut complete_expressions: Vec<Expression> = Vec::new();
    let mut open_expressions: Vec<Expression> = Vec::new();

    let mut current_operand = None;
    let mut current_arg_list = Vec::new();

    let mut scanner_state = ScannerState::ExpectingExpressionStart;
    let mut current_token = "".to_string();
    let mut scanned_char: Option<char> = None;
    let mut num_open_parentheses = 0;
    for (idx, char) in input.as_str().chars().enumerate() {
        match (char, &scanner_state) {
            ('(', ScannerState::ExpectingExpressionStart) => {
                scanner_state = ScannerState::ExpectingOperand;
                num_open_parentheses += 1;
            }
            (
                'a'..='z' | 'A'..='Z' | '+' | '-' | '*' | '/' | '=' | '<' | '>',
                ScannerState::ExpectingOperand,
            ) => {
                scanner_state = ScannerState::ScanningOperand;
                current_token.push(char);
            }
            (
                ' ' | '\x09'..='\x0d',
                ScannerState::ExpectingExpressionStart
                | ScannerState::ExpectingOperand
                | ScannerState::ExpectingArg,
            ) => {}
            ('\'', ScannerState::ScanningChar) => match scanned_char {
                Some(ch) => {
                    current_arg_list.push(ExpressionArg::Char(ch));
                    scanner_state = ScannerState::ExpectingArg;
                }
                None => return Err(SyntaxError { position: idx }),
            },
            (_, ScannerState::ScanningChar) => match scanned_char {
                Some(_ch) => return Err(SyntaxError { position: idx }),
                None => scanned_char = Some(char),
            },
            ('a'..='z' | 'A'..='Z' | '_' | '-', ScannerState::ScanningOperand) => {
                current_token.push(char);
            }
            ('0'..='9', ScannerState::ScanningNumber | ScannerState::ScanningFloat) => {
                current_token.push(char);
            }
            (')' | ' ' | '\x09'..='\x0d', ScannerState::ScanningOperand) => {
                let operand = get_operand(&mut current_token, idx)?;
                current_operand = Some(operand);
                current_token.clear();
                scanner_state = ScannerState::ExpectingArg;
            }
            (')' | ' ' | '\x09'..='\x0d', ScannerState::ScanningFloat) => {
                let parsed_float = current_token.clone().parse::<f64>();
                match parsed_float {
                    Ok(float_value) => {
                        current_arg_list.push(ExpressionArg::Float(float_value));
                    }
                    Err(_) => return Err(SyntaxError { position: idx }),
                }
                current_token.clear();
                scanner_state = ScannerState::ExpectingArg;
            }
            (')' | ' ' | '\x09'..='\x0d', ScannerState::ScanningNumber) => {
                let parsed_int = current_token.clone().parse::<i32>().unwrap();
                current_arg_list.push(ExpressionArg::Int(parsed_int));
                current_token.clear();
                scanner_state = ScannerState::ExpectingArg;
            }
            ('.', ScannerState::ScanningNumber) => {
                current_token.push(char);
                scanner_state = ScannerState::ScanningFloat;
            }
            ('(', ScannerState::ExpectingArg) => {
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
            }
            ('0'..='9', ScannerState::ExpectingArg) => {
                current_token.push(char);
                scanner_state = ScannerState::ScanningNumber;
            }
            ('\'', ScannerState::ExpectingArg) => {
                scanner_state = ScannerState::ScanningChar;
            }
            (')', ScannerState::ExpectingArg) => {}
            _ => {
                return Err(SyntaxError { position: idx });
            }
        }

        if let (
            ')',
            ScannerState::ScanningFloat
            | ScannerState::ScanningNumber
            | ScannerState::ExpectingArg
            | ScannerState::ScanningOperand,
        ) = (char, &scanner_state)
        {
            num_open_parentheses -= 1;
            if num_open_parentheses < 0 {
                return Err(SyntaxError { position: idx });
            }
            let expression = match current_operand {
                Some(op) => Expression {
                    operand: op.clone(),
                    arguments: current_arg_list.clone(),
                },
                None => unreachable!(),
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
        }
    }

    Ok(complete_expressions)
}
