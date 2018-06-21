use std::env;
use std::io;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        welcome_message();
    } else if args[1] == "1" {
        calculate_percentage();
    } else if args[1] == "2" {
        calculate_bfp();
    } else {
        println!("Invalid argument/s");
    }
}

fn welcome_message() {
    println!("Select Option: ");
    println!("1 - Calculate percentage done.");
    println!("2 - Calculate body fat percentage.");

    match readline().trim() {
        "1" => calculate_percentage(),
        "2" => calculate_bfp(),
        _ => println!("Invalid option"),
    }
}

fn calculate_percentage() {
    let start = readline_as_float("Please input starting weight: ");
    let target = readline_as_float("Please input target weight: ");
    let current = readline_as_float("Please input current weight: ");
    println!("{}% done!", ((start - current) / (start - target)) * 100.0);
}

fn calculate_bfp() {
    let weight = readline_as_float("Please input weight: ");
    let lm = readline_as_float("Please input leanmass: ");
    println!("{}% done!", ((weight - lm) / weight) * 100.0);
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
