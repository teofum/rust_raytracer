use std::{f64::consts::PI, sync::Arc};

use rand_pcg::Pcg64Mcg;

use crate::object::HitRecord;
use crate::pdf::CosinePDF;
use crate::ray::Ray;
use crate::texture::Sampler;
use crate::vec4::{Color, Vec4};

use super::{Material, ScatterResult};

pub struct Glossy {
    albedo: Arc<dyn Sampler<Output = Color>>,
    roughness: Arc<dyn Sampler<Output = f64>>,
}

impl Glossy {
    pub fn new(
        albedo: Arc<dyn Sampler<Output = Color>>,
        roughness: Arc<dyn Sampler<Output = f64>>,
    ) -> Self {
        Glossy { albedo, roughness }
    }
}

impl Material for Glossy {
    fn scatter(&self, _: &Ray, hit: &HitRecord, _: &mut Pcg64Mcg) -> ScatterResult {
        let pdf = CosinePDF::new(hit.normal());
        let pdf = Box::new(pdf);

        ScatterResult::ScatteredWithPDF {
            attenuation: self.albedo.sample(hit.uv(), &hit.pos()),
            pdf,
        }
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
