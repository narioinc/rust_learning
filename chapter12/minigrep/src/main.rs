use std::env;
use std::vec::Vec;
use std::fs;

fn main() {

    println!(" {} ", env::args().len());
    println!(" {:#?} ", env::args());

    for arg in env::args() {
        println!(" {:#?} ", arg);
    }

    //access args
    let args: Vec<String> = env::args().collect();
    println!(" {:?} ", args.get(1));

    // we can also ask the debugger to print the whole vector
    dbg!(&args);

    let config: Config = parse_config(&args);

    //process the variables
    let searchString = &args[1];
    let fileName = &args[2];
    
    println!(" Searching for string \"{}\" in file {}", searchString, fileName);

    //open the file and read content
    let contents = std::fs::read_to_string(fileName)
        .expect("Something went wrong reading the file");
    println!(" File content: \n {}", contents);

}

struct Config {
    query: String,
    filename: String,
}   

fn parse_config(args: &[String]) -> Config{

    // borrowing rules apply here. Unless you want to take the burden of telling the compiler of the lifetimes,
    // we can just use clone() to create a full copy of the string
    //without this, the Config struct would have taken over the ownership of the string 
    let query = args[1].clone();
    let filename = args[2].clone();
    Config {query, filename}
}
