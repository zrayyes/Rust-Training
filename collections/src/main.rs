fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    println!("{:?}", v);
    // panics if value does not exist
    println!("{}", &v[3]);
    // returns None if value does not exist
    println!("{:?}", v.get(3));

    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);
}
