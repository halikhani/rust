use std::env;
use std::fs;
use std::process;
use std::error::Error;

use minigrep::{search, search_case_insensitive};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    // NOTE: env::args() returns an iterator, so we can just pass ownership of the iterator to the build function
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

impl Config {
    // fn build(arg: &[String]) -> Result<Config, &'static str> {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // input signature means args can be any type that implements the Iterator trait and returns an Item of type String
        // if arg.len() < 3 {
        //     return Err("Not enough arguments");
        // }
        // let query = arg[1].clone();
        // let file_path = arg[2].clone();

        // let ignore_case = env::var("IGNORE_CASE").is_ok(); // is_ok() to see if the environment variable is set (returns bool)
        // Ok(Config { query, file_path, ignore_case })

        args.next(); // consumes the first value and discards it (the program name)
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

