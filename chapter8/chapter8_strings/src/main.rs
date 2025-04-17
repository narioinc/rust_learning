fn main() {
    let str = "Hello, naresh";
    let mut str2 = str.to_string();
    println!(" String :: {}", str2);

    str2.push_str(" from rust");
    println!(" String :: {}", str2);    

    let str3 = format!("{str} :: {str2}");
    println!(" String :: {}", str3);
    println!(" String :: {}", str2);

    match str2.get(..1) {
        Some(first) => println!("First character is {}", first),
        None => println!("There is no first character"),
    }
    
    //iterate over strings using chars() or bytes()
    for i in str.chars() {
        println!("Character: {}", i);
    }
}
