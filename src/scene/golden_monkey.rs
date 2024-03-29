use std::error::Error;
use std::fs::File;
use std::sync::Arc;

use crate::camera::Camera;
use crate::config::{Config, SceneConfig, DEFAULT_SCENE_CONFIG};
use crate::loaders::obj::load_mesh_from_file;
use crate::material::{Dielectric, Glossy, LambertianDiffuse, Material, Metal};
use crate::object::bvh::{self, BoundingVolumeHierarchyNode};
use crate::object::transform::Transform;
use crate::object::{Hit, ObjectList, Plane, Sky, Sphere, Sun};
use crate::texture::{CheckerboardTexture, ConstantTexture};
use crate::vec4::Vec4;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

use super::{Scene, SceneData};

pub struct GoldenMonkeyScene;

impl Scene for GoldenMonkeyScene {
    fn init(config: Config) -> Result<SceneData, Box<dyn Error>> {
        let scene_defaults = SceneConfig {
            output_width: Some(600),
            aspect_ratio: Some(1.5),
            focal_length: Some(50.0),
            f_number: Some(2.8),
            focus_distance: None,
            camera_pos: Some(Vec4::point(5.0, 2.0, 9.0)),
            camera_target: Some(Vec4::point(0.0, 0.5, 0.0)),
        };

        let scene_config = SceneConfig::merge(
            &SceneConfig::merge(&DEFAULT_SCENE_CONFIG, &scene_defaults),
            &config.scene,
        );

        let config = Config {
            scene: scene_config,
            ..config
        };

        // Set up camera
        let camera = Camera::new(&config);

        // Set up materials
        let mat_ground: Arc<dyn Material> =
            Arc::new(LambertianDiffuse::new(Arc::new(CheckerboardTexture::new(
                Arc::new(ConstantTexture::from_values(0.2, 0.3, 0.1)),
                Arc::new(ConstantTexture::from_values(0.9, 0.9, 0.9)),
                0.02,
            ))));
        let mat_metal: Arc<dyn Material> = Arc::new(Metal::new(
            Arc::new(ConstantTexture::from_values(0.8, 0.6, 0.2)),
            Arc::new(ConstantTexture::new(0.05)),
        ));
        let mat_glass: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));

        // Set up objects
        let sky = Sky::new(Arc::new(ConstantTexture::from_values(0.2, 0.6, 2.0)));
        let sky: Arc<dyn Hit> = Arc::new(sky);

        let sun = Sun::new(
            Arc::new(ConstantTexture::from_values(20.0, 20.0, 20.0)),
            Vec4::vec(-1.0, 1.0, 0.0),
        );
        let sun: Arc<dyn Hit> = Arc::new(sun);

        let floor = Plane::new(
            Vec4::point(0.0, 0.0, 0.0),
            (Vec4::vec(20.0, 0.0, 0.0), Vec4::vec(0.0, 0.0, -20.0)),
            Arc::clone(&mat_ground),
        );

        let mesh_file = File::open("scenes/resource/monkey.obj")?;
        let mesh = load_mesh_from_file(&mesh_file, Arc::clone(&mat_metal))?;
        let mut mesh = Transform::new(Arc::new(mesh));
        mesh.translate(0.0, 1.0, 0.0);

        // Random spheres
        let mut rng = Pcg64Mcg::from_rng(rand::thread_rng()).unwrap();

        let mut random_spheres: Vec<Arc<dyn Hit>> = Vec::with_capacity(21);
        for i in -10..11 {
            for j in -10..11 {
                let (x, z) = (i as f64, j as f64);
                let center = Vec4::point(
                    x + rng.gen_range(0.0..0.9),
                    0.2,
                    z + rng.gen_range(0.0..0.9),
                );

                if (center - Vec4::vec(0.0, 0.2, 0.0)).length_squared() < 1.0 {
                    continue;
                }

                let mat_type = rng.gen_range(0.0..1.0);

                if mat_type < 0.95 {
                    let albedo = Vec4::random_vec(&mut rng) * Vec4::random_vec(&mut rng);
                    let material: Arc<dyn Material> = Arc::new(Glossy::new(
                        Arc::new(ConstantTexture::new(albedo)),
                        Arc::new(ConstantTexture::new(0.1)),
                        1.5,
                    ));

                    let sphere = Sphere::new(center, 0.2, material);
                    random_spheres.push(Arc::new(sphere));
                } else {
                    let sphere = Sphere::new(center, 0.2, Arc::clone(&mat_glass));
                    let sphere_in = Sphere::new(center, -0.18, Arc::clone(&mat_glass));
                    random_spheres.push(Arc::new(sphere));
                    random_spheres.push(Arc::new(sphere_in));
                }
            }
        }

        let spheres_bvh = BoundingVolumeHierarchyNode::from(random_spheres, bvh::AXES_XZ, &mut rng);

        let mut world = ObjectList::new();
        world.add(Arc::new(mesh));
        world.add(Arc::new(floor));
        world.add(Arc::new(spheres_bvh));
        world.add(Arc::clone(&sky));
        world.add(Arc::clone(&sun));

        let world = Arc::new(world);

        let mut lights = ObjectList::new();
        lights.add(sky);
        lights.add(sun);

        let lights = Arc::new(lights);

        Ok((camera, world, lights))
    }
}
