use std::fmt::Debug;

use crate::vec4::Point4;

use super::{Sampler, TexturePointer};

#[derive(Debug)]
pub struct CheckerboardTexture<T> {
    even_squares: TexturePointer<T>,
    odd_squares: TexturePointer<T>,
    scale: f64,
}

impl<T> CheckerboardTexture<T> {
    pub fn new(
        even_squares: TexturePointer<T>,
        odd_squares: TexturePointer<T>,
        scale: f64,
    ) -> Self {
        CheckerboardTexture {
            even_squares,
            odd_squares,
            scale,
        }
    }
}

impl<T> Sampler for CheckerboardTexture<T>
where
    T: Send + Sync + Copy + Debug,
{
    type Output = T;

    fn sample(&self, (u, v): (f64, f64), p: &Point4) -> Self::Output {
        let iu = (u * 2.0 / self.scale) as u32;
        let iv = (v * 2.0 / self.scale) as u32;

        let is_even = (iu + iv) % 2 == 0;
        if is_even {
            self.even_squares.sample((u, v), p)
        } else {
            self.odd_squares.sample((u, v), p)
        }
    }
}

#[derive(Debug)]
pub struct CheckerboardSolidTexture<T> {
    even_volumes: TexturePointer<T>,
    odd_volumes: TexturePointer<T>,
    scale: f64,
}

impl<T> CheckerboardSolidTexture<T> {
    pub fn new(
        even_volumes: TexturePointer<T>,
        odd_volumes: TexturePointer<T>,
        scale: f64,
    ) -> Self {
        CheckerboardSolidTexture {
            even_volumes,
            odd_volumes,
            scale,
        }
    }
}

impl<T> Sampler for CheckerboardSolidTexture<T>
where
    T: Send + Sync + Copy + Debug,
{
    type Output = T;

    fn sample(&self, uv: (f64, f64), p: &Point4) -> Self::Output {
        let ix = (p.x() / self.scale).floor() as i32;
        let iy = (p.y() / self.scale).floor() as i32;
        let iz = (p.z() / self.scale).floor() as i32;

        let is_even = (ix + iy + iz) % 2 == 0;
        if is_even {
            self.even_volumes.sample(uv, p)
        } else {
            self.odd_volumes.sample(uv, p)
        }
    }
}
