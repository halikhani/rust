use std::env;
use std::fs;
use std::process;
use std::error::Error;

use minigrep::search;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}

struct Config {
    query: String,
    file_path: String,
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // println!("With text:\n{contents}");
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

impl Config {
    fn build(arg: &[String]) -> Result<Config, &'static str> {
        if arg.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = arg[1].clone();
        let file_path = arg[2].clone();
        Ok(Config { query, file_path })
    }
}

