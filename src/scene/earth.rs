use std::error::Error;
use std::sync::Arc;

use crate::camera::Camera;
use crate::config::{Config, DEFAULT_SCENE_CONFIG, SceneConfig};
use crate::material::{LambertianDiffuse, Material};
use crate::object::{Hit, ObjectList, Plane, Sphere, Sun};
use crate::texture::{ConstantTexture, ImageTexture};
use crate::vec4::Vec4;

use super::{SceneData, SceneInit};

pub struct EarthScene;

impl SceneInit for EarthScene {
    fn init(config: Config) -> Result<SceneData, Box<dyn Error>> {
        let scene_defaults = SceneConfig {
            output_width: Some(600),
            aspect_ratio: Some(1.5),
            focal_length: Some(70.0),
            f_number: None,
            focus_distance: None,
            camera_pos: Some(Vec4::point(13.0, 2.0, 3.0)),
            camera_target: Some(Vec4::point(0.0, 0.0, 0.0)),
            background: None,
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
        let tex_earth = ImageTexture::from_file("scenes/resource/earthmap.jpg")?;
        let mat_earth: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(tex_earth)));

        let mat_floor: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(
            ConstantTexture::from_values(0.5, 0.5, 0.5),
        )));

        // Set up objects
        let sun = Sun::new(
            Arc::new(ConstantTexture::from_values(10.0, 10.0, 10.0)),
            Vec4::vec(0.0, 1.0, 2.0),
        );
        let sun: Arc<dyn Hit> = Arc::new(sun);

        let earth = Arc::new(Sphere::new(Vec4::point(0.0, 0.0, 0.0), 1.5, mat_earth));

        let floor = Arc::new(Plane::new(
            Vec4::point(0.0, -1.5, 0.0),
            (Vec4::vec(-10.0, 0.0, 0.0), Vec4::vec(0.0, 0.0, 10.0)),
            mat_floor,
        ));

        let mut world = ObjectList::new();
        world.add(earth);
        world.add(floor);
        world.add(Arc::clone(&sun));

        let world = Arc::new(world);

        let mut lights = ObjectList::new();
        lights.add(sun);

        let lights = Arc::new(lights);

        Ok((camera, world, lights))
    }
}
