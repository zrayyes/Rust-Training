use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // .collect -> iterator to vector
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        // let path = &args[0];
        let query = &args[1];
        let filename = &args[2];

        println!("Searching for {}", query);
        println!("In file {}", filename);

        // ? -> handle error
        let mut file = File::open(filename).expect("file not found");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("failed to read file");

        println!("Found: \n{:?}", content);
    } else {
        println!("Invalid number of arguments, need query and filename");
    }
}
