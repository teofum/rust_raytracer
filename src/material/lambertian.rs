use crate::object::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

use super::Material;

pub struct LambertianDiffuse {
    albedo: Color,
}

impl LambertianDiffuse {
    pub fn new(albedo: Color) -> Self {
        LambertianDiffuse { albedo }
    }

    pub fn albedo(&self) -> Color {
        self.albedo
    }
}

impl Material for LambertianDiffuse {
    fn scatter(&self, ray: &mut Ray, hit: &HitRecord) -> Option<Color> {
        ray.origin = hit.pos();
        ray.dir = hit.normal() + Vec3::random_unit();

        Some(self.albedo)
    }
}
