use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    /// Create a new ray.
    ///
    /// # Params
    ///
    /// `origin` is the origin point of the ray.
    /// 
    /// `direction` is the direction vector of the ray.
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (self.dir * t)
    }
}
