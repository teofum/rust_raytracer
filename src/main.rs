use std::env;
use std::error::Error;
use std::time::Instant;

use rust_raytracer::config::Config;
use rust_raytracer::output::Writer;

mod scene;
use rust_raytracer::tonemapping;
use scene::CornellBoxScene;
use scene::CornellSmokeScene;
use scene::EarthScene;
use scene::GoldenMonkeyScene;
use scene::LightTestScene;
use scene::PerlinScene;
use scene::Scene;
use scene::TonemapTestScene;

const OUT_FILENAME: &str = "out.png";

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args(env::args());

    let time = Instant::now();
    let scene = &config.scene_name[..];
    let (camera, world, lights) = match scene {
        "golden_monkey" | "" => GoldenMonkeyScene::init(config)?,
        "earth" => EarthScene::init(config)?,
        "perlin" => PerlinScene::init(config)?,
        "light_test" => LightTestScene::init(config)?,
        "cornell" => CornellBoxScene::init(config)?,
        "cornell_smoke" => CornellSmokeScene::init(config)?,
        "tonemap_test" => TonemapTestScene::init(config)?,
        _ => panic!("Invalid scene name!"), // TODO support loading scenes from files
    };

    let elapsed = time.elapsed();
    println!("Ready: {:.2?}", elapsed);

    // Output
    let mut buf = camera.create_buffer();
    camera.render(world, lights, &mut buf);

    let elapsed = time.elapsed();
    println!("Done: {:.2?}. Writing output to file...", elapsed);

    let mut writer = Writer::new(buf);
    writer.tonemap = tonemapping::tonemap_aces;
    writer.save(OUT_FILENAME)?;

    let elapsed = time.elapsed();
    println!("Done! Took {:.2?}. Goodbye :)", elapsed);

    Ok(())
}
