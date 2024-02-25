use std::{f64::consts::PI, sync::Arc};

use rand_pcg::Pcg64Mcg;

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
    fn scatter(&self, _: &Ray, hit: &HitRecord, rng: &mut Pcg64Mcg) -> Option<ScatterResult> {
        // let scatter_dir = onb_from_vec(hit.normal()) * Vec4::random_cosine(rng);

        let scattered = Ray::new(hit.pos(), hit.normal());
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
