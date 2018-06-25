#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    // panics if value does not exist
    println!("{}", &v[3]);
    // returns None if value does not exist
    println!("{:?}", v.get(3));

    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    row.push(SpreadsheetCell::Int(5));
    row.push(SpreadsheetCell::Text(String::from("five")));
    row.push(SpreadsheetCell::Float(5.001));

    println!("{:?}", row);
}
