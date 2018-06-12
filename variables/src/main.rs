fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1); // x.0 - x.2
    let (x, y, z) = tup;

    // array
    let a = [1, 2, 3, 4, 5]; // a[0] - a[4]

    println!("X = {}", a[0]);

    another_function(15, 20);

    loop {
        println!("Six = {}", plus_one(5));
        break;
    }

    let mut counter = 3;
    while counter != 0 {
        println!("{}!", counter);
        counter = counter - 1;
    }

    for element in a.iter() {
        println!("{}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn another_function(y: i32, z: i32) {
    let b = {
        if y > z {
            y
        } else {
            z
        }
    };

    if y > z {
        println!("Y is greater than Z, b = {}", b);
    } else {
        println!("Z is greater than Y, b = {}", b);
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
