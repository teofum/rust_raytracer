use std::fs::File;
use std::io;

use rust_raytracer::buffer::Buffer;
use rust_raytracer::camera::Camera;
use rust_raytracer::object::{Hit, ObjectList, Sphere};
use rust_raytracer::ppm;
use rust_raytracer::ray::Ray;
use rust_raytracer::vec3::{Color, Vec3};

// Config variables

/// Aspect ratio of the output image
const ASPECT_RATIO: f64 = 3.0 / 2.0;

/// Output image width, in pixels. Height is determined by width and aspect ratio.
const OUTPUT_WIDTH: usize = 600;

/// Internal viewport height. Arbitrary value without units.
const VIEWPORT_HEIGHT: f64 = 2.0;

/// Camera focal length. Arbitrary value without units.
const FOCAL_LENGTH: f64 = 1.0;

fn main() -> io::Result<()> {
    // Set up camera
    let camera = Camera::new(OUTPUT_WIDTH, ASPECT_RATIO, VIEWPORT_HEIGHT, FOCAL_LENGTH);
    let (image_width, image_height) = camera.image_size();

    // Set up objects
    let sphere = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    let ground = Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
    };

    let mut world = ObjectList::new();
    world.add(Box::new(sphere));
    world.add(Box::new(ground));

    // Output
    let mut buf = Buffer::new(image_width, image_height);

    // Draw to the buffer
    for y in 0..image_height {
        print!("Rendering... [line {}/{image_height}]\r", y + 1);

        for x in 0..image_width {
            let ray = camera.get_ray(x, y);
            let color = ray_color(&ray, &world);

            buf.set_pixel(x, y, color);
        }
    }

    println!("Done! Writing output to file...");

    let mut file = File::create("out.ppm")?;
    ppm::write_to_file(&mut file, &buf)?;

    println!("Done! Goodbye :)");

    Ok(())
}

fn ray_color(ray: &Ray, object: &dyn Hit) -> Color {
    if let Some(hit) = object.test(&ray, 0.0, f64::INFINITY) {
        return (Vec3(1.0, 1.0, 1.0) + hit.normal()) * 0.5;
    }

    let unit_dir = ray.direction().to_unit();
    let t = 0.5 * (unit_dir.y() + 1.0);

    Vec3::lerp(Vec3(1.0, 1.0, 1.0), Vec3(0.5, 0.7, 1.0), t)
}
