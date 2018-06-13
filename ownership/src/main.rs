fn main() {
    let mut s = String::from("hello");
    let x = s; // shallow copy or MOVE

    println!("{}", s);
}
