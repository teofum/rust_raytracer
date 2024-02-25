use rand_pcg::Pcg64Mcg;

use crate::aabb::AxisAlignedBoundingBox;
use crate::constants::INFINITY;
use crate::interval::Interval;
use crate::object::{Hit, HitRecord};
use crate::ray::Ray;
use crate::vec4::{Point4, Vec4};

/// Hittable object that can never be hit.
/// Used to fill out a BVH with a songle node.
pub struct NullObject();

impl Hit for NullObject {
    fn test(&self, _: &Ray, _: Interval, _: &mut Pcg64Mcg) -> Option<HitRecord> {
        None
    }

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        (INFINITY, -INFINITY)
    }

    fn pdf_value(&self, _: Point4, _: Vec4, _: &mut Pcg64Mcg) -> f64 {
        0.0
    }

    fn random(&self, _: Point4, _: &mut Pcg64Mcg) -> Vec4 {
        Vec4::vec(1.0, 0.0, 0.0)
    }
}
