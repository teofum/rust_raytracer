use crate::aabb::{self, AxisAlignedBoundingBox};
use crate::vec4::{Point4, Vec4};

use super::Triangle;

const MAX_TRIS_PER_LEAF: usize = 50;

pub enum OctreeNodeData {
    Leaf(Vec<usize>),
    Branch([Box<OctreeNode>; 8]),
}

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
        let triangle_idx_pairs: Vec<_> = triangles
            .iter()
            .enumerate()
            .filter(|(i, _)| match filter {
                None => true,
                Some(filter) => filter.contains(i),
            })
            .collect();

        if triangle_idx_pairs.len() <= MAX_TRIS_PER_LEAF {
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
                Box::new(Self::new(
                    vertices,
                    triangles,
                    Some(&index_lists[0]),
                    [b_min, midpoint],
                )),
                Box::new(Self::new(
                    vertices,
                    triangles,
                    Some(&index_lists[1]),
                    [
                        Vec4::point(min_x, min_y, mid_z),
                        Vec4::point(mid_x, mid_y, max_z),
                    ],
                )),
                Box::new(Self::new(
                    vertices,
                    triangles,
                    Some(&index_lists[2]),
                    [
                        Vec4::point(min_x, mid_y, min_z),
                        Vec4::point(mid_x, max_y, mid_z),
                    ],
                )),
                Box::new(Self::new(
                    vertices,
                    triangles,
                    Some(&index_lists[3]),
                    [
                        Vec4::point(min_x, mid_y, mid_z),
                        Vec4::point(mid_x, max_y, max_z),
                    ],
                )),
                Box::new(Self::new(
                    vertices,
                    triangles,
                    Some(&index_lists[4]),
                    [
                        Vec4::point(mid_x, min_y, min_z),
                        Vec4::point(max_x, mid_y, mid_z),
                    ],
                )),
                Box::new(Self::new(
                    vertices,
                    triangles,
                    Some(&index_lists[5]),
                    [
                        Vec4::point(mid_x, min_y, mid_z),
                        Vec4::point(max_x, mid_y, max_z),
                    ],
                )),
                Box::new(Self::new(
                    vertices,
                    triangles,
                    Some(&index_lists[6]),
                    [
                        Vec4::point(mid_x, mid_y, min_z),
                        Vec4::point(max_x, max_y, mid_z),
                    ],
                )),
                Box::new(Self::new(
                    vertices,
                    triangles,
                    Some(&index_lists[7]),
                    [midpoint, b_max],
                )),
            ]);

            OctreeNode {
                data,
                bounding_box: [b_min, b_max],
            }
        }
    }
}
