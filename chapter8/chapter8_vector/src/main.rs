use std::vec::Vec;
use rand::prelude::*;

fn main() {

   //mutable vector of type i32
   let mut vector : Vec<i32> = Vec::new();

   let mut rng = rand::rng();


   //rust shorthand, rust infers the type is i32 automatically
   let mut vector2 = vec![0,2,4,8];
   
   for i in 0..10 {
       vector.push(i+1);
   }

   //reads the contents of the vector
   for (index, value) in vector.iter().enumerate() {
       println!("Value: {}, {}", index, value);
   }

   println!("Vector: {:?}", vector);

   //mutate the vector in a loop
   for i in &mut vector {
         *i = *i + 1;
   }

   println!("Vector: {:?}", vector);
   println!("Vector2: {:?}", vector2);

   //get() returns an Option<T> where T is the type of the vector
   //if the index is out of bounds, it returns None
   let third = vector2.get(2);
   match third {
       Some(third) => println!("The third element is {}", third),
       None => println!("There is no third element"),
   }


   //Creating vectors of having different types
   #[derive(Debug)]
   enum Shape {
      RECTANGLE(i32, i32),
      CIRCLE(i32),
      TRIANGLE(i32, i32, i32),
   } 

   let mut shape_vec = vec![Shape::RECTANGLE(10,10)];

   shape_vec.push(Shape::CIRCLE(10));
   shape_vec.push(Shape::TRIANGLE(10,12,13));

   let rand_index = rng.random_range(0..3);
   match shape_vec.get(rand_index){
        Some(value) => {
            match value {
                Shape::RECTANGLE(x,y) => println!("Rectangle with width {} and height {}", x, y),
                _ => println!("Not a rectangle"),
            }
        },
        None => ()

   };

   println!("vector : {:?}", shape_vec);

   let vector3 = &vector[2..6];
   println!("vector3 {:?}", vector3);
   
}
