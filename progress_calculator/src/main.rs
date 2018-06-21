fn main() {
    println!("{}% done",do_math(3.0, 1.0, 2.3)*100.0);
}

fn do_math(start: f32, end: f32, current: f32) -> f32 {
    (start-current)/(start-end)
}
