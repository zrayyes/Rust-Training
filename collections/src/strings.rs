fn main() {
    let mut s = String::from("initial contents");
    let s2 = " bar";
    s.push_str(s2);
    println!("s2 is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    for c in s3.chars() {
        println!("{}", c);
    }
}
