use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::vec4::Vec4;

use super::PDF;

pub struct MixPDF<'a> {
    source: (&'a dyn PDF, &'a dyn PDF),
    mix: f64,
}

impl<'a> MixPDF<'a> {
    pub fn new(first: &'a dyn PDF, second: &'a dyn PDF, mix: f64) -> Self {
        MixPDF {
            mix,
            source: (first, second),
        }
    }
}

impl<'a> PDF for MixPDF<'a> {
    fn value(&self, dir: &Vec4, rng: &mut Pcg64Mcg) -> f64 {
        let first_val = self.source.0.value(dir, rng);
        let second_val = self.source.1.value(dir, rng);

        first_val * (1.0 - self.mix) + second_val * self.mix
    }

    fn generate(&self, rng: &mut Pcg64Mcg) -> Vec4 {
        if rng.gen_range(0.0..1.0) < self.mix {
            self.source.1.generate(rng)
        } else {
            self.source.0.generate(rng)
        }
    }
}
