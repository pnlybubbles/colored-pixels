use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io;

pub struct Image<T> {
  pub data: Vec<Vec<[T; 3]>>,
  pub width: usize,
  pub height: usize,
}

impl<T: Copy> Image<T> {
  pub fn new(fill: T, width: usize, height: usize) -> Image<T> {
    Image {
      data: vec![vec![[fill; 3]; width]; height],
      width: width,
      height: height,
    }
  }

  pub fn each_pixel_mut<F: Fn(&mut [T; 3], usize, usize)>(&mut self, f: F) {
    for (y, row) in self.data.iter_mut().enumerate() {
      for (x, pixel) in row.iter_mut().enumerate() {
        f(pixel, x, y)
      }
    }
  }

  pub fn save_ppm<F: Fn(T) -> u8>(&self, path: &Path, f: F) -> io::Result<()> {
    let mut file = File::create(&path)?;
    file.write_all(format!("P3\n{} {}\n{}\n", self.width, self.height, 255).as_bytes())?;
    for y in 0..self.height {
      for x in 0..self.width {
        let c = self.data[y][x];
        file.write_all(format!("{} {} {}\n", f(c[0]), f(c[1]), f(c[2])).as_bytes())?;
      }
    }
    Ok(())
  }
}
