use std::env;
use std::error::Error;
use std::fs::File;
use std::time::Instant;

use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;

use rust_raytracer::config::Config;
use rust_raytracer::loaders::assimp::AssimpLoader;
use rust_raytracer::loaders::scene::SceneLoader;
use rust_raytracer::output::Writer;
use rust_raytracer::scene::CornellBoxScene;
use rust_raytracer::scene::CornellSmokeScene;
use rust_raytracer::scene::EarthScene;
use rust_raytracer::scene::GoldenMonkeyScene;
use rust_raytracer::scene::LightTestScene;
use rust_raytracer::scene::PerlinScene;
use rust_raytracer::scene::SceneInit;
use rust_raytracer::scene::TonemapTestScene;
use rust_raytracer::tonemapping;

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
        file_path if file_path.starts_with("model:") => {
            let loader = AssimpLoader::new(file_path.strip_prefix("model:").unwrap())?;
            loader.load(config)?
        }
        file_path => {
            let scene_file = File::open(file_path)?;
            let mut rng = Pcg64Mcg::from_rng(rand::thread_rng())?;

            let asset_path = if file_path.contains('/') {
                if let Some((path, _)) = file_path.rsplit_once('/') {
                    path.to_owned() + "/"
                } else {
                    "".to_owned()
                }
            } else {
                "".to_owned()
            };

            let loader = SceneLoader::new(&mut rng, &asset_path);
            loader.load(&scene_file, config)?
        }
    };

    let elapsed = time.elapsed();
    println!("Ready: {:.2?}", elapsed);

    let (w, h) = camera.image_size();
    let spp = camera.samples_per_pixel();
    let threads = camera.thread_count();
    let spt = spp / threads;
    println!(
        "Rendering: {}x{} @{}spp on {} threads ({} samples/thread)",
        w, h, spp, threads, spt
    );

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
