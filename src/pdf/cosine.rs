use std::f64::consts::PI;

use rand_pcg::Pcg64Mcg;

use crate::mat4::Mat4;
use crate::utils::onb_from_vec;
use crate::vec4::Vec4;

use super::PDF;

pub struct CosinePDF {
    basis: Mat4,
    w: Vec4,
}

impl CosinePDF {
    pub fn new(w: Vec4) -> Self {
        CosinePDF {
            basis: onb_from_vec(w),
            w,
        }
    }
}

impl PDF for CosinePDF {
    fn value(&self, dir: &Vec4, _: &mut Pcg64Mcg) -> f64 {
        let cos_theta = Vec4::dot(&dir.to_unit(), &self.w);
        (cos_theta / PI).max(0.0)
    }

    fn generate(&self, rng: &mut Pcg64Mcg) -> Vec4 {
        self.basis * Vec4::random_cosine(rng)
    }
}
