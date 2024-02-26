use std::ops::{Add, Mul};

use crate::vec4::Point4;

use super::{Sampler, TexturePointer};

pub struct Interpolate<T> {
    start: TexturePointer<T>,
    end: TexturePointer<T>,
    t: TexturePointer<f64>,
}

impl<T> Interpolate<T> {
    pub fn new(start: TexturePointer<T>, end: TexturePointer<T>, t: TexturePointer<f64>) -> Self {
        Interpolate { start, end, t }
    }
}

impl<T> Sampler for Interpolate<T>
where
    T: Send + Sync + Copy + Mul<f64, Output = T> + Add<T, Output = T>,
{
    type Output = T;

    fn sample(&self, uv: (f64, f64), p: &Point4) -> Self::Output {
        let t = self.t.sample(uv, p);

        if t == 0.0 {
            self.start.sample(uv, p)
        } else if t == 1.0 {
            self.end.sample(uv, p)
        } else {
            self.start.sample(uv, p) * (1.0 - t) + self.end.sample(uv, p) * t
        }
    }
}
