fn main() {

// String type    
let mut string = String::from("hello");
string.push_str(", world!");
println!("{}", string);

//heap variables 'move'

let s1 = String::from("hello");
let s2 = s1;
println!("{}", s2); // This will work because s2 is a valid reference to the String
// println!("{}", s1); // This will cause an error because s1 is no longer valid bccause rust has already invalidated s1 with the = operator
// consider all assignments on objects as moves unless otherwise specified

//Clone to do deep copy instead of move

let s3 = s2.clone();
println!("{}", s3); // This will work because s3 is a valid reference to the String 
println!("{}", s2); // This will not cause an error because s2 is no longer valid


//Example where a fuction call moves the owenership for the variable. 
let s1 = String::from("hello");
//say_hi(s1);
println!("{}", s1); // This will cause an error because s1 is no longer valid IF say_Hi() is called


//example of a function call with reference
let s: String = String::from("hello");
let len  = count_length(&s); // pass by reference
println!("The length of '{}' is {}.", s, len); // This will work because s is still valid


}

//function that takes ownership of the variable
fn say_hi(name: String) {
    println!("Hello, {}!", name);
}

//function that uses reference.
fn count_length(s: &String) -> usize {
    s.len()
}
