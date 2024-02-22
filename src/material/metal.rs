use rand_xorshift::XorShiftRng;

use crate::object::HitRecord;
use crate::ray::Ray;
use crate::vec4::{Color, Vec4};

use super::Material;

pub struct Metal {
    albedo: Color,
    roughness: f64,
}

impl Metal {
    pub fn new(albedo: Color, roughness: f64) -> Self {
        let roughness = roughness.clamp(0.0, 1.0);

        Metal { albedo, roughness }
    }

    pub fn albedo(&self) -> Color {
        self.albedo
    }

    pub fn roughness(&self) -> f64 {
        self.roughness
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &mut Ray, hit: &HitRecord, rng: &mut XorShiftRng) -> Option<Color> {
        let reflected = ray.dir.reflect(hit.normal());
        let scatter_dir = reflected + Vec4::random_unit(rng) * self.roughness * reflected.length();

        ray.origin = hit.pos();
        ray.dir = scatter_dir;

        if scatter_dir.dot(&hit.normal()) > 0.0 {
            Some(self.albedo)
        } else {
            None // Absorb ray if it would be scattered inside the surface
        }
    }
}
