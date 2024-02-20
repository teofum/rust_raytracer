use crate::vec3::{Color, Point3, Vec3};

use super::Texture;

pub struct ConstantColorTexture {
    color: Color,
}

impl ConstantColorTexture {
    pub fn new(color: Color) -> Self {
        ConstantColorTexture { color }
    }

    pub fn from_values(r: f64, g: f64, b: f64) -> Self {
        Self::new(Vec3(r, g, b))
    }
}

impl Texture for ConstantColorTexture {
    fn sample(&self, _: (f64, f64), _: &Point3) -> Color {
        self.color
    }
}
