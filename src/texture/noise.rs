use std::sync::Arc;

use crate::noise::Noise3D;
use crate::vec4::{Point4, Vec4};

use super::Sampler;

pub type FloatNoise = Arc<dyn Noise3D<Output = f64>>;

#[derive(Debug)]
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
            map: |p, sampled| 0.5 * (1.0 + f64::sin(p.z() + 10.0 * sampled)),
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
