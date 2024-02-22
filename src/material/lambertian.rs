use std::sync::Arc;

use rand_xorshift::XorShiftRng;

use crate::object::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec4::{Color, Vec4};

use super::Material;

pub struct LambertianDiffuse {
    albedo: Arc<dyn Texture>,
}

impl LambertianDiffuse {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        LambertianDiffuse { albedo }
    }
}

impl Material for LambertianDiffuse {
    fn scatter(&self, ray: &mut Ray, hit: &HitRecord, rng: &mut XorShiftRng) -> Option<Color> {
        let scatter_dir = hit.normal() + Vec4::random_unit(rng);

        ray.origin = hit.pos();
        ray.dir = if scatter_dir.near_zero() {
            hit.normal()
        } else {
            scatter_dir
        };

        Some(self.albedo.sample(hit.uv(), &hit.pos()))
    }
}
