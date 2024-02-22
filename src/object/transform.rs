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
    ///
    /// This does *not* compute the inverse 4x4 matrix: because of how transform
    /// matrices for rotation, scale and translation are formed, we can get the
    /// inverse by calculating the inverse for the top-left 3x3 part and negating
    /// the translation column.
    ///
    /// # Panics
    /// Panics if the matrix is not invertible (determinant is =0)
    fn inverse_transform(transform: Mat4) -> Mat4 {
        // Find the inverse of the 3x3 part using Cramer's Rule
        let [a, b, c, tx, d, e, f, ty, g, h, i, tz, ..] = transform.0;
        let aa = e * i - f * h;
        let bb = f * g - d * i;
        let cc = d * h - e * g;

        let det = a * aa + b * bb + c * cc;
        assert!(det != 0.0, "Matrix is not invertible!");
        let inv_det = 1.0 / det;

        let aa = aa * inv_det;
        let bb = bb * inv_det;
        let cc = cc * inv_det;
        let dd = (c * h - b * i) * inv_det;
        let ee = (a * i - c * g) * inv_det;
        let ff = (b * g - a * h) * inv_det;
        let gg = (b * f - c * e) * inv_det;
        let hh = (c * d - a * f) * inv_det;
        let ii = (a * e - b * d) * inv_det;

        Mat4::from_values([
            aa, dd, gg, -tx, //
            bb, ee, hh, -ty, //
            cc, ff, ii, -tz, //
            0.0, 0.0, 0.0, 1.0,
        ])
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
