extern crate libc;

extern "C" {
    fn doubler(input: libc::c_int) -> libc::c_int;
}

fn main() {
    let a = 15;
    println!("C is doubling {} to {}", a, unsafe { doubler(a) });
}
