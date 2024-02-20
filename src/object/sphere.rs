use std::f64::consts::PI;
use std::sync::Arc;

use crate::aabb::AxisAlignedBoundingBox;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use super::{Hit, HitRecord};

pub struct Sphere {
    pub material: Arc<dyn Material>,

    center: Point3,
    radius: f64,
    bounds: AxisAlignedBoundingBox,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        let radius_vec = Vec3(radius, radius, radius);
        let bounds = (center - radius_vec, center + radius_vec);

        Sphere {
            center,
            radius,
            bounds,
            material,
        }
    }
}

impl Hit for Sphere {
    fn test(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        let center_diff = ray.origin - self.center;

        // Test for ray-sphere intersection using quadratic formula
        let a = ray.dir.length_squared();
        let half_b = ray.dir.dot(&center_diff);
        let c = center_diff.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }
        // Find the nearest root in the acceptable range
        let d_sqrt = discriminant.sqrt();

        let mut root = (-half_b - d_sqrt) / a;
        if root <= t.min() || t.max() <= root {
            root = (-half_b + d_sqrt) / a;
            if root <= t.min() || t.max() <= root {
                return None;
            }
        }

        let hit_pos = ray.at(root);
        let normal = (hit_pos - self.center) / self.radius;

        // Get UV coordinates
        let theta = f64::acos(-normal.y());
        let phi = f64::atan2(-normal.z(), normal.x()) + PI;
        let uv = (phi / (2.0 * PI), theta / PI);

        Some(HitRecord::new(
            ray,
            hit_pos,
            root,
            uv,
            normal,
            Arc::as_ref(&self.material),
        ))
    }

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.bounds
    }
}
