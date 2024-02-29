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
    let scene = 0;
    let (camera, world, lights) = match scene {
        1 => EarthScene::init(&config)?,
        2 => PerlinScene::init(&config)?,
        3 => LightTestScene::init(&config)?,
        4 => CornellBoxScene::init(&config)?,
        5 => CornellSmokeScene::init(&config)?,
        6 => TonemapTestScene::init(&config)?,
        _ => GoldenMonkeyScene::init(&config)?,
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
