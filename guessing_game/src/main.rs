use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("guess the number !");
   

    let secret_number = rand::rng().random_range(1..100);
    
    // comment out the secret number ot actually play the game !! :)
    //println!("The secret number is: {}", secret_number);

    loop{
    println!("Please input your guess.");
    
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    //expect allows you to catch the Err object from Result
    //let guess: u32 = guess.trim().parse().expect("please type a number !");

    //Here, we can use the OK and Err from Result to create alternate code paths
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
            continue;
        },
    };

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }    
}
    //println!("You guessed: {}", guess);

    
}
