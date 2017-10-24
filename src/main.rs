#![feature(box_syntax)]

mod vector;
mod image;

use std::path::Path;
use image::Image;
use vector::*;

const PI: f64 = 3.14159265358979323846264338327950288_f64;
const EPS: f64 = 1e-5;
const WIDTH: usize = 512;
const HEIGHT: usize = 512;

fn main() {
  // 画像データの初期化
  let mut image = Image::new(Vector::new(0.0, 0.0, 0.0), WIDTH, HEIGHT);
  // シーンの構成
  let sphere = box Sphere {
    radius: 1.0,
    position: Vector::new(0.0, 0.0, 0.0),
  };
  let scene: Vec<Box<Shape>> = vec![sphere];
  // 各ピクセルで処理
  image.each_pixel_mut(|pixel, x, y| {
    // レイの生成
    let ray = Ray {
      origin: Vector::new(0.0, 0.0, 5.0),
      direction: Vector::new(
        x as f64 / WIDTH as f64 - 0.5,
        -(y as f64 / HEIGHT as f64 - 0.5),
        -1.0,
      ).normalize(),
    };
    // すべてのシーン内のオブジェクトと当たり判定
    let intersection = scene.iter().flat_map(|v| v.intersect(&ray)).min_by(
      |a, b| {
        a.distance.partial_cmp(&b.distance).unwrap()
      },
    );
    // 色付け
    *pixel = match intersection {
      None => Vector::new(0.0, 0.0, 0.0),
      Some(i) => i.normal / 2.0 + Vector::new(0.5, 0.5, 0.5),
    };
  });
  // 画像を保存
  image
    .save_ppm(&Path::new("img.ppm"), |pixel| {
      [to_color(pixel.x), to_color(pixel.y), to_color(pixel.z)]
    })
    .unwrap();
}

fn to_color(x: f64) -> u8 {
  (x.max(0.0).min(1.0) * 255.0) as u8
}

// レイの情報
struct Ray {
  direction: Vector,
  origin: Vector,
}

// 衝突点の情報
struct Intersection {
  position: Vector,
  normal: Vector,
  distance: f64,
}

// 形状(衝突判定が実装された型)
trait Shape {
  fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

// 球
struct Sphere {
  radius: f64,
  position: Vector,
}

impl Shape for Sphere {
  fn intersect(&self, ray: &Ray) -> Option<Intersection> {
    let po = ray.origin - self.position;
    let pod = po.dot(ray.direction);
    let det = pod * pod - po.sqr_norm() + self.radius * self.radius;
    if det < 0.0 {
      return None;
    }
    let t1 = -pod - det.sqrt();
    let t2 = -pod + det.sqrt();
    if t1 < EPS && t2 < EPS {
      return None;
    }
    let distance = if t1 > EPS { t1 } else { t2 };
    let position = ray.origin + ray.direction * distance;
    let normal = (position - self.position).normalize();
    Some(Intersection {
      position: position,
      normal: normal,
      distance: distance,
    })
  }
}
