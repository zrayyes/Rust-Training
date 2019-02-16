extern crate image;
use image::{DynamicImage, GenericImageView, ImageBuffer, RgbaImage};
use rand::Rng;
use std::fs;

fn main() {
    let mut imgs = vec![];
    let paths = fs::read_dir("./img").unwrap();
    for path in paths {
        imgs.push(image::open(path.unwrap().path()).unwrap());
    }
    if imgs.len() > 0 {
        let joined = join(&imgs);
        joined.save("join.png").unwrap();

        let random = randomize(&imgs);
        random.save("random.png").unwrap();
    }
}

fn join(images: &Vec<DynamicImage>) -> RgbaImage {
    let mut buffer = ImageBuffer::new(images[0].width(), images[0].height());
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let imgg = &images[((x + y) % images.len() as u32) as usize];
        if imgg.width() == images[0].width() && imgg.height() == images[0].height() {
            let pix = imgg.get_pixel(x, y);
            if (pix[0] as u64 + pix[1] as u64 + pix[2] as u64 + pix[3] as u64) == 0 {
                *pixel = images[0].get_pixel(x, y);
            } else {
                *pixel = pix;
            }
        }
    }
    buffer
}

fn randomize(images: &Vec<DynamicImage>) -> RgbaImage {
    let mut buffer = ImageBuffer::new(images[0].width(), images[0].height());
    let mut rng = rand::thread_rng();
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let imgg = &images[(rng.gen_range(0, images.len())) as usize];
        if imgg.width() == images[0].width() && imgg.height() == images[0].height() {
            let pix = imgg.get_pixel(x, y);
            if (pix[0] as u64 + pix[1] as u64 + pix[2] as u64 + pix[3] as u64) == 0 {
                *pixel = images[0].get_pixel(x, y);
            } else {
                *pixel = pix;
            }
        }
    }
    buffer
}
