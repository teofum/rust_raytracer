use std::fs::File;
use std::io;
use std::sync::Arc;
use std::time::Instant;

use rust_raytracer::camera::Camera;
use rust_raytracer::material::dielectric::Dielectric;
use rust_raytracer::material::{LambertianDiffuse, Material, Metal};
use rust_raytracer::object::plane::Plane;
use rust_raytracer::object::{ObjectList, Sphere};
use rust_raytracer::ppm;
use rust_raytracer::vec3::Vec3;

// Config variables

/// Aspect ratio of the output image
const ASPECT_RATIO: f64 = 3.0 / 2.0;

/// Output image width, in pixels. Height is determined by width and aspect ratio.
const OUTPUT_WIDTH: usize = 600;

/// Camera focal length, in millimetres for 35mm-equivalent FOV
const FOCAL_LENGTH: f64 = 70.0;

fn main() -> io::Result<()> {
    let time = Instant::now();

    // Set up camera
    let mut camera = Camera::new(OUTPUT_WIDTH, ASPECT_RATIO, FOCAL_LENGTH);
    camera.move_and_look_at(Vec3(-2.0, 2.0, 1.0), Vec3(0.0, 0.0, -1.0));
    camera.set_f_number(Some(2.0));

    let mat_ground: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Vec3(0.7, 0.8, 0.0)));
    let mat_diffuse: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Vec3(0.1, 0.2, 0.5)));
    let mat_metal: Arc<dyn Material> = Arc::new(Metal::new(Vec3(0.8, 0.6, 0.2), 0.1));
    let mat_glass: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));

    // Set up objects
    let sphere = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::clone(&mat_diffuse),
    };

    let sphere_metal = Sphere {
        center: Vec3(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::clone(&mat_metal),
    };

    let sphere_glass = Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Arc::clone(&mat_glass),
    };

    let sphere_glass_inner = Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: -0.4,
        material: Arc::clone(&mat_glass),
    };

    let ground = Plane::new(
        Vec3(0.0, -0.5, -1.0),
        (Vec3(4.0, 0.0, 0.0), Vec3(0.0, 0.0, 3.0)),
        Arc::clone(&mat_ground),
    );

    let wall = Plane::new(
        Vec3(0.0, 1.0, -2.5),
        (Vec3(4.0, 0.0, 0.0), Vec3(0.0, 3.0, 0.0)),
        Arc::clone(&mat_metal),
    );

    let mut world = ObjectList::new();
    world.add(Box::new(sphere));
    world.add(Box::new(sphere_metal));
    world.add(Box::new(sphere_glass));
    world.add(Box::new(sphere_glass_inner));
    world.add(Box::new(ground));
    world.add(Box::new(wall));

    let world = Arc::new(world);

    let elapsed = time.elapsed();
    println!("Ready: {:.2?}", elapsed);

    // Output
    let mut buf = camera.create_buffer();
    camera.render(world, &mut buf);

    let elapsed = time.elapsed();
    println!("Done: {:.2?}. Writing output to file...", elapsed);

    let mut file = File::create("out.ppm")?;
    ppm::write_to_file(&mut file, &buf)?;

    let elapsed = time.elapsed();
    println!("Done! Took {:.2?}. Goodbye :)", elapsed);

    Ok(())
}
