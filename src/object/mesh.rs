use std::sync::Arc;

use crate::aabb::{self, AxisAlignedBoundingBox};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use super::{Hit, HitRecord};

mod octree;
use octree::{OctreeNode, OctreeNodeData};

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

    bounds: AxisAlignedBoundingBox,
    octree: OctreeNode,
}

impl TriangleMesh {
    pub fn new(
        vertices: Vec<Point3>,
        vertex_uvs: Vec<Point3>,
        vertex_normals: Vec<Vec3>,
        triangles: Vec<Triangle>,
        material: Arc<dyn Material>,
    ) -> Self {
        let bounds = aabb::get_bounding_box(&vertices);
        let octree = OctreeNode::new(&vertices, &triangles, None, bounds);

        TriangleMesh {
            transformed_vertices: vertices.to_vec(),
            vertices,
            vertex_uvs,
            vertex_normals,
            position: Vec3::origin(),
            triangles,
            material,
            flat_shading: false,
            bounds,
            octree,
        }
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
        self.bounds = aabb::get_bounding_box(&self.transformed_vertices);
        self.rebuild_octree();
    }

    fn rebuild_octree(&mut self) {
        self.octree = OctreeNode::new(
            &self.transformed_vertices,
            &self.triangles,
            None,
            self.bounds,
        );
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

    fn test_octree_node(&self, node: &OctreeNode, ray: &Ray, t: Interval) -> Option<HitRecord> {
        if !aabb::test_bounding_box(node.bounding_box, ray, &t) {
            return None;
        }

        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t = t.max();

        match &node.data {
            OctreeNodeData::Leaf(triangle_indices) => {
                for idx in triangle_indices {
                    let triangle = &self.triangles[*idx];

                    if let Some(hit) = self.test_tri(triangle, ray, Interval(t.min(), closest_t)) {
                        closest_t = hit.t;
                        closest_hit = Some(hit);
                    }
                }
            }
            OctreeNodeData::Branch(nodes) => {
                for node in nodes {
                    if let Some(hit) =
                        self.test_octree_node(node, ray, Interval(t.min(), closest_t))
                    {
                        closest_t = hit.t;
                        closest_hit = Some(hit);
                    }
                }
            }
        }

        closest_hit
    }
}

impl Hit for TriangleMesh {
    fn test(&self, ray: &Ray, t: Interval) -> Option<HitRecord> {
        self.test_octree_node(&self.octree, ray, t)
    }

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.bounds
    }
}
