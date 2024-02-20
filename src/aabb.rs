use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub type AxisAlignedBoundingBox = (Point3, Point3);

// Expand bounding boxes slightly to handle grazing rays
const EPSILON_VEC: Vec3 = Vec3(0.001, 0.001, 0.001);

pub fn combine_bounds(bounds: &[AxisAlignedBoundingBox]) -> AxisAlignedBoundingBox {
    let mut bounds_min = Vec3(f64::INFINITY, f64::INFINITY, f64::INFINITY);
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

pub fn get_bounding_box(vertices: &[Point3]) -> AxisAlignedBoundingBox {
    let mut bounds_min = Vec3(f64::INFINITY, f64::INFINITY, f64::INFINITY);
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
    (b_min, b_max): AxisAlignedBoundingBox,
    ray: &Ray,
    t_int: &Interval,
) -> bool {
    let mut inside = true; // Ray origin inside bounds
    let mut quadrant: [i8; 3] = [0; 3];
    let mut candidate_plane = Vec3::origin();
    let mut max_t = Vec3::origin();

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
