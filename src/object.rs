use std::fmt::Debug;

use rand_pcg::Pcg64Mcg;

use crate::aabb::AxisAlignedBoundingBox;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec4::{Point4, Vec4};

pub mod bvh;
pub mod list;
pub mod mesh;
pub mod obj_box;
pub mod plane;
pub mod sky;
pub mod sphere;
pub mod sun;
pub mod transform;
pub mod volume;

pub use bvh::BoundingVolumeHierarchyNode;
pub use list::ObjectList;
pub use obj_box::make_box;
pub use plane::Plane;
pub use sky::Sky;
pub use sphere::Sphere;
pub use sun::Sun;
pub use transform::Transform;
pub use volume::Volume;

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
        let front_face = ray.dir().dot(&outward_normal) < 0.0;
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

pub trait Hit: Send + Sync + Debug {
    fn test(&self, ray: &Ray, t: Interval, rng: &mut Pcg64Mcg) -> Option<HitRecord>;

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox;

    fn pdf_value(&self, origin: Point4, dir: Vec4, rng: &mut Pcg64Mcg) -> f64;

    fn random(&self, origin: Point4, rng: &mut Pcg64Mcg) -> Vec4;
}
