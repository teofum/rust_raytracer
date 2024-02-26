use std::{f64::consts::PI, sync::Arc};

use rand_pcg::Pcg64Mcg;

use crate::object::HitRecord;
use crate::pdf::UniformPDF;
use crate::ray::Ray;
use crate::texture::Sampler;
use crate::vec4::Color;

use super::{Material, ScatterResult};

pub struct Isotropic {
    albedo: Arc<dyn Sampler<Output = Color>>,
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Sampler<Output = Color>>) -> Self {
        Isotropic { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, _: &Ray, hit: &HitRecord, _: &mut Pcg64Mcg) -> ScatterResult {
        let pdf = UniformPDF::new();
        let pdf = Box::new(pdf);

        ScatterResult::ScatteredWithPDF {
            attenuation: self.albedo.sample(hit.uv(), &hit.pos()),
            pdf,
        }
    }

    fn scattering_pdf(&self, _: &Ray, _: &Ray, _: &HitRecord) -> f64 {
        1.0 / (4.0 * PI)
    }
}
