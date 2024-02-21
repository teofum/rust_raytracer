use std::error::Error;
use std::time::Instant;

use rust_raytracer::output::Writer;

mod scene;
use scene::EarthScene;
use scene::GoldenMonkeyScene;
use scene::LightTestScene;
use scene::PerlinScene;
use scene::Scene;

const OUT_FILENAME: &'static str = "out.png";

fn main() -> Result<(), Box<dyn Error>> {
    let time = Instant::now();
    let (camera, world) = LightTestScene::init()?;

    let elapsed = time.elapsed();
    println!("Ready: {:.2?}", elapsed);

    // Output
    let mut buf = camera.create_buffer();
    camera.render(world, &mut buf);

    let elapsed = time.elapsed();
    println!("Done: {:.2?}. Writing output to file...", elapsed);

    let writer = Writer::new(buf);
    writer.save(OUT_FILENAME)?;

    let elapsed = time.elapsed();
    println!("Done! Took {:.2?}. Goodbye :)", elapsed);

    Ok(())
}
