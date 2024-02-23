use std::error::Error;
use std::sync::Arc;

use rust_raytracer::camera::Camera;
use rust_raytracer::object::Hit;

mod cornell_box;
pub use cornell_box::CornellBoxScene;
mod cornell_smoke;
pub use cornell_smoke::CornellSmokeScene;
mod earth;
pub use earth::EarthScene;
mod golden_monkey;
pub use golden_monkey::GoldenMonkeyScene;
mod light_test;
pub use light_test::LightTestScene;
mod perlin_noise;
pub use perlin_noise::PerlinScene;
mod tonemap_test;
pub use tonemap_test::TonemapTestScene;

pub trait Scene {
    fn init() -> Result<(Camera, Arc<dyn Hit>), Box<dyn Error>>;
}
