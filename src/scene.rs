use std::error::Error;
use std::sync::Arc;

pub use cornell_box::CornellBoxScene;
pub use cornell_smoke::CornellSmokeScene;
pub use earth::EarthScene;
pub use golden_monkey::GoldenMonkeyScene;
pub use light_test::LightTestScene;
pub use perlin_noise::PerlinScene;
pub use tonemap_test::TonemapTestScene;

use crate::camera::Camera;
use crate::config::Config;
use crate::object::Hit;

mod cornell_box;

mod cornell_smoke;

mod earth;

mod golden_monkey;

mod light_test;

mod perlin_noise;

mod tonemap_test;

pub type SceneData = (Camera, Arc<dyn Hit>, Arc<dyn Hit>);

pub trait SceneInit {
    fn init(config: Config) -> Result<SceneData, Box<dyn Error>>;
}
