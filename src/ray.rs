use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    /// Create a new ray.
    ///
    /// # Params
    ///
    /// `origin` is the origin point of the ray.
    ///
    /// `dir` is the direction vector of the ray.
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (self.dir * t)
    }
}
