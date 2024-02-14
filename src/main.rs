use std::fs::File;
use std::io;

use rust_raytracer::camera::Camera;
use rust_raytracer::ppm;
use rust_raytracer::ray::Ray;
use rust_raytracer::vec3::{Color, Vec3};

// Config variables

/// Aspect ratio of the output image
const ASPECT_RATIO: f64 = 3.0 / 2.0;

/// Output image width, in pixels. Height is determined by width and aspect ratio.
const OUTPUT_WIDTH: u32 = 600;

/// Internal viewport height. Arbitrary value without units.
const VIEWPORT_HEIGHT: f64 = 2.0;

/// Camera focal length. Arbitrary value without units.
const FOCAL_LENGTH: f64 = 1.0;

fn main() -> io::Result<()> {
    // Set up camera
    let camera = Camera::new(OUTPUT_WIDTH, ASPECT_RATIO, VIEWPORT_HEIGHT, FOCAL_LENGTH);
    let (image_width, image_height) = camera.image_size();

    // Output

    let mut file = File::create("out.ppm")?;
    ppm::write_to_file(&mut file, image_width, image_height)?;

    Ok(())
}

fn ray_color(r: &Ray) -> Color {
    Vec3::origin()
}
