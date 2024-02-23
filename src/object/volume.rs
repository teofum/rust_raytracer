use std::sync::Arc;

use rand::Rng;
use rand_distr::Standard;
use rand_pcg::Pcg64Mcg;

use crate::material::Material;
use crate::ray::Ray;
use crate::{interval::Interval, vec4::Vec4};

use super::{Hit, HitRecord};

pub struct Volume {
    boundary: Box<dyn Hit>,
    material: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl Volume {
    pub fn new(boundary: Box<dyn Hit>, material: Arc<dyn Material>, density: f64) -> Self {
        let neg_inv_density = -1.0 / density;
        Volume {
            boundary,
            material,
            neg_inv_density,
        }
    }
}

impl Hit for Volume {
    fn test(&self, ray: &Ray, t: Interval, rng: &mut Pcg64Mcg) -> Option<HitRecord> {
        if let Some(hit_enter) = self.boundary.test(ray, Interval::UNIVERSE, rng) {
            let exit_t = Interval(hit_enter.t() + 0.0001, f64::INFINITY);

            if let Some(hit_exit) = self.boundary.test(ray, exit_t, rng) {
                let mut t_min = f64::max(hit_enter.t(), t.min());
                let t_max = f64::min(hit_exit.t(), t.max());

                if t_min >= t_max {
                    return None;
                }

                t_min = f64::max(t_min, 0.0);
                let ray_len = ray.dir.length();
                let dist_inside_boundary = (t_max - t_min) * ray_len;
                let hit_dist = self.neg_inv_density * f64::ln(rng.sample(Standard));

                if hit_dist > dist_inside_boundary {
                    return None;
                }

                let t = t_min + hit_dist / ray_len;
                let hit_pos = ray.at(t);

                return Some(HitRecord::new(
                    ray,
                    hit_pos,
                    t,
                    (0.0, 0.0),               // Arbitrary, unused
                    Vec4::vec(1.0, 0.0, 0.0), // Arbitrary, unused
                    self.material.as_ref(),
                ));
            }
        }

        None
    }

    fn get_bounding_box(&self) -> crate::aabb::AxisAlignedBoundingBox {
        self.boundary.get_bounding_box()
    }
}
