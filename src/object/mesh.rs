use std::sync::Arc;

use crate::aabb::{self, AxisAlignedBoundingBox};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec4::{Point4, Vec4};

use super::{Hit, HitRecord};

mod octree;
use octree::{OctreeNode, OctreeNodeData};
use rand_pcg::Pcg64Mcg;

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub vert_indices: [usize; 3],
    pub normal_indices: [usize; 3],
    pub uv_indices: Option<[usize; 3]>,
}

#[derive(Debug)]
pub struct TriangleMesh {
    pub material: Arc<dyn Material>,
    pub flat_shading: bool,
    pub hit_back_faces: bool,

    vertices: Vec<Point4>,
    vertex_uvs: Vec<Point4>,
    vertex_normals: Vec<Vec4>,
    triangles: Vec<Triangle>,

    bounds: AxisAlignedBoundingBox,
    octree: OctreeNode,
}

impl TriangleMesh {
    pub fn new(
        vertices: Vec<Point4>,
        vertex_uvs: Vec<Point4>,
        vertex_normals: Vec<Vec4>,
        triangles: Vec<Triangle>,
        material: Arc<dyn Material>,
    ) -> Self {
        let bounds = aabb::get_bounding_box(&vertices);
        let octree = OctreeNode::new(&vertices, &triangles, None, bounds);

        TriangleMesh {
            vertices,
            vertex_uvs,
            vertex_normals,
            triangles,
            material,
            flat_shading: false,
            hit_back_faces: false,
            bounds,
            octree,
        }
    }

    // Möller–Trumbore intersection
    fn test_tri(&self, triangle: &Triangle, ray: &Ray, t_int: Interval) -> Option<HitRecord> {
        let [v0, v1, v2] = [
            self.vertices[triangle.vert_indices[0]],
            self.vertices[triangle.vert_indices[1]],
            self.vertices[triangle.vert_indices[2]],
        ];

        // Calculate the plane the triangle lies on
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;

        // Compute the barycentric coordinates t, u, v using Cramer's Rule
        let ray_x_edge2 = Vec4::cross(&ray.dir(), &edge2);

        let det = edge1.dot(&ray_x_edge2);
        let dd = if self.hit_back_faces { det.abs() } else { det };
        if dd < f64::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let b = ray.origin() - v0; // The 'b' vector in the Ax = b equation we're solving

        let u = b.dot(&ray_x_edge2) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None; // Intersection lies outside triangle
        }

        // Uses the property a.(b×c) = b.(c×a) = c.(a×b)
        // and a×b = b×(-a)
        let b_x_edge1 = Vec4::cross(&b, &edge1);
        let v = ray.dir().dot(&b_x_edge1) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None; // Intersection lies outside triangle
        }

        let t = edge2.dot(&b_x_edge1) * inv_det;

        if t <= t_int.min() || t_int.max() <= t {
            None
        } else {
            let hit_pos = ray.at(t);
            let w = 1.0 - u - v;

            // Calculate normals
            let normal = if self.flat_shading {
                Vec4::cross(&edge1, &edge2).to_unit()
            } else {
                let [n0, n1, n2] = [
                    self.vertex_normals[triangle.normal_indices[0]],
                    self.vertex_normals[triangle.normal_indices[1]],
                    self.vertex_normals[triangle.normal_indices[2]],
                ];

                n0 * w + n1 * u + n2 * v
            };

            let tex_coords = match triangle.uv_indices {
                Some(uv_i) => {
                    let [uv0, uv1, uv2] = [
                        self.vertex_uvs[uv_i[0]],
                        self.vertex_uvs[uv_i[1]],
                        self.vertex_uvs[uv_i[2]],
                    ];

                    uv0 * w + uv1 * u + uv2 * v
                }
                None => Vec4::vec(0.0, 0.0, 0.0),
            };

            Some(HitRecord::new(
                ray,
                hit_pos,
                t,
                (tex_coords.x(), tex_coords.y()),
                normal,
                normal,
                normal,
                Arc::as_ref(&self.material),
            ))
        }
    }

    fn test_octree_node(&self, node: &OctreeNode, ray: &Ray, t: Interval) -> Option<HitRecord> {
        if !aabb::test_bounding_box(&node.bounding_box, ray, &t) {
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
    fn test(&self, ray: &Ray, t: Interval, _: &mut Pcg64Mcg) -> Option<HitRecord> {
        self.test_octree_node(&self.octree, ray, t)
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
