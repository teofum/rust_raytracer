use std::sync::Arc;

use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::aabb::{get_bounding_box, AxisAlignedBoundingBox};
use crate::ray::Ray;
use crate::vec4::{Point4, Vec4};
use crate::{interval::Interval, material::Material};

use super::{Hit, HitRecord};

#[derive(Debug)]
pub struct Plane {
    pub material: Arc<dyn Material>,
    pub render_backface: bool,

    corner: Point4,
    normal: Vec4,
    u: Vec4,
    v: Vec4,
    inv_u: Vec4,
    inv_v: Vec4,
    area: f64,
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

        let corners = [
            center + u + v,
            center + u - v,
            center - u + v,
            center - u - v,
        ];

        let bounds = get_bounding_box(&corners);

        Plane {
            corner: corners[3],
            material,
            normal,
            u,
            v,
            inv_u: u_unit * 0.5 / u.length(),
            inv_v: v_unit * 0.5 / v.length(),
            area,
            bounds,
            render_backface: false,
        }
    }
}

impl Hit for Plane {
    fn test(&self, ray: &Ray, t: Interval, _: &mut Pcg64Mcg) -> Option<HitRecord> {
        let dot_ray_normal = self.normal.dot(&ray.dir());

        let dd = if self.render_backface {
            dot_ray_normal.abs()
        } else {
            -dot_ray_normal
        };
        if dd < f64::EPSILON {
            return None;
        }

        let hit_t = self.normal.dot(&(self.corner - ray.origin())) / dot_ray_normal;
        if hit_t <= t.0 || t.1 <= hit_t {
            return None;
        }

        let hit_pos = ray.at(hit_t);
        let local_pos = hit_pos - self.corner;
        let u = local_pos.dot(&self.inv_u);
        let v = local_pos.dot(&self.inv_v);
        if u < 0.0 || u > 1.0 || v < 0.0 || v > 1.0 {
            return None;
        }

        Some(HitRecord::new(
            ray,
            hit_pos,
            hit_t,
            (u, v),
            self.normal,
            self.u.to_unit(),
            self.v.to_unit(),
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
        let u = rng.gen_range(0.0..1.0);
        let v = rng.gen_range(0.0..1.0);
        let p = self.corner + self.u * u + self.v * v;

        p - origin
    }
}
