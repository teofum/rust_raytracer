pub mod sphere;

pub use sphere::Sphere;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub hit_pos: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hit {
    fn test(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
