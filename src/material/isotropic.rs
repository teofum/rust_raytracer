use std::{f64::consts::PI, sync::Arc};

use rand_xorshift::XorShiftRng;

use crate::object::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec4::Vec4;

use super::{Material, ScatterResult};

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Isotropic { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _: &Ray, hit: &HitRecord, rng: &mut XorShiftRng) -> Option<ScatterResult> {
        let scattered = Ray::new(hit.pos(), Vec4::random_unit(rng));
        Some(ScatterResult {
            attenuation: self.albedo.sample(hit.uv(), &hit.pos()),
            scattered,
        })
    }

    fn scattering_pdf(&self, _: &Ray, _: &Ray, _: &HitRecord) -> f64 {
        1.0 / (4.0 * PI)
    }
}
