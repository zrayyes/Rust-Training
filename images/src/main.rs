extern crate image;

use image::{GenericImageView, ImageBuffer};

fn main() {
    let boy = image::open("boy.png").unwrap();
    let girl = image::open("girl.png").unwrap();

    let mut imgbuf = ImageBuffer::new(boy.width(), boy.height());

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 {
            *pixel = boy.get_pixel(x, y);
        } else {
            *pixel = girl.get_pixel(x, y);
        }
    }

    imgbuf.save("join.png").unwrap();
}
