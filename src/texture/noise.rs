use crate::noise::Noise3D;
use crate::vec3::{Color, Point3, Vec3};

use super::Texture;

pub type FloatNoise = Box<dyn Noise3D<Output = f64>>;

pub struct NoiseSolidTexture {
    noise: FloatNoise,

    pub scale: Vec3,
    pub samples: usize,
    pub map: fn(p: Point3, sampled: f64) -> f64,
}

impl NoiseSolidTexture {
    pub fn new(noise: FloatNoise) -> Self {
        NoiseSolidTexture {
            noise,
            scale: Vec3(1.0, 1.0, 1.0),
            samples: 7,
            map: |_, s| s,
        }
    }
}

impl Texture for NoiseSolidTexture {
    fn sample(&self, _: (f64, f64), p: &Point3) -> Color {
        let p_scaled = *p * self.scale;
        let sampled = self.noise.sample_turbulence(&p_scaled, self.samples);
        Vec3(1.0, 1.0, 1.0) * (self.map)(p_scaled, sampled)
    }
}
