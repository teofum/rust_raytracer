use crate::aabb::{self, AxisAlignedBoundingBox};
use crate::interval::Interval;
use crate::mat4::Mat4;
use crate::ray::Ray;
use crate::vec4::Vec4;

use super::{Hit, HitRecord};

pub struct Transform {
    object: Box<dyn Hit>,
    transform: Mat4,
    inv_transform: Mat4,
    bounds: AxisAlignedBoundingBox,
}

impl Transform {
    pub fn new(object: Box<dyn Hit>, transform: Mat4) -> Self {
        let inv_transform = Self::inverse_transform(transform);
        let bounds = Self::get_bounds(object.get_bounding_box(), transform);

        Transform {
            object,
            transform,
            inv_transform,
            bounds,
        }
    }

    fn get_bounds(
        (o_min, o_max): AxisAlignedBoundingBox,
        transform: Mat4,
    ) -> AxisAlignedBoundingBox {
        let (sx, sy, sz) = (o_max - o_min).xyz();
        let mut corners = [
            o_min,
            o_min + Vec4::vec(0.0, 0.0, sz),
            o_min + Vec4::vec(0.0, sy, 0.0),
            o_min + Vec4::vec(0.0, sy, sz),
            o_min + Vec4::vec(sx, 0.0, 0.0),
            o_min + Vec4::vec(sx, 0.0, sz),
            o_min + Vec4::vec(sx, sy, 0.0),
            o_max,
        ];

        for i in 0..corners.len() {
            corners[i] = transform * corners[i]
        }

        aabb::get_bounding_box(&corners)
    }

    /// Returns the inverse of a transform matrix.
    pub fn inverse_transform(transform: Mat4) -> Mat4 {
        let (mut u, mut v, mut w, mut t) = (
            transform.column(0),
            transform.column(1),
            transform.column(2),
            transform.column(3),
        );

        t[3] = 0.0;
        u[3] = -Vec4::dot(&u, &t);
        v[3] = -Vec4::dot(&v, &t);
        w[3] = -Vec4::dot(&w, &t);

        Mat4::from_rows(u, v, w, Vec4::point(0.0, 0.0, 0.0))
    }
}

impl Hit for Transform {
    fn test(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        // Transform ray to object space
        let ray_obj = Ray {
            origin: self.inv_transform * ray.origin,
            dir: self.inv_transform * ray.dir,
        };

        // Test for hit in object space
        if let Some(mut hit) = self.object.test(&ray_obj, t) {
            // Transform position and normal to world space
            hit.hit_pos = self.transform * hit.hit_pos;
            hit.normal = self.transform * hit.normal;

            Some(hit)
        } else {
            None
        }
    }

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.bounds
    }
}
