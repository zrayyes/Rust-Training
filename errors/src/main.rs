use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    // Return value inside Ok if available, else panic!
    let g = File::open("text.txt").unwrap();

    // Similar to unwrap, but with panic! message
    let h = File::open("text.txt").expect("Failed to open text.txt");
}

// Returns a Result with concrete types String, and io::Error
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// same as above but shorter with ?
fn read_username_shortcut() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
