use std::error::Error;
use std::sync::Arc;

use rust_raytracer::camera::Camera;
use rust_raytracer::object::Hit;

mod test_scene_1;
pub use test_scene_1::TestScene1;
mod test_scene_2;
pub use test_scene_2::TestScene2;

pub trait Scene {
    fn init() -> Result<(Camera, Arc<dyn Hit>), Box<dyn Error>>;
}
