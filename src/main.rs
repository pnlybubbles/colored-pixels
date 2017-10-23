mod vector;
mod image;

use std::path::Path;

use image::Image;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn main() {
  let mut image = Image::new(0.0f64, WIDTH, HEIGHT);
  image.each_pixel_mut( |pixel, x, y| {
    *pixel = [
      x as f64 / WIDTH as f64,
      y as f64 / WIDTH as f64,
      1.0,
    ]
  });
  image.save_ppm(&Path::new("img.ppm"), |x| (x * 255.0) as u8).unwrap();
}
