use std::error::Error;
use std::sync::Arc;

use rust_raytracer::camera::Camera;
use rust_raytracer::material::{Dielectric, Emissive, LambertianDiffuse, Material};
use rust_raytracer::object::{make_box, Hit, ObjectList, Plane, Sphere, Transform};
use rust_raytracer::texture::ConstantColorTexture;
use rust_raytracer::utils::deg_to_rad;
use rust_raytracer::vec4::Vec4;

use super::Scene;

// Config variables
const ASPECT_RATIO: f64 = 1.0;
const OUTPUT_WIDTH: usize = 600;
const FOCAL_LENGTH: f64 = 33.0;

pub struct CornellBoxScene;

impl Scene for CornellBoxScene {
    fn init() -> Result<(Camera, Arc<dyn Hit>, Arc<dyn Hit>), Box<dyn Error>> {
        // Set up camera
        let mut camera = Camera::new(OUTPUT_WIDTH, ASPECT_RATIO, FOCAL_LENGTH);
        camera.move_and_look_at(
            Vec4::point(277.5, 277.5, -800.0),
            Vec4::point(277.5, 277.5, 0.0),
        );

        // Set up materials
        let mat_white: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(
            ConstantColorTexture::from_values(0.73, 0.73, 0.73),
        )));
        let mat_green: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(
            ConstantColorTexture::from_values(0.12, 0.45, 0.15),
        )));
        let mat_red: Arc<dyn Material> = Arc::new(LambertianDiffuse::new(Arc::new(
            ConstantColorTexture::from_values(0.65, 0.05, 0.05),
        )));
        let mat_light: Arc<dyn Material> = Arc::new(Emissive::new(Arc::new(
            ConstantColorTexture::from_values(15.0, 15.0, 15.0),
        )));
        let mat_glass: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));

        // Set up objects
        let floor = Plane::new(
            Vec4::point(277.5, 0.0, 277.5),
            (Vec4::vec(277.5, 0.0, 0.0), Vec4::vec(0.0, 0.0, 277.5)),
            Arc::clone(&mat_white),
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
            (Vec4::vec(0.0, 276.5, 0.0), Vec4::vec(0.0, 0.0, 277.5)),
            Arc::clone(&mat_green),
        );
        let right_wall = Plane::new(
            Vec4::point(0.0, 277.5, 277.5),
            (Vec4::vec(0.0, 277.5, 0.0), Vec4::vec(0.0, 0.0, -277.5)),
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

        let lights: Arc<dyn Hit> = Arc::clone(&glass_ball);

        let mut world = ObjectList::new();
        world.add(Arc::new(floor));
        world.add(Arc::new(ceiling));
        world.add(Arc::new(back_wall));
        world.add(Arc::new(left_wall));
        world.add(Arc::new(right_wall));
        world.add(light);
        world.add(Arc::new(box1));
        // world.add(Arc::new(box2));
        world.add(glass_ball);

        let world = Arc::new(world);

        Ok((camera, world, lights))
    }
}
