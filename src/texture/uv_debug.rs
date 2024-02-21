use crate::vec3::{Color, Point3, Vec3};

use super::Texture;

pub struct UvDebugTexture;

impl Texture for UvDebugTexture {
    fn sample(&self, (u, v): (f64, f64), _: &Point3) -> Color {
        Vec3(u, v, 0.5)
    }
}
