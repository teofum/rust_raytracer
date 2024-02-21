use std::error::Error;
use std::sync::Arc;

use rust_raytracer::camera::Camera;
use rust_raytracer::material::{LambertianDiffuse, Material};
use rust_raytracer::object::{Hit, ObjectList, Plane, Sphere};
use rust_raytracer::texture::{ConstantColorTexture, ImageTexture};
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
        let mat_earth: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(tex_earth)));

        let mat_floor: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(
            ConstantColorTexture::from_values(0.5, 0.5, 0.5),
        )));

        // Set up objects
        let earth = Box::new(Sphere::new(Vec3(0.0, 0.0, 0.0), 1.5, mat_earth));

        let floor = Box::new(Plane::new(
            Vec3(0.0, -1.5, 0.0),
            (Vec3(-10.0, 0.0, 0.0), Vec3(0.0, 0.0, 10.0)),
            mat_floor,
        ));

        let mut world = ObjectList::new();
        world.add(earth);
        world.add(floor);

        let world = Arc::new(world);

        Ok((camera, world))
    }
}
