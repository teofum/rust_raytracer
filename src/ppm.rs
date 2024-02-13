use std::{
    fs::File,
    io::{self, Write},
};

use crate::vec3::{Color, Vec3};

fn format_color(color: &Color) -> String {
    let r = (color.r() * 255.999) as u8;
    let g = (color.g() * 255.999) as u8;
    let b = (color.b() * 255.999) as u8;

    format!("{r} {g} {b}\n")
}

pub fn write_to_file(file: &mut File, width: u32, height: u32) -> io::Result<()> {
    file.write_all(format!("P3\n{width} {height}\n255\n").as_bytes())?;

    // Generate test image
    for y in 0..height {
        for x in 0..width {
            let color = Vec3(
                x as f64 / (width - 1) as f64,
                y as f64 / (height - 1) as f64,
                0.0,
            );

            file.write_all(format_color(&color).as_bytes())?;
        }
    }

    Ok(())
}
