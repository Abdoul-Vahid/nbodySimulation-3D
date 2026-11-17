use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm_sq(self) -> f64 {
        self.dot(self)
    }

    pub fn norm(self) -> f64 {
        self.norm_sq().sqrt()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_subtract_vectors() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        let sum = v1 + v2;
        assert_eq!(sum, Vec3::new(5.0, 7.0, 9.0));

        let difference = v1 - v2;
        assert_eq!(difference, Vec3::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn scalar_ops() {
        let a = Vec3::new(1.0, -2.0, 4.0);
        assert_eq!(a * 2.0, Vec3::new(2.0, -4.0, 8.0));
        assert_eq!(a / 2.0, Vec3::new(0.5, -1.0, 2.0));
        assert_eq!(-a, Vec3::new(-1.0, 2.0, -4.0));
    }

    #[test]
    fn dot_and_norm() {
        let a = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(a.norm_sq(), 25.0);
        assert_eq!(a.norm(), 5.0);
        assert_eq!(a.dot(Vec3::new(1.0, 0.0, 0.0)), 3.0);
    }

    #[test]
    fn assign_ops() {
        let mut a = Vec3::ZERO;
        a += Vec3::new(1.0, 1.0, 1.0);
        a -= Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(a, Vec3::new(1.0, 0.0, 1.0));
    }
}
