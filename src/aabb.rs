use crate::constants::INFINITY;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec4::{Point4, Vec4};

pub type AxisAlignedBoundingBox = (Point4, Point4);

// Expand bounding boxes slightly to handle grazing rays
const EPSILON_VEC: Vec4 = Vec4([0.001, 0.001, 0.001, 0.0]);

pub fn combine_bounds(bounds: &[AxisAlignedBoundingBox]) -> AxisAlignedBoundingBox {
    let mut bounds_min = INFINITY;
    let mut bounds_max = -bounds_min;

    for bounding_box in bounds {
        for i in 0..3 {
            if bounding_box.0[i] < bounds_min[i] {
                bounds_min[i] = bounding_box.0[i];
            }
            if bounding_box.1[i] > bounds_max[i] {
                bounds_max[i] = bounding_box.1[i];
            }
        }
    }

    (bounds_min - EPSILON_VEC, bounds_max + EPSILON_VEC)
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

    (bounds_min - EPSILON_VEC, bounds_max + EPSILON_VEC)
}

// Fast ray-box intersection by Andrew Woo
// from Graphics Gems, 1990
pub fn test_bounding_box(
    (b_min, b_max): &AxisAlignedBoundingBox,
    ray: &Ray,
    t_int: &Interval,
) -> bool {
    let mut inside = true; // Ray origin inside bounds
    let mut quadrant: [i8; 3] = [0; 3];
    let mut candidate_plane = Vec4::point(0.0, 0.0, 0.0);
    let mut max_t = Vec4::point(0.0, 0.0, 0.0);

    for i in 0..3 {
        if ray.origin[i] < b_min[i] {
            quadrant[i] = -1;
            candidate_plane[i] = b_min[i];
            inside = false;
        } else if ray.origin[i] > b_max[i] {
            quadrant[i] = 1;
            candidate_plane[i] = b_max[i];
            inside = false;
        } else {
            quadrant[i] = 0;
        }
    }

    if inside {
        return true;
    }

    // Calculate t distance to candidate planes
    for i in 0..3 {
        max_t[i] = if quadrant[i] != 0 && ray.dir[i] != 0.0 {
            (candidate_plane[i] - ray.origin[i]) / ray.dir[i]
        } else {
            -1.0
        }
    }

    // Use the largest max_t to pick a plane to intersect
    let mut plane_idx = 0;
    for i in 1..3 {
        if max_t[i] > max_t[plane_idx] {
            plane_idx = i;
        }
    }

    if max_t[plane_idx] <= t_int.0 || max_t[plane_idx] >= t_int.1 {
        return false;
    }

    for i in 0..3 {
        if i != plane_idx {
            let hit_coord = ray.origin[i] + max_t[plane_idx] * ray.dir[i];
            if hit_coord < b_min[i] || hit_coord > b_max[i] {
                return false;
            }
        }
    }

    true
}
