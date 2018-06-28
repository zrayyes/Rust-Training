use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        //             match guard, check if error returned is 'NotFound'
        // Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
        //     Ok(fc) => fc,
        //     Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        // },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };

    // Shortcuts

    // Return value inside Ok if available, else panic!
    let g = File::open("text.txt").unwrap();

    // Similar to unwrap, but with panic! message
    let h = File::open("text.txt").expect("Failed to open text.txt");
}
