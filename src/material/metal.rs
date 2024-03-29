use std::sync::Arc;

use rand_pcg::Pcg64Mcg;

use crate::object::HitRecord;
use crate::ray::Ray;
use crate::texture::Sampler;
use crate::vec4::{Color, Vec4};

use super::{Material, ScatterResult};

#[derive(Debug)]
pub struct Metal {
    albedo: Arc<dyn Sampler<Output = Color>>,
    roughness: Arc<dyn Sampler<Output = f64>>,
}

impl Metal {
    pub fn new(
        albedo: Arc<dyn Sampler<Output = Color>>,
        roughness: Arc<dyn Sampler<Output = f64>>,
    ) -> Self {
        Metal { albedo, roughness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Pcg64Mcg) -> ScatterResult {
        let reflected = ray.dir().reflect(hit.normal());
        let scatter_dir = reflected
            + Vec4::random_unit(rng)
                * self.roughness.sample(hit.uv(), &hit.pos())
                * reflected.length();

        if scatter_dir.dot(&hit.normal()) > 0.0 {
            let scattered = Ray::new(hit.pos(), scatter_dir);
            ScatterResult::ScatteredWithRay {
                attenuation: self.albedo.sample(hit.uv(), &hit.pos()),
                scattered,
            }
        } else {
            ScatterResult::Absorbed // Absorb ray if it would be scattered inside the surface
        }
    }

    fn scattering_pdf(&self, _: &Ray, _: &Ray, _: &HitRecord) -> f64 {
        1.0
    }
}
