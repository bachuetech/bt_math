/// BT MATH is a simple implementation of an expression evaluator that can handle basic arithmetic operations, parentheses, and some mathematical functions
/// BT MATH provides a way to evaluate mathematical expressions using RPN (Reverse Polish Notation)
/// The RPN evaluator is implemented in two parts: parsing and evaluation.


use regex::Regex;
use std::collections::VecDeque;
use std::fmt;
use std::str::FromStr;

/// Enum Token represents different types of tokens in the RPN expression:
/// Number represents a number value, which is stored as a floating-point number (f64).
/// Operator represents an operator (e.g., +, -, *, /) and stores the operator as a string.
/// Function represents a mathematical function (e.g., sin, cos, tan) and stores the function name as a string.
/// LeftParen and RightParen represent parentheses, which are used to group expressions.
#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Operator(String),
    Function(String),
    LeftParen,
    RightParen,
}

/// Implementing Display trait for Token enum. useful for debug
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Operator(op) => write!(f, "{}", op),
            Token::Function(func) => write!(f, "{}", func),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
        }
    }
}

impl Token {
    ///returns an integer that represents how strongly an operator or function binds to its operands. Operators have higher precedence than functions and multiplication/division have higher precedence than addition/subtraction
    fn precedence(&self) -> i32 {
        match self {
            Token::Operator(op) => match op.as_str() {
                "+" | "-" => 1,
                "*" | "/" => 2,
                "^" => 3,
                _ => 0,
            },
            Token::Function(_) => 4,
            _ => 0,
        }
    }
}

/// Public function that evaluate a mathematical expression with a combination of basic arithmetic operations and mathematical functions
/// It strips spaces, tokenizes the input string, converts it to RPN, and then evaluates the RPN expression.
/// Returns the results as a Float
pub fn evaluate_expression(expression: &str) -> Result<f64, String> {
    let expression = expression.replace(" ", ""); // Remove spaces
    let tokens = tokenize(&expression)?;
    let rpn = to_rpn(&tokens)?;
    evaluate_rpn(&rpn)
}

/// Tokenize the input expression
/// Uses a regular expression to break down the input string into numbers, operators, parentheses, and function names
fn tokenize(expression: &str) -> Result<Vec<Token>, String> {
    let re = Regex::new(r"(\d+\.?\d*|\+|\-|\*|\/|\^|\(|\)|log|ln|log2|exp|sin|cos|tan|asin|acos|atan|abs|sqrt)")
        .unwrap();
    let mut tokens = Vec::new();

    for cap in re.captures_iter(expression) {
        let token = &cap[0];
        if let Ok(number) = f64::from_str(token) {
            tokens.push(Token::Number(number));
        } else if token == "+" || token == "-" || token == "*" || token == "/" || token == "^" {
            tokens.push(Token::Operator(token.to_string()));
        } else if token == "(" {
            tokens.push(Token::LeftParen);
        } else if token == ")" {
            tokens.push(Token::RightParen);
        } else {
            tokens.push(Token::Function(token.to_string()));
        }
    }

    Ok(tokens)
}

/// Convert infix notation to Reverse Polish Notation (RPN) using the Shunting Yard algorithm
/// It uses a stack to temporarily hold operators until they can be placed behind their operands according to their precedence.
fn to_rpn(tokens: &[Token]) -> Result<Vec<Token>, String> {
    let mut output = Vec::new();
    let mut operators = VecDeque::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token.clone()),
            //Token::Function(_) => operators.push_back(token.clone()),
            Token::LeftParen => operators.push_back(Token::LeftParen),
            Token::RightParen => {
                while let Some(op) = operators.pop_back() {
                    match op {
                        Token::LeftParen => break,
                        _ => output.push(op),
                    }
                }
            }
            Token::Operator(_) | Token::Function(_) => {
                while let Some(op) = operators.back() {
                    let _p_token = Token::Operator("^".to_string());
                    if matches!(op, Token::Operator(_) | Token::Function(_))
                        && (op.precedence() > token.precedence()
                            || (op.precedence() == token.precedence() && matches!(token, _p_token)))
                    {
                        output.push(operators.pop_back().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push_back(token.clone());
            }
        }
        //println!("output {:?}, operators: {:?}", output, operators);
    }

    while let Some(op) = operators.pop_back() {
        output.push(op);
    }

    Ok(output)
}

/// Evaluate the expression in Reverse Polish Notation (RPN)
/// Numbers are pushed onto the stack, and when an operator is encountered, it pops two numbers from the stack, applies the operation, and pushes the result back onto the stack. Functions also pop arguments from the stack and apply mathematical operations accordingly.
fn evaluate_rpn(rpn: &[Token]) -> Result<f64, String> {
    let mut stack = VecDeque::new();

    //println!("RPN: {:?}",&rpn);
    for token in rpn {
        match token {
            Token::Number(value) => {
                stack.push_back(*value);
            }
            Token::Operator(op) => {
                let b = stack
                    .pop_back()
                    .ok_or("Invalid expression: not enough values for operator")?;
                let a = stack
                    .pop_back()
                    .ok_or("Invalid expression: not enough values for operator")?;
                let result = match op.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    "^" => a.powf(b),
                    _ => panic!("Unknown operator"),
                };
                stack.push_back(result);
            }
            Token::Function(func) => {
                let arg = stack
                    .pop_back()
                    .ok_or("Invalid expression: not enough values for function")?;
                let result = match func.as_str() {
                    "sin" => arg.sin(),
                    "cos" => arg.cos(),
                    "tan" => arg.tan(),
                    "asin" => arg.asin(),
                    "acos" => arg.acos(),
                    "atan" => arg.atan(),
                    "exp" => arg.exp(),
                    "ln" => arg.ln(),
                    "log" => arg.log10(),
                    "log2" => arg.log2(),
                    "abs" => arg.abs(),
                    "sqrt" => arg.sqrt(),
                    _ => panic!("Unknown function"),
                };
                stack.push_back(result);
            }
            _ => return Err(format!("Invalid token: {:?}", token)),
        }
    }

    stack
        .pop_back()
        .ok_or("Invalid expression: no result on stack".to_owned())
}
