use std::error::Error;
use std::fs::File;
use std::sync::Arc;

use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use rust_raytracer::camera::Camera;
use rust_raytracer::loaders::obj::load_mesh_from_file;
use rust_raytracer::material::{LambertianDiffuse, Material, Metal};
use rust_raytracer::noise::PerlinNoise3D;
use rust_raytracer::object::{Hit, ObjectList, Plane, Sphere};
use rust_raytracer::texture::NoiseSolidTexture;
use rust_raytracer::vec3::Vec3;

use super::Scene;

// Config variables
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const OUTPUT_WIDTH: usize = 600;
const FOCAL_LENGTH: f64 = 70.0;

pub struct PerlinScene;

impl Scene for PerlinScene {
    fn init() -> Result<(Camera, Arc<dyn Hit>), Box<dyn Error>> {
        // Set up camera
        let mut camera = Camera::new(OUTPUT_WIDTH, ASPECT_RATIO, FOCAL_LENGTH);
        camera.move_and_look_at(Vec3(13.0, 1.0, 4.0), Vec3::origin());
        camera.set_f_number(Some(4.0));
        camera.background_color = |ray| {
            let unit_dir = ray.dir.to_unit();
            let t = 0.5 * (unit_dir.y() + 1.0);

            Vec3::lerp(Vec3(5.0, 5.0, 5.0), Vec3(1.0, 1.4, 2.0), t)
        };

        // Set up materials
        let mut rng = XorShiftRng::from_rng(rand::thread_rng()).unwrap();

        let noise_perlin = Box::new(PerlinNoise3D::new(&mut rng));
        let mut tex_marble = NoiseSolidTexture::new(noise_perlin);
        tex_marble.scale = Vec3(2.0, 2.0, 2.0);
        tex_marble.map = |p, sampled| 0.5 * (1.0 + f64::sin(p.z() + 10.0 * sampled));
        let mat_marble: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(tex_marble)));

        let mat_floor: Arc<dyn Material> = Arc::new(Metal::new(Vec3(0.8, 0.8, 0.8), 0.02));

        // Set up objects
        let sphere = Sphere::new(Vec3(0.0, 0.0, 1.5), 1.0, Arc::clone(&mat_marble));

        let floor = Plane::new(
            Vec3(0.0, -1.0, 0.0),
            (Vec3(-10.0, 0.0, 0.0), Vec3(0.0, 0.0, 10.0)),
            mat_floor,
        );

        let mesh_file = File::open("monkey.obj")?;
        let mut mesh = load_mesh_from_file(&mesh_file, Arc::clone(&mat_marble))?;
        mesh.set_position(Vec3(0.0, 0.0, -1.5));

        let mut world = ObjectList::new();
        world.add(Box::new(sphere));
        world.add(Box::new(floor));
        world.add(Box::new(mesh));

        let world = Arc::new(world);

        Ok((camera, world))
    }
}
