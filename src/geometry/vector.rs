use serde::Serialize;
use std::fmt::Formatter;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

// Purposely not generic right now
// In the long term, this can be made generic over f32 and f64 (at the very least).
#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    pub fn zero() -> Self {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn abs(&self) -> Self {
        Vector {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn cross(&self, v: Vector) -> Vector {
        Vector {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn dot(&self, v: Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn len(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn unit(&self) -> Vector {
        self / self.len()
    }

    pub const X: Vector = Vector {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const Y: Vector = Vector {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const Z: Vector = Vector {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
}

// Element-wise operations
// These are non-geometric, so their kept out
// of the regular operations. You should only use these if you
// plan on doing something non-geometric (opt-in).
impl Vector {
    // Element-wise division
    pub fn div_elem(&self, rhs: Vector) -> Self {
        Vector {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }

    // Element-wise multiplication
    pub fn mul_elem(&self, rhs: Vector) -> Self {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }

    // Element-wise addition
    pub fn add_elem(&self, rhs: Vector) -> Self {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    // Element-wise subtraction
    pub fn sub_elem(&self, rhs: Vector) -> Self {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<f64> for &Vector {
    type Output = Vector;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    #[inline]
    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vector> for &Vector {
    type Output = Vector;

    #[inline]
    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Vector> for Vector {
    type Output = Vector;

    #[inline]
    fn add(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;

    #[inline]
    fn add(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vector> for &Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<&Vector> for Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<&Vector> for &Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign<f64> for Vector {
    #[inline]
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl SubAssign<f64> for Vector {
    #[inline]
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl MulAssign<f64> for Vector {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vector {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
