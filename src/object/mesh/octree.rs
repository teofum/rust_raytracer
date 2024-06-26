use crate::aabb::{self, AxisAlignedBoundingBox};
use crate::vec4::{Point4, Vec4};

use super::Triangle;

const MAX_TRIS_PER_LEAF: usize = 50;
const MAX_DEPTH: usize = 50;

#[derive(Debug)]
pub enum OctreeNodeData {
    Leaf(Vec<usize>),
    Branch([Box<OctreeNode>; 8]),
}

#[derive(Debug)]
pub struct OctreeNode {
    pub data: OctreeNodeData,
    pub bounding_box: AxisAlignedBoundingBox,
}

impl OctreeNode {
    pub fn new(
        vertices: &Vec<Point4>,
        triangles: &Vec<Triangle>,
        filter: Option<&Vec<usize>>,
        [b_min, b_max]: AxisAlignedBoundingBox,
    ) -> Self {
        Self::new_impl(vertices, triangles, filter, [b_min, b_max], 0)
    }

    fn new_impl(
        vertices: &Vec<Point4>,
        triangles: &Vec<Triangle>,
        filter: Option<&Vec<usize>>,
        [b_min, b_max]: AxisAlignedBoundingBox,
        depth: usize,
    ) -> Self {
        let triangle_idx_pairs: Vec<_> = match filter {
            None => triangles.iter().enumerate().collect(),
            Some(filter) => filter.iter().map(|i| (*i, &triangles[*i])).collect(),
        };

        if triangle_idx_pairs.len() <= MAX_TRIS_PER_LEAF || depth >= MAX_DEPTH {
            let indices: Vec<_> = triangle_idx_pairs.into_iter().map(|(i, _)| i).collect();
            OctreeNode {
                data: OctreeNodeData::Leaf(indices),
                bounding_box: [b_min, b_max],
            }
        } else {
            let midpoint = (b_min + b_max) / 2.0;

            let mut index_lists: [Vec<usize>; 8] = [
                Vec::new(), // -x -y -z
                Vec::new(), // -x -y +z
                Vec::new(), // -x +y -z
                Vec::new(), // -x +y +z
                Vec::new(), // +x -y -z
                Vec::new(), // +x -y +z
                Vec::new(), // +x +y -z
                Vec::new(), // +x +y +z
            ];

            for (index, triangle) in triangle_idx_pairs {
                // Get triangle bounds
                let triangle_vertices = [
                    vertices[triangle.vert_indices[0]],
                    vertices[triangle.vert_indices[1]],
                    vertices[triangle.vert_indices[2]],
                ];

                let [t_min, t_max] = aabb::get_bounding_box(&triangle_vertices);

                let mut in_lists = [true; 8];
                if t_min.x() > midpoint.x() {
                    in_lists[0] = false;
                    in_lists[1] = false;
                    in_lists[2] = false;
                    in_lists[3] = false;
                }
                if t_max.x() < midpoint.x() {
                    in_lists[4] = false;
                    in_lists[5] = false;
                    in_lists[6] = false;
                    in_lists[7] = false;
                }
                if t_min.y() > midpoint.y() {
                    in_lists[0] = false;
                    in_lists[1] = false;
                    in_lists[4] = false;
                    in_lists[5] = false;
                }
                if t_max.y() < midpoint.y() {
                    in_lists[2] = false;
                    in_lists[3] = false;
                    in_lists[6] = false;
                    in_lists[7] = false;
                }
                if t_min.z() > midpoint.z() {
                    in_lists[0] = false;
                    in_lists[2] = false;
                    in_lists[4] = false;
                    in_lists[6] = false;
                }
                if t_max.z() < midpoint.z() {
                    in_lists[1] = false;
                    in_lists[3] = false;
                    in_lists[5] = false;
                    in_lists[7] = false;
                }

                for i in 0..8 {
                    if in_lists[i] {
                        index_lists[i].push(index);
                    }
                }
            }

            let min_x = b_min.x();
            let min_y = b_min.y();
            let min_z = b_min.z();
            let max_x = b_max.x();
            let max_y = b_max.y();
            let max_z = b_max.z();
            let mid_x = midpoint.x();
            let mid_y = midpoint.y();
            let mid_z = midpoint.z();

            let data = OctreeNodeData::Branch([
                Box::new(Self::new_impl(
                    vertices,
                    triangles,
                    Some(&index_lists[0]),
                    [b_min, midpoint],
                    depth + 1,
                )),
                Box::new(Self::new_impl(
                    vertices,
                    triangles,
                    Some(&index_lists[1]),
                    [
                        Vec4::point(min_x, min_y, mid_z),
                        Vec4::point(mid_x, mid_y, max_z),
                    ],
                    depth + 1,
                )),
                Box::new(Self::new_impl(
                    vertices,
                    triangles,
                    Some(&index_lists[2]),
                    [
                        Vec4::point(min_x, mid_y, min_z),
                        Vec4::point(mid_x, max_y, mid_z),
                    ],
                    depth + 1,
                )),
                Box::new(Self::new_impl(
                    vertices,
                    triangles,
                    Some(&index_lists[3]),
                    [
                        Vec4::point(min_x, mid_y, mid_z),
                        Vec4::point(mid_x, max_y, max_z),
                    ],
                    depth + 1,
                )),
                Box::new(Self::new_impl(
                    vertices,
                    triangles,
                    Some(&index_lists[4]),
                    [
                        Vec4::point(mid_x, min_y, min_z),
                        Vec4::point(max_x, mid_y, mid_z),
                    ],
                    depth + 1,
                )),
                Box::new(Self::new_impl(
                    vertices,
                    triangles,
                    Some(&index_lists[5]),
                    [
                        Vec4::point(mid_x, min_y, mid_z),
                        Vec4::point(max_x, mid_y, max_z),
                    ],
                    depth + 1,
                )),
                Box::new(Self::new_impl(
                    vertices,
                    triangles,
                    Some(&index_lists[6]),
                    [
                        Vec4::point(mid_x, mid_y, min_z),
                        Vec4::point(max_x, max_y, mid_z),
                    ],
                    depth + 1,
                )),
                Box::new(Self::new_impl(
                    vertices,
                    triangles,
                    Some(&index_lists[7]),
                    [midpoint, b_max],
                    depth + 1,
                )),
            ]);

            OctreeNode {
                data,
                bounding_box: [b_min, b_max],
            }
        }
    }
}
