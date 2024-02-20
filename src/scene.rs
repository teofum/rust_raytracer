use std::io;
use std::sync::Arc;

use rust_raytracer::camera::Camera;
use rust_raytracer::object::Hit;

mod test_scene_1;
pub use test_scene_1::TestScene1;

pub trait Scene {
    fn init() -> io::Result<(Camera, Arc<dyn Hit>)>;
}
