fn main() {
    println!("{}", multiple_3_5(1000));
}

fn multiple_3_5(number: i32) -> i32 {
    let mut sum: i32 = 0;
    for n in 0..number {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }
    sum
}
