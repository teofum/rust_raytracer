use std::{f64::consts::PI, sync::Arc};

use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::object::HitRecord;
use crate::pdf::CosinePDF;
use crate::ray::Ray;
use crate::texture::Sampler;
use crate::utils::reflectance;
use crate::vec4::{Color, Vec4};

use super::{Material, ScatterResult};

#[derive(Debug)]
pub struct Glossy {
    albedo: Arc<dyn Sampler<Output = Color>>,
    roughness: Arc<dyn Sampler<Output = f64>>,

    inv_ior: f64,
}

impl Glossy {
    pub fn new(
        albedo: Arc<dyn Sampler<Output = Color>>,
        roughness: Arc<dyn Sampler<Output = f64>>,
        ior: f64,
    ) -> Self {
        Glossy {
            albedo,
            roughness,
            inv_ior: 1.0 / ior,
        }
    }
}

impl Material for Glossy {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Pcg64Mcg) -> ScatterResult {
        let unit_dir = ray.dir().to_unit();
        let cos_theta = f64::min(1.0, (-unit_dir).dot(&hit.normal()));

        let specular = reflectance(cos_theta, self.inv_ior) > rng.gen_range(0.0..1.0);
        if specular {
            let roughness = self.roughness.sample(hit.uv(), &hit.pos());
            let reflected = ray.dir().reflect(hit.normal());
            let scatter_dir = reflected + Vec4::random_unit(rng) * roughness * reflected.length();

            if scatter_dir.dot(&hit.normal()) > 0.0 {
                let scattered = Ray::new(hit.pos(), scatter_dir);
                ScatterResult::ScatteredWithRay {
                    attenuation: Vec4::vec(1.0, 1.0, 1.0),
                    scattered,
                }
            } else {
                ScatterResult::Absorbed // Absorb ray if it would be scattered inside the surface
            }
        } else {
            let pdf = CosinePDF::new(hit.normal());
            let pdf = Box::new(pdf);

            ScatterResult::ScatteredWithPDF {
                attenuation: self.albedo.sample(hit.uv(), &hit.pos()),
                pdf,
            }
        }
    }

    fn scattering_pdf(&self, _: &Ray, scattered: &Ray, hit: &HitRecord) -> f64 {
        let cos_theta = Vec4::dot(&hit.normal(), &scattered.dir().to_unit());

        if cos_theta < 0.0 {
            0.0
        } else {
            cos_theta / PI
        }
    }
}
