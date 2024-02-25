use std::sync::Arc;

use rand_pcg::Pcg64Mcg;

use crate::object::Hit;
use crate::vec4::Vec4;

use super::PDF;

pub struct HittablePDF {
    pub object: Arc<dyn Hit>,
    pub origin: Vec4,
}

impl HittablePDF {
    pub fn new(object: Arc<dyn Hit>, origin: Vec4) -> Self {
        HittablePDF { object, origin }
    }
}

impl PDF for HittablePDF {
    fn value(&self, dir: &Vec4, rng: &mut Pcg64Mcg) -> f64 {
        self.object.pdf_value(self.origin, *dir, rng)
    }

    fn generate(&self, rng: &mut Pcg64Mcg) -> Vec4 {
        self.object.random(self.origin, rng)
    }
}
