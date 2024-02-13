use std::{
    fs::File,
    io::{self, Write},
};

pub fn write_to_file(file: &mut File, width: u32, height: u32) -> io::Result<()> {
    file.write_all(format!("P3\n{width} {height}\n255\n").as_bytes())?;

    // Generate test image
    for y in 0..height {
        for x in 0..width {
            let r = x as f64 / (width - 1) as f64;
            let g = y as f64 / (height - 1) as f64;
            let b = 0.0;

            let r = (r * 255.999) as u8;
            let g = (g * 255.999) as u8;
            let b = (b * 255.999) as u8;

            file.write_all(format!("{r} {g} {b}\n").as_bytes())?;
        }
    }

    Ok(())
}
