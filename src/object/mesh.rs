use std::sync::Arc;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::{interval::Interval, material::Material};

use super::{Hit, HitRecord};

#[derive(Clone, Copy)]
pub struct Triangle {
    pub vert_indices: [usize; 3],
    pub normal_indices: [usize; 3],
    pub uv_indices: Option<[usize; 3]>,
}

pub struct TriangleMesh {
    pub material: Arc<dyn Material>,
    pub flat_shading: bool,

    vertices: Vec<Point3>,
    vertex_uvs: Vec<Point3>,
    vertex_normals: Vec<Vec3>,
    position: Point3,

    transformed_vertices: Vec<Point3>,
    triangles: Vec<Triangle>,

    bounding_box: (Point3, Point3),
}

impl TriangleMesh {
    pub fn new(
        vertices: Vec<Point3>,
        vertex_uvs: Vec<Point3>,
        vertex_normals: Vec<Vec3>,
        triangles: Vec<Triangle>,
        material: Arc<dyn Material>,
    ) -> Self {
        let mut mesh = TriangleMesh {
            transformed_vertices: vertices.to_vec(),
            vertices,
            vertex_uvs,
            vertex_normals,
            position: Vec3::origin(),
            triangles,
            material,
            flat_shading: false,
            bounding_box: (Vec3::origin(), Vec3::origin()),
        };

        mesh.calculate_bounding_box();
        mesh
    }

    pub fn set_position(&mut self, pos: Point3) {
        self.position = pos;

        self.update_transformed();
    }

    fn update_transformed(&mut self) {
        for (i, vert) in self.vertices.iter().enumerate() {
            self.transformed_vertices[i] = *vert + self.position;
        }
        self.calculate_bounding_box();
    }

    fn calculate_bounding_box(&mut self) {
        let mut bounds_min = Vec3(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut bounds_max = -bounds_min;

        for vertex_pos in &self.transformed_vertices {
            for i in 0..3 {
                if vertex_pos[i] < bounds_min[i] {
                    bounds_min[i] = vertex_pos[i];
                }
                if vertex_pos[i] > bounds_max[i] {
                    bounds_max[i] = vertex_pos[i];
                }
            }
        }

        self.bounding_box = (bounds_min, bounds_max);
    }

    // Fast ray-box intersection by Andrew Woo
    // from Graphics Gems, 1990
    fn test_bounds(&self, ray: &Ray, t_int: &Interval) -> bool {
        let (b_min, b_max) = self.bounding_box;

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

    // Möller–Trumbore intersection
    fn test_tri(&self, triangle: &Triangle, ray: &Ray, t_int: Interval) -> Option<HitRecord> {
        let [v0, v1, v2] = [
            self.transformed_vertices[triangle.vert_indices[0]],
            self.transformed_vertices[triangle.vert_indices[1]],
            self.transformed_vertices[triangle.vert_indices[2]],
        ];

        // Calculate the plane the triangle lies on
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;

        // Compute the barycentric coordinates t, u, v using Cramer's Rule
        let ray_x_edge2 = Vec3::cross(&ray.dir, &edge2);

        let det = edge1.dot(&ray_x_edge2);
        if det.abs() < f64::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let b = ray.origin - v0; // The 'b' vector in the Ax = b equation we're solving

        let u = b.dot(&ray_x_edge2) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None; // Intersection lies outside triangle
        }

        // Uses the property a.(b×c) = b.(c×a) = c.(a×b)
        // and a×b = b×(-a)
        let b_x_edge1 = Vec3::cross(&b, &edge1);
        let v = ray.dir.dot(&b_x_edge1) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None; // Intersection lies outside triangle
        }

        let t = edge2.dot(&b_x_edge1) * inv_det;

        if t <= t_int.min() || t_int.max() <= t {
            None
        } else {
            let hit_pos = ray.at(t);

            // Calculate normals
            let normal = if self.flat_shading {
                Vec3::cross(&edge1, &edge2).to_unit()
            } else {
                let w = 1.0 - u - v;

                let [n0, n1, n2] = [
                    self.vertex_normals[triangle.normal_indices[0]],
                    self.vertex_normals[triangle.normal_indices[1]],
                    self.vertex_normals[triangle.normal_indices[2]],
                ];

                n0 * w + n1 * u + n2 * v
            };

            Some(HitRecord::new(
                ray,
                hit_pos,
                t,
                normal,
                Arc::as_ref(&self.material),
            ))
        }
    }
}

impl Hit for TriangleMesh {
    fn test(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        if !self.test_bounds(ray, &t) {
            return None;
        }

        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t = t.max();

        for triangle in &self.triangles {
            if let Some(hit) = self.test_tri(triangle, ray, Interval(t.min(), closest_t)) {
                closest_t = hit.t;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}
