fn main() {
    let  mut x = 3;

    println!("the value of x is : {}", x);
    x = 6;
    println!("the value of x is : {}", x);

    //shadowing

    let x = 5;

    let x = x+1;

    {
        let x = x*6;
        println!("x is :{} ", x);
        
    }

    println!("x is :{} ", x);

    //Data types
    let number: u32 = "42".trim().parse().expect("not a number");
    println!("number is : {}", number);

    //Math operations

    println!("add {}, {}", 5+6, 5.5+6.1);
    println!("sub {}, {}", 5-6, 5.5-6.1);
    println!("mul {}, {}", 5*6, 5.5*6.1);   
    println!("div {}, {}", 5/6, 5.5/6.1);

    //creating a 32 bit float number and seeing difference in the value when dividing 
    let ans: f32 = 5.5/6.1;
    println!("division of f32: {}", ans);

    //Tuples

    let value : (i32, f64, char) = (5, 6.1, 'a');
    println!("value is : {:?}", value);
    let (x, y, z) = value;
    println!("x is : {}, y is : {}, z is : {}", x, y, z);
    println!("value is : {} {} {}", value.0, value.1, value.2);

    //Arrays
    let arr = [1, 2, 3, 4, 5];
    println!("arr is : {:?}", arr);

    //specific index
    println!("arr[0] is : {}", arr[0]);

    let arr2: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("arr2 is : {:?}", arr2);
    println!("arr2[0] is : {}", arr2[0]);

    second_function();

    third_function(5);

    multi_params(5, String::from("hello"));

    let result = add(5,6);
    println!("result is : {}", result);

    //Conditionals
    let x: i32 = 5;
    if x > 10 {
        println!("x is greater than 10");
    } else if x < 10 {
        println!("x is less than 10");
    } else {
        println!("x is equal to 10");
    }

    let is_gt_ten: bool = check_num(11);
    println!("{} ", is_gt_ten);

}

// simple function
fn second_function() {
    println!("this is second function");
}   

//function with parameters
fn third_function(x: i32) {
    println!("this is third function with parameter {}", x);
}

//function with multiple parameters
fn multi_params(x: i32, name: String) {
    println!("{} {}", x, name);
}

//function with return value
fn add(x: i32, y: i32) -> i32 {
    x + y
}

//function with return value and multiple parameters
fn check_num(x: i32) -> bool {
    if x > 10 {
        true
    } else {
        false
    }
}
