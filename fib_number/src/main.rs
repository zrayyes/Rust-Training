fn main() {
    println!("{}", fibonacci(11));
}

fn fibonacci(c: i32) -> i32 {
    let mut old = 0;
    let mut current = 1;
    let mut new =  0;
    if c == 0 {
        0
    } else {
        for _num in 0..c-1 {
            new = old + current;
            old = current;
            current = new;
        }
        new
    }
}
