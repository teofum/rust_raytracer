use crate::vec3::Point3;

pub mod perlin;
pub use perlin::PerlinNoise3D;

pub trait Noise3D: Sync + Send {
    type Output;

    fn sample(&self, p: &Point3) -> Self::Output;
    fn sample_turbulence(&self, p: &Point3, samples: usize) -> Self::Output;
}
