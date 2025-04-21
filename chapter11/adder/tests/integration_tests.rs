use adder;

#[test]
fn int_test_factorial(){
    let result = adder::factorial(5);
    assert_eq!(result, 120, 
        "Expected 5! to be 120, but got {}", result);
}

#[test]
fn int_test_factorial_neg(){
    let result = adder::factorial_result(-1);
    assert_eq!(result, Err("Negative numbers are not allowed".to_string()));
}