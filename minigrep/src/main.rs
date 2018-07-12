use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

fn main() {
    // .collect -> iterator to vector
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // ? -> handle error
    let mut f = File::open(config.filename).expect("file not found");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("failed to read file");

    println!("Found: \n{}", content);
}
