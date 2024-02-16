use rand_xorshift::XorShiftRng;

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
    fn scatter(&self, ray: &mut Ray, hit: &HitRecord, rng: &mut XorShiftRng) -> Option<Color> {
        let scatter_dir = hit.normal() + Vec3::random_unit(rng);

        ray.origin = hit.pos();
        ray.dir = if scatter_dir.near_zero() {
            hit.normal()
        } else {
            scatter_dir
        };

        Some(self.albedo)
    }
}
