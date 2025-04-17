use std::collections::HashMap;

#[derive(Debug)]
enum Shape {
    CIRCLE(i32),
    RECTANGLE(i32, i32),
    TRIANGLE(i32, i32, i32),
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, Shape::CIRCLE(5));
    map.insert(2, Shape::RECTANGLE(5, 10));
    map.insert(3, Shape::TRIANGLE(3, 4, 5));
    map.insert(4, Shape::CIRCLE(10));

    for (key, shape) in &map {
        println!("Key: {}", key);
        match shape {
            Shape::CIRCLE(radius) => println!("Circle with radius: {}", radius),
            Shape::RECTANGLE(width, height) => println!("Rectangle with width: {} and height: {}", width, height),
            Shape::TRIANGLE(a, b, c) => println!("Triangle with sides: {}, {}, {}", a, b, c),
        }
    }

    // using get to get a paritcualr key
    let shape = map.get(&2);
    println!("Shape with key 2: {:?}", shape);

    //updating a key only if it is not present
    map.entry(2).or_insert(Shape::CIRCLE(20));
    map.entry(5).or_insert(Shape::CIRCLE(20));

    println!("After updating:");
    println!("{:?}", map);
}
