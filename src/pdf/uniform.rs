use std::f64::consts::PI;

use rand_pcg::Pcg64Mcg;

use crate::vec4::Vec4;

use super::PDF;

pub struct UniformPDF;

impl UniformPDF {
    pub fn new() -> Self {
        UniformPDF
    }
}

impl PDF for UniformPDF {
    fn value(&self, _: &Vec4, _: &mut Pcg64Mcg) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self, rng: &mut Pcg64Mcg) -> Vec4 {
        Vec4::random_unit(rng)
    }
}
