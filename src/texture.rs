use crate::vec3::{Color, Point3};

pub mod constant_color;
pub use constant_color::ConstantColorTexture;

pub mod checkerboard;
pub use checkerboard::{CheckerboardSolidTexture, CheckerboardTexture};

pub trait Texture: Send + Sync {
    fn sample(&self, uv: (f64, f64), p: &Point3) -> Color;
}
