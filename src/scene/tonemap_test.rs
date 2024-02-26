use std::error::Error;
use std::sync::Arc;

use rust_raytracer::camera::Camera;
use rust_raytracer::material::{LambertianDiffuse, Material};
use rust_raytracer::object::{Hit, ObjectList, Plane, Sky, Sphere};
use rust_raytracer::texture::ConstantColorTexture;
use rust_raytracer::vec4::Vec4;

use super::Scene;

// Config variables
const ASPECT_RATIO: f64 = 1.0;
const OUTPUT_WIDTH: usize = 400;
const FOCAL_LENGTH: f64 = 35.0;

pub struct TonemapTestScene;

impl Scene for TonemapTestScene {
    fn init() -> Result<(Camera, Arc<dyn Hit>, Arc<dyn Hit>), Box<dyn Error>> {
        // Set up camera
        let mut camera = Camera::new(OUTPUT_WIDTH, ASPECT_RATIO, FOCAL_LENGTH);
        camera.move_and_look_at(Vec4::point(0.0, 30.0, 15.0), Vec4::point(0.0, 0.0, -0.75));

        // Set up materials
        let color_r_0 = ConstantColorTexture::from_values(0.1, 0.0, 0.0);
        let mat_r_0: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_r_0)));
        let color_r_1 = ConstantColorTexture::from_values(0.2, 0.0, 0.0);
        let mat_r_1: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_r_1)));
        let color_r_2 = ConstantColorTexture::from_values(0.5, 0.0, 0.0);
        let mat_r_2: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_r_2)));
        let color_r_3 = ConstantColorTexture::from_values(1.0, 0.0, 0.0);
        let mat_r_3: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_r_3)));

        let color_g_0 = ConstantColorTexture::from_values(0.0, 0.1, 0.0);
        let mat_g_0: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_g_0)));
        let color_g_1 = ConstantColorTexture::from_values(0.0, 0.2, 0.0);
        let mat_g_1: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_g_1)));
        let color_g_2 = ConstantColorTexture::from_values(0.0, 0.5, 0.0);
        let mat_g_2: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_g_2)));
        let color_g_3 = ConstantColorTexture::from_values(0.0, 1.0, 0.0);
        let mat_g_3: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_g_3)));

        let color_b_0 = ConstantColorTexture::from_values(0.0, 0.0, 0.1);
        let mat_b_0: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_b_0)));
        let color_b_1 = ConstantColorTexture::from_values(0.0, 0.0, 0.2);
        let mat_b_1: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_b_1)));
        let color_b_2 = ConstantColorTexture::from_values(0.0, 0.0, 0.5);
        let mat_b_2: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_b_2)));
        let color_b_3 = ConstantColorTexture::from_values(0.0, 0.0, 1.0);
        let mat_b_3: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(color_b_3)));

        let mat_floor: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(
            ConstantColorTexture::from_values(0.5, 0.5, 0.5),
        )));

        // Set up objects
        let sky = Sky::new(Arc::new(ConstantColorTexture::from_values(
            50.0, 50.0, 50.0,
        )));
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
