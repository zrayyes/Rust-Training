extern crate minigrep;

use minigrep::run;
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // .collect -> iterator to vector
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
