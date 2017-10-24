mod vector;
mod image;

use std::path::Path;

use image::Image;

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn main() {
  let mut image = Image::new([0.0f64; 3], WIDTH, HEIGHT);
  image.each_pixel_mut( |pixel, x, y| {
    *pixel = [
      x as f64 / WIDTH as f64,
      y as f64 / WIDTH as f64,
      1.0,
    ]
  });
  image.save_ppm(&Path::new("img.ppm"), |pixel| {
    let mut color = [0u8; 3];
    for (i, x) in color.iter_mut().enumerate() {
      *x = (clamp(pixel[i]) * 255.0) as u8
    }
    color
  }).unwrap();
}

fn clamp(x: f64) -> f64 {
  x.max(0.0).min(1.0)
}
