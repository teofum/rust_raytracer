use std::sync::Arc;

use crate::aabb::{get_bounding_box, AxisAlignedBoundingBox};
use crate::mat4::Mat4;
use crate::ray::Ray;
use crate::vec4::{Point4, Vec4};
use crate::{interval::Interval, material::Material};

use super::{Hit, HitRecord};

pub struct Plane {
    pub material: Arc<dyn Material>,

    center: Point4,
    size_half: (f64, f64),
    normal: Vec4,
    inverse_basis: Mat4,
    bounds: AxisAlignedBoundingBox,
}

impl Plane {
    pub fn new(center: Point4, (u, v): (Vec4, Vec4), material: Arc<dyn Material>) -> Self {
        if Vec4::dot(&u, &v) != 0.0 {
            panic!("The UV vectors must be orthogonal!");
        }

        let u_unit = u.to_unit();
        let v_unit = v.to_unit();
        let normal = Vec4::cross(&u_unit, &v_unit);

        // Since u_unit, v_unit and normal are orthonormal vectors, basis is an
        // orthogonal matrix, and thus its inverse is its transpose
        let basis = Mat4::from_columns(u_unit, v_unit, normal, Vec4::point(0.0, 0.0, 0.0));
        let inverse_basis = basis.transposed();

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
            inverse_basis,
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
        let hit_on_plane = self.inverse_basis * (hit_pos - self.center);
        if hit_on_plane.x().abs() > self.size_half.0 || hit_on_plane.y().abs() > self.size_half.1 {
            return None;
        }

        let uv = (
            hit_on_plane.x() / (self.size_half.0 * 2.0) + 0.5,
            hit_on_plane.y() / (self.size_half.1 * 2.0) + 0.5,
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
