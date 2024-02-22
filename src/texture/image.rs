use std::error::Error;

use crate::buffer::Buffer;
use crate::vec4::{Color, Point4};

use super::Texture;

pub enum TextureRepeat {
    Clamp,
    Repeat,
}

pub struct ImageTexture {
    buffer: Buffer,
    pub repeat: TextureRepeat,
}

impl ImageTexture {
    pub fn from_buffer(buffer: Buffer) -> Self {
        ImageTexture {
            buffer,
            repeat: TextureRepeat::Repeat,
        }
    }

    pub fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let buffer = Buffer::from_image(file_path)?;
        Ok(ImageTexture {
            buffer,
            repeat: TextureRepeat::Repeat,
        })
    }
}

impl Texture for ImageTexture {
    fn sample(&self, (u, v): (f64, f64), _: &Point4) -> Color {
        // Handle repeating if UV outside [0; 1] range
        let (u, v) = match self.repeat {
            TextureRepeat::Clamp => (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0)),
            TextureRepeat::Repeat => (u - u.floor(), v - v.floor()),
        };

        // Nearest-neighbor sampling
        let (width, height) = self.buffer.size();
        let (width, height) = (width as f64 - 0.001, height as f64 - 0.001);
        let (x, y) = ((u * width) as usize, (v * height) as usize);

        let sampled = self.buffer.get_pixel(x, y);
        sampled
    }
}
