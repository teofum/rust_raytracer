use std::sync::Arc;
use std::thread;
use std::time::Instant;

use rand::{Rng, SeedableRng};
use rand_distr::Standard;
use rand_pcg::Pcg64Mcg;

use crate::buffer::Buffer;
use crate::interval::Interval;
use crate::material::ScatterResult;
use crate::object::Hit;
use crate::pdf::{HittablePDF, MixPDF, PDF};
use crate::ray::Ray;
use crate::vec4::{Color, Point4, Vec4};

// Config;
const SQRT_SAMPLES_PER_THREAD: usize = 10;
const LIGHT_BIAS: f64 = 0.5;
const THREAD_COUNT: usize = 10; // Number of threads to spawn
const MAX_DEPTH: usize = 20; // Max ray bounces

/// Inverse Square Root of Samples Per Thread
const ISRSPT: f64 = 1.0 / SQRT_SAMPLES_PER_THREAD as f64;
const SAMPLES_PER_THREAD: usize = SQRT_SAMPLES_PER_THREAD * SQRT_SAMPLES_PER_THREAD;
const SAMPLES_PER_PIXEL: usize = SAMPLES_PER_THREAD * THREAD_COUNT; // Total number of random samples per pixel

pub struct Camera {
    pub background_color: fn(ray: &Ray) -> Color,

    image_width: usize,
    aspect_ratio: f64,
    focal_length: f64,
    f_number: Option<f64>,
    focus_distance: Option<f64>,
    position: Point4,
    look_at: Point4,
    v_up: Vec4,

    image_height: usize,
    pixel_delta: (Vec4, Vec4),
    first_pixel: Point4,
    basis: [Point4; 3],
    aperture_radius: Option<f64>,
}

impl Camera {
    pub fn new(image_width: usize, aspect_ratio: f64, focal_length: f64) -> Self {
        let mut camera = Camera {
            background_color: |_| Vec4::vec(0.0, 0.0, 0.0),

            image_width,
            aspect_ratio,
            focal_length,
            f_number: None,
            focus_distance: None,
            position: Vec4::point(0.0, 0.0, 0.0),
            look_at: Vec4::point(0.0, 0.0, -1.0),
            v_up: Vec4::vec(0.0, 1.0, 0.0),

            image_height: 0,
            basis: [Vec4::vec(0.0, 0.0, 0.0); 3],
            pixel_delta: (Vec4::vec(0.0, 0.0, 0.0), Vec4::vec(0.0, 0.0, 0.0)),
            first_pixel: Vec4::point(0.0, 0.0, 0.0),
            aperture_radius: None,
        };

        camera.init();

        camera
    }

    fn init(&mut self) {
        self.image_height = usize::max(1, (self.image_width as f64 / self.aspect_ratio) as usize);

        let direction = self.position - self.look_at;
        let focus_dist = self.focus_distance.unwrap_or_else(|| direction.length());

        // Relative size of the viewport to get 35mm (36x24mm frame) equivalent FOV
        let h = 24.0 / self.focal_length;

        // Final aspect ratio, accounting for image height rounding
        let real_aspect_ratio = self.image_width as f64 / self.image_height as f64;
        let viewport_height = focus_dist * h;
        let viewport_width = viewport_height * real_aspect_ratio;

        // Calculate the unit basis for the camera coordinate frame
        let w = direction.to_unit();
        let u = self.v_up.cross(&w);
        let v = w.cross(&u);
        self.basis = [u, v, w];

        // Calculate viewport vectors
        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        self.pixel_delta = (
            viewport_u / self.image_width as f64,
            viewport_v / self.image_height as f64,
        );

        // Make the image plane match the focus plane, makes the math a lot easier
        // Not how a real camera works, but we're not constrained by the laws of physics!
        let viewport_upper_left =
            self.position - w * focus_dist - viewport_u / 2.0 - viewport_v / 2.0;

        // Top-left pixel, shifted half a pixel from the top left corner of the viewport
        self.first_pixel = viewport_upper_left + (self.pixel_delta.0 + self.pixel_delta.1) * 0.5;

        // Calculate aperture radius from f-number
        // f = (focal length) / (aperture radius)
        self.aperture_radius = if let Some(f) = self.f_number {
            Some((self.focal_length / 1000.0) / f)
        } else {
            None
        };
    }

    // Getters

    pub fn create_buffer(&self) -> Buffer {
        Buffer::new(self.image_width, self.image_height)
    }

    // Property setters

    pub fn set_focal_length(&mut self, f: f64) {
        self.focal_length = f;
        self.init();
    }

    /// Set `None` to disable depth-of-field effect
    pub fn set_f_number(&mut self, f: Option<f64>) {
        self.f_number = f;
        self.init();
    }

    /// Set `None` to AF (place focus at the look_at point)
    ///
    /// This setting has no effect if depth-of-field is disabled.
    pub fn focus(&mut self, d: Option<f64>) {
        self.focus_distance = d;
        self.init();
    }

    pub fn set_position(&mut self, pos: Point4) {
        self.position = pos;
        self.init();
    }

    pub fn look_at(&mut self, target: Point4) {
        self.look_at = target;
        self.init();
    }

    pub fn move_and_look_at(&mut self, pos: Point4, target: Point4) {
        self.position = pos;
        self.look_at = target;
        self.init();
    }

    // Rendering

    pub fn render(self, world: Arc<dyn Hit>, lights: Arc<dyn Hit>, buf: &mut Buffer) {
        let mut threads = Vec::new();

        let self_ref = Arc::new(self);

        for tid in 0..THREAD_COUNT {
            let mut thread_buf = self_ref.create_buffer();
            let thread_world = Arc::clone(&world);
            let thread_lights = Arc::clone(&lights);
            let thread_self_ref = Arc::clone(&self_ref);

            let thread = thread::spawn(move || {
                let time = Instant::now();

                let mut thread_rng =
                    Pcg64Mcg::from_rng(rand::thread_rng()).expect("Failed to init RNG");
                let image_height = thread_self_ref.image_height;
                let image_width = thread_self_ref.image_width;

                for y in 0..image_height {
                    // print!("Rendering... [line {}/{}]\r", y + 1, self.image_height);

                    for x in 0..image_width {
                        let mut color = Vec4::vec(0.0, 0.0, 0.0);

                        for sy in 0..SQRT_SAMPLES_PER_THREAD {
                            for sx in 0..SQRT_SAMPLES_PER_THREAD {
                                let mut ray =
                                    thread_self_ref.get_ray(x, y, sx, sy, &mut thread_rng);
                                color += thread_self_ref.ray_color(
                                    &mut ray,
                                    &thread_world,
                                    &thread_lights,
                                    MAX_DEPTH,
                                    &mut thread_rng,
                                );
                            }
                        }
                        color /= SAMPLES_PER_PIXEL as f64;

                        thread_buf.set_pixel(x, y, color);
                    }
                }

                let elapsed = time.elapsed();
                println!("Thread {tid} finished in {:.2?}", elapsed);
                thread_buf
            });

            threads.push(thread);
        }

        for thread in threads {
            let thread_buf = thread.join().expect("Thread failed!");

            // Add thread buffer to main buffer
            for y in 0..self_ref.image_height {
                for x in 0..self_ref.image_width {
                    let thread_color = thread_buf.get_pixel(x, y);
                    let local_color = buf.get_pixel(x, y);

                    buf.set_pixel(x, y, local_color + thread_color);
                }
            }
        }
    }

    // Rendering helpers

    fn get_ray(
        &self,
        pixel_x: usize,
        pixel_y: usize,
        sample_x: usize,
        sample_y: usize,
        rng: &mut Pcg64Mcg,
    ) -> Ray {
        let pixel_center = self.first_pixel
            + (self.pixel_delta.0 * pixel_x as f64)
            + (self.pixel_delta.1 * pixel_y as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square(sample_x, sample_y, rng);

        let ray_origin = match self.aperture_radius {
            Some(_) => self.defocus_disk_sample(rng),
            None => self.position,
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn ray_color(
        &self,
        ray: &Ray,
        object: &Arc<dyn Hit>,
        lights: &Arc<dyn Hit>,
        depth: usize,
        rng: &mut Pcg64Mcg,
    ) -> Color {
        if depth <= 0 {
            return Vec4::vec(0.0, 0.0, 0.0);
        }

        if let Some(hit) = object.test(&ray, Interval(0.001, f64::INFINITY), rng) {
            let from_emission = hit.material().emit(&hit);

            return match hit.material().scatter(ray, &hit, rng) {
                ScatterResult::ScatteredWithPDF {
                    attenuation,
                    pdf: material_pdf,
                } => {
                    let hittable_pdf = Box::new(HittablePDF::new(Arc::clone(lights), hit.pos()));
                    let mix_pdf = MixPDF::new(material_pdf, hittable_pdf, LIGHT_BIAS);

                    let scattered = Ray::new(hit.pos(), mix_pdf.generate(rng));
                    let pdf = mix_pdf.value(&scattered.dir, rng);

                    let scattering_pdf = hit.material().scattering_pdf(ray, &scattered, &hit);

                    let scatter_color = self.ray_color(&scattered, object, lights, depth - 1, rng);
                    let from_scatter = (scatter_color * attenuation * scattering_pdf) / pdf;

                    from_emission + from_scatter
                }
                ScatterResult::ScatteredWithRay {
                    attenuation,
                    scattered,
                } => {
                    let scatter_color = self.ray_color(&scattered, object, lights, depth - 1, rng);
                    let from_scatter = scatter_color * attenuation;

                    from_emission + from_scatter
                }
                ScatterResult::Absorbed => Vec4::vec(0.0, 0.0, 0.0),
                ScatterResult::Emissive => from_emission,
            };
        }

        (self.background_color)(ray)
    }

    fn pixel_sample_square(&self, sample_x: usize, sample_y: usize, rng: &mut Pcg64Mcg) -> Vec4 {
        let rx: f64 = rng.sample(Standard);
        let ry: f64 = rng.sample(Standard);
        let x = (sample_x as f64 + rx) * ISRSPT - 0.5;
        let y = (sample_y as f64 + ry) * ISRSPT - 0.5;

        self.pixel_delta.0 * x + self.pixel_delta.1 * y
    }

    /// # Panics
    /// Panics if aperture_radius is `None`. Caller should make sure aperture radius is set.
    fn defocus_disk_sample(&self, rng: &mut Pcg64Mcg) -> Vec4 {
        let v = Vec4::random_in_unit_disk(rng);
        self.position
            + (self.basis[0] * v[0] + self.basis[1] * v[1]) * self.aperture_radius.unwrap()
    }
}
