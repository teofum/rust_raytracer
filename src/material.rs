use std::fmt::Debug;

use rand_pcg::Pcg64Mcg;

use crate::object::HitRecord;
use crate::pdf::PDF;
use crate::ray::Ray;
use crate::vec4::{Color, Vec4};

pub mod dielectric;
pub mod emissive;
pub mod glossy;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

pub use dielectric::Dielectric;
pub use emissive::Emissive;
pub use glossy::Glossy;
pub use isotropic::Isotropic;
pub use lambertian::LambertianDiffuse;
pub use metal::Metal;

pub enum ScatterResult {
    ScatteredWithPDF {
        attenuation: Color,
        pdf: Box<dyn PDF>,
    },
    ScatteredWithRay {
        attenuation: Color,
        scattered: Ray,
    },
    Absorbed,
    Emissive,
}

pub trait Material: Send + Sync + Debug {
    /// Scatter a ray according to material properties.
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Pcg64Mcg) -> ScatterResult;

    fn emit(&self, _: &HitRecord) -> Color {
        Vec4::vec(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, ray_in: &Ray, scattered: &Ray, hit: &HitRecord) -> f64;
}
