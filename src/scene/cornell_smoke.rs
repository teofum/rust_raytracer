use std::error::Error;
use std::sync::Arc;

use crate::camera::Camera;
use crate::config::{Config, DEFAULT_SCENE_CONFIG, SceneConfig};
use crate::material::{Emissive, Isotropic, LambertianDiffuse, Material};
use crate::object::{Hit, make_box, ObjectList, Plane, Transform, Volume};
use crate::texture::ConstantTexture;
use crate::utils::deg_to_rad;
use crate::vec4::Vec4;

use super::{SceneData, SceneInit};

pub struct CornellSmokeScene;

impl SceneInit for CornellSmokeScene {
    fn init(config: Config) -> Result<SceneData, Box<dyn Error>> {
        let scene_defaults = SceneConfig {
            output_width: Some(600),
            aspect_ratio: Some(1.0),
            focal_length: Some(35.0),
            f_number: None,
            focus_distance: None,
            camera_pos: Some(Vec4::point(0.0, 0.0, 110.0)),
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
        let mat_white: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(
            ConstantTexture::from_values(0.73, 0.73, 0.73),
        )));
        let mat_green: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(
            ConstantTexture::from_values(0.12, 0.45, 0.15),
        )));
        let mat_red: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(
            ConstantTexture::from_values(0.65, 0.05, 0.05),
        )));
        let mat_light: Arc<dyn Material> = Arc::new(Emissive::new(Arc::new(
            ConstantTexture::from_values(15.0, 15.0, 15.0),
        )));

        let mat_smoke: Arc<dyn Material> = Arc::new(Isotropic::new(Arc::new(
            ConstantTexture::from_values(0.0, 0.0, 0.0),
        )));
        let mat_fog: Arc<dyn Material> = Arc::new(Isotropic::new(Arc::new(
            ConstantTexture::from_values(1.0, 1.0, 1.0),
        )));

        // Set up objects
        let floor = Plane::new(
            Vec4::point(0.0, -27.5, 0.0),
            (Vec4::vec(-27.5, 0.0, 0.0), Vec4::vec(0.0, 0.0, 27.5)),
            Arc::clone(&mat_white),
        );
        let ceiling = Plane::new(
            Vec4::point(0.0, 27.5, 0.0),
            (Vec4::vec(27.5, 0.0, 0.0), Vec4::vec(0.0, 0.0, -27.5)),
            Arc::clone(&mat_white),
        );
        let back_wall = Plane::new(
            Vec4::point(0.0, 0.0, -27.5),
            (Vec4::vec(0.0, 27.5, 0.0), Vec4::vec(-27.5, 0.0, 0.0)),
            Arc::clone(&mat_white),
        );
        let left_wall = Plane::new(
            Vec4::point(-27.5, 0.0, 0.0),
            (Vec4::vec(0.0, 27.5, 0.0), Vec4::vec(0.0, 0.0, -27.5)),
            Arc::clone(&mat_green),
        );
        let right_wall = Plane::new(
            Vec4::point(27.5, 0.0, 0.0),
            (Vec4::vec(0.0, 27.5, 0.0), Vec4::vec(0.0, 0.0, 27.5)),
            Arc::clone(&mat_red),
        );

        let light = Plane::new(
            Vec4::point(0.0, 27.49, 0.0),
            (Vec4::vec(13.0, 0.0, 0.0), Vec4::vec(0.0, 0.0, 10.5)),
            Arc::clone(&mat_light),
        );
        let light: Arc<dyn Hit> = Arc::new(light);

        let mut box1 = make_box(
            Vec4::point(0.0, 0.0, 0.0),
            Vec4::vec(16.5, 16.5, 16.5),
            Arc::clone(&mat_white),
        );
        box1.disable_bounds_check = true;
        let mut box1 = Transform::new(Arc::new(box1));
        box1.rotate_y(deg_to_rad(-15.0));
        box1.translate(27.5 - 21.25, 8.25 - 27.5, 27.5 - 14.75);
        let box1 = Volume::new(Arc::new(box1), mat_smoke, 0.15);

        let mut box2 = make_box(
            Vec4::point(0.0, 0.0, 0.0),
            Vec4::vec(16.5, 33.0, 16.5),
            Arc::clone(&mat_white),
        );
        box2.disable_bounds_check = true;
        let mut box2 = Transform::new(Arc::new(box2));
        box2.rotate_y(deg_to_rad(18.0));
        box2.translate(27.5 - 34.75, 16.5 - 27.5, 27.5 - 37.75);
        let box2 = Volume::new(Arc::new(box2), mat_fog, 0.15);

        let mut world = ObjectList::new();
        world.add(Arc::new(floor));
        world.add(Arc::new(ceiling));
        world.add(Arc::new(back_wall));
        world.add(Arc::new(left_wall));
        world.add(Arc::new(right_wall));
        world.add(Arc::clone(&light));
        world.add(Arc::new(box1));
        world.add(Arc::new(box2));

        let world = Arc::new(world);

        let mut lights = ObjectList::new();
        lights.add(light);

        let lights = Arc::new(lights);

        Ok((camera, world, lights))
    }
}
