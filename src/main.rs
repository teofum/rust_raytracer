use std::error::Error;
use std::time::Instant;

use rust_raytracer::output::Writer;

mod scene;
use rust_raytracer::tonemapping;
use scene::EarthScene;
use scene::GoldenMonkeyScene;
use scene::LightTestScene;
use scene::PerlinScene;
use scene::Scene;

const OUT_FILENAME: &'static str = "out.png";

fn main() -> Result<(), Box<dyn Error>> {
    let time = Instant::now();
    let (camera, world) = GoldenMonkeyScene::init()?;

    let elapsed = time.elapsed();
    println!("Ready: {:.2?}", elapsed);

    // Output
    let mut buf = camera.create_buffer();
    camera.render(world, &mut buf);

    let elapsed = time.elapsed();
    println!("Done: {:.2?}. Writing output to file...", elapsed);

    let mut writer = Writer::new(buf);
    writer.tonemap = tonemapping::tonemap_aces;
    writer.save(OUT_FILENAME)?;

    let elapsed = time.elapsed();
    println!("Done! Took {:.2?}. Goodbye :)", elapsed);

    Ok(())
}
