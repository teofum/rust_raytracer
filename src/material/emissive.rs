use std::sync::Arc;

use rand_xorshift::XorShiftRng;

use crate::object::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec4::Color;

use super::Material;

pub struct Emissive {
    emission_map: Arc<dyn Texture>,
}

impl Emissive {
    pub fn new(emission_map: Arc<dyn Texture>) -> Self {
        Emissive { emission_map }
    }
}

impl Material for Emissive {
    fn scatter(&self, _: &mut Ray, _: &HitRecord, _: &mut XorShiftRng) -> Option<Color> {
        None
    }

    fn emit(&self, hit: &HitRecord) -> Color {
        self.emission_map.sample(hit.uv(), &hit.pos())
    }
}
