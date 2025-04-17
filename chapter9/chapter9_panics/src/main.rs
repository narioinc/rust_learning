use std::vec::Vec;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //panic!("Die, die infidel !!! muhahahahah !!!");

  
    let vector = vec![1, 2, 3, 4, 5];
    //run this code with RUST_BACKTRACE=1 to see the full panic trace.
    //vector[10];

    // a line like this can have a panic . File::open returns a result object 
    // the OK will contain the file handle while Err will contain the error panic details. 
    // use match
    //let file = File::open("hello.txt");

    /*let file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };*/

    //Fiel error extended to check exact error cause
    /*t file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => {
            //nic!("There was a problem opening the file: {:?}", error);
            match error.kind() {
                ErrorKind::NotFound => {
                    panic!("File not found: {:?}", error);
                }
                ErrorKind::PermissionDenied => {
                    panic!("Permission denied: {:?}", error);
                }
                _ => {
                    panic!("There was a problem opening the file: {:?}", error);
                }
            }
        }
    };*/

    //using the unwrap shorthand. wont be able to take advantage of precise handling
    //unwrap chooses the Result OK(file) and returns it. If the result is an error, it panics.
    //let file =  File::open("hello.txt").unwrap();


    //expect() same as unwrap and returns OK. we can have a string for the error optionally which makes it easier
    //to give an exact cause for the panic in the logs. Helps debug better
    //let file = File::open("hello.txt").expect("Failed to open hello.txt");

    //the ? operator allows returning the error to the caller. It is a shorthand for the match statement.
    //The ? operator can only be used in functions that return a Result or an Option.
    let file = File::open("hello.txt")?;

    /*  As part of the chapter, it is good to note that it is better to wrap logic of
    //  checking correctness of an user inputs of configs or such inside a type and then
    // return the type or raise a panic. this ensures that the interface consumers exactly know how to 
    // interact with the type and handle errors based on how you document the type 
    */  

}
