use crate::vec4::{Color, Point4, Vec4};

use super::Texture;

pub struct UvDebugTexture;

impl Texture for UvDebugTexture {
    fn sample(&self, (u, v): (f64, f64), _: &Point4) -> Color {
        Vec4::vec(u, v, 0.5)
    }
}
