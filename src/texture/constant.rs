use crate::vec4::{Color, Point4, Vec4};

use super::Sampler;

pub struct ConstantTexture<T> {
    value: T,
}

impl<T> ConstantTexture<T> {
    pub fn new(value: T) -> Self {
        ConstantTexture { value }
    }
}

impl ConstantTexture<Color> {
    pub fn from_values(r: f64, g: f64, b: f64) -> Self {
        Self::new(Vec4::vec(r, g, b))
    }
}

impl<T> Sampler for ConstantTexture<T>
where
    T: Send + Sync + Copy,
{
    type Output = T;

    fn sample(&self, _: (f64, f64), _: &Point4) -> Self::Output {
        self.value
    }
}
