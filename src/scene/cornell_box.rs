use std::error::Error;
use std::sync::Arc;

use rust_raytracer::camera::Camera;
use rust_raytracer::mat4::Mat4;
use rust_raytracer::material::{Emissive, LambertianDiffuse, Material};
use rust_raytracer::object::transform::Transform;
use rust_raytracer::object::{make_box, Hit, ObjectList, Plane};
use rust_raytracer::texture::ConstantColorTexture;
use rust_raytracer::utils::deg_to_rad;
use rust_raytracer::vec4::Vec4;

use super::Scene;

// Config variables
const ASPECT_RATIO: f64 = 1.0;
const OUTPUT_WIDTH: usize = 600;
const FOCAL_LENGTH: f64 = 35.0;

pub struct CornellBoxScene;

impl Scene for CornellBoxScene {
    fn init() -> Result<(Camera, Arc<dyn Hit>), Box<dyn Error>> {
        // Set up camera
        let mut camera = Camera::new(OUTPUT_WIDTH, ASPECT_RATIO, FOCAL_LENGTH);
        camera.move_and_look_at(Vec4::point(0.0, 0.0, 110.0), Vec4::point(0.0, 0.0, 0.0));

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
            (Vec4::vec(6.5, 0.0, 0.0), Vec4::vec(0.0, 0.0, -5.25)),
            Arc::clone(&mat_light),
        );

        let box1 = make_box(
            Vec4::point(0.0, 0.0, 0.0),
            Vec4::vec(16.5, 16.5, 16.5),
            Arc::clone(&mat_white),
        );
        let box1 = Transform::new(
            Box::new(box1),
            Mat4::rotate_y(deg_to_rad(-15.0)),
        );
        let box1 = Transform::new(
            Box::new(box1),
            Mat4::translation(27.5 - 21.25, 8.25 - 27.5, 27.5 - 14.75),
        );

        let box2 = make_box(
            Vec4::point(0.0, 0.0, 0.0),
            Vec4::vec(16.5, 33.0, 16.5),
            Arc::clone(&mat_white),
        );
        let box2 = Transform::new(
            Box::new(box2),
            Mat4::rotate_y(deg_to_rad(18.0)),
        );
        let box2 = Transform::new(
            Box::new(box2),
            Mat4::translation(27.5 - 34.75, 16.5 - 27.5, 27.5 - 37.75),
        );

        let mut world = ObjectList::new();
        world.add(Box::new(floor));
        world.add(Box::new(ceiling));
        world.add(Box::new(back_wall));
        world.add(Box::new(left_wall));
        world.add(Box::new(right_wall));
        world.add(Box::new(light));
        world.add(Box::new(box1));
        world.add(Box::new(box2));

        let world = Arc::new(world);

        Ok((camera, world))
    }
}
