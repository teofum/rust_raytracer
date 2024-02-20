use rand::Rng;
use rand_xorshift::XorShiftRng;

use crate::aabb::{self, AxisAlignedBoundingBox};
use crate::interval::Interval;
use crate::ray::Ray;

use self::null_obj::NullObject;

use super::{Hit, HitRecord};

mod null_obj;

pub struct BoundingVolumeHierarchyNode {
    children: (Box<dyn Hit>, Box<dyn Hit>),
    bounds: AxisAlignedBoundingBox,
}

impl BoundingVolumeHierarchyNode {
    pub fn from(mut objects: Vec<Box<dyn Hit>>, rng: &mut XorShiftRng) -> Self {
        let axis_idx = rng.gen_range(0..3);
        let comparator = |a: &Box<dyn Hit>, b: &Box<dyn Hit>| {
            let min_a = a.get_bounding_box().0[axis_idx];
            let min_b = b.get_bounding_box().0[axis_idx];

            min_a.total_cmp(&min_b)
        };

        let object_count = objects.len();
        let children: (Box<dyn Hit>, Box<dyn Hit>);
        let bounds: AxisAlignedBoundingBox;

        if object_count == 1 {
            children = (objects.pop().unwrap(), Box::new(NullObject()));
            bounds = children.0.get_bounding_box();

            BoundingVolumeHierarchyNode { children, bounds }
        } else if object_count == 2 {
            children = (objects.pop().unwrap(), objects.pop().unwrap());
            bounds = aabb::combine_bounds(&[
                children.0.get_bounding_box(),
                children.1.get_bounding_box(),
            ]);

            BoundingVolumeHierarchyNode { children, bounds }
        } else {
            objects.sort_unstable_by(comparator);

            let midpoint = object_count / 2;
            let second_half = objects.split_off(midpoint);
            let first_half = objects;

            children = (
                Box::new(Self::from(first_half, rng)),
                Box::new(Self::from(second_half, rng)),
            );
            bounds = aabb::combine_bounds(&[
                children.0.get_bounding_box(),
                children.1.get_bounding_box(),
            ]);

            BoundingVolumeHierarchyNode { children, bounds }
        }
    }
}

impl Hit for BoundingVolumeHierarchyNode {
    fn test(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        if !aabb::test_bounding_box(self.bounds, ray, &t) {
            return None;
        }

        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t = t.max();

        if let Some(hit) = self.children.0.test(ray, Interval(t.min(), closest_t)) {
            closest_t = hit.t;
            closest_hit = Some(hit);
        }
        if let Some(hit) = self.children.1.test(ray, Interval(t.min(), closest_t)) {
            closest_hit = Some(hit);
        }

        closest_hit
    }

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.bounds
    }
}
