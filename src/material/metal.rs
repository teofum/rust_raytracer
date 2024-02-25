use rand_pcg::Pcg64Mcg;

use crate::object::HitRecord;
use crate::ray::Ray;
use crate::vec4::{Color, Vec4};

use super::{Material, ScatterResult};

pub struct Metal {
    albedo: Color,
    roughness: f64,
}

impl Metal {
    pub fn new(albedo: Color, roughness: f64) -> Self {
        let roughness = roughness.clamp(0.0, 1.0);

        Metal { albedo, roughness }
    }

    pub fn albedo(&self) -> Color {
        self.albedo
    }

    pub fn roughness(&self) -> f64 {
        self.roughness
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Pcg64Mcg) -> ScatterResult {
        let reflected = ray.dir.reflect(hit.normal());
        let scatter_dir = reflected + Vec4::random_unit(rng) * self.roughness * reflected.length();

        if scatter_dir.dot(&hit.normal()) > 0.0 {
            let scattered = Ray::new(hit.pos(), scatter_dir);
            ScatterResult::ScatteredWithRay {
                attenuation: self.albedo,
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
