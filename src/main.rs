#![feature(box_syntax)]
extern crate rand;

mod vector;
mod image;

use std::path::Path;
use image::Image;
use vector::*;
use rand::{Rng};
use rand::distributions::{Range, IndependentSample};
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::Arc;

const PI: f64 = 3.14159265358979323846264338327950288_f64;
const EPS: f64 = 1e-2;
const WIDTH: usize = 512;
const HEIGHT: usize = 512;
const SPP: usize = 100;

fn main() {
  // 画像データの初期化
  let mut image = Image::new(Vector::new(0.0, 0.0, 0.0), WIDTH, HEIGHT);
  // シーンの構成
  let white = Material {
    color: Vector::new(0.8, 0.8, 0.8),
    emittance: Vector::new(0.0, 0.0, 0.0),
  };
  let red = Material {
    color: Vector::new(0.8, 0.2, 0.2),
    emittance: Vector::new(0.0, 0.0, 0.0),
  };
  let scene: Arc<Vec<Box<Shape + Sync + Send>>> = Arc::new(vec![
    box Sphere { radius: 1.0, position: Vector::new(0.0, 0.0, 0.0), material: red.clone() },
    box Sphere { radius: 1e5, position: Vector::new(0.0, -1.0 - 1e5, 0.0), material: white.clone() },
  ]);
  // 各ピクセルで処理
  let (tx, rx): (Sender<(usize, usize, Vector)>, Receiver<(usize, usize, Vector)>) = channel();
  for x in 0..WIDTH {
    for y in 0..HEIGHT {
      // マルチスレッドで処理
      let tx = tx.clone();
      let scene = scene.clone();
      thread::spawn(move || {
        // 乱数ジェネレータの初期化
        let mut rng = rand::thread_rng();
        let l = (0..SPP).fold(Vector::new(0.0, 0.0, 0.0), |sum, _| {
          // レイの生成
          let ray = Ray {
            origin: Vector::new(0.0, 0.0, 5.0),
            direction: Vector::new(
              x as f64 / WIDTH as f64 - 0.5,
              -(y as f64 / HEIGHT as f64 - 0.5),
              -1.0,
            ).normalize(),
          };
          // レイを飛ばす
          sum + radiance(&scene, &ray, 0, &mut rng)
        });
        tx.send((x, y, l / SPP as f64)).unwrap()
      });
    }
  }
  // 各スレッドから受け取る
  for _ in 0..HEIGHT * WIDTH {
    let (x, y, l) = rx.recv().unwrap();
    image.set(x, y, l)
  }
  // 画像を保存
  println!("\nsaving...");
  image
    .save_ppm(&Path::new("img.ppm"), |pixel| {
      [to_color(pixel.x), to_color(pixel.y), to_color(pixel.z)]
    })
    .unwrap();
}

fn radiance<R, T>(scene: &Vec<Box<T>>, ray: &Ray, depth: usize, mut rng: R) -> Vector
    where R: Rng, T: Shape + ?Sized {
  // すべてのシーン内のオブジェクトと当たり判定
  let maybe_intersection = scene.iter().flat_map(|v| v.intersect(&ray)).min_by(
    |a, b| {
      a.distance.partial_cmp(&b.distance).unwrap()
    },
  );
  match maybe_intersection {
    // 何にも当たらなかった場合
    None => Vector::new(1.0, 1.0, 1.0),
    // 物体表面で相互作用
    Some(intersection) => {
      // 放射
      let l_e = intersection.material.emittance;
      // スタックオーバーフロー対策のために適当に反射を抑制する(biased)
      if depth > 5 {
        return l_e;
      }
      // 当たった方向を加味した法線方向
      let normal = if intersection.normal.dot(ray.direction) < 0.0 {
        intersection.normal
      } else {
        -intersection.normal
      };
      // 衝突表面における正規直交基底を生成
      let tangent = if normal.x.abs() > EPS {
        Vector::new(0.0, 1.0, 0.0)
      } else {
        Vector::new(1.0, 0.0, 0.0)
      }.cross(normal).normalize();
      let binormal = normal.cross(tangent);
      // 単位半球面上の1点サンプリング
      let dist = Range::new(0.0f64, 1.0);
      let s1 = dist.ind_sample(&mut rng);
      let s2 = dist.ind_sample(&mut rng);
      let phi = 2.0 * PI * s1;
      let sin_theta = (1.0 - s2 * s2).sqrt();
      let cos_theta = s2;
      let new_direction = tangent * (sin_theta * phi.cos())
        + binormal * (sin_theta * phi.sin())
        + normal * (cos_theta);
      // 新しいレイの生成
      let new_ray = Ray {
        origin: intersection.position,
        direction: new_direction,
      };
      // 入射光
      let l_i = radiance(&scene, &new_ray, depth + 1, rng);
      // Lambertian拡散反射面でのBRDF
      let brdf = intersection.material.color / PI;
      // コサイン項
      let cos_term = new_direction.dot(normal);
      // 確率密度
      let pdf = 1.0 / (2.0 * PI);
      // レンダリング方程式
      l_e + (brdf * l_i * cos_term / pdf)
    },
  }
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
  material: Material,
}

#[derive(Clone)]
struct Material {
  emittance: Vector,
  color: Vector,
}

// 形状(衝突判定が実装された型)
trait Shape {
  fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

// 球
struct Sphere {
  radius: f64,
  position: Vector,
  material: Material,
}

impl Shape for Sphere {
  fn intersect(&self, ray: &Ray) -> Option<Intersection> {
    let po = ray.origin - self.position;
    let b = ray.direction.dot(po);
    let c = po.sqr_norm() - self.radius * self.radius;
    // 判別式 Δ = b^2 - a*c
    let det = b * b - c;
    // 交差しない
    if det < 0.0 {
      return None;
    }
    let t1 = -b - det.sqrt();
    let t2 = -b + det.sqrt();
    // 出射方向と反対側で交差
    if t1 < EPS && t2 < EPS {
      return None;
    }
    // 近い方が正の場合はそれを採用
    // それ以外(球体内部からの交差の場合)は正の方を採用
    let distance = if t1 > EPS { t1 } else { t2 };
    // r = o + t * d
    let position = ray.origin + ray.direction * distance;
    // 法線は球体中心から外向き
    let normal = (position - self.position).normalize();
    Some(Intersection {
      position: position,
      normal: normal,
      distance: distance,
      material: self.material.clone(),
    })
  }
}
