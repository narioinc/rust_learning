fn main() {
    let shape1 = Shapes::RECTANGLE(10.0, 20.0);
    let shape2 = Shapes::CIRCLE(10.0);
    let shape3 = Shapes::TRIANGLE(10.0, 20.0);
    let shape4 = Shapes::TRAPEZOID(10.0, 20.0);  

    println!("Area of shape1: {}", shape1.area());
    println!("Area of shape2: {}", shape2.area());  
    println!("Area of shape3: {}", shape3.area());
    println!("Area of shape4: {}", shape4.area());

    let result = square(Some(5));
    match result {
        Some(value) => println!("Square of 5 is: {}", value),
        None => println!("No value provided"),
    }
    let result2 = square(None);
    match result2 {
        Some(value) => println!("Square of None is: {}", value),
        None => println!("No value provided"),
    }
}


//enum to store shapes
enum Shapes{
    RECTANGLE(f32, f32),
    CIRCLE(f32),
    TRIANGLE(f32,f32),
    TRAPEZOID(f32,f32)
}   

//implementation of enums
impl Shapes {
    fn area(&self) -> f32 {
        match self {
            Shapes::RECTANGLE(width, height) => width * height,
            Shapes::CIRCLE(radius) => (3.14 * radius * radius),
            Shapes::TRIANGLE(base, height) => (base * height) / 2.0,

            //catch all special shot hand. We can also use other special keyword which will allow passing the "enum" into the case's logic body
            _ => {
                println!("Not implemented yet");
                0.0
            }
        }
    }
}

fn square(num: Option<i32>) -> Option<i32> {
    match num {
        Some(n) => Some(n * n),
        None => None,
    }
}