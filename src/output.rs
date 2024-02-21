use image::{ImageBuffer, ImageResult, Rgb};

use crate::buffer::Buffer;
use crate::tonemapping::{self, TonemapFn};

const GAMMA: f64 = 1.0 / 2.2;

pub struct Writer {
    pub tonemap: TonemapFn,

    buffer: Buffer,
}

impl Writer {
    pub fn new(buffer: Buffer) -> Self {
        Writer {
            buffer,
            tonemap: tonemapping::tonemap_clamp,
        }
    }

    pub fn save(&self, file_path: &str) -> ImageResult<()> {
        let (width, height) = self.buffer.size();
        let img = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
            let color = self.buffer.get_pixel(x as usize, y as usize);
            let (r, g, b) = (self.tonemap)(color)
                .map_components(|x| x.powf(GAMMA))
                .values();

            let (r, g, b) = (
                (r * 255.999) as u8,
                (g * 255.999) as u8,
                (b * 255.999) as u8,
            );

            Rgb([r, g, b])
        });

        img.save(file_path)
    }
}
