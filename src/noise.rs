use crate::vec4::Point4;

pub mod perlin;
pub use perlin::PerlinNoise3D;

pub trait Noise3D: Sync + Send {
    type Output;

    fn sample(&self, p: &Point4) -> Self::Output;
    fn sample_turbulence(&self, p: &Point4, samples: usize) -> Self::Output;
}
