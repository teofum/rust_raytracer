use rand::Rng;

use crate::buffer::Buffer;
use crate::interval::Interval;
use crate::object::Hit;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};

const VIEWPORT_HEIGHT: f64 = 24.0 / 1000.0;
const SAMPLES_PER_PIXEL: u32 = 10;

pub struct Camera {
    image_width: usize,
    image_height: usize,

    camera_center: Point3,
    pixel_delta: (Vec3, Vec3),
    first_pixel: Point3,
}

impl Camera {
    pub fn new(image_width: usize, aspect_ratio: f64, focal_length: f64) -> Self {
        let image_height = usize::max(1, (image_width as f64 / aspect_ratio) as usize);

        // Final aspect ratio, accounting for image height rounding
        let real_aspect_ratio = image_width as f64 / image_height as f64;
        let viewport_width = VIEWPORT_HEIGHT * real_aspect_ratio;

        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -VIEWPORT_HEIGHT, 0.0);

        let pixel_delta = (
            viewport_u / image_width as f64,
            viewport_v / image_height as f64,
        );

        let camera_center = Vec3::origin();
        let viewport_upper_left =
            camera_center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        // Top-left pixel, shifted half a pixel from the top left corner of the viewport
        let first_pixel = viewport_upper_left + (pixel_delta.0 + pixel_delta.1) * 0.5;

        Camera {
            image_width,
            image_height,
            camera_center,
            pixel_delta,
            first_pixel,
        }
    }

    // Getters

    pub fn create_buffer(&self) -> Buffer {
        Buffer::new(self.image_width, self.image_height)
    }

    // Rendering

    pub fn render(&self, world: &dyn Hit, buf: &mut Buffer) {
        for y in 0..self.image_height {
            print!("Rendering... [line {}/{}]\r", y + 1, self.image_height);

            for x in 0..self.image_width {
                let mut color = Vec3::origin();

                for _ in 0..SAMPLES_PER_PIXEL {
                    let ray = self.get_ray(x, y);
                    color += self.ray_color(&ray, world);
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

        let ray_direction = pixel_sample - self.camera_center;

        Ray::new(self.camera_center, ray_direction)
    }

    fn ray_color(&self, ray: &Ray, object: &dyn Hit) -> Color {
        if let Some(hit) = object.test(&ray, Interval(0.0, f64::INFINITY)) {
            return (Vec3(1.0, 1.0, 1.0) + hit.normal()) * 0.5;
        }

        let unit_dir = ray.direction().to_unit();
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
