use std::env;
use std::vec::Vec;
use std::fs;
use std::error::Error;

use minigrep::Config;

fn main() {

    //println!(" {} ", env::args().len());
    //println!(" {:#?} ", env::args());

    /*for arg in env::args() {
        println!(" {:#?} ", arg);
    }*/

    // we can also ask the debugger to print the whole vector
    //dbg!(&args);
    
    //process the variables
    //let searchString = &args[1];
    //let file_path = &args[2];
    //println!(" Searching for string \"{}\" in file {}", searchString, file_path);

    //open the file and read content
    //let contents = std::fs::read_to_string(file_path)
    //    .expect("Something went wrong reading the file");
    //println!(" File content: \n {}", contents);

    //Call the new() constructor
    //let config: Config = Config::new(&args);

    //access args
    let args: Vec<String> = env::args().collect();
    //println!(" {:?} ", args.get(1));

    //building a config using the build helper
    let config2: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    let e = minigrep::run(config2);
    match e {
        Ok(_) => println!("Success"),
        Err(e) => {
            println!("Application error: {}", e);
            std::process::exit(1);
        }
    }
}





//Function to parse args - this is before we create a struct to parse the prgramme arguments
fn parse_config(args: &[String]) -> Config{

    // borrowing rules apply here. Unless you want to take the burden of telling the compiler of the lifetimes,
    // we can just use clone() to create a full copy of the string
    // without this, the Config struct would have taken over the ownership of the string 
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config {query, file_path, ignore_case: false}
}
