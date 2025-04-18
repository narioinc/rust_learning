struct Point<T> {
    x: T,
    y: T,
}
fn main() {
    println!("Hello, world!");
}


//this fucntion wont compile since the compiler doesnt know which reference to return and hence needs 
// the programmer to specify the lifetime of the reference
//fn fucn1(point1: &Point<i32>, point2: &Point<i32>) -> &Point<i32> {
    //point1.x + point2.x
//}

//this function will compile since the compiler knows which reference to return and hence needs
// the programmer to specify the lifetime of the reference  
fn fucn2<'a>(point1: &'a Point<i32>, point2: &'a Point<i32>) -> &'a Point<i32> {
    if point1.x > point2.x {
        point1
    } else {
        point2
    }
}

//take note of lifetine elison rules to ensure that you know when to explicitly mention lifetimes and
// when not to