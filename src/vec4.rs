use rand::Rng;
use rand_distr::{Standard, StandardNormal};
use rand_pcg::Pcg64Mcg;
use std::{
    f64::consts::PI,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec4(pub [f64; 4]);

// Type aliases
pub type Color = Vec4;
pub type Point4 = Vec4;

impl Vec4 {
    // Constructors

    pub fn vec(x: f64, y: f64, z: f64) -> Self {
        Vec4([x, y, z, 0.0])
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Vec4([x, y, z, 1.0])
    }

    pub fn random_vec(rng: &mut Pcg64Mcg) -> Self {
        let x = rng.sample(Standard);
        let y = rng.sample(Standard);
        let z = rng.sample(Standard);

        Vec4([x, y, z, 0.0])
    }

    pub fn random_in_unit_disk(rng: &mut Pcg64Mcg) -> Self {
        let x = rng.sample(StandardNormal);
        let y = rng.sample(StandardNormal);

        Vec4([x, y, 0.0, 0.0]).to_unit()
    }

    pub fn random_unit(rng: &mut Pcg64Mcg) -> Self {
        let x = rng.sample(StandardNormal);
        let y = rng.sample(StandardNormal);
        let z = rng.sample(StandardNormal);

        Vec4([x, y, z, 0.0]).to_unit()
    }

    pub fn random_cosine(rng: &mut Pcg64Mcg) -> Vec4 {
        let r1: f64 = rng.sample(Standard);
        let r2: f64 = rng.sample(Standard);

        let phi = r1 * 2.0 * PI;
        let sqrt_r2 = r2.sqrt();
        let x = phi.cos() * sqrt_r2;
        let y = phi.sin() * sqrt_r2;
        let z = (1.0 - r2).sqrt();

        Vec4::vec(x, y, z)
    }

    // Getters

    pub fn xyz(&self) -> (f64, f64, f64) {
        (self.0[0], self.0[1], self.0[2])
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }

    pub fn w(&self) -> f64 {
        self.0[3]
    }

    // Color aliases

    pub fn r(&self) -> f64 {
        self.0[0]
    }

    pub fn g(&self) -> f64 {
        self.0[1]
    }

    pub fn b(&self) -> f64 {
        self.0[2]
    }

    // Utility functions

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2]
    }

    pub fn dot(&self, other: &Vec4) -> f64 {
        self.0[0] * other.0[0] + self.0[1] * other.0[1] + self.0[2] * other.0[2]
    }

    pub fn cross(&self, other: &Vec4) -> Vec4 {
        Vec4([
            self.0[1] * other.0[2] - self.0[2] * other.0[1],
            self.0[2] * other.0[0] - self.0[0] * other.0[2],
            self.0[0] * other.0[1] - self.0[1] * other.0[0],
            0.0,
        ])
    }

    pub fn to_unit(self) -> Vec4 {
        self / self.length()
    }

    pub fn lerp(self, other: Vec4, t: f64) -> Vec4 {
        self * (1.0 - t) + other * t
    }

    pub fn near_zero(&self) -> bool {
        let eps = 1e-8;
        (self.0[0].abs() < eps) && (self.0[1].abs() < eps) && (self.0[2].abs() < eps)
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
        Vec4([f(self.0[0]), f(self.0[1]), f(self.0[2]), f(self.0[3])])
    }
}

// Index operators

impl Index<usize> for Vec4 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

// Operators (copy)

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec4([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }
}

impl Add<f64> for Vec4 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Vec4([
            self.0[0] + rhs,
            self.0[1] + rhs,
            self.0[2] + rhs,
            self.0[3] + rhs,
        ])
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec4([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
        ])
    }
}

impl Sub<f64> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Vec4([
            self.0[0] - rhs,
            self.0[1] - rhs,
            self.0[2] - rhs,
            self.0[3] - rhs,
        ])
    }
}

impl Mul for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec4([
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
            self.0[3] * rhs.0[3],
        ])
    }
}

impl Mul<f64> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec4([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
        ])
    }
}

impl Div<f64> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec4([
            self.0[0] / rhs,
            self.0[1] / rhs,
            self.0[2] / rhs,
            self.0[3] / rhs,
        ])
    }
}

// Operators (mutation)

impl AddAssign for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
        self.0[3] += rhs.0[3];
    }
}

impl SubAssign for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0[0] -= rhs.0[0];
        self.0[1] -= rhs.0[1];
        self.0[2] -= rhs.0[2];
        self.0[3] -= rhs.0[3];
    }
}

impl MulAssign for Vec4 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0[0] *= rhs.0[0];
        self.0[1] *= rhs.0[1];
        self.0[2] *= rhs.0[2];
        self.0[3] *= rhs.0[3];
    }
}

impl MulAssign<f64> for Vec4 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0[0] *= rhs;
        self.0[1] *= rhs;
        self.0[2] *= rhs;
        self.0[3] *= rhs;
    }
}

impl DivAssign<f64> for Vec4 {
    fn div_assign(&mut self, rhs: f64) {
        self.0[0] /= rhs;
        self.0[1] /= rhs;
        self.0[2] /= rhs;
        self.0[3] /= rhs;
    }
}

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec4([-self.0[0], -self.0[1], -self.0[2], -self.0[3]])
    }
}
