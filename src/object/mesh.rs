use std::sync::Arc;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::{interval::Interval, material::Material};

use super::{Hit, HitRecord};

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Point3,
    pub normal: Vec3,
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub vert_indices: [usize; 3],
}

pub struct TriangleMesh {
    pub material: Arc<dyn Material>,
    pub flat_shading: bool,

    vertices: Vec<Vertex>,
    position: Point3,

    transformed_vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
}

impl TriangleMesh {
    pub fn new(
        vertices: Vec<Vertex>,
        triangles: Vec<Triangle>,
        material: Arc<dyn Material>,
    ) -> Self {
        TriangleMesh {
            transformed_vertices: vertices.to_vec(),
            vertices,
            position: Vec3::origin(),
            triangles,
            material,
            flat_shading: false,
        }
    }

    pub fn set_position(&mut self, pos: Point3) {
        self.position = pos;

        self.update_transformed();
    }

    fn update_transformed(&mut self) {
        for (i, vert) in self.vertices.iter().enumerate() {
            self.transformed_vertices[i].position = vert.position + self.position;
        }
    }

    fn test_tri(&self, triangle: &Triangle, ray: &Ray, t_int: Interval) -> Option<HitRecord> {
        let vertices = [
            &self.transformed_vertices[triangle.vert_indices[0]],
            &self.transformed_vertices[triangle.vert_indices[1]],
            &self.transformed_vertices[triangle.vert_indices[2]],
        ];

        // Calculate the plane the triangle lies on
        let v0 = vertices[0].position;
        let edge1 = vertices[1].position - v0;
        let edge2 = vertices[2].position - v0;

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
                vertices[0].normal * w + vertices[1].normal * u + vertices[2].normal * t
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
        //TODO bounding box test

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
