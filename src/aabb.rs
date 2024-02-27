use crate::constants::INFINITY;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec4::{Point4, Vec4};

pub type AxisAlignedBoundingBox = [Point4; 2];

// Expand bounding boxes slightly to handle grazing rays
const EPSILON_VEC: Vec4 = Vec4([0.001, 0.001, 0.001, 0.0]);

pub fn combine_bounds(bounds: &[AxisAlignedBoundingBox]) -> AxisAlignedBoundingBox {
    let mut bounds_min = INFINITY;
    let mut bounds_max = -bounds_min;

    for bounding_box in bounds {
        for i in 0..3 {
            if bounding_box[0][i] < bounds_min[i] {
                bounds_min[i] = bounding_box[0][i];
            }
            if bounding_box[1][i] > bounds_max[i] {
                bounds_max[i] = bounding_box[1][i];
            }
        }
    }

    [bounds_min - EPSILON_VEC, bounds_max + EPSILON_VEC]
}

pub fn get_bounding_box(vertices: &[Point4]) -> AxisAlignedBoundingBox {
    let mut bounds_min = INFINITY;
    let mut bounds_max = -bounds_min;

    for vertex_pos in vertices {
        for i in 0..3 {
            if vertex_pos[i] < bounds_min[i] {
                bounds_min[i] = vertex_pos[i];
            }
            if vertex_pos[i] > bounds_max[i] {
                bounds_max[i] = vertex_pos[i];
            }
        }
    }

    [bounds_min - EPSILON_VEC, bounds_max + EPSILON_VEC]
}

// From "An Efficient and Robust Ray–Box Intersection Algorithm"
// by Amy Williams et al.
#[inline(always)]
pub fn test_bounding_box(bounds: &AxisAlignedBoundingBox, ray: &Ray, t_int: &Interval) -> bool {
    let inv_dir = ray.inv_dir();
    let sign = ray.sign();

    // Indexing into a Vec3 is slightly faster than calling the x/y/z getters, enough
    // that it makes a difference with large BVHs/octrees
    let mut t_min = (bounds[sign[0] as usize][0] - ray.origin()[0]) * inv_dir[0];
    let mut t_max = (bounds[1 - sign[0] as usize][0] - ray.origin()[0]) * inv_dir[0];
    let ty_min = (bounds[sign[1] as usize][1] - ray.origin()[1]) * inv_dir[1];
    let ty_max = (bounds[1 - sign[1] as usize][1] - ray.origin()[1]) * inv_dir[1];

    if (t_min > ty_max) || (ty_min > t_max) {
        return false;
    }

    if ty_min > t_min {
        t_min = ty_min;
    }
    if ty_max < t_max {
        t_max = ty_max;
    }

    let tz_min = (bounds[sign[2] as usize][2] - ray.origin()[2]) * inv_dir[2];
    let tz_max = (bounds[1 - sign[2] as usize][2] - ray.origin()[2]) * inv_dir[2];

    if (t_min > tz_max) || (tz_min > t_max) {
        return false;
    }

    if tz_min > t_min {
        t_min = tz_min;
    }
    if tz_max < t_max {
        t_max = tz_max;
    }

    t_min < t_int.1 && t_max > t_int.0
}
