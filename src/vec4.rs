use rand::Rng;
use rand_distr::{Standard, StandardNormal};
use rand_xorshift::XorShiftRng;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec4(pub f64, pub f64, pub f64, pub f64);

// Type aliases
pub type Color = Vec4;
pub type Point4 = Vec4;

impl Vec4 {
    // Constructors

    pub fn vec(x: f64, y: f64, z: f64) -> Self {
        Vec4(x, y, z, 0.0)
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Vec4(x, y, z, 1.0)
    }

    pub fn random_vec(rng: &mut XorShiftRng) -> Self {
        let x = rng.sample(Standard);
        let y = rng.sample(Standard);
        let z = rng.sample(Standard);

        Vec4(x, y, z, 0.0)
    }

    pub fn random_in_unit_disk(rng: &mut XorShiftRng) -> Self {
        let x = rng.sample(StandardNormal);
        let y = rng.sample(StandardNormal);

        Vec4(x, y, 0.0, 0.0).to_unit()
    }

    pub fn random_unit(rng: &mut XorShiftRng) -> Self {
        let x = rng.sample(StandardNormal);
        let y = rng.sample(StandardNormal);
        let z = rng.sample(StandardNormal);

        Vec4(x, y, z, 0.0).to_unit()
    }

    // Getters

    pub fn xyz(&self) -> (f64, f64, f64) {
        (self.0, self.1, self.2)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn w(&self) -> f64 {
        self.3
    }

    // Color aliases

    pub fn r(&self) -> f64 {
        self.0
    }

    pub fn g(&self) -> f64 {
        self.1
    }

    pub fn b(&self) -> f64 {
        self.2
    }

    // Utility functions

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn dot(&self, other: &Vec4) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Vec4) -> Vec4 {
        Vec4(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
            0.0,
        )
    }

    pub fn to_unit(self) -> Vec4 {
        self / self.length()
    }

    pub fn lerp(self, other: Vec4, t: f64) -> Vec4 {
        self * (1.0 - t) + other * t
    }

    pub fn near_zero(&self) -> bool {
        let eps = 1e-8;
        (self.0.abs() < eps) && (self.1.abs() < eps) && (self.2.abs() < eps)
    }

    pub fn reflect(self, normal: Vec4) -> Vec4 {
        self - normal * (2.0 * self.dot(&normal))
    }

    /// Note: assumes the vector being refracted is a unit vector
    pub fn refract(self, normal: Vec4, ior_ratio: f64) -> Vec4 {
        let cos_theta = f64::min(1.0, (-self).dot(&normal));

        let refracted_perp = (self + (normal * cos_theta)) * ior_ratio;
        let refracted_parallel = normal * -(1.0 - refracted_perp.length_squared()).sqrt();

        refracted_perp + refracted_parallel
    }

    pub fn map_components(self, f: fn(x: f64) -> f64) -> Vec4 {
        Vec4(f(self.0), f(self.1), f(self.2), f(self.3))
    }
}

// Index operators

impl Index<usize> for Vec4 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("Index out of bounds for Vec3"),
        }
    }
}

// Operators (copy)

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec4(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl Add<f64> for Vec4 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Vec4(self.0 + rhs, self.1 + rhs, self.2 + rhs, self.3 + rhs)
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec4(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl Sub<f64> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Vec4(self.0 - rhs, self.1 - rhs, self.2 - rhs, self.3 - rhs)
    }
}

impl Mul for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec4(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
            self.3 * rhs.3,
        )
    }
}

impl Mul<f64> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec4(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl Div<f64> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec4(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

// Operators (mutation)

impl AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self.3 += rhs.3;
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
        self.3 -= rhs.3;
    }
}

impl MulAssign for Vec4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
        self.3 *= rhs.3;
    }
}

impl MulAssign<f64> for Vec4 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
        self.3 *= rhs;
    }
}

impl DivAssign<f64> for Vec4 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
        self.3 /= rhs;
    }
}

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec4(-self.0, -self.1, -self.2, -self.3)
    }
}
