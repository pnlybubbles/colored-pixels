use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io;

pub struct Image<T> {
  pub data: Vec<Vec<T>>,
  pub width: usize,
  pub height: usize,
}

impl<T: Copy> Image<T> {
  pub fn new(fill: T, width: usize, height: usize) -> Image<T> {
    Image {
      data: vec![vec![fill; width]; height],
      width: width,
      height: height,
    }
  }

  pub fn each_pixel_mut<F: FnMut(&mut T, usize, usize)>(&mut self, mut f: F) {
    for (y, row) in self.data.iter_mut().enumerate() {
      for (x, pixel) in row.iter_mut().enumerate() {
        f(pixel, x, y);
      }
    }
  }

  pub fn save_ppm<F>(&self, path: &Path, f: F) -> io::Result<()>
  where
    F: Fn(T) -> [u8; 3],
  {
    let mut file = File::create(&path)?;
    file.write_all(
      format!("P3\n{} {}\n{}\n", self.width, self.height, 255)
        .as_bytes(),
    )?;
    for y in 0..self.height {
      for x in 0..self.width {
        let c = f(self.data[y][x]);
        file.write_all(
          format!("{} {} {}\n", c[0], c[1], c[2]).as_bytes(),
        )?;
      }
    }
    Ok(())
  }
}
