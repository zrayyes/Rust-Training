mod process;

use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    let all_processes = process::api::get_processes();
    for p in all_processes {
        println!("PID: {} - NAME: {}", p.pid, p.name);
    }
    pause();
}
