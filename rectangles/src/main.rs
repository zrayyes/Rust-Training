struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(rect: Rectangle) -> u32 {
    rect.width * rect.height
}
