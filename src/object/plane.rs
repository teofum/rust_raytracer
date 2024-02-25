use std::sync::Arc;

use rand::Rng;
use rand_pcg::Pcg64Mcg;

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
    u: Vec4,
    v: Vec4,
    area: f64,
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

        let n = Vec4::cross(&u, &v);
        let area = n.length() * 4.0;
        let normal = n.to_unit();

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
            u,
            v,
            area,
            inverse_basis,
            bounds,
        }
    }
}

impl Hit for Plane {
    fn test(&self, ray: &Ray, t: Interval, _: &mut Pcg64Mcg) -> Option<HitRecord> {
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

    fn pdf_value(&self, origin: Point4, dir: Vec4, rng: &mut Pcg64Mcg) -> f64 {
        let ray = Ray::new(origin, dir);

        if let Some(hit) = self.test(&ray, Interval(0.001, f64::INFINITY), rng) {
            let dist_squared = hit.t() * hit.t() * dir.length_squared();
            let cosine = (dir.dot(&hit.normal()) / dir.length()).abs();

            dist_squared / (cosine * self.area)
        } else {
            0.0
        }
    }

    fn random(&self, origin: Point4, rng: &mut Pcg64Mcg) -> Vec4 {
        let u = rng.gen_range(0.0..1.0) - 0.5;
        let v = rng.gen_range(0.0..1.0) - 0.5;
        let p = self.center + self.u * u + self.v * v;

        p - origin
    }
}
