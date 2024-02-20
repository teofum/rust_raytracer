pub mod list;
pub mod sphere;
pub mod plane;
pub mod mesh;

pub use list::ObjectList;
pub use sphere::Sphere;
pub use plane::Plane;

use crate::aabb::AxisAlignedBoundingBox;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord<'a> {
    hit_pos: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    /// Note: outward_normal must have unit length
    pub fn new(
        ray: &Ray,
        hit_pos: Point3,
        t: f64,
        outward_normal: Vec3,
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
            front_face,
            material,
        }
    }

    pub fn pos(&self) -> Point3 {
        self.hit_pos
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
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
