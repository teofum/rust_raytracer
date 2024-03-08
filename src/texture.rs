use std::{fmt::Debug, sync::Arc};

use crate::vec4::Point4;

pub mod channel;
pub mod checkerboard;
pub mod constant;
pub mod image;
pub mod interpolate;
pub mod noise;
pub mod uv_debug;

pub use channel::Channel;
pub use checkerboard::{CheckerboardSolidTexture, CheckerboardTexture};
pub use constant::ConstantTexture;
pub use image::ImageTexture;
pub use interpolate::Interpolate;
pub use noise::NoiseSolidTexture;
pub use uv_debug::UvDebugTexture;

pub trait Sampler: Send + Sync + Debug {
    type Output: Send + Sync + Copy;

    fn sample(&self, uv: (f64, f64), p: &Point4) -> Self::Output;
}

pub type TexturePointer<T> = Arc<dyn Sampler<Output = T>>;
