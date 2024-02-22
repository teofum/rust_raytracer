use rand_xorshift::XorShiftRng;

use crate::aabb::AxisAlignedBoundingBox;
use crate::constants::INFINITY;
use crate::interval::Interval;
use crate::object::{Hit, HitRecord};
use crate::ray::Ray;

/// Hittable object that can never be hit.
/// Used to fill out a BVH with a songle node.
pub struct NullObject();

impl Hit for NullObject {
    fn test(&self, _: &Ray, _: Interval, _: &mut XorShiftRng) -> Option<HitRecord> {
        None
    }

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        (INFINITY, -INFINITY)
    }
}
