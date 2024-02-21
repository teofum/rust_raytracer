use std::error::Error;
use std::fs::File;
use std::sync::Arc;

use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use rust_raytracer::camera::Camera;
use rust_raytracer::loaders::obj::load_mesh_from_file;
use rust_raytracer::material::{Dielectric, LambertianDiffuse, Material, Metal};
use rust_raytracer::object::bvh::BoundingVolumeHierarchyNode;
use rust_raytracer::object::{Hit, ObjectList, Plane, Sphere};
use rust_raytracer::texture::{CheckerboardTexture, ConstantColorTexture};
use rust_raytracer::vec3::Vec3;

use super::Scene;

// Config variables
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const OUTPUT_WIDTH: usize = 600;
const FOCAL_LENGTH: f64 = 70.0;

pub struct GoldenMonkeyScene;

impl Scene for GoldenMonkeyScene {
    fn init() -> Result<(Camera, Arc<dyn Hit>), Box<dyn Error>> {
        // Set up camera
        let mut camera = Camera::new(OUTPUT_WIDTH, ASPECT_RATIO, FOCAL_LENGTH);
        camera.move_and_look_at(Vec3(5.0, 2.0, 9.0), Vec3(0.0, 0.5, 0.0));
        camera.set_f_number(Some(2.8));
        camera.focus(Some(10.0));
        camera.background_color = |ray| {
            let unit_dir = ray.dir.to_unit();
            let t = 0.5 * (unit_dir.y() + 1.0);

            Vec3::lerp(Vec3(10.0, 10.0, 10.0), Vec3(1.0, 1.4, 2.0), t)
        };

        // Set up materials
        let mat_ground: Arc<dyn Material> =
            Arc::new(LambertianDiffuse::new(Arc::new(CheckerboardTexture::new(
                Arc::new(ConstantColorTexture::from_values(0.2, 0.3, 0.1)),
                Arc::new(ConstantColorTexture::from_values(0.9, 0.9, 0.9)),
                0.02,
            ))));
        let mat_metal: Arc<dyn Material> = Arc::new(Metal::new(Vec3(0.8, 0.6, 0.2), 0.05));
        let mat_glass: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));

        // Set up objects
        let floor = Plane::new(
            Vec3(0.0, 0.0, 0.0),
            (Vec3(20.0, 0.0, 0.0), Vec3(0.0, 0.0, -20.0)),
            Arc::clone(&mat_ground),
        );

        let mesh_file = File::open("monkey.obj")?;
        let mut mesh = load_mesh_from_file(&mesh_file, Arc::clone(&mat_metal))?;
        mesh.set_position(Vec3(0.0, 1.0, 0.0));

        // Random spheres
        let mut rng = XorShiftRng::from_rng(rand::thread_rng()).unwrap();

        let mut random_spheres: Vec<Box<dyn Hit>> = Vec::with_capacity(21);
        for i in -10..11 {
            for j in -10..11 {
                let (x, z) = (i as f64, j as f64);
                let center = Vec3(
                    x + rng.gen_range(0.0..0.9),
                    0.2,
                    z + rng.gen_range(0.0..0.9),
                );

                if (center - Vec3(0.0, 0.2, 0.0)).length_squared() < 1.0 {
                    continue;
                }

                let mat_type = rng.gen_range(0.0..1.0);

                if mat_type < 0.95 {
                    let albedo = Vec3::random(&mut rng) * Vec3::random(&mut rng);
                    let material: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(
                        ConstantColorTexture::new(albedo),
                    )));

                    let sphere = Sphere::new(center, 0.2, material);
                    random_spheres.push(Box::new(sphere));
                } else {
                    let sphere = Sphere::new(center, 0.2, Arc::clone(&mat_glass));
                    let sphere_in = Sphere::new(center, -0.18, Arc::clone(&mat_glass));
                    random_spheres.push(Box::new(sphere));
                    random_spheres.push(Box::new(sphere_in));
                }
            }
        }

        let spheres_bvh = BoundingVolumeHierarchyNode::from(random_spheres, &mut rng);

        let mut world = ObjectList::new();
        world.add(Box::new(mesh));
        world.add(Box::new(floor));
        world.add(Box::new(spheres_bvh));

        let world = Arc::new(world);

        Ok((camera, world))
    }
}
