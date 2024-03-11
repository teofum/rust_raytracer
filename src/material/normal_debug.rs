use std::sync::Arc;

use rand_pcg::Pcg64Mcg;

use crate::mat4::Mat4;
use crate::object::HitRecord;
use crate::ray::Ray;
use crate::texture::Sampler;
use crate::vec4::{Color, Vec4};

use super::{Material, ScatterResult};

#[derive(Debug)]
pub struct NormalDebug {
    pub normal_map: Option<Arc<dyn Sampler<Output = Color>>>,
}

impl NormalDebug {
    pub fn new() -> Self {
        NormalDebug { normal_map: None }
    }

    fn get_normal(&self, hit: &HitRecord) -> Vec4 {
        if let Some(normal_map) = &self.normal_map {
            // Calculate surface-space normal
            let sampled = normal_map.sample(hit.uv(), &hit.pos());
            let basis = Mat4::from_columns(
                hit.tangent(),
                hit.bitangent(),
                hit.normal(),
                Vec4([0.0, 0.0, 0.0, 1.0]),
            );

            (basis * (sampled - Vec4::vec(0.5, 0.5, 0.5))).to_unit()
        } else {
            hit.normal()
        }
    }
}

impl Material for NormalDebug {
    fn scatter(&self, _: &Ray, _: &HitRecord, _: &mut Pcg64Mcg) -> ScatterResult {
        ScatterResult::Emissive
    }

    fn emit(&self, hit: &HitRecord) -> Color {
        self.get_normal(hit) * 0.5 + Vec4::vec(0.5, 0.5, 0.5)
    }

    fn scattering_pdf(&self, _: &Ray, _: &Ray, _: &HitRecord) -> f64 {
        1.0
    }
}
