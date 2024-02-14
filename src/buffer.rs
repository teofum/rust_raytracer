use crate::vec3::{Color, Vec3};

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
            data.push(Vec3::origin());
        }

        Buffer {
            width,
            height,
            data,
        }
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
