use std::error::Error;
use std::sync::Arc;

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

use crate::camera::Camera;
use crate::config::Config;
use crate::object::Hit;

pub type SceneData = (Camera, Arc<dyn Hit>, Arc<dyn Hit>);

pub trait Scene {
    fn init(config: Config) -> Result<SceneData, Box<dyn Error>>;
}
