fn main() {
    let mut list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    println!("Length: {}", list.len());
    println!("Mean: {}", mean(&list));
    println!("Median: {}", median(&mut list));
}

fn mean(list: &Vec<i32>) -> f32 {
    let mut count = 0;
    for number in list {
        count += number
    }
    (count as f32) / (list.len() as f32)
}

fn median(list: &mut Vec<i32>) -> f32 {
    list.sort();
    match list.len() % 2 {
        0 => ((list[list.len() / 2] as f32) + (list[(list.len() / 2) - 1] as f32)) / 2.0,
        1 => list[list.len() / 2] as f32,
        _ => -1.0,
    }
}
