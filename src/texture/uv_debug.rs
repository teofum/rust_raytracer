use crate::vec4::{Color, Point4, Vec4};

use super::Sampler;

#[derive(Debug)]
pub struct UvDebugTexture;

impl Sampler for UvDebugTexture {
    type Output = Color;

    fn sample(&self, (u, v): (f64, f64), _: &Point4) -> Self::Output {
        Vec4::vec(u, v, 0.5)
    }
}
