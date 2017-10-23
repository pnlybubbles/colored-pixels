use std::ops::{Neg, Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy)]
pub struct Vector { x: f64, y: f64, z: f64 }

impl Vector {
  pub fn new(x: f64, y: f64, z: f64) -> Vector {
    Vector { x: x, y: y, z: z }
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
