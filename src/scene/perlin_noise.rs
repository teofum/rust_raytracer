use std::error::Error;
use std::f64::consts::PI;
use std::fs::File;
use std::sync::Arc;

use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;
use rust_raytracer::camera::Camera;
use rust_raytracer::loaders::obj::load_mesh_from_file;
use rust_raytracer::material::{LambertianDiffuse, Material, Metal};
use rust_raytracer::noise::PerlinNoise3D;
use rust_raytracer::object::transform::Transform;
use rust_raytracer::object::{Hit, ObjectList, Plane, Sphere};
use rust_raytracer::texture::{ConstantColorTexture, NoiseSolidTexture};
use rust_raytracer::vec4::Vec4;

use super::Scene;

// Config variables
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const OUTPUT_WIDTH: usize = 600;
const FOCAL_LENGTH: f64 = 70.0;

pub struct PerlinScene;

impl Scene for PerlinScene {
    fn init() -> Result<(Camera, Arc<dyn Hit>, Arc<dyn Hit>), Box<dyn Error>> {
        // Set up camera
        let mut camera = Camera::new(OUTPUT_WIDTH, ASPECT_RATIO, FOCAL_LENGTH);
        camera.move_and_look_at(Vec4::point(13.0, 1.0, 4.0), Vec4::point(0.0, 0.0, 0.0));
        camera.set_f_number(Some(4.0));
        camera.background_color = |ray| {
            let unit_dir = ray.dir.to_unit();
            let t = 0.5 * (unit_dir.y() + 1.0);

            Vec4::lerp(Vec4::vec(3.0, 3.0, 3.0), Vec4::vec(1.0, 1.4, 2.0), t)
        };

        // Set up materials
        let mut rng = Pcg64Mcg::from_rng(rand::thread_rng()).unwrap();

        let noise_perlin = Box::new(PerlinNoise3D::new(&mut rng));
        let mut tex_marble = NoiseSolidTexture::new(noise_perlin);
        tex_marble.scale = Vec4::vec(2.0, 2.0, 2.0);
        tex_marble.map = |p, sampled| 0.5 * (1.0 + f64::sin(p.z() + 10.0 * sampled));
        let mat_marble: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(tex_marble)));

        let mat_floor: Arc<dyn Material> = Arc::new(Metal::new(
            Arc::new(ConstantColorTexture::from_values(0.8, 0.8, 0.8)),
            0.02,
        ));

        // Set up objects
        let sphere = Sphere::new(Vec4::point(0.0, 0.0, 1.5), 1.0, Arc::clone(&mat_marble));

        let floor = Plane::new(
            Vec4::point(0.0, -1.0, 0.0),
            (Vec4::vec(-10.0, 0.0, 0.0), Vec4::vec(0.0, 0.0, 10.0)),
            mat_floor,
        );

        let mesh_file = File::open("monkey.obj")?;
        let mesh = load_mesh_from_file(&mesh_file, Arc::clone(&mat_marble))?;
        let mut mesh = Transform::new(Box::new(mesh));
        mesh.scale_uniform(1.5);
        mesh.rotate_y(PI / 4.0);
        mesh.translate(0.0, 0.45, -2.0);

        let mut world = ObjectList::new();
        world.add(Arc::new(sphere));
        world.add(Arc::new(floor));
        world.add(Arc::new(mesh));

        let world = Arc::new(world);

        Ok((camera, world, Arc::new(ObjectList::new())))
    }
}
