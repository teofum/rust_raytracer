use std::{f64::consts::PI, sync::Arc};

use rand_xorshift::XorShiftRng;

use crate::object::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec4::Vec4;

use super::{Material, ScatterResult};

pub struct LambertianDiffuse {
    albedo: Arc<dyn Texture>,
}

impl LambertianDiffuse {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        LambertianDiffuse { albedo }
    }
}

impl Material for LambertianDiffuse {
    fn scatter(&self, _: &Ray, hit: &HitRecord, rng: &mut XorShiftRng) -> Option<ScatterResult> {
        let scatter_dir = hit.normal() + Vec4::random_unit(rng);

        let scatter_dir = if scatter_dir.near_zero() {
            hit.normal()
        } else {
            scatter_dir
        };

        let scattered = Ray::new(hit.pos(), scatter_dir);
        Some(ScatterResult {
            attenuation: self.albedo.sample(hit.uv(), &hit.pos()),
            scattered,
        })
    }

    fn scattering_pdf(&self, _: &Ray, scattered: &Ray, hit: &HitRecord) -> f64 {
        let cos_theta = Vec4::dot(&hit.normal(), &scattered.dir.to_unit());

        if cos_theta < 0.0 {
            0.0
        } else {
            cos_theta / PI
        }
    }
}
