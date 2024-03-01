use std::error::Error;
use std::sync::Arc;

use rust_raytracer::camera::Camera;
use rust_raytracer::config::{Config, SceneConfig, DEFAULT_SCENE_CONFIG};
use rust_raytracer::material::{LambertianDiffuse, Material};
use rust_raytracer::object::{Hit, ObjectList, Plane, Sky, Sphere};
use rust_raytracer::texture::ConstantTexture;
use rust_raytracer::vec4::Vec4;

use super::{Scene, SceneData};

pub struct TonemapTestScene;

impl Scene for TonemapTestScene {
    fn init(config: Config) -> Result<SceneData, Box<dyn Error>> {
        let scene_defaults = SceneConfig {
            output_width: Some(400),
            aspect_ratio: Some(1.0),
            focal_length: Some(35.0),
            f_number: None,
            focus_distance: None,
            camera_pos: Some(Vec4::point(0.0, 30.0, 15.0)),
            camera_target: Some(Vec4::point(0.0, 0.0, -0.75)),
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
        let color_r_0 = ConstantTexture::from_values(0.1, 0.0, 0.0);
        let mat_r_0: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_r_0)));
        let color_r_1 = ConstantTexture::from_values(0.2, 0.0, 0.0);
        let mat_r_1: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_r_1)));
        let color_r_2 = ConstantTexture::from_values(0.5, 0.0, 0.0);
        let mat_r_2: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_r_2)));
        let color_r_3 = ConstantTexture::from_values(1.0, 0.0, 0.0);
        let mat_r_3: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_r_3)));

        let color_g_0 = ConstantTexture::from_values(0.0, 0.1, 0.0);
        let mat_g_0: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_g_0)));
        let color_g_1 = ConstantTexture::from_values(0.0, 0.2, 0.0);
        let mat_g_1: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_g_1)));
        let color_g_2 = ConstantTexture::from_values(0.0, 0.5, 0.0);
        let mat_g_2: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_g_2)));
        let color_g_3 = ConstantTexture::from_values(0.0, 1.0, 0.0);
        let mat_g_3: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_g_3)));

        let color_b_0 = ConstantTexture::from_values(0.0, 0.0, 0.1);
        let mat_b_0: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_b_0)));
        let color_b_1 = ConstantTexture::from_values(0.0, 0.0, 0.2);
        let mat_b_1: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_b_1)));
        let color_b_2 = ConstantTexture::from_values(0.0, 0.0, 0.5);
        let mat_b_2: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_b_2)));
        let color_b_3 = ConstantTexture::from_values(0.0, 0.0, 1.0);
        let mat_b_3: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_b_3)));

        let mat_floor: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(
            ConstantTexture::from_values(0.5, 0.5, 0.5),
        )));

        // Set up objects
        let sky = Sky::new(Arc::new(ConstantTexture::from_values(50.0, 50.0, 50.0)));
        let sky: Arc<dyn Hit> = Arc::new(sky);

        let sphere_r_0 = Arc::new(Sphere::new(Vec4::point(-2.5, 0.5, -5.0), 1.0, mat_r_0));
        let sphere_r_1 = Arc::new(Sphere::new(Vec4::point(-2.5, 0.5, -2.5), 1.0, mat_r_1));
        let sphere_r_2 = Arc::new(Sphere::new(Vec4::point(-2.5, 0.5, 0.0), 1.0, mat_r_2));
        let sphere_r_3 = Arc::new(Sphere::new(Vec4::point(-2.5, 0.5, 2.5), 1.0, mat_r_3));

        let sphere_g_0 = Arc::new(Sphere::new(Vec4::point(0.0, 0.5, -5.0), 1.0, mat_g_0));
        let sphere_g_1 = Arc::new(Sphere::new(Vec4::point(0.0, 0.5, -2.5), 1.0, mat_g_1));
        let sphere_g_2 = Arc::new(Sphere::new(Vec4::point(0.0, 0.5, 0.0), 1.0, mat_g_2));
        let sphere_g_3 = Arc::new(Sphere::new(Vec4::point(0.0, 0.5, 2.5), 1.0, mat_g_3));

        let sphere_b_0 = Arc::new(Sphere::new(Vec4::point(2.5, 0.5, -5.0), 1.0, mat_b_0));
        let sphere_b_1 = Arc::new(Sphere::new(Vec4::point(2.5, 0.5, -2.5), 1.0, mat_b_1));
        let sphere_b_2 = Arc::new(Sphere::new(Vec4::point(2.5, 0.5, 0.0), 1.0, mat_b_2));
        let sphere_b_3 = Arc::new(Sphere::new(Vec4::point(2.5, 0.5, 2.5), 1.0, mat_b_3));

        let floor = Arc::new(Plane::new(
            Vec4::point(0.0, 0.0, 0.0),
            (Vec4::vec(-10.0, 0.0, 0.0), Vec4::vec(0.0, 0.0, 10.0)),
            mat_floor,
        ));

        let mut world = ObjectList::new();
        world.add(sphere_r_0);
        world.add(sphere_r_1);
        world.add(sphere_r_2);
        world.add(sphere_r_3);
        world.add(sphere_g_0);
        world.add(sphere_g_1);
        world.add(sphere_g_2);
        world.add(sphere_g_3);
        world.add(sphere_b_0);
        world.add(sphere_b_1);
        world.add(sphere_b_2);
        world.add(sphere_b_3);
        world.add(floor);
        world.add(Arc::clone(&sky));

        let world = Arc::new(world);

        let mut lights = ObjectList::new();
        lights.add(sky);

        let lights = Arc::new(lights);

        Ok((camera, world, lights))
    }
}
