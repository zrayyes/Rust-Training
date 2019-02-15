extern crate image;
use image::{GenericImageView, ImageBuffer};
use std::fs;

fn main() {
    let mut imgs = vec![];
    let paths = fs::read_dir("./img").unwrap();
    for path in paths {
        imgs.push(image::open(path.unwrap().path()).unwrap());
    }
    if imgs.len() > 0 {
        let mut imgbuf = ImageBuffer::new(imgs[0].width(), imgs[0].height());

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let imgg = &imgs[((x + y) % imgs.len() as u32) as usize];
            if imgg.width() == imgs[0].width() && imgg.height() == imgs[0].height() {
                let pix = imgg.get_pixel(x, y);
                if (pix[0]as u64 + pix[1]as u64 + pix[2]as u64 + pix[3]as u64 ) == 0 {
                    *pixel = imgs[0].get_pixel(x, y);
                } else {
                    *pixel = pix;
                }
                
            }
        }

        imgbuf.save("join.png").unwrap();
    }
}