use std::fs::File;
use std::io;
use std::sync::Arc;
use std::time::Instant;

use rust_raytracer::camera::Camera;
use rust_raytracer::loaders::obj::load_mesh_from_file;
use rust_raytracer::material::{Dielectric, LambertianDiffuse, Material, Metal};
use rust_raytracer::object::mesh::{Triangle, TriangleMesh};
use rust_raytracer::object::{ObjectList, Plane, Sphere};
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
    camera.move_and_look_at(Vec3(-1.5, 1.5, 3.0), Vec3(0.0, 0.0, 0.0));
    camera.set_f_number(Some(8.0));

    // Set up materials
    let mat_ground: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Vec3(0.7, 0.8, 0.0)));
    let mat_diffuse: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Vec3(0.1, 0.2, 0.5)));
    let mat_metal: Arc<dyn Material> = Arc::new(Metal::new(Vec3(0.8, 0.6, 0.2), 0.05));
    // let mat_glass: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));

    // Set up objects
    let mesh_file = File::open("monkey_low.obj")?;
    let mesh = load_mesh_from_file(&mesh_file, Arc::clone(&mat_diffuse))?;

    let floor = Plane::new(
        Vec3(0.0, -1.0, 0.0),
        (Vec3(4.0, 0.0, 0.0), Vec3(0.0, 0.0, -4.0)),
        Arc::clone(&mat_ground),
    );

    let wall = Plane::new(
        Vec3(2.0, 0.5, 0.0),
        (Vec3(0.0, 3.0, 0.0), Vec3(0.0, 0.0, -4.0)),
        Arc::clone(&mat_metal),
    );

    let mut world = ObjectList::new();
    world.add(Box::new(mesh));
    world.add(Box::new(floor));
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
