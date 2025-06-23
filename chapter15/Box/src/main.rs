use crate::List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    
    //create a boxed value
    let x: Box<i32> = Box::new(5);  
    println!("x: {}", x);
    
    //println!("Hello, world!");
    //create a boxed list

    // Here the rust compile time will nt allow since the List is ending up in a recuesivce call with infinte size
    // compiler cannot determine what size to allocate on the heap.

    //let list = List::Cons(1, List::Cons(2, List::Cons(3, Nil)));

    //this line fixes the issue as now we use Box to create a pointer to the list that is allocated in the heap
    // the Box pointer has a finite size that the compiler can determine
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(Nil))))));
    println!("list: {:?}", list);

    //using a custom box type defined by us
    let x: i32 = 5;
    let y = MyBox::new(x);

    println!("x: {}", x);

    //this call still works because rust internall calls *(y.deref()) where the deref will be called with the MyBox type
    // and the deref will return a reference to the value inside the MyBox
    println!("y: {}", *y);
}   

//enable debug
#[derive(Debug)]
enum List {

    //this wont work due to the recursive nature of the Listy
    //Cons(i32, List),
    
    //lets use Box to solve this problem
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;    

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T>  MyBox<T> {

    fn new(value: T) -> MyBox<T> {
        MyBox( value )
    }
}