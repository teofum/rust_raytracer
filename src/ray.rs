use crate::vec4::{Point4, Vec4};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point4,
    pub dir: Vec4,
}

impl Ray {
    /// Create a new ray.
    ///
    /// # Params
    ///
    /// `origin` is the origin point of the ray.
    ///
    /// `dir` is the direction vector of the ray.
    pub fn new(origin: Point4, dir: Vec4) -> Self {
        Ray { origin, dir }
    }

    pub fn at(&self, t: f64) -> Point4 {
        self.origin + (self.dir * t)
    }
}
