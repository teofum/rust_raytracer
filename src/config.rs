use std::env::Args;

use regex::Regex;

use crate::{utils::parse_vec, vec4::Vec4};

#[derive(Debug)]
pub struct SceneConfig {
    pub output_width: Option<usize>,
    pub aspect_ratio: Option<f64>,
    pub focal_length: Option<f64>,
    pub f_number: Option<f64>,
    pub focus_distance: Option<f64>,
    pub camera_pos: Option<Vec4>,
    pub camera_target: Option<Vec4>,
}

pub const DEFAULT_SCENE_CONFIG: SceneConfig = SceneConfig {
    output_width: Some(600),
    aspect_ratio: Some(1.5),
    focal_length: Some(50.0),
    f_number: None,
    focus_distance: None,
    camera_pos: Some(Vec4([0.0, 0.0, 1.0, 1.0])),
    camera_target: Some(Vec4([0.0, 0.0, 0.0, 1.0])),
};

impl SceneConfig {
    pub fn merge(base: &SceneConfig, overrides: &SceneConfig) -> Self {
        SceneConfig {
            output_width: overrides.output_width.or(base.output_width),
            aspect_ratio: overrides.aspect_ratio.or(base.aspect_ratio),
            focal_length: overrides.focal_length.or(base.focal_length),
            f_number: overrides.f_number.or(base.f_number),
            focus_distance: overrides.focus_distance.or(base.focus_distance),
            camera_pos: overrides.camera_pos.or(base.camera_pos),
            camera_target: overrides.camera_target.or(base.camera_target),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CameraConfig {
    pub thread_count: usize,
    pub sqrt_samples_per_thread: usize,
    pub max_depth: usize,
    pub light_bias: f64,
}

#[derive(Debug)]
pub struct Config {
    pub scene: SceneConfig,
    pub camera: CameraConfig,
    pub scene_name: String,
}

impl Config {
    pub fn from_args(args: Args) -> Self {
        let arg_regex = Regex::new(r"^-([^=\s]+)=([^=\s]+)$").unwrap();

        let mut output_width: Option<usize> = None;
        let mut aspect_ratio: Option<f64> = None;
        let mut focal_length: Option<f64> = None;
        let mut f_number: Option<f64> = None;
        let mut focus_distance: Option<f64> = None;
        let mut camera_pos: Option<Vec4> = None;
        let mut camera_target: Option<Vec4> = None;

        let mut thread_count = 1;
        let mut samples_per_pixel = 250;
        let mut max_depth = 20;
        let mut light_bias = 0.25;

        let mut scene_name = String::new();

        for arg in args.skip(1) {
            if arg.starts_with("-") {
                for (_, [key, value]) in arg_regex.captures_iter(&arg).map(|c| c.extract()) {
                    match key {
                        "w" | "-width" => {
                            output_width = Some(
                                value
                                    .parse::<usize>()
                                    .expect("Output width must be a positive integer"),
                            );
                        }
                        "r" | "-aspect-ratio" => {
                            aspect_ratio =
                                Some(value.parse::<f64>().expect("Aspect ratio must be a number"));
                        }
                        "f" | "-focal-length" => {
                            focal_length =
                                Some(value.parse::<f64>().expect("Focal length must be a number"));
                        }
                        "a" | "-aperture" => {
                            f_number =
                                Some(value.parse::<f64>().expect("Aperture must be a number"));
                        }
                        "d" | "-focus-dist" => {
                            focus_distance = Some(
                                value
                                    .parse::<f64>()
                                    .expect("Focus distance must be a number"),
                            );
                        }
                        "c" | "-camera-position" => {
                            let [x, y, z] = parse_vec(value).unwrap();
                            camera_pos = Some(Vec4::point(x, y, z));
                        }
                        "l" | "-look-at" => {
                            let [x, y, z] = parse_vec(value).unwrap();
                            camera_target = Some(Vec4::point(x, y, z));
                        }
                        "t" | "-threads" => {
                            thread_count = value
                                .parse::<usize>()
                                .expect("Thread count must be a positive integer");
                        }
                        "s" | "-samples" => {
                            samples_per_pixel = value
                                .parse::<usize>()
                                .expect("Sample count must be a positive integer");
                        }
                        "-max-depth" => {
                            max_depth = value
                                .parse::<usize>()
                                .expect("Max ray depth must be a positive integer");
                        }
                        "-light-bias" => {
                            light_bias = value.parse::<f64>().expect("Light bias must be a number");

                            assert!(
                                light_bias >= 0.0 && light_bias <= 1.0,
                                "Light bias must be in range [0; 1]"
                            );
                        }
                        _ => (),
                    }
                }
            } else {
                scene_name = arg;
            }
        }

        let samples_per_thread = samples_per_pixel / thread_count;
        let sqrt_samples_per_thread = (samples_per_thread as f64).sqrt() as usize;

        Config {
            scene: SceneConfig {
                output_width,
                aspect_ratio,
                focal_length,
                f_number,
                focus_distance,
                camera_pos,
                camera_target,
            },
            camera: CameraConfig {
                thread_count,
                sqrt_samples_per_thread,
                max_depth,
                light_bias,
            },
            scene_name,
        }
    }
}
