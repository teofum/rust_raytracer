use std::error::Error;

use image::{io::Reader as ImageReader, Pixel};

use crate::vec4::{Color, Vec4};

#[derive(Debug)]
pub struct Buffer {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let mut data = Vec::with_capacity(width * height);

        // Initialize the buffer
        for _ in 0..(width * height) {
            data.push(Vec4::vec(0.0, 0.0, 0.0));
        }

        Buffer {
            width,
            height,
            data,
        }
    }

    pub fn from_image(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let image = ImageReader::open(file_path)?.decode()?.into_rgb32f();

        let width = image.width() as usize;
        let height = image.height() as usize;
        let mut data = Vec::with_capacity(width * height);

        // Initialize the buffer with image contents
        for p in image.pixels() {
            let rgb = p.channels();
            data.push(Vec4::vec(rgb[0] as f64, rgb[1] as f64, rgb[2] as f64));
        }

        Ok(Buffer {
            width,
            height,
            data,
        })
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.data[y * self.width + x]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.data[y * self.width + x] = color;
    }
}
