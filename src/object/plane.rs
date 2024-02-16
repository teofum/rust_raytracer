use std::sync::Arc;

use crate::mat3::Mat3;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::{interval::Interval, material::Material};

use super::{Hit, HitRecord};

pub struct Plane {
    pub center: Point3,
    pub material: Arc<dyn Material>,
    
    size_half: (f64, f64),
    normal: Vec3,
    inverse_basis: Mat3,
}

impl Plane {
    pub fn new(center: Point3, uv: (Vec3, Vec3), material: Arc<dyn Material>) -> Self {
        if Vec3::dot(&uv.0, &uv.1) != 0.0 {
            panic!("The UV vectors must be orthogonal!");
        }

        let u_unit = uv.0.to_unit();
        let v_unit = uv.1.to_unit();
        let normal = Vec3::cross(&u_unit, &v_unit);

        // Since u_unit, v_unit and normal are orthonormal vectors, basis is an
        // orthogonal matrix, and thus its inverse is its transpose
        let basis = Mat3::from_columns(u_unit, v_unit, normal);
        let inverse_basis = basis.transposed();

        Plane {
            center,
            size_half: (uv.0.length().abs() * 0.5, uv.1.length().abs() * 0.5),
            material,
            normal,
            inverse_basis,
        }
    }
}

impl Hit for Plane {
    fn test(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        let dot_ray_normal = self.normal.dot(&ray.dir);

        if dot_ray_normal.abs() < 0.0001 {
            return None;
        }

        let hit_t = self.normal.dot(&(self.center - ray.origin)) / dot_ray_normal;
        if hit_t <= t.min() || t.max() <= hit_t {
            return None;
        }

        let hit_pos = ray.at(hit_t);
        let hit_on_plane = self.inverse_basis * (hit_pos - self.center);
        if hit_on_plane.x().abs() > self.size_half.0 || hit_on_plane.y().abs() > self.size_half.1 {
            return None;
        }

        Some(HitRecord::new(
            ray,
            hit_pos,
            hit_t,
            self.normal,
            Arc::as_ref(&self.material),
        ))
    }
}
