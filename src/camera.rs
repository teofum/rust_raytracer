use std::sync::Arc;
use std::thread;
use std::time::Instant;

use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::buffer::Buffer;
use crate::interval::Interval;
use crate::object::Hit;
use crate::ray::Ray;
use crate::vec4::{Color, Point4, Vec4};

const SAMPLES_PER_PIXEL: u32 = 500; // Number of random samples per pixel
const MAX_DEPTH: u32 = 20; // Max ray bounces

const THREAD_COUNT: u32 = 10; // Number of threads to spawn
const SAMPLES_PER_THREAD: u32 = SAMPLES_PER_PIXEL / THREAD_COUNT;

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

    pub fn render(self, world: Arc<dyn Hit>, buf: &mut Buffer) {
        let mut threads = Vec::new();

        let self_ref = Arc::new(self);

        for tid in 0..THREAD_COUNT {
            let mut thread_buf = self_ref.create_buffer();
            let thread_world = Arc::clone(&world);
            let thread_self_ref = Arc::clone(&self_ref);

            let thread = thread::spawn(move || {
                let time = Instant::now();

                let mut thread_rng =
                    XorShiftRng::from_rng(rand::thread_rng()).expect("Failed to init RNG");
                let image_height = thread_self_ref.image_height;
                let image_width = thread_self_ref.image_width;

                for y in 0..image_height {
                    // print!("Rendering... [line {}/{}]\r", y + 1, self.image_height);

                    for x in 0..image_width {
                        let mut color = Vec4::vec(0.0, 0.0, 0.0);

                        for _ in 0..SAMPLES_PER_THREAD {
                            let mut ray = thread_self_ref.get_ray(x, y, &mut thread_rng);
                            color += thread_self_ref.ray_color(
                                &mut ray,
                                &thread_world,
                                MAX_DEPTH,
                                &mut thread_rng,
                            );
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

    fn get_ray(&self, pixel_x: usize, pixel_y: usize, rng: &mut XorShiftRng) -> Ray {
        let pixel_center = self.first_pixel
            + (self.pixel_delta.0 * pixel_x as f64)
            + (self.pixel_delta.1 * pixel_y as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square(rng);

        let ray_origin = match self.aperture_radius {
            Some(_) => self.defocus_disk_sample(rng),
            None => self.position,
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn ray_color(
        &self,
        ray: &mut Ray,
        object: &Arc<dyn Hit>,
        depth: u32,
        rng: &mut XorShiftRng,
    ) -> Color {
        if depth <= 0 {
            return Vec4::vec(0.0, 0.0, 0.0);
        }

        if let Some(hit) = object.test(&ray, Interval(0.001, f64::INFINITY)) {
            let emitted = hit.material().emit(&hit);

            if let Some(att) = hit.material().scatter(ray, &hit, rng) {
                return emitted + self.ray_color(ray, object, depth - 1, rng) * att;
            }

            return emitted; // Ray was absorbed by material
        }

        (self.background_color)(ray)
    }

    fn pixel_sample_square(&self, rng: &mut XorShiftRng) -> Vec4 {
        let x = rng.gen_range(0.0..1.0) - 0.5;
        let y = rng.gen_range(0.0..1.0) - 0.5;

        self.pixel_delta.0 * x + self.pixel_delta.1 * y
    }

    /// # Panics
    /// Panics if aperture_radius is `None`. Caller should make sure aperture radius is set.
    fn defocus_disk_sample(&self, rng: &mut XorShiftRng) -> Vec4 {
        let v = Vec4::random_in_unit_disk(rng);
        self.position
            + (self.basis[0] * v[0] + self.basis[1] * v[1]) * self.aperture_radius.unwrap()
    }
}
