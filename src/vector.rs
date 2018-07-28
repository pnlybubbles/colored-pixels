use std::ops::{Neg, Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vector {
  pub fn new(x: f32, y: f32, z: f32) -> Vector {
    Vector { x: x, y: y, z: z }
  }
}

pub trait Dot: Sized + Copy + Div<f32, Output = Self> {
  fn dot(self, Self) -> f32;
}

pub trait Cross {
  fn cross(self, Self) -> Self;
}

pub trait Norm {
  fn norm(self) -> f32;
  fn sqr_norm(self) -> f32;
}

pub trait Normalize {
  fn normalize(self) -> Self;
}

impl<T> Norm for T
  where T: Copy + Dot
{
  fn norm(self) -> f32 {
    self.sqr_norm().sqrt()
  }

  fn sqr_norm(self) -> f32 {
    self.dot(self)
  }
}

impl<T> Normalize for T
  where T: Copy + Norm + Div<f32, Output = Self>
{
  fn normalize(self) -> T {
    self / self.norm()
  }
}

impl Dot for Vector {
  fn dot(self, rhs: Vector) -> f32 {
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

impl Mul<f32> for Vector {
  type Output = Vector;

  fn mul(self, rhs: f32) -> Vector {
    Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
  }
}

impl Mul<Vector> for f32 {
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

impl Div<f32> for Vector {
  type Output = Vector;

  fn div(self, rhs: f32) -> Vector {
    Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
  }
}

impl Div<Vector> for f32 {
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
