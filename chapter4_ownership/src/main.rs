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


}
