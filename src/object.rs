use crate::{ray::Ray, vec3::Point3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn test_hit(&self, ray: &Ray) -> f64 {
        let center_diff = ray.origin() - self.center;

        // Test for ray-sphere intersection using quadratic formula
        let a = ray.direction().length_squared();
        let half_b = ray.direction().dot(&center_diff);
        let c = center_diff.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }
}
