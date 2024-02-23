use rand_pcg::Pcg64Mcg;

use crate::aabb::{self, AxisAlignedBoundingBox};
use crate::constants::INFINITY;
use crate::interval::Interval;
use crate::ray::Ray;

use super::{Hit, HitRecord};

pub struct ObjectList {
    objects: Vec<Box<dyn Hit>>,
    bounds: AxisAlignedBoundingBox,

    /// Disables the bounding box check before hit test.
    ///
    /// Needed as a workaround for volumes. Don't use it otherwise as it has a
    /// big impact on performance.
    pub disable_bounds_check: bool,
}

impl ObjectList {
    pub fn new() -> Self {
        ObjectList {
            objects: Vec::new(),
            bounds: (INFINITY, -INFINITY),
            disable_bounds_check: false,
        }
    }

    pub fn from(objects: Vec<Box<dyn Hit>>) -> Self {
        let object_bounds: Vec<_> = objects.iter().map(|obj| obj.get_bounding_box()).collect();
        let bounds = aabb::combine_bounds(&object_bounds);

        ObjectList {
            objects,
            bounds,
            disable_bounds_check: false,
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.bounds = (INFINITY, -INFINITY);
    }

    pub fn add(&mut self, object: Box<dyn Hit>) {
        self.bounds = aabb::combine_bounds(&[self.bounds, object.get_bounding_box()]);
        self.objects.push(object);
    }
}

impl Hit for ObjectList {
    fn test(&self, ray: &Ray, t: Interval, rng: &mut Pcg64Mcg) -> Option<HitRecord> {
        if !self.disable_bounds_check && !aabb::test_bounding_box(&self.bounds, ray, &t) {
            return None;
        }

        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t = t.max();

        for object in &self.objects {
            if let Some(hit) = object.test(ray, Interval(t.min(), closest_t), rng) {
                closest_t = hit.t;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.bounds
    }
}
