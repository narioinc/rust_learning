fn main() {
   let string: String = String::from("hello world");


   let result =  first_word(&string);
   println!("The sentence is: {}", string);
   println!("The first word is: {}", result);

   let string2 = "narioinc world";
   let result2 =  first_word2(&string2);
   println!("The sentence is: {}", string);
   println!("The first word is: {}", result2);

}


//function that show slices in the works. This fucntion returns the first word of a string
fn first_word(str: &String) -> String {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //println!("The first word is: {}", &str[0..i]);
            return String::from(&str[0..i]);
        }

        
    }
    
    return String::from(&str[..]);
}

//much better bversion of first_worf that accepts str and returns &str

fn first_word2(str: &str) -> &str {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //println!("The first word is: {}", &str[0..i]);
            return &str[0..i];
        }

        
    }
    
    &str[..]
}