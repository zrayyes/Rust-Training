use std::env;
use std::io;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        welcome_message();

        match readline().trim() {
            "1" => println!("{}% done!", calculate_percentage()),
            "2" => println!("two"),
            _ => println!("Invalid option"),
        }
    } else if args[1] == "1" {
        println!("{}% done!", calculate_percentage());
    } else {
        println!("Invalid argument/s");
    }
}

fn welcome_message() -> {
        println!("Select Option: ");
        println!("1 - Calculate percentage done.");
        println!("2 - Calculate targeted percentage.");
}

fn calculate_percentage() -> f32 {
    let start = readline_as_float("Please input starting weight: ");
    let target = readline_as_float("Please input target weight: ");
    let current = readline_as_float("Please input current weight: ");
    ((start - current) / (start - target)) * 100.0
}

fn readline() -> String {
    let mut read = String::new();
    io::stdin()
        .read_line(&mut read)
        .expect("Failed to read line");

    read
}

fn readline_as_float(message: &str) -> f32 {
    println!("{}", message);
    let read = readline();

    if let "0" = read.trim() {
        println!("Invalid number");
        process::exit(1);
    }

    read.trim().parse::<f32>().expect("Invalid Number")
}
