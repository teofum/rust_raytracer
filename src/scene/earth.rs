use std::error::Error;
use std::sync::Arc;

use rust_raytracer::camera::Camera;
use rust_raytracer::material::{LambertianDiffuse, Material};
use rust_raytracer::object::{Hit, ObjectList, Plane, Sphere};
use rust_raytracer::texture::{ConstantColorTexture, ImageTexture};
use rust_raytracer::vec4::Vec4;

use super::Scene;

// Config variables
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const OUTPUT_WIDTH: usize = 600;
const FOCAL_LENGTH: f64 = 70.0;

pub struct EarthScene;

impl Scene for EarthScene {
    fn init() -> Result<(Camera, Arc<dyn Hit>, Arc<dyn Hit>), Box<dyn Error>> {
        // Set up camera
        let mut camera = Camera::new(OUTPUT_WIDTH, ASPECT_RATIO, FOCAL_LENGTH);
        camera.move_and_look_at(Vec4::point(13.0, 2.0, 3.0), Vec4::point(0.0, 0.0, 0.0));
        camera.background_color = |ray| {
            let unit_dir = ray.dir.to_unit();
            let t = 0.5 * (unit_dir.y() + 1.0);

            Vec4::lerp(Vec4::vec(5.0, 5.0, 5.0), Vec4::vec(1.0, 1.4, 2.0), t)
        };

        // Set up materials
        let tex_earth = ImageTexture::from_file("resource/earthmap.jpg")?;
        let mat_earth: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(tex_earth)));

        let mat_floor: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(
            ConstantColorTexture::from_values(0.5, 0.5, 0.5),
        )));

        // Set up objects
        let earth = Arc::new(Sphere::new(Vec4::point(0.0, 0.0, 0.0), 1.5, mat_earth));

        let floor = Arc::new(Plane::new(
            Vec4::point(0.0, -1.5, 0.0),
            (Vec4::vec(-10.0, 0.0, 0.0), Vec4::vec(0.0, 0.0, 10.0)),
            mat_floor,
        ));

        let mut world = ObjectList::new();
        world.add(earth);
        world.add(floor);

        let world = Arc::new(world);

        Ok((camera, world, Arc::new(ObjectList::new())))
    }
}
