use rand::Rng;

use crate::buffer::Buffer;
use crate::interval::Interval;
use crate::object::Hit;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

const REAL_FOCAL_LENGTH: f64 = 1.0;
const SAMPLES_PER_PIXEL: u32 = 100; // Number of random samples per pixel
const MAX_DEPTH: u32 = 10; // Max ray bounces

pub struct Camera {
    image_width: usize,
    aspect_ratio: f64,
    focal_length: f64,
    position: Point3,
    look_at: Point3,

    image_height: usize,
    pixel_delta: (Vec3, Vec3),
    first_pixel: Point3,
    camera_center: Point3,
    basis: [Point3; 3],
}

impl Camera {
    pub fn new(image_width: usize, aspect_ratio: f64, focal_length: f64) -> Self {
        let mut camera = Camera {
            image_width,
            aspect_ratio,
            focal_length,
            position: Vec3::origin(),
            look_at: Vec3(0.0, 0.0, -1.0),

            image_height: 0,
            camera_center: Vec3::origin(),
            basis: [Vec3::origin(); 3],
            pixel_delta: (Vec3::origin(), Vec3::origin()),
            first_pixel: Vec3::origin(),
        };

        camera.init();

        camera
    }

    fn init(&mut self) {
        self.image_height = usize::max(1, (self.image_width as f64 / self.aspect_ratio) as usize);

        // Relative size of the viewport to get 35mm (36x24mm frame) equivalent FOV
        let h = 24.0 / self.focal_length;

        // Final aspect ratio, accounting for image height rounding
        let real_aspect_ratio = self.image_width as f64 / self.image_height as f64;
        let viewport_height = REAL_FOCAL_LENGTH * h;
        let viewport_width = viewport_height * real_aspect_ratio;

        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -viewport_height, 0.0);

        self.camera_center = self.position;
        self.pixel_delta = (
            viewport_u / self.image_width as f64,
            viewport_v / self.image_height as f64,
        );

        let viewport_upper_left = self.camera_center
            - Vec3(0.0, 0.0, REAL_FOCAL_LENGTH)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        // Top-left pixel, shifted half a pixel from the top left corner of the viewport
        self.first_pixel = viewport_upper_left + (self.pixel_delta.0 + self.pixel_delta.1) * 0.5;
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

    pub fn set_position(&mut self, pos: Point3) {
        self.position = pos;
        self.init();
    }

    pub fn look_at(&mut self, p: Point3) {
        self.look_at = p;
        self.init();
    }

    // Rendering

    pub fn render(&self, world: &dyn Hit, buf: &mut Buffer) {
        for y in 0..self.image_height {
            print!("Rendering... [line {}/{}]\r", y + 1, self.image_height);

            for x in 0..self.image_width {
                let mut color = Vec3::origin();

                for _ in 0..SAMPLES_PER_PIXEL {
                    let mut ray = self.get_ray(x, y);
                    color += self.ray_color(&mut ray, world, MAX_DEPTH);
                }
                color /= SAMPLES_PER_PIXEL as f64;

                buf.set_pixel(x, y, color);
            }
        }
    }

    // Rendering helpers

    fn get_ray(&self, pixel_x: usize, pixel_y: usize) -> Ray {
        let pixel_center = self.first_pixel
            + (self.pixel_delta.0 * pixel_x as f64)
            + (self.pixel_delta.1 * pixel_y as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_direction = pixel_sample - self.position;

        Ray::new(self.position, ray_direction)
    }

    fn ray_color(&self, ray: &mut Ray, object: &dyn Hit, depth: u32) -> Color {
        if depth <= 0 {
            return Vec3::origin();
        }

        if let Some(hit) = object.test(&ray, Interval(0.001, f64::INFINITY)) {
            if let Some(att) = hit.material().scatter(ray, &hit) {
                return self.ray_color(ray, object, depth - 1) * att;
            }

            return Vec3::origin(); // Ray was absorbed by material
        }

        let unit_dir = ray.dir.to_unit();
        let t = 0.5 * (unit_dir.y() + 1.0);

        Vec3::lerp(Vec3(1.0, 1.0, 1.0), Vec3(0.5, 0.7, 1.0), t)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..1.0) - 0.5;
        let y = rng.gen_range(0.0..1.0) - 0.5;

        self.pixel_delta.0 * x + self.pixel_delta.1 * y
    }
}
