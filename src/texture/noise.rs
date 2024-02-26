use crate::noise::Noise3D;
use crate::vec4::{Point4, Vec4};

use super::Sampler;

pub type FloatNoise = Box<dyn Noise3D<Output = f64>>;

pub struct NoiseSolidTexture {
    noise: FloatNoise,

    pub scale: Vec4,
    pub samples: usize,
    pub map: fn(p: Point4, sampled: f64) -> f64,
}

impl NoiseSolidTexture {
    pub fn new(noise: FloatNoise) -> Self {
        NoiseSolidTexture {
            noise,
            scale: Vec4::vec(1.0, 1.0, 1.0),
            samples: 7,
            map: |_, s| s,
        }
    }
}

impl Sampler for NoiseSolidTexture {
    type Output = f64;

    fn sample(&self, _: (f64, f64), p: &Point4) -> Self::Output {
        let p_scaled = *p * self.scale;
        let sampled = self.noise.sample_turbulence(&p_scaled, self.samples);
        (self.map)(p_scaled, sampled)
    }
}
