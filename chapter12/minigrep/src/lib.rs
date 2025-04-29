use std::fs;
use std::error::Error;
use std::env;


//a Simple struct to hold config data
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}   

impl Config {
    // we can use the new function to create a new instance of the struct
    pub fn new(args : &[String]) -> Config {

        if args.len() < 3 {
            panic!("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Config { query, file_path , ignore_case}
    }

    // a build fucntion that builds the Config struct and returns a Result object
    // this is a better way to handle errors, instead of panicking
    pub fn build(args : &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            
            //instead of panicking, lets resturn a Resuslt object
            return Err("Not enough arguments");
            //panic!("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        Ok(Config { query, file_path, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //open the file and read content

    match config.ignore_case {
        true => {
            //println!(" Ignoring case");
            let contents = fs::read_to_string(config.file_path)?;
            for line in search_case_insensitive(&config.query, &contents) {
                println!("{}", line);
            }
        }
        false => {
            //println!(" Not ignoring case");
            let contents = fs::read_to_string(config.file_path)?;
            for line in search(&config.query, &contents) {
                println!("{}", line);
            }
        }
    }
    
    Ok(())
}

pub fn search<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    //vec![]
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            //println!(" Found: {}", line);
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    //vec![]
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            //println!(" Found: {}", line);
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:   
safe, fast, productive.
Pick three.
Duct tape.
";
assert_eq!(search(&query, &contents), vec!["safe, fast, productive."]);

    }
}

    #[test]
    fn case_insensitve(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
";
        assert_eq!(search_case_insensitive(&query, &contents), vec!["Rust:", "Trust me."]);
    }

