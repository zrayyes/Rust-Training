use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
    println!("{}", scores["Blue"]);

    // OR

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
}
