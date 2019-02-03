use std::ops::{Add, Div, Mul, Sub};
use num_traits::{Float, Zero};
use crate::materials::Material;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Zero> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3 {
            x: Zero::zero(),
            y: Zero::zero(),
            z: Zero::zero(),
        }
    }

    pub fn length(self) -> T
    where
        T: Float,
    {
        dot(&self, &self).sqrt()
    }

    pub fn normalise(self) -> Vec3<T>
    where
        T: Float,
    {
        self / self.length()
    }
}

impl<T: Zero> Default for Vec3<T> {
    fn default() -> Self {
        Vec3::zero()
    }
}

pub fn dot<T>(lhs: &Vec3<T>, rhs: &Vec3<T>) -> T
where
    T: Float,
{
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Vec3<T> {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3<f32>> for f32 {
    type Output = Vec3<f32>;

    fn mul(self, rhs: Vec3<f32>) -> Vec3<f32> {
        rhs * self
    }
}

impl Mul<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    fn mul(self, rhs: Vec3<f64>) -> Vec3<f64> {
        rhs * self
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Vec3<T> {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3<f32>,
    pub direction: Vec3<f32>,
}

#[derive(Debug)]
pub struct Sphere {
    pub centre: Vec3<f32>,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(centre: Vec3<f32>, radius: f32, material: Material) -> Self {
        Sphere {
            centre,
            radius,
            material,
        }
    }

    // TODO understand this and make it more idiomatic in Rust
    pub fn ray_intersect(&self, ray: &Ray) -> Option<f32> {
        let l = self.centre - ray.origin;
        let tca = dot(&l, &ray.direction);
        let d2 = dot(&l, &l) - tca * tca;

        if d2 > self.radius * self.radius {
            return None;
        }

        let thc = (self.radius * self.radius - d2).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;

        if t0 < 0.0 {
            t0 = t1;
        }

        if t0 < 0.0 {
            return None;
        }

        Some(t0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_f32() {
        let v1: Vec3<f32> = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2: Vec3<f32> = Vec3 {
            x: 4.0,
            y: 6.0,
            z: 8.0,
        };
        assert_eq!(
            v1 + v2,
            Vec3 {
                x: 5.0,
                y: 8.0,
                z: 11.0
            }
        );
    }

    #[test]
    fn add_f64() {
        let v1: Vec3<f64> = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let v2: Vec3<f64> = Vec3 {
            x: 4.0,
            y: 6.0,
            z: 8.0,
        };
        assert_eq!(
            v1 + v2,
            Vec3 {
                x: 5.0,
                y: 8.0,
                z: 11.0
            }
        );
    }

    #[test]
    fn mul_f32_vec() {
        let x: f32 = 3.5;
        let v: Vec3<f32> = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(
            v * x,
            Vec3 {
                x: 1.0 * 3.5,
                y: 2.0 * 3.5,
                z: 3.0 * 3.5
            }
        );
    }

    #[test]
    fn mul_f64_vec() {
        let x: f64 = 3.5;
        let v: Vec3<f64> = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert_eq!(
            v * x,
            Vec3 {
                x: 1.0 * 3.5,
                y: 2.0 * 3.5,
                z: 3.0 * 3.5
            }
        );
    }
}
