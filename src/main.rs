use std::fs::File;
use std::io;

use rust_raytracer::ppm;

fn main() -> io::Result<()> {
    let mut file = File::create("out.ppm")?;
    ppm::write_to_file(&mut file, 256, 256)?;

    Ok(())
}
