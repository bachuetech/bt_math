/// BT MATH is a simple implementation of an expression evaluator that can handle basic arithmetic operations, parentheses, and some mathematical functions
/// that provide a way to evaluate mathematical expressions using RPN (Reverse Polish Notation) implemented in two parts: parsing and evaluation.
/// Usage:
/// let expression = "2 + 3 * 4";
/// let f = evaluate_expression(expression).unwrap();

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

    fn to_string(&self) -> String {
        match self {
            Token::Number(num) => num.to_string(),
            Token::Operator(op) => op.clone(),
            Token::Function(func) => func.clone(),
            Token::LeftParen => String::from("("),
            Token::RightParen => String::from(")"),
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
    let rexpression = Regex::new(r"(\d+\.?\d*|\+|\-|\*|\/|\^|\(|\)|ln|log2|exp|asin|acos|atan|sin|cos|tan|abs|sqrt|log10|PI|E)")
        .unwrap();
    let mut tokens = Vec::new();

    let mut iter = rexpression.captures_iter(expression);
    //for cap in rexpression.captures_iter(expression) { //Change While 
    while let Some(cap) = iter.next(){
        let token = &cap[0];
        if let Ok(number) = f64::from_str(token) {
            tokens.push(Token::Number(number));
        }else if token == "-" {
                match tokens.last(){
                    None => {
                        tokens.push(Token::Number(-1.00));
                        tokens.push(Token::Operator("*".to_owned()));
                    },
                    Some(c) => {
                        if c.to_string() == "(" || is_operator(&c.to_string())  {
                            tokens.push(Token::Number(-1.00));
                            tokens.push(Token::Operator("*".to_owned()));
                        }else{
                            tokens.push(Token::Operator(token.to_string()));
                        }
                    },
                }
        } else if token == "+" || token == "*" || token == "/" || token == "^" {
            tokens.push(Token::Operator(token.to_string()));
        } else if token == "(" {
            tokens.push(Token::LeftParen);
        } else if token == ")" {
            tokens.push(Token::RightParen);
        } else if let Ok(number) = evaluate_const(&token.to_string()){
            tokens.push(Token::Number(number));
        }else{
            tokens.push(Token::Function(token.to_string()));
        }
    }

    Ok(tokens)
}

fn is_operator(c: &str) -> bool {
    c == "+" || c == "-" || c == "*" || c == "/" || c == "^"
}

/// Evaluate constants and returns its f64 value or same received strings as error.
fn evaluate_const(p_const: &String) -> Result<f64, &str>{
    match p_const.as_str(){
        "PI" => return Ok(std::f64::consts::PI),
        "E"  => return Ok(std::f64::consts::E),
        _    => return Err(p_const)
    };
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
    for token in rpn {
        match token {
            Token::Number(value) => {
                stack.push_back(*value);
            }
            Token::Operator(op) => {
                let b = stack
                    .pop_back()
                    .ok_or("Invalid expression: not enough values for operator (b)")?;
                let a = stack
                    .pop_back()
                    .ok_or("Invalid expression: not enough values for operator (a)")?;
                let result = match op.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    "^" => a.powf(b),
                    _ =>return Err(format!("Unknown operator {:?}", token)), // panic!("Unknown operator"),
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
                    "log10" => arg.log10(),
                    _ => return Err(format!("Unknown function: {:?}", token)) //panic!("Unknown function"),
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