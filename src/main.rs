use std::error::Error;
use std::fs::File;
use std::time::Instant;

use rust_raytracer::ppm;

mod scene;
use scene::EarthScene;
use scene::GoldenMonkeyScene;
use scene::LightTestScene;
use scene::PerlinScene;
use scene::Scene;

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

    let mut file = File::create("out.ppm")?;
    ppm::write_to_file(&mut file, &buf)?;

    let elapsed = time.elapsed();
    println!("Done! Took {:.2?}. Goodbye :)", elapsed);

    Ok(())
}
