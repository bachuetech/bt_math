use bt_math::evaluate_expression;

#[test]
fn test_basic_arithmetic(){
    assert_eq!(evaluate_expression("1 + 2 - 3 / 10 ").unwrap(), 2.7);
}

#[test]
fn test_evaluate_with_numbers_and_operators() {
    let expression = "2 + 3 * 4";
    let expected_result = 14.0;
    assert_eq!(evaluate_expression(expression).unwrap(), expected_result);
}
#[test]
fn test_evaluate_with_nested_parentheses() {
    let expression = "(2 + 3) * 4";
    let expected_result = 20.0;
    assert_eq!(evaluate_expression(expression).unwrap(), expected_result);
}
// Test cases for evaluating expressions with functions
#[test]
fn test_evaluate_with_functions() {
    let expression = "cos(0)";
    let expected_result = 1.0;
    assert_eq!(evaluate_expression(expression).unwrap(), expected_result);
}
#[test]
fn test_evaluate_with_exp_function() {
    let expression = "exp(2.0)";
    let expected_result = 7.38905609893065; // Approximately 7.38906
    assert_eq!(evaluate_expression(expression).unwrap(), expected_result);
}
#[test]
fn test_evaluate_with_log10_function() {
    let expression = "log10(100)";
    let expected_result = f64::log10(f64::powi(10.0, 2)); // Exactly 2
    assert_eq!(evaluate_expression(expression).unwrap(), expected_result);
}
// Test cases for handling invalid expressions
#[test]
fn test_evaluate_with_invalid_expression() {
    let expression = "abc";
    assert!(evaluate_expression(expression).is_err());
}
#[test]
fn test_evaluate_with_missing_parentheses() {
    let expression = "(2 + 3";
    assert!(evaluate_expression(expression).is_err());
}
// Test cases for edge cases
#[test]
fn test_evaluate_div_zero() {
    let expression = "0 / 0";
    assert!(evaluate_expression(expression).unwrap().is_nan());
}
#[test]
fn test_evaluate_with_neg_one() {
    let expression = "log(-1)";
    assert!(evaluate_expression(expression).is_err());
}