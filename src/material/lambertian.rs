use std::{f64::consts::PI, sync::Arc};

use rand::Rng;
use rand_distr::Standard;
use rand_xorshift::XorShiftRng;

use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec4::Vec4;
use crate::{object::HitRecord, utils::onb_from_vec};

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
        let scatter_dir = onb_from_vec(hit.normal()) * random_cosine_vec(rng);

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

fn random_cosine_vec(rng: &mut XorShiftRng) -> Vec4 {
    let r1: f64 = rng.sample(Standard);
    let r2: f64 = rng.sample(Standard);

    let phi = r1 * 2.0 * PI;
    let sqrt_r2 = r2.sqrt();
    let x = phi.cos() * sqrt_r2;
    let y = phi.sin() * sqrt_r2;
    let z = (1.0 - r2).sqrt();

    Vec4::vec(x, y, z)
}
