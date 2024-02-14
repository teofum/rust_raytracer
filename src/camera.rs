use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    image_width: u32,
    image_height: u32,

    camera_center: Point3,
    pixel_delta: (Vec3, Vec3),
    first_pixel: Point3,
}

impl Camera {
    pub fn new(
        image_width: u32,
        aspect_ratio: f64,
        viewport_height: f64,
        focal_length: f64,
    ) -> Self {
        let image_height = u32::max(1, (image_width as f64 / aspect_ratio) as u32);

        // Final aspect ratio, accounting for image height rounding
        let real_aspect_ratio = image_width as f64 / image_height as f64;
        let viewport_width = viewport_height * real_aspect_ratio;

        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -viewport_height, 0.0);

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

    pub fn image_size(&self) -> (u32, u32) {
        (self.image_width, self.image_height)
    }

    // Ray caster

    pub fn get_ray(&self, pixel_x: u32, pixel_y: u32) -> Ray {
        let pixel_center =
            self.first_pixel + (self.pixel_delta.0 * pixel_x as f64) + (self.pixel_delta.1 * pixel_y as f64);

        let ray_direction = pixel_center - self.camera_center;

        Ray::new(self.camera_center, ray_direction)
    }
}
