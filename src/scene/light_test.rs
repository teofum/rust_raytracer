use std::error::Error;
use std::fs::File;
use std::sync::Arc;

use crate::camera::Camera;
use crate::config::{Config, SceneConfig, DEFAULT_SCENE_CONFIG};
use crate::loaders::obj::load_mesh_from_file;
use crate::material::{Emissive, LambertianDiffuse, Material, Metal};
use crate::object::transform::Transform;
use crate::object::{Hit, ObjectList, Plane, Sphere};
use crate::texture::{CheckerboardTexture, ConstantTexture};
use crate::vec4::Vec4;

use super::{Scene, SceneData};

pub struct LightTestScene;

impl Scene for LightTestScene {
    fn init(config: Config) -> Result<SceneData, Box<dyn Error>> {
        let scene_defaults = SceneConfig {
            output_width: Some(600),
            aspect_ratio: Some(1.5),
            focal_length: Some(70.0),
            f_number: Some(4.0),
            focus_distance: None,
            camera_pos: Some(Vec4::point(10.0, 1.0, 6.0)),
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
        let mat_ground: Arc<dyn Material> =
            Arc::new(LambertianDiffuse::new(Arc::new(CheckerboardTexture::new(
                Arc::new(ConstantTexture::from_values(0.2, 0.3, 0.1)),
                Arc::new(ConstantTexture::from_values(0.9, 0.9, 0.9)),
                0.02,
            ))));
        let mat_metal: Arc<dyn Material> = Arc::new(Metal::new(
            Arc::new(ConstantTexture::from_values(0.8, 0.6, 0.2)),
            0.05,
        ));
        let mat_light: Arc<dyn Material> = Arc::new(Emissive::new(Arc::new(
            ConstantTexture::from_values(7.0, 1.0, 7.0),
        )));
        let mat_light_2: Arc<dyn Material> = Arc::new(Emissive::new(Arc::new(
            ConstantTexture::from_values(1.0, 6.0, 8.0),
        )));

        // Set up objects
        let sphere = Sphere::new(Vec4::vec(-1.0, 0.0, 1.0), 0.5, mat_light);
        let sphere: Arc<dyn Hit> = Arc::new(sphere);
        let sphere_2 = Sphere::new(Vec4::vec(2.0, 0.5, -1.2), 0.4, mat_light_2);
        let sphere_2: Arc<dyn Hit> = Arc::new(sphere_2);

        let floor = Plane::new(
            Vec4::point(0.0, -1.0, 0.0),
            (Vec4::vec(-10.0, 0.0, 0.0), Vec4::vec(0.0, 0.0, 10.0)),
            mat_ground,
        );

        let mesh_file = File::open("monkey.obj")?;
        let mesh = load_mesh_from_file(&mesh_file, mat_metal)?;
        let mut mesh = Transform::new(Box::new(mesh));
        mesh.translate(0.0, 0.0, -1.5);

        let mut world = ObjectList::new();
        world.add(Arc::clone(&sphere));
        world.add(Arc::clone(&sphere_2));
        world.add(Arc::new(floor));
        world.add(Arc::new(mesh));

        let world = Arc::new(world);

        let mut lights = ObjectList::new();
        lights.add(sphere);
        lights.add(sphere_2);

        let lights = Arc::new(lights);

        Ok((camera, world, lights))
    }
}
