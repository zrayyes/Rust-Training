use std::env;

fn main() {
    // .collect -> iterator to vector
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {

        // let path = &args[0];
        let query = &args[1];
        let filename = &args[2];

        println!("Searching for {}", query);
        println!("In file {}", filename);
    } else {
        println!("Invalid number of arguments, need query and filename");
    }
}
