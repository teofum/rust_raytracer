use std::{
    fs::File,
    io::{self, Write},
};

use crate::buffer::Buffer;
use crate::vec3::Color;

fn clamp(x: f64) -> f64 {
    x.clamp(0.0, 1.0)
}

fn format_color(color: &Color) -> String {
    let r = (clamp(color.r()) * 255.999) as u8;
    let g = (clamp(color.g()) * 255.999) as u8;
    let b = (clamp(color.b()) * 255.999) as u8;

    format!("{r} {g} {b}\n")
}

pub fn write_to_file(file: &mut File, buffer: &Buffer) -> io::Result<()> {
    let (width, height) = buffer.size();

    // Write header
    file.write_all(format!("P3\n{width} {height}\n255\n").as_bytes())?;

    // Write buffer to file
    for y in 0..height {
        for x in 0..width {
            let color = buffer.get_pixel(x, y);
            file.write_all(format_color(&color).as_bytes())?;
        }
    }

    Ok(())
}
