use std::fs;
use std::path::Path;

fn main() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let filename = path.unwrap().path();
        let filename_str = filename.to_str().unwrap();

        match &filename_str[..5] {
            "pre__" => fs::rename(&filename,Path::new(&filename_str[5..])).unwrap(),
            _ => {},
        }
    }
}
