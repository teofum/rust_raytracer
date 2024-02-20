use std::sync::Arc;

use crate::vec3::{Color, Point3};

use super::Texture;

pub struct CheckerboardTexture {
    even_squares: Arc<dyn Texture>,
    odd_squares: Arc<dyn Texture>,
    scale: f64,
}

impl CheckerboardTexture {
    pub fn new(
        even_squares: Arc<dyn Texture>,
        odd_squares: Arc<dyn Texture>,
        scale: f64,
    ) -> Self {
        CheckerboardTexture {
            even_squares,
            odd_squares,
            scale,
        }
    }
}

impl Texture for CheckerboardTexture {
    fn sample(&self, (u, v): (f64, f64), p: &Point3) -> Color {
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

pub struct CheckerboardSolidTexture {
    even_volumes: Arc<dyn Texture>,
    odd_volumes: Arc<dyn Texture>,
    scale: f64,
}

impl CheckerboardSolidTexture {
    pub fn new(
        even_volumes: Arc<dyn Texture>,
        odd_volumes: Arc<dyn Texture>,
        scale: f64,
    ) -> Self {
        CheckerboardSolidTexture {
            even_volumes,
            odd_volumes,
            scale,
        }
    }
}

impl Texture for CheckerboardSolidTexture {
    fn sample(&self, uv: (f64, f64), p: &Point3) -> Color {
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
