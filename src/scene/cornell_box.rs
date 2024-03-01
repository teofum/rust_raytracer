use std::error::Error;
use std::sync::Arc;

use rust_raytracer::camera::Camera;
use rust_raytracer::config::{Config, SceneConfig, DEFAULT_SCENE_CONFIG};
use rust_raytracer::material::{Dielectric, Emissive, Glossy, LambertianDiffuse, Material};
use rust_raytracer::object::{make_box, Hit, ObjectList, Plane, Sphere, Transform};
use rust_raytracer::texture::{CheckerboardTexture, ConstantTexture};
use rust_raytracer::utils::deg_to_rad;
use rust_raytracer::vec4::Vec4;

use super::{Scene, SceneData};

pub struct CornellBoxScene;

impl Scene for CornellBoxScene {
    fn init(config: Config) -> Result<SceneData, Box<dyn Error>> {
        let scene_defaults = SceneConfig {
            output_width: Some(600),
            aspect_ratio: Some(1.0),
            focal_length: Some(33.0),
            f_number: None,
            focus_distance: None,
            camera_pos: Some(Vec4::point(277.5, 277.5, -800.0)),
            camera_target: Some(Vec4::point(277.5, 277.5, 0.0)),
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
        let mat_glass: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));

        let mat_gloss_test: Arc<dyn Material> = Arc::new(Glossy::new(
            Arc::new(ConstantTexture::from_values(0.95, 0.95, 0.95)),
            Arc::new(CheckerboardTexture::new(
                Arc::new(ConstantTexture::new(0.0)),
                Arc::new(ConstantTexture::new(1.0)),
                0.25,
            )),
            1.5,
        ));

        // Set up objects
        let floor = Plane::new(
            Vec4::point(277.5, 0.0, 277.5),
            (Vec4::vec(277.5, 0.0, 0.0), Vec4::vec(0.0, 0.0, -277.5)),
            Arc::clone(&mat_gloss_test),
        );
        let ceiling = Plane::new(
            Vec4::point(277.5, 555.0, 277.5),
            (Vec4::vec(277.5, 0.0, 0.0), Vec4::vec(0.0, 0.0, 277.5)),
            Arc::clone(&mat_white),
        );
        let back_wall = Plane::new(
            Vec4::point(277.5, 277.5, 555.0),
            (Vec4::vec(0.0, 277.5, 0.0), Vec4::vec(277.5, 0.0, 0.0)),
            Arc::clone(&mat_white),
        );
        let left_wall = Plane::new(
            Vec4::point(555.0, 277.5, 277.5),
            (Vec4::vec(0.0, 277.5, 0.0), Vec4::vec(0.0, 0.0, -277.5)),
            Arc::clone(&mat_green),
        );
        let right_wall = Plane::new(
            Vec4::point(0.0, 277.5, 277.5),
            (Vec4::vec(0.0, 277.5, 0.0), Vec4::vec(0.0, 0.0, 277.5)),
            Arc::clone(&mat_red),
        );

        let light = Plane::new(
            Vec4::point(277.5, 554.9, 277.5),
            (Vec4::vec(-65.0, 0.0, 0.0), Vec4::vec(0.0, 0.0, -52.5)),
            Arc::clone(&mat_light),
        );
        let light: Arc<dyn Hit> = Arc::new(light);

        let box1 = make_box(
            Vec4::point(0.0, 0.0, 0.0),
            Vec4::vec(165.0, 330.0, 165.0),
            Arc::clone(&mat_white),
        );
        let mut box1 = Transform::new(Box::new(box1));
        box1.translate(82.5, 165.0, 82.5);
        box1.rotate_y(deg_to_rad(18.0));
        box1.translate(265.0, 0.0, 295.0);

        // let box2 = make_box(
        //     Vec4::point(0.0, 0.0, 0.0),
        //     Vec4::vec(165.0, 165.0, 165.0),
        //     Arc::clone(&mat_white),
        // );
        // let mut box2 = Transform::new(Box::new(box2));
        // box2.translate(82.5, 82.5, 82.5);
        // box2.rotate_y(deg_to_rad(-15.0));
        // box2.translate(130.0, 0.0, 65.0);

        let glass_ball = Sphere::new(Vec4::point(212.5, 82.5, 147.5), 82.5, mat_glass);
        let glass_ball: Arc<dyn Hit> = Arc::new(glass_ball);

        let mut world = ObjectList::new();
        world.add(Arc::new(floor));
        world.add(Arc::new(ceiling));
        world.add(Arc::new(back_wall));
        world.add(Arc::new(left_wall));
        world.add(Arc::new(right_wall));
        world.add(Arc::clone(&light));
        world.add(Arc::new(box1));
        // world.add(Arc::new(box2));
        world.add(Arc::clone(&glass_ball));

        let world = Arc::new(world);

        let mut lights = ObjectList::new();
        lights.add(light);
        lights.add(glass_ball);

        let lights = Arc::new(lights);

        Ok((camera, world, lights))
    }
}
