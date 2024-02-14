use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

use super::{Hit, HitRecord};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hit for Sphere {
    fn test(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        let center_diff = ray.origin() - self.center;

        // Test for ray-sphere intersection using quadratic formula
        let a = ray.direction().length_squared();
        let half_b = ray.direction().dot(&center_diff);
        let c = center_diff.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }
        // Find the nearest root in the acceptable range
        let d_sqrt = discriminant.sqrt();

        let mut root = (-half_b - d_sqrt) / a;
        if root <= t.min() || t.max() <= root {
            root = (-half_b - d_sqrt) / a;
            if root <= t.min() || t.max() <= root {
                return None;
            }
        }

        let hit_pos = ray.at(root);
        Some(HitRecord::new(
            ray,
            hit_pos,
            root,
            (hit_pos - self.center) / self.radius,
        ))
    }
}