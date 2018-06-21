use std::env;
use std::io;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() == 1) {
        println!("Normal");
    } else if (args[1] == "1") {
        println!("{}% done!", calculate_percentage());
    } else {
        println!("Invalid argument/s");
    }
}

fn calculate_percentage() -> f32 {
    let start = readline_as_float("Please input starting weight: ");
    let target = readline_as_float("Please input target weight: ");
    let current = readline_as_float("Please input current weight: ");
    do_math(start, target, current) * 100.0
}

fn do_math(start: f32, target: f32, current: f32) -> f32 {
    (start - current) / (start - target)
}

fn readline_as_float(message: &str) -> f32 {
    let mut read = String::new();

    println!("{}", message);

    io::stdin()
        .read_line(&mut read)
        .expect("Failed to read line");

    if let "0" = read.trim() {
        println!("Invalid number");
        process::exit(1);
    }

    read.trim().parse::<f32>().expect("Invalid Number")
}
