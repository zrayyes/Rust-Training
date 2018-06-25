fn main() {
    let mut s = String::from("initial contents");
    let s2 = "bar";
    s.push_str(s2);
    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used
    println!("s3 is {}", s3);
}
