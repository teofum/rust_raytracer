use rand_xorshift::XorShiftRng;

use crate::object::HitRecord;
use crate::ray::Ray;
use crate::vec4::{Color, Vec4};

pub mod dielectric;
pub mod emissive;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

pub use dielectric::Dielectric;
pub use emissive::Emissive;
pub use isotropic::Isotropic;
pub use lambertian::LambertianDiffuse;
pub use metal::Metal;

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    /// Scatter a ray according to material properties.
    ///
    /// Returns `None` if the ray is absorbed.
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut XorShiftRng) -> Option<ScatterResult>;

    fn emit(&self, _: &HitRecord) -> Color {
        Vec4::vec(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, ray_in: &Ray, scattered: &Ray, hit: &HitRecord) -> f64;
}
