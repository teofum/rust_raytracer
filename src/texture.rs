use crate::vec3::{Color, Point3};

pub mod constant_color;
pub use constant_color::ConstantColorTexture;

pub mod checkerboard;
pub use checkerboard::CheckerboardTexture;

pub trait Texture {
    fn sample(&self, u: f64, v: f64, p: &Point3) -> Color;
}
