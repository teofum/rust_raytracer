use std::error::Error;
use std::f64::consts::PI;
use std::fs::File;
use std::sync::Arc;

use crate::camera::Camera;
use crate::config::{Config, SceneConfig, DEFAULT_SCENE_CONFIG};
use crate::loaders::obj::load_mesh_from_file;
use crate::material::{LambertianDiffuse, Material, Metal};
use crate::noise::PerlinNoise3D;
use crate::object::transform::Transform;
use crate::object::{Hit, ObjectList, Plane, Sky, Sphere};
use crate::texture::{ConstantTexture, Interpolate, NoiseSolidTexture};
use crate::vec4::Vec4;
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;

use super::{Scene, SceneData};

pub struct PerlinScene;

impl Scene for PerlinScene {
    fn init(config: Config) -> Result<SceneData, Box<dyn Error>> {
        let scene_defaults = SceneConfig {
            output_width: Some(600),
            aspect_ratio: Some(1.5),
            focal_length: Some(70.0),
            f_number: Some(4.0),
            focus_distance: None,
            camera_pos: Some(Vec4::point(13.0, 1.0, 4.0)),
            camera_target: Some(Vec4::point(0.0, 0.0, 0.0)),
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
        let mut rng = Pcg64Mcg::from_rng(rand::thread_rng()).unwrap();

        let noise_perlin = Box::new(PerlinNoise3D::new(&mut rng));
        let mut tex_marble = NoiseSolidTexture::new(noise_perlin);
        tex_marble.scale = Vec4::vec(2.0, 2.0, 2.0);
        tex_marble.map = |p, sampled| 0.5 * (1.0 + f64::sin(p.z() + 10.0 * sampled));
        let mat_marble: Arc<dyn Material> =
            Arc::new(LambertianDiffuse::new(Arc::new(Interpolate::new(
                Arc::new(ConstantTexture::from_values(0.2, 0.15, 0.1)),
                Arc::new(ConstantTexture::from_values(1.0, 1.0, 1.0)),
                Arc::new(tex_marble),
            ))));

        let mat_floor: Arc<dyn Material> = Arc::new(Metal::new(
            Arc::new(ConstantTexture::from_values(0.8, 0.8, 0.8)),
            Arc::new(ConstantTexture::new(0.02)),
        ));

        // Set up objects
        let sky = Sky::new(Arc::new(ConstantTexture::from_values(1.0, 1.0, 1.0)));
        let sky: Arc<dyn Hit> = Arc::new(sky);

        let sphere = Sphere::new(Vec4::point(0.0, 0.0, 1.5), 1.0, Arc::clone(&mat_marble));

        let floor = Plane::new(
            Vec4::point(0.0, -1.0, 0.0),
            (Vec4::vec(-10.0, 0.0, 0.0), Vec4::vec(0.0, 0.0, 10.0)),
            mat_floor,
        );

        let mesh_file = File::open("scenes/resource/monkey.obj")?;
        let mesh = load_mesh_from_file(&mesh_file, Arc::clone(&mat_marble))?;
        let mut mesh = Transform::new(Arc::new(mesh));
        mesh.scale_uniform(1.5);
        mesh.rotate_y(PI / 4.0);
        mesh.translate(0.0, 0.45, -2.0);

        let mut world = ObjectList::new();
        world.add(Arc::new(sphere));
        world.add(Arc::new(floor));
        world.add(Arc::new(mesh));
        world.add(Arc::clone(&sky));

        let world = Arc::new(world);

        let mut lights = ObjectList::new();
        lights.add(sky);

        let lights = Arc::new(lights);

        Ok((camera, world, lights))
    }
}
