use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Config {
    query: String,
    filename: String,
}

fn main() {
    // .collect -> iterator to vector
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let config = parse_config(&args);

        println!("Searching for {}", config.query);
        println!("In file {}", config.filename);

        // ? -> handle error
        let mut f = File::open(config.filename).expect("file not found");
        let mut content = String::new();
        f.read_to_string(&mut content).expect("failed to read file");

        println!("Found: \n{}", content);
    } else {
        println!("Invalid number of arguments, need query and filename");
    }
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
