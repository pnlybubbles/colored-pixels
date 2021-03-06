use std::ops::{Neg, Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vector {
  pub fn new(x: f64, y: f64, z: f64) -> Vector {
    Vector { x: x, y: y, z: z }
  }
}

pub trait Dot: Sized + Copy + Div<f64, Output = Self> {
  fn dot(self, Self) -> f64;
}

pub trait Cross {
  fn cross(self, Self) -> Self;
}

pub trait Norm {
  fn norm(self) -> f64;
  fn sqr_norm(self) -> f64;
}

pub trait Normalize {
  fn normalize(self) -> Self;
}

impl<T> Norm for T
  where T: Copy + Dot
{
  fn norm(self) -> f64 {
    self.sqr_norm().sqrt()
  }

  fn sqr_norm(self) -> f64 {
    self.dot(self)
  }
}

impl<T> Normalize for T
  where T: Copy + Norm + Div<f64, Output = Self>
{
  fn normalize(self) -> T {
    self / self.norm()
  }
}

impl Dot for Vector {
  fn dot(self, rhs: Vector) -> f64 {
    self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
  }
}

impl Cross for Vector {
  fn cross(self, rhs: Vector) -> Vector {
    Vector::new(
      self.y * rhs.z - self.z * rhs.y,
      self.z * rhs.x - self.x * rhs.z,
      self.x * rhs.y - self.y * rhs.x,
    )
  }
}

impl Neg for Vector {
  type Output = Vector;

  fn neg(self) -> Vector {
    Vector::new(-self.x, -self.y, -self.z)
  }
}

impl Add for Vector {
  type Output = Vector;

  fn add(self, rhs: Vector) -> Vector {
    Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}

impl Sub for Vector {
  type Output = Vector;

  fn sub(self, rhs: Vector) -> Vector {
    Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

impl Mul<f64> for Vector {
  type Output = Vector;

  fn mul(self, rhs: f64) -> Vector {
    Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
  }
}

impl Mul<Vector> for f64 {
  type Output = Vector;

  fn mul(self, rhs: Vector) -> Vector {
    Vector::new(self * rhs.x, self * rhs.y, self * rhs.z)
  }
}

impl Mul for Vector {
  type Output = Vector;

  fn mul(self, rhs: Vector) -> Vector {
    Vector::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
  }
}

impl Div<f64> for Vector {
  type Output = Vector;

  fn div(self, rhs: f64) -> Vector {
    Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
  }
}

impl Div<Vector> for f64 {
  type Output = Vector;

  fn div(self, rhs: Vector) -> Vector {
    Vector::new(self / rhs.x, self / rhs.y, self / rhs.z)
  }
}

impl Div for Vector {
  type Output = Vector;

  fn div(self, rhs: Vector) -> Vector {
    Vector::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
  }
}
