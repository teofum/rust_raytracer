use rand::Rng;
use rand_xorshift::XorShiftRng;

use crate::object::HitRecord;
use crate::ray::Ray;
use crate::vec4::Vec4;

use super::{Material, ScatterResult};

pub struct Dielectric {
    ior: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Dielectric {
            ior: index_of_refraction,
        }
    }

    pub fn index_of_refraction(&self) -> f64 {
        self.ior
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut XorShiftRng) -> Option<ScatterResult> {
        let ior_ratio = if hit.front_face() {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_dir = ray.dir.to_unit();
        let cos_theta = f64::min(1.0, (-unit_dir).dot(&hit.normal()));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let tir = ior_ratio * sin_theta > 1.0; // Total Internal Reflection
        let reflected = tir || reflectance(cos_theta, ior_ratio) > rng.gen_range(0.0..1.0);

        let scatter_dir = if reflected {
            unit_dir.reflect(hit.normal())
        } else {
            unit_dir.refract(hit.normal(), ior_ratio)
        };

        let scattered = Ray::new(hit.pos(), scatter_dir);
        Some(ScatterResult {
            attenuation: Vec4::vec(1.0, 1.0, 1.0),
            scattered,
        })
    }

    fn scattering_pdf(&self, _: &Ray, _: &Ray, _: &HitRecord) -> f64 {
        1.0
    }
}

// Schlick's approximation for reflectance
fn reflectance(cos_theta: f64, ior_ratio: f64) -> f64 {
    let r0 = (1.0 - ior_ratio) / (1.0 + ior_ratio);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}
