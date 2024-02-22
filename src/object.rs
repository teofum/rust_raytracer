pub mod bvh;
pub mod list;
pub mod mesh;
pub mod plane;
pub mod sphere;
pub mod transform;

pub use bvh::BoundingVolumeHierarchyNode;
pub use list::ObjectList;
pub use plane::Plane;
pub use sphere::Sphere;

use crate::aabb::AxisAlignedBoundingBox;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec4::{Point4, Vec4};

pub struct HitRecord<'a> {
    hit_pos: Point4,
    normal: Vec4,
    t: f64,
    uv: (f64, f64),
    front_face: bool,
    material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    /// Note: outward_normal must have unit length
    pub fn new(
        ray: &Ray,
        hit_pos: Point4,
        t: f64,
        uv: (f64, f64),
        outward_normal: Vec4,
        material: &'a dyn Material,
    ) -> Self {
        let front_face = ray.dir.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            hit_pos,
            normal,
            t,
            uv,
            front_face,
            material,
        }
    }

    pub fn pos(&self) -> Point4 {
        self.hit_pos
    }

    pub fn normal(&self) -> Vec4 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn uv(&self) -> (f64, f64) {
        self.uv
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn material(&self) -> &'a dyn Material {
        self.material
    }
}

pub trait Hit: Send + Sync {
    fn test(&self, ray: &Ray, t: Interval) -> Option<HitRecord>;

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox;
}
