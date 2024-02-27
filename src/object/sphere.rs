use std::f64::consts::PI;
use std::sync::Arc;

use rand::Rng;
use rand_distr::Standard;
use rand_pcg::Pcg64Mcg;

use crate::aabb::AxisAlignedBoundingBox;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::onb_from_vec;
use crate::vec4::{Point4, Vec4};

use super::{Hit, HitRecord};

pub struct Sphere {
    pub material: Arc<dyn Material>,

    center: Point4,
    radius: f64,
    bounds: AxisAlignedBoundingBox,
}

impl Sphere {
    pub fn new(center: Point4, radius: f64, material: Arc<dyn Material>) -> Self {
        let radius_vec = Vec4::vec(radius, radius, radius);
        let bounds = [center - radius_vec, center + radius_vec];

        Sphere {
            center,
            radius,
            bounds,
            material,
        }
    }

    #[inline(always)]
    fn test_impl(&self, ray: &Ray, t: Interval, skip_uvs: bool) -> Option<HitRecord> {
        let center_diff = ray.origin() - self.center;

        // Test for ray-sphere intersection using quadratic formula
        let a = ray.dir().length_squared();
        let half_b = ray.dir().dot(&center_diff);
        let c = center_diff.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }
        // Find the nearest root in the acceptable range
        let d_sqrt = discriminant.sqrt();

        let mut root = (-half_b - d_sqrt) / a;
        if root <= t.min() || t.max() <= root {
            root = (-half_b + d_sqrt) / a;
            if root <= t.min() || t.max() <= root {
                return None;
            }
        }

        let hit_pos = ray.at(root);
        let normal = (hit_pos - self.center) / self.radius;

        // Get UV coordinates
        let uv = if !skip_uvs {
            let theta = f64::acos(normal.y());
            let phi = f64::atan2(-normal.z(), normal.x()) + PI;
            (phi / (2.0 * PI), theta / PI)
        } else {
            (0.0, 0.0)
        };

        Some(HitRecord::new(
            ray,
            hit_pos,
            root,
            uv,
            normal,
            Arc::as_ref(&self.material),
        ))
    }
}

impl Hit for Sphere {
    fn test(&self, ray: &Ray, t: Interval, _: &mut Pcg64Mcg) -> Option<HitRecord> {
        self.test_impl(ray, t, false)
    }

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.bounds
    }

    fn pdf_value(&self, origin: Point4, dir: Vec4, _: &mut Pcg64Mcg) -> f64 {
        if self
            .test_impl(&Ray::new(origin, dir), Interval(0.001, f64::INFINITY), true)
            .is_some()
        {
            let radius_squared = self.radius * self.radius;
            let cos_theta_max =
                (1.0 - radius_squared / (self.center - origin).length_squared()).sqrt();

            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, origin: Point4, rng: &mut Pcg64Mcg) -> Vec4 {
        let dir = self.center - origin;
        let basis = onb_from_vec(dir);

        basis * random_to_sphere(self.radius, dir.length_squared(), rng)
    }
}

fn random_to_sphere(radius: f64, distance_squared: f64, rng: &mut Pcg64Mcg) -> Vec4 {
    let radius_squared = radius * radius;
    let cos_theta_max = (1.0 - radius_squared / distance_squared).sqrt();

    let r1: f64 = rng.sample(Standard);
    let r2: f64 = rng.sample(Standard);

    let phi = r1 * 2.0 * PI;
    let z = 1.0 + r2 * (cos_theta_max - 1.0);

    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vec4::vec(x, y, z)
}
