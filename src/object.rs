use crate::{ray::Ray, vec3::Point3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn test_hit(&self, ray: &Ray) -> bool {
        let center_diff = ray.origin() - self.center;

        // Test for ray-sphere intersection using quadratic formula
        let a = ray.direction().length_squared();
        let b = ray.direction().dot(&center_diff) * 2.0;
        let c = center_diff.length_squared() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        discriminant >= 0.0
    }
}
