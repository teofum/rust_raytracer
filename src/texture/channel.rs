use std::fmt::Debug;

use crate::vec4::{Color, Point4};

use super::{Sampler, TexturePointer};

#[derive(Debug)]
pub struct Channel {
    color: TexturePointer<Color>,
    channel: usize,
}

impl Channel {
    pub fn new(color: TexturePointer<Color>, channel: usize) -> Self {
        Channel { color, channel }
    }
}

impl Sampler for Channel {
    type Output = f64;

    fn sample(&self, uv: (f64, f64), p: &Point4) -> Self::Output {
        let color = self.color.sample(uv, p);
        color[self.channel]
    }
}
