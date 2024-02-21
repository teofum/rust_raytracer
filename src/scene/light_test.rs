use std::error::Error;
use std::fs::File;
use std::sync::Arc;

use rust_raytracer::camera::Camera;
use rust_raytracer::loaders::obj::load_mesh_from_file;
use rust_raytracer::material::{Emissive, LambertianDiffuse, Material, Metal};
use rust_raytracer::object::{Hit, ObjectList, Plane, Sphere};
use rust_raytracer::texture::{CheckerboardTexture, ConstantColorTexture};
use rust_raytracer::vec3::Vec3;

use super::Scene;

// Config variables
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const OUTPUT_WIDTH: usize = 600;
const FOCAL_LENGTH: f64 = 70.0;

pub struct LightTestScene;

impl Scene for LightTestScene {
    fn init() -> Result<(Camera, Arc<dyn Hit>), Box<dyn Error>> {
        // Set up camera
        let mut camera = Camera::new(OUTPUT_WIDTH, ASPECT_RATIO, FOCAL_LENGTH);
        camera.move_and_look_at(Vec3(10.0, 1.0, 6.0), Vec3::origin());
        camera.set_f_number(Some(4.0));

        // Set up materials
        let mat_ground: Arc<dyn Material> =
            Arc::new(LambertianDiffuse::new(Arc::new(CheckerboardTexture::new(
                Arc::new(ConstantColorTexture::from_values(0.2, 0.3, 0.1)),
                Arc::new(ConstantColorTexture::from_values(0.9, 0.9, 0.9)),
                0.02,
            ))));
        let mat_metal: Arc<dyn Material> = Arc::new(Metal::new(Vec3(0.8, 0.6, 0.2), 0.05));
        let mat_light: Arc<dyn Material> = Arc::new(Emissive::new(Arc::new(
            ConstantColorTexture::from_values(4.0, 4.0, 4.0),
        )));

        // Set up objects
        let sphere = Sphere::new(Vec3(-1.0, 0.0, 1.0), 0.5, mat_light);

        let floor = Plane::new(
            Vec3(0.0, -1.0, 0.0),
            (Vec3(-10.0, 0.0, 0.0), Vec3(0.0, 0.0, 10.0)),
            mat_ground,
        );

        let mesh_file = File::open("monkey.obj")?;
        let mut mesh = load_mesh_from_file(&mesh_file, mat_metal)?;
        mesh.set_position(Vec3(0.0, 0.0, -1.5));

        let mut world = ObjectList::new();
        world.add(Box::new(sphere));
        world.add(Box::new(floor));
        world.add(Box::new(mesh));

        let world = Arc::new(world);

        Ok((camera, world))
    }
}
