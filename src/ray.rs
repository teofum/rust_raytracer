use crate::vec4::{Point4, Vec4};

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point4,
    dir: Vec4,
    inv_dir: Vec4,
    sign: [u8; 3],
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
        let inv_dir = Vec4::vec(1.0 / dir.0[0], 1.0 / dir.0[1], 1.0 / dir.0[2]);
        let sign: [u8; 3] = [
            if inv_dir.x() < 0.0 { 1 } else { 0 },
            if inv_dir.y() < 0.0 { 1 } else { 0 },
            if inv_dir.z() < 0.0 { 1 } else { 0 },
        ];

        Ray {
            origin,
            dir,
            inv_dir,
            sign,
        }
    }

    pub fn at(&self, t: f64) -> Point4 {
        self.origin() + (self.dir() * t)
    }

    pub fn origin(&self) -> Vec4 {
        self.origin
    }

    pub fn dir(&self) -> Vec4 {
        self.dir
    }

    pub fn inv_dir(&self) -> Vec4 {
        self.inv_dir
    }

    pub fn sign(&self) -> [u8; 3] {
        self.sign
    }
}
