use std::io;
use std::sync::Arc;

use rust_raytracer::camera::Camera;
use rust_raytracer::material::{LambertianDiffuse, Material};
use rust_raytracer::object::{Hit, ObjectList, Sphere};
use rust_raytracer::texture::{CheckerboardTexture, ConstantColorTexture};
use rust_raytracer::vec3::Vec3;

use super::Scene;

// Config variables
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const OUTPUT_WIDTH: usize = 600;
const FOCAL_LENGTH: f64 = 70.0;

pub struct TestScene2;

impl Scene for TestScene2 {
    fn init() -> io::Result<(Camera, Arc<dyn Hit>)> {
        // Set up camera
        let mut camera = Camera::new(OUTPUT_WIDTH, ASPECT_RATIO, FOCAL_LENGTH);
        camera.move_and_look_at(Vec3(13.0, 2.0, 3.0), Vec3::origin());
        // camera.set_f_number(Some(2.8));

        // Set up materials
        let material: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(
            CheckerboardTexture::new(
                Arc::new(ConstantColorTexture::from_values(0.2, 0.3, 0.1)),
                Arc::new(ConstantColorTexture::from_values(0.9, 0.9, 0.9)),
                0.02,
            ),
        )));

        // Set up objects
        let bottom = Box::new(Sphere::new(
            Vec3(0.0, -10.0, 0.0),
            10.0,
            Arc::clone(&material),
        ));
        let top = Box::new(Sphere::new(
            Vec3(0.0, 10.0, 0.0),
            10.0,
            Arc::clone(&material),
        ));

        let mut world = ObjectList::new();
        world.add(top);
        world.add(bottom);

        let world = Arc::new(world);

        Ok((camera, world))
    }
}
