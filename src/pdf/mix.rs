use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::vec4::Vec4;

use super::PDF;

pub struct MixPDF {
    source: (Box<dyn PDF>, Box<dyn PDF>),
    mix: f64,
}

impl MixPDF {
    pub fn new(first: Box<dyn PDF>, second: Box<dyn PDF>, mix: f64) -> Self {
        MixPDF {
            mix,
            source: (first, second),
        }
    }
}

impl PDF for MixPDF {
    fn value(&self, dir: &Vec4, rng: &mut Pcg64Mcg) -> f64 {
        let first_val = self.source.0.value(dir, rng);
        let second_val = self.source.1.value(dir, rng);

        first_val * self.mix + second_val * (1.0 - self.mix)
    }

    fn generate(&self, rng: &mut Pcg64Mcg) -> Vec4 {
        if rng.gen_range(0.0..1.0) < self.mix {
            self.source.0.generate(rng)
        } else {
            self.source.1.generate(rng)
        }
    }
}
