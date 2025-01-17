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
    let expression = "-1*1*-(2*3)";
    let expected_result = 6.00;
    assert_eq!(evaluate_expression(expression).unwrap(), expected_result);
}
#[test]
fn test_evaluate_log2() {
    let expression = "2 + log2(8)";
    let expected_result = 5.0;
    assert_eq!(evaluate_expression(expression).unwrap(), expected_result);
}

#[test]
fn test_evaluate_pi() {
    let expression = "2 + PI";
    let expected_result = 5.141592653589793;
    assert_eq!(evaluate_expression(expression).unwrap(), expected_result);
}

#[test]
fn test_evaluate_e() {
    let expression = "2 * E";
    let expected_result = 5.43656365691809;
    assert_eq!(evaluate_expression(expression).unwrap(), expected_result);
}

#[test]
fn test_evaluate_abs() {
    let expression = "abs(-sin(45) * tan(45) * cos(45))";
    let expected_result = 0.7240368080645851;
    assert_eq!(evaluate_expression(expression).unwrap(), expected_result);
}

#[test]
fn test_evaluate_arcs() {
    let expression = "asin(-0.98803162)+acos(-0.98803162)+atan(-0.98803162)";
    let expected_result = 0.7914183067858805;
    assert_eq!(evaluate_expression(expression).unwrap(), expected_result);
}

#[test]
fn test_evaluate_invalid_funct() {
    let expression = "wxyz(-0.98803162)";
    let expected_result = -0.98803162;
    assert_eq!(evaluate_expression(expression).unwrap(), expected_result);    
}

#[test]
fn test_invalid_funct_param() {
    let expression = "atan(hellow) * acos(-0.988031))";
    assert!(evaluate_expression(expression).is_err());
}