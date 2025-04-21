pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn factorial(n: i64) -> i64 {
    if n < 0 {
        panic!("Negative numbers are not allowed");
        //return 0;
    }

    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

pub fn factorial_result(n: i64) -> Result<i64, String> {
    if n < 0 {
        Err("Negative numbers are not allowed".to_string())
    } else {
        Ok(factorial(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //a smimple assert equlas where the two params to the asset_eq() call should be equated for the test to pass
    #[test]
    fn adder_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    //a general asset with a bool expression inside it. 
    #[test]
    fn test_fib(){
        let result = factorial(5);
        assert!(result == 120, 
            "Expected 5! to be 120, but got {}", result);
    }


    //showing an example of should panic test cases - here we are testing if the factorial fucntion panic when it find s a 
    // negative number
    #[test]
    #[should_panic]
    fn test_neg() {
        factorial(-1);
    }

    //another example of should panic, but here, we also expect the correct expected log
    #[test]
    #[should_panic(expected = "Negative numbers are not allowed")]
    fn test_neg2() {
        factorial(-1);
    }

    //another test that uses Result object 
    #[test]
    fn test_result() -> Result<(), String> {
        let result = factorial(5);
        if result == 120 {
            Ok(())
        } else {
            Err(format!("Expected 5! to be 120, but got {}", result))
        }
    }

     //another test that uses Result object. see the usage of the ? operator to immedietely return the error if it is found
     //this is a more idiomatic way of writing the test, as it uses the Result object to handle errors

     #[test]
     fn test_result2() -> Result<(), String> {
         let result = factorial_result(5)?;
         if result == 120 {
             Ok(())
         } else {
             Err(format!("Expected 5! to be 120, but got {}", result))
         }
     }

     #[test]
     #[ignore]
        fn test_ignore() {

        }
}


//SOME NOTES

// Tests can be run in a sync fashions by specific test-threads as 1
// Tests can be run by giving their name. here the name is used as patter and runs the tests that match the pattern (contains)
// uyou can ignore tests, by using the #[ignore] attribute. this is useful for tests that are not ready yet or are not needed to be run all the time