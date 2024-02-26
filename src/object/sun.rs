use std::sync::Arc;

use rand_pcg::Pcg64Mcg;

use crate::aabb::AxisAlignedBoundingBox;
use crate::interval::Interval;
use crate::material::Emissive;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec4::{Point4, Vec4};

use super::{Hit, HitRecord};

const THETA_MAX: f64 = 0.001;

pub struct Sun {
    direction: Vec4,
    material: Emissive,
}

impl Sun {
    pub fn new(emission_map: Arc<dyn Texture>, direction: Vec4) -> Self {
        let material = Emissive::new(emission_map);
        Sun {
            material,
            direction: direction.to_unit(),
        }
    }
}

impl Hit for Sun {
    fn test(&self, ray: &Ray, t: Interval, _: &mut Pcg64Mcg) -> Option<HitRecord> {
        let unit_dir = ray.dir.to_unit();
        if (self.direction.dot(&unit_dir) - 1.0).abs() > THETA_MAX {
            return None;
        }

        let hit_t = f64::MAX;

        if hit_t >= t.max() {
            return None;
        }

        let hit_pos = ray.at(hit_t);

        let normal = -unit_dir;
        let u = 0.0;
        let v = 0.0;

        Some(HitRecord::new(
            ray,
            hit_pos,
            hit_t,
            (u, v),
            normal,
            &self.material,
        ))
    }

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        let max = Vec4::point(f64::MAX, f64::MAX, f64::MAX);
        let min = Vec4::point(f64::MIN, f64::MIN, f64::MIN);

        (min, max)
    }

    fn pdf_value(&self, _: Point4, _: Vec4, _: &mut Pcg64Mcg) -> f64 {
        1.0
    }

    fn random(&self, _: Point4, _: &mut Pcg64Mcg) -> Vec4 {
        self.direction
    }
}
