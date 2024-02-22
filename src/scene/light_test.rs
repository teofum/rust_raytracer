use std::error::Error;
use std::fs::File;
use std::sync::Arc;

use rust_raytracer::camera::Camera;
use rust_raytracer::loaders::obj::load_mesh_from_file;
use rust_raytracer::mat4::Mat4;
use rust_raytracer::material::{Emissive, LambertianDiffuse, Material, Metal};
use rust_raytracer::object::transform::Transform;
use rust_raytracer::object::{Hit, ObjectList, Plane, Sphere};
use rust_raytracer::texture::{CheckerboardTexture, ConstantColorTexture};
use rust_raytracer::vec4::Vec4;

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
        camera.move_and_look_at(Vec4::point(10.0, 1.0, 6.0), Vec4::point(0.0, 0.0, 0.0));
        camera.set_f_number(Some(4.0));

        // Set up materials
        let mat_ground: Arc<dyn Material> =
            Arc::new(LambertianDiffuse::new(Arc::new(CheckerboardTexture::new(
                Arc::new(ConstantColorTexture::from_values(0.2, 0.3, 0.1)),
                Arc::new(ConstantColorTexture::from_values(0.9, 0.9, 0.9)),
                0.02,
            ))));
        let mat_metal: Arc<dyn Material> = Arc::new(Metal::new(Vec4::vec(0.8, 0.6, 0.2), 0.05));
        let mat_light: Arc<dyn Material> = Arc::new(Emissive::new(Arc::new(
            ConstantColorTexture::from_values(7.0, 1.0, 7.0),
        )));
        let mat_light_2: Arc<dyn Material> = Arc::new(Emissive::new(Arc::new(
            ConstantColorTexture::from_values(1.0, 6.0, 8.0),
        )));

        // Set up objects
        let sphere = Sphere::new(Vec4::vec(-1.0, 0.0, 1.0), 0.5, mat_light);
        let sphere_2 = Sphere::new(Vec4::vec(2.0, 0.5, -1.2), 0.4, mat_light_2);

        let floor = Plane::new(
            Vec4::point(0.0, -1.0, 0.0),
            (Vec4::vec(-10.0, 0.0, 0.0), Vec4::vec(0.0, 0.0, 10.0)),
            mat_ground,
        );

        let mesh_file = File::open("monkey.obj")?;
        let mesh = load_mesh_from_file(&mesh_file, mat_metal)?;
        let mesh_transform = Mat4::translation(0.0, 0.0, -1.5);
        let mesh = Transform::new(Box::new(mesh), mesh_transform);

        let mut world = ObjectList::new();
        world.add(Box::new(sphere));
        world.add(Box::new(sphere_2));
        world.add(Box::new(floor));
        world.add(Box::new(mesh));

        let world = Arc::new(world);

        Ok((camera, world))
    }
}
