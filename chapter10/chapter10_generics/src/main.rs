use std::vec::Vec;
use std::cmp::PartialOrd;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;

fn main() {
    //println!("Hello, world!");
let vector = vec![1, 2, 3, 4, 5];
    let largest1 = largest(&vector);
    println!("largest: {}", largest1);

    let vector2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let largest2 = largest(&vector2);
    println!("largest2: {}", largest2);

    let vector3 = vec!['a', 'e', 'x', 'd', 'f'];
    let largest3 = largest(&vector3);
    println!("largest3: {}", largest3);

    let coords1: Coords<f32> = Coords {
        x: 0.2,
        y: 0.1,
        name: String::from("p1"),
    };

    println!("Dist for point {}: {}", &coords1.name, coords1.dist())

}

//largest int
fn largest_int(numbers: &Vec<i32>) -> &i32 {

    let mut largest = &numbers[0];
    for i in numbers {
        println!("i: {}", i);
        if i > largest {
            largest = i;
        }
    }

    largest
}

//largest char
fn largest_char(chars: &Vec<char>) -> &char {

    let mut largest = &chars[0];
    for c in chars {
        println!("c: {}", c);
        if c > largest {
            largest = c;
        }
    }

    largest
}

//largest with a template example

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    
    let mut largest = &list[0];

    for i in list {
        //println!("i: {}", i);
        if i > largest {
            largest = i;
        }
    }

    largest
}

// struct with generic type: single generic
struct Coords<T> {
    x: T,
    y: T,
    name: String
}

// struct with generic type: multiple generics
struct Coords_adv<T, U> {
    x: T,
    y: U,
    name: String
}

impl<T> Coords<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

//only coords of float type will have dist function
impl Coords<f32>{
    fn dist(&self) ->  f32 {
        let x = self.x;
        let y = self.y;
        (x * x + y * y).sqrt()

    }
}



