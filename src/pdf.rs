use crate::vec4::Vec4;

use rand_pcg::Pcg64Mcg;

mod cosine;
pub use cosine::CosinePDF;
mod hittable;
pub use hittable::HittablePDF;
mod mix;
pub use mix::MixPDF;
mod uniform;
pub use uniform::UniformPDF;

pub trait PDF: Send + Sync {
    fn value(&self, dir: &Vec4, rng: &mut Pcg64Mcg) -> f64;

    fn generate(&self, rng: &mut Pcg64Mcg) -> Vec4;
}
