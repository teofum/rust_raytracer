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

pub trait Material: Send + Sync {
    /// Scatter a ray according to material properties.
    ///
    /// Mutates the original ray, and returns an attenuation value or `None`
    /// if the ray is absorbed.
    fn scatter(&self, ray: &mut Ray, hit: &HitRecord, rng: &mut XorShiftRng) -> Option<Color>;

    fn emit(&self, _: &HitRecord) -> Color {
        Vec4::vec(0.0, 0.0, 0.0)
    }
}
