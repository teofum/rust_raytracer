use std::error::Error;
use std::sync::Arc;

use rust_raytracer::camera::Camera;
use rust_raytracer::material::{LambertianDiffuse, Material};
use rust_raytracer::object::{Hit, ObjectList, Sphere};
use rust_raytracer::texture::ImageTexture;
use rust_raytracer::vec3::Vec3;

use super::Scene;

// Config variables
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const OUTPUT_WIDTH: usize = 600;
const FOCAL_LENGTH: f64 = 70.0;

pub struct TestScene2;

impl Scene for TestScene2 {
    fn init() -> Result<(Camera, Arc<dyn Hit>), Box<dyn Error>> {
        // Set up camera
        let mut camera = Camera::new(OUTPUT_WIDTH, ASPECT_RATIO, FOCAL_LENGTH);
        camera.move_and_look_at(Vec3(13.0, 2.0, 3.0), Vec3::origin());
        // camera.set_f_number(Some(2.8));

        // Set up materials
        let tex_earth = ImageTexture::from_file("resource/earthmap.jpg")?;
        let material: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(tex_earth)));

        // Set up objects
        let earth = Box::new(Sphere::new(Vec3(0.0, 0.0, 0.0), 2.0, Arc::clone(&material)));

        let mut world = ObjectList::new();
        world.add(earth);

        let world = Arc::new(world);

        Ok((camera, world))
    }
}
