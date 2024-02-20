use crate::aabb::AxisAlignedBoundingBox;
use crate::constants::INFINITY_VEC;
use crate::interval::Interval;
use crate::object::{Hit, HitRecord};
use crate::ray::Ray;

/// Hittable object that can never be hit.
/// Used to fill out a BVH with a songle node.
pub struct NullObject();

impl Hit for NullObject {
    fn test(&self, _: &Ray, _: Interval) -> Option<HitRecord> {
        None
    }

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        (INFINITY_VEC, -INFINITY_VEC)
    }
}
