use crate::object::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;

pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub use lambertian::LambertianDiffuse;
pub use metal::Metal;

pub trait Material {
    /// Scatter a ray according to material properties.
    ///
    /// Mutates the original ray, and returns an attenuation value or `None`
    /// if the ray is absorbed.
    fn scatter(&self, ray: &mut Ray, hit: &HitRecord) -> Option<Color>;
}
