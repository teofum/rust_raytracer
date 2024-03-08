use std::f64::consts::PI;

use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::object::HitRecord;
use crate::pdf::CosinePDF;
use crate::ray::Ray;
use crate::texture::TexturePointer;
use crate::utils::{onb_from_vec, reflectance};
use crate::vec4::{Color, Vec4};

use super::{Material, ScatterResult};

#[derive(Debug)]
pub struct Glossy {
    albedo: TexturePointer<Color>,
    roughness: TexturePointer<f64>,
    pub normal_map: Option<TexturePointer<Vec4>>,

    inv_ior: f64,
}

impl Glossy {
    pub fn new(albedo: TexturePointer<Color>, roughness: TexturePointer<f64>, ior: f64) -> Self {
        Glossy {
            albedo,
            roughness,
            normal_map: None,
            inv_ior: 1.0 / ior,
        }
    }

    fn get_normal(&self, hit: &HitRecord) -> Vec4 {
        if let Some(normal_map) = &self.normal_map {
            // Calculate surface-space normal
            let sampled = normal_map.sample(hit.uv(), &hit.pos());
            let basis = onb_from_vec(hit.normal());
            (basis * (sampled * 2.0 - Vec4::vec(1.0, 1.0, 1.0))).to_unit()

            // (hit.normal() + offset).to_unit()
        } else {
            hit.normal()
        }
    }
}

impl Material for Glossy {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Pcg64Mcg) -> ScatterResult {
        let normal = self.get_normal(hit);

        let unit_dir = ray.dir().to_unit();
        let cos_theta = f64::min(1.0, (-unit_dir).dot(&normal));

        let specular = reflectance(cos_theta, self.inv_ior) > rng.gen_range(0.0..1.0);
        if specular {
            let roughness = self.roughness.sample(hit.uv(), &hit.pos());
            let reflected = ray.dir().reflect(normal);
            let scatter_dir = reflected + Vec4::random_unit(rng) * roughness * reflected.length();

            if scatter_dir.dot(&normal) > 0.0 {
                let scattered = Ray::new(hit.pos(), scatter_dir);
                ScatterResult::ScatteredWithRay {
                    attenuation: Vec4::vec(1.0, 1.0, 1.0),
                    scattered,
                }
            } else {
                ScatterResult::Absorbed // Absorb ray if it would be scattered inside the surface
            }
        } else {
            let pdf = CosinePDF::new(normal);
            let pdf = Box::new(pdf);

            ScatterResult::ScatteredWithPDF {
                attenuation: self.albedo.sample(hit.uv(), &hit.pos()),
                pdf,
            }
        }
    }

    fn scattering_pdf(&self, _: &Ray, scattered: &Ray, hit: &HitRecord) -> f64 {
        let normal = self.get_normal(hit);
        let cos_theta = Vec4::dot(&normal, &scattered.dir().to_unit());

        if cos_theta < 0.0 {
            0.0
        } else {
            cos_theta / PI
        }
    }
}
