use std::fs::File;
use std::io;
use std::rc::Rc;

use rust_raytracer::camera::Camera;
use rust_raytracer::material::dielectric::Dielectric;
use rust_raytracer::material::{LambertianDiffuse, Material, Metal};
use rust_raytracer::object::{ObjectList, Sphere};
use rust_raytracer::ppm;
use rust_raytracer::vec3::Vec3;

// Config variables

/// Aspect ratio of the output image
const ASPECT_RATIO: f64 = 3.0 / 2.0;

/// Output image width, in pixels. Height is determined by width and aspect ratio.
const OUTPUT_WIDTH: usize = 600;

/// Camera focal length, in millimetres for 35mm-equivalent FOV
const FOCAL_LENGTH: f64 = 24.0;

fn main() -> io::Result<()> {
    // Set up camera
    let camera = Camera::new(OUTPUT_WIDTH, ASPECT_RATIO, FOCAL_LENGTH / 1000.0);

    let mat_default: Rc<dyn Material> = Rc::new(LambertianDiffuse::new(Vec3(0.75, 0.25, 0.25)));
    let mat_ground: Rc<dyn Material> = Rc::new(LambertianDiffuse::new(Vec3(0.7, 0.8, 0.0)));
    let mat_metal: Rc<dyn Material> = Rc::new(Metal::new(Vec3(0.8, 0.8, 0.8), 0.2));
    let mat_glass: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));

    // Set up objects
    let sphere = Sphere {
        center: Vec3(-0.6, 0.0, -2.0),
        radius: 0.5,
        material: Rc::clone(&mat_default),
    };

    let sphere2 = Sphere {
        center: Vec3(0.6, 0.0, -4.0),
        radius: 0.5,
        material: Rc::clone(&mat_metal),
    };

    let sphere3 = Sphere {
        center: Vec3(0.9, -0.2, -2.5),
        radius: 0.3,
        material: Rc::clone(&mat_glass),
    };

    let sphere3_inner = Sphere {
        center: Vec3(0.9, -0.2, -2.5),
        radius: -0.28,
        material: Rc::clone(&mat_glass),
    };

    let ground = Sphere {
        center: Vec3(0.0, -100.5, -3.0),
        radius: 100.0,
        material: Rc::clone(&mat_ground),
    };

    let mut world = ObjectList::new();
    world.add(Box::new(sphere));
    world.add(Box::new(sphere2));
    world.add(Box::new(sphere3));
    world.add(Box::new(sphere3_inner));
    world.add(Box::new(ground));

    // Output
    let mut buf = camera.create_buffer();
    camera.render(&world, &mut buf);

    println!("Done! Writing output to file...");

    let mut file = File::create("out.ppm")?;
    ppm::write_to_file(&mut file, &buf)?;

    println!("Done! Goodbye :)");

    Ok(())
}
