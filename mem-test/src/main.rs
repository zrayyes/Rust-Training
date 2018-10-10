mod process_api;

use std::io;
use std::io::prelude::*;

fn main() {
    let processes = process_api::get_processes();
    for p in processes {
        println!("{} - {}", p.pid, p.name);
    }
    pause();
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}
