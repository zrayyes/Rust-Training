fn main() {
    let mut s = String::from("hello");
    let x = s; // shallow copy or MOVE
    let x_2 = x.clone(); // clone

    println!("{}", x_2);
}
