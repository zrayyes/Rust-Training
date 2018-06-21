use std::io;

fn main() {
    let start = readline_as_float();
    let stop = readline_as_float();
    let current = readline_as_float();

    println!("{}% done", do_math(start, stop, current) * 100.0);
}

fn do_math(start: f32, end: f32, current: f32) -> f32 {
    (start - current) / (start - end)
}

fn readline_as_float() -> f32 {
    let mut read = String::new();

    io::stdin()
        .read_line(&mut read)
        .expect("Failed to read line");
    
    read.trim().parse::<f32>().unwrap()
}
