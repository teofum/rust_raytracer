use crate::vec4::{Color, Point4};

pub mod checkerboard;
pub use checkerboard::{CheckerboardSolidTexture, CheckerboardTexture};

pub mod constant_color;
pub use constant_color::ConstantColorTexture;

pub mod image;
pub use image::ImageTexture;

pub mod noise;
pub use noise::NoiseSolidTexture;

pub mod uv_debug;
pub use uv_debug::UvDebugTexture;

pub trait Texture: Send + Sync {
    fn sample(&self, uv: (f64, f64), p: &Point4) -> Color;
}
