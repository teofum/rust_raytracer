use std::sync::Arc;

use rand_xorshift::XorShiftRng;

use crate::object::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec4::{Color, Vec4};

use super::Material;

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Isotropic { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &mut Ray, hit: &HitRecord, rng: &mut XorShiftRng) -> Option<Color> {
        ray.origin = hit.pos();
        ray.dir = Vec4::random_unit(rng);

        Some(self.albedo.sample(hit.uv(), &hit.pos()))
    }
}
