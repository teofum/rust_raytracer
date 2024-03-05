use std::sync::Arc;

use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::aabb::{self, AxisAlignedBoundingBox};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec4::{Point4, Vec4};

use self::null_obj::NullObject;

use super::{Hit, HitRecord};

mod null_obj;

pub const AXES_X: [bool; 3] = [true, false, false];
pub const AXES_Y: [bool; 3] = [false, true, false];
pub const AXES_Z: [bool; 3] = [false, false, true];
pub const AXES_XY: [bool; 3] = [true, true, false];
pub const AXES_XZ: [bool; 3] = [true, false, true];
pub const AXES_YZ: [bool; 3] = [false, true, true];
pub const AXES_ALL: [bool; 3] = [true, true, true];

#[derive(Debug)]
pub struct BoundingVolumeHierarchyNode {
    children: (Arc<dyn Hit>, Arc<dyn Hit>),
    bounds: AxisAlignedBoundingBox,
}

impl BoundingVolumeHierarchyNode {
    pub fn from(mut objects: Vec<Arc<dyn Hit>>, axes: [bool; 3], rng: &mut Pcg64Mcg) -> Self {
        let mut axis_idx = rng.gen_range(0..3);
        while !axes[axis_idx] {
            axis_idx = rng.gen_range(0..3);
        }

        let comparator = |a: &Arc<dyn Hit>, b: &Arc<dyn Hit>| {
            let min_a = a.get_bounding_box()[0][axis_idx];
            let min_b = b.get_bounding_box()[0][axis_idx];

            min_a.total_cmp(&min_b)
        };

        let object_count = objects.len();
        let children: (Arc<dyn Hit>, Arc<dyn Hit>);
        let bounds: AxisAlignedBoundingBox;

        if object_count == 1 {
            children = (objects.pop().unwrap(), Arc::new(NullObject()));
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
                Arc::new(Self::from(first_half, axes, rng)),
                Arc::new(Self::from(second_half, axes, rng)),
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
    fn test(&self, ray: &Ray, t: Interval, rng: &mut Pcg64Mcg) -> Option<HitRecord> {
        if !aabb::test_bounding_box(&self.bounds, ray, &t) {
            return None;
        }

        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t = t.max();

        if let Some(hit) = self.children.0.test(ray, Interval(t.min(), closest_t), rng) {
            closest_t = hit.t;
            closest_hit = Some(hit);
        }
        if let Some(hit) = self.children.1.test(ray, Interval(t.min(), closest_t), rng) {
            closest_hit = Some(hit);
        }

        closest_hit
    }

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.bounds
    }

    fn pdf_value(&self, _: Point4, _: Vec4, _: &mut Pcg64Mcg) -> f64 {
        0.0
    }

    fn random(&self, _: Point4, _: &mut Pcg64Mcg) -> Vec4 {
        Vec4::vec(1.0, 0.0, 0.0)
    }
}
