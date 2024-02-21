use std::sync::Arc;

use crate::aabb::{get_bounding_box, AxisAlignedBoundingBox};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::{interval::Interval, material::Material};

use super::{Hit, HitRecord};

pub struct Plane {
    pub material: Arc<dyn Material>,

    center: Point3,
    size_half: (f64, f64),
    normal: Vec3,
    u: Vec3,
    v: Vec3,
    bounds: AxisAlignedBoundingBox,
}

impl Plane {
    pub fn new(center: Point3, (u, v): (Vec3, Vec3), material: Arc<dyn Material>) -> Self {
        if Vec3::dot(&u, &v) != 0.0 {
            panic!("The UV vectors must be orthogonal!");
        }

        let u_unit = u.to_unit();
        let v_unit = v.to_unit();
        let normal = Vec3::cross(&u_unit, &v_unit);

        let corners = [
            center + u + v,
            center + u - v,
            center - u + v,
            center - u - v,
        ];

        let bounds = get_bounding_box(&corners);

        Plane {
            center,
            size_half: (u.length().abs(), v.length().abs()),
            material,
            normal,
            u: u_unit,
            v: v_unit,
            bounds,
        }
    }
}

impl Hit for Plane {
    fn test(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        let dot_ray_normal = self.normal.dot(&ray.dir);

        if dot_ray_normal.abs() < f64::EPSILON {
            return None;
        }

        let hit_t = self.normal.dot(&(self.center - ray.origin)) / dot_ray_normal;
        if hit_t <= t.min() || t.max() <= hit_t {
            return None;
        }

        let hit_pos = ray.at(hit_t);
        let rel_pos = hit_pos - self.center;
        let (u, v) = (
            (self.u * rel_pos.dot(&self.u)).length(),
            (self.v * rel_pos.dot(&self.v)).length(),
        );
        if u.abs() > self.size_half.0 || v.abs() > self.size_half.1 {
            return None;
        }

        let uv = (
            u / (self.size_half.0 * 2.0) + 0.5,
            v / (self.size_half.1 * 2.0) + 0.5,
        );

        Some(HitRecord::new(
            ray,
            hit_pos,
            hit_t,
            uv,
            self.normal,
            Arc::as_ref(&self.material),
        ))
    }

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.bounds
    }
}
