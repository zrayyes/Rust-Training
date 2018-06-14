fn main() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("{}", len);

    let mut x = String::from("hello");

    change(&mut x);

    println!("{}", x);

    // Avoid data races
    {
        let r1 = &mut x;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut x;

    let word = first_word(&r2);

    println!("{}", word);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// usize = byte index
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // convert string to array of bytes

    // loop over bytes array
    for (i, &item) in bytes.iter().enumerate() {
        // search for byte that equals the 'byte literal' syntax
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
