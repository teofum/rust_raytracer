use std::f64::consts::PI;
use std::sync::Arc;

use rand_pcg::Pcg64Mcg;

use crate::aabb::AxisAlignedBoundingBox;
use crate::interval::Interval;
use crate::material::Emissive;
use crate::ray::Ray;
use crate::texture::Sampler;
use crate::vec4::{Color, Point4, Vec4};

use super::{Hit, HitRecord};

#[derive(Debug)]
pub struct Sky {
    material: Emissive,
}

impl Sky {
    pub fn new(emission_map: Arc<dyn Sampler<Output = Color>>) -> Self {
        let material = Emissive::new(emission_map);
        Sky { material }
    }
}

impl Hit for Sky {
    fn test(&self, ray: &Ray, t: Interval, _: &mut Pcg64Mcg) -> Option<HitRecord> {
        let hit_t = f64::INFINITY;

        if hit_t > t.max() {
            return None;
        }

        let hit_pos = ray.at(hit_t);

        let unit_dir = ray.dir().to_unit();
        let normal = -unit_dir;
        let u = f64::atan2(unit_dir.x(), unit_dir.z()) / (2.0 * PI) + 0.5;
        let v = unit_dir.dot(&Vec4::vec(0.0, 1.0, 0.0)) / 2.0 + 0.5;

        Some(HitRecord::new(
            ray,
            hit_pos,
            hit_t,
            (u, v),
            normal,
            Vec4::vec(1.0, 0.0, 0.0), // Arbitrary, unused
            Vec4::vec(1.0, 0.0, 0.0), // Arbitrary, unused
            &self.material,
        ))
    }

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        let max = Vec4::point(f64::MAX, f64::MAX, f64::MAX);
        let min = Vec4::point(f64::MIN, f64::MIN, f64::MIN);

        [min, max]
    }

    fn pdf_value(&self, _: Point4, _: Vec4, _: &mut Pcg64Mcg) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn random(&self, _: Point4, rng: &mut Pcg64Mcg) -> Vec4 {
        Vec4::random_unit(rng)
    }
}
