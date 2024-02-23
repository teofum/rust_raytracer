use image::{ImageBuffer, ImageResult, Rgb};

use crate::buffer::Buffer;
use crate::tonemapping::{self, TonemapFn};
use crate::vec4::Color;

const GAMMA: f64 = 1.0 / 2.4;

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
            let (r, g, b) = linear_to_srgb((self.tonemap)(color)).xyz();

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

fn linear_to_srgb(color: Color) -> Color {
    color.map_components(|x| {
        if x < 0.0031308 {
            x * 12.92
        } else {
            x.powf(GAMMA) * 1.055 - 0.055
        }
    })
}
