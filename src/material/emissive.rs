use std::sync::Arc;

use rand_pcg::Pcg64Mcg;

use crate::object::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec4::Color;

use super::{Material, ScatterResult};

pub struct Emissive {
    emission_map: Arc<dyn Texture>,
}

impl Emissive {
    pub fn new(emission_map: Arc<dyn Texture>) -> Self {
        Emissive { emission_map }
    }
}

impl Material for Emissive {
    fn scatter(&self, _: &Ray, _: &HitRecord, _: &mut Pcg64Mcg) -> Option<ScatterResult> {
        None
    }

    fn emit(&self, hit: &HitRecord) -> Color {
        self.emission_map.sample(hit.uv(), &hit.pos())
    }

    fn scattering_pdf(&self, _: &Ray, _: &Ray, _: &HitRecord) -> f64 {
        1.0
    }
}
