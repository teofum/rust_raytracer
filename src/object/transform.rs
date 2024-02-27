use rand_pcg::Pcg64Mcg;

use crate::aabb::{self, AxisAlignedBoundingBox};
use crate::interval::Interval;
use crate::mat4::Mat4;
use crate::ray::Ray;
use crate::vec4::{Point4, Vec4};

use super::{Hit, HitRecord};

pub struct Transform {
    object: Box<dyn Hit>,
    transform: Mat4,
    inv_transform: Mat4,
    bounds: AxisAlignedBoundingBox,
}

impl Transform {
    pub fn new(object: Box<dyn Hit>) -> Self {
        let transform = Mat4::identity();
        let inv_transform = Mat4::identity();

        Transform {
            bounds: object.get_bounding_box(),
            object,
            transform,
            inv_transform,
        }
    }

    pub fn translate(&mut self, x: f64, y: f64, z: f64) {
        let translation = Mat4::translation(x, y, z);
        let inv_translation = Mat4::translation(-x, -y, -z);

        self.transform = translation * self.transform;
        self.inv_transform *= inv_translation;

        self.update_bounds();
    }

    pub fn rotate_x(&mut self, theta: f64) {
        let rotation = Mat4::rotate_x(theta);
        let inv_rotation = Mat4::rotate_x(-theta);

        self.transform = rotation * self.transform;
        self.inv_transform *= inv_rotation;

        self.update_bounds();
    }

    pub fn rotate_y(&mut self, theta: f64) {
        let rotation = Mat4::rotate_y(theta);
        let inv_rotation = Mat4::rotate_y(-theta);

        self.transform = rotation * self.transform;
        self.inv_transform *= inv_rotation;

        self.update_bounds();
    }

    pub fn rotate_z(&mut self, theta: f64) {
        let rotation = Mat4::rotate_z(theta);
        let inv_rotation = Mat4::rotate_z(-theta);

        self.transform = rotation * self.transform;
        self.inv_transform *= inv_rotation;

        self.update_bounds();
    }

    pub fn scale(&mut self, x: f64, y: f64, z: f64) {
        let scale = Mat4::scale(x, y, z);
        let inv_scale = Mat4::scale(1.0 / x, 1.0 / y, 1.0 / z);

        self.transform = scale * self.transform;
        self.inv_transform *= inv_scale;

        self.update_bounds();
    }

    pub fn scale_uniform(&mut self, s: f64) {
        self.scale(s, s, s);
    }

    fn update_bounds(&mut self) {
        let (o_min, o_max) = self.object.get_bounding_box();

        let (sx, sy, sz) = (o_max - o_min).xyz();
        let mut corners = [
            o_min,
            o_min + Vec4::vec(0.0, 0.0, sz),
            o_min + Vec4::vec(0.0, sy, 0.0),
            o_min + Vec4::vec(0.0, sy, sz),
            o_min + Vec4::vec(sx, 0.0, 0.0),
            o_min + Vec4::vec(sx, 0.0, sz),
            o_min + Vec4::vec(sx, sy, 0.0),
            o_max,
        ];

        for corner in &mut corners {
            *corner = self.transform * *corner
        }

        self.bounds = aabb::get_bounding_box(&corners);
    }
}

impl Hit for Transform {
    fn test(&self, ray: &Ray, t: Interval, rng: &mut Pcg64Mcg) -> Option<HitRecord> {
        // Transform ray to object space
        let ray_obj = Ray {
            origin: self.inv_transform * ray.origin,
            dir: self.inv_transform * ray.dir,
        };

        // Test for hit in object space
        if let Some(mut hit) = self.object.test(&ray_obj, t, rng) {
            // Transform position and normal to world space
            hit.hit_pos = self.transform * hit.hit_pos;
            hit.normal = (self.transform * hit.normal).to_unit();

            Some(hit)
        } else {
            None
        }
    }

    fn get_bounding_box(&self) -> AxisAlignedBoundingBox {
        self.bounds
    }

    fn pdf_value(&self, _: Point4, _: Vec4, _: &mut Pcg64Mcg) -> f64 {
        0.0
    }

    fn random(&self, _: Point4, _: &mut Pcg64Mcg) -> Vec4 {
        Vec4::vec(1.0, 0.0, 0.0)
    }
}
