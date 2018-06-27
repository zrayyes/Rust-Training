fn main() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

    println!("{:?}", mean(&list));
}

fn mean(list: &Vec<i32>) -> f32 {
    let mut count = 0;
    for number in list {
        count += number
    }
    (count as f32) / (list.len() as f32)
}
