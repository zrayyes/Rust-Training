use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let scores = vec![10, 50];
    // _ - infer type from values
    let scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are now invalid

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // check if value exists and if not insert
    let mut new_scores = HashMap::new();
    new_scores.entry(String::from("Green")).or_insert(15);
    new_scores.entry(String::from("Red")).or_insert(25);

    println!("{:?}", new_scores);

    let text = "hello world new world";

    let mut wordcount = HashMap::new();

    for word in text.split_whitespace() {
        // returns a mutable refernce to the value for this key
        let count = wordcount.entry(word).or_insert(0);
        // derefrence count (which is &mut V)
        *count += 1;
    }

    println!("{:?}", wordcount);
}
