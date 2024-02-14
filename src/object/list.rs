use crate::ray::Ray;

use super::{Hit, HitRecord};

pub struct ObjectList {
    objects: Vec<Box<dyn Hit>>,
}

impl ObjectList {
    pub fn new() -> Self {
        ObjectList {
            objects: Vec::new(),
        }
    }

    pub fn from(vec: Vec<Box<dyn Hit>>) -> Self {
        ObjectList { objects: vec }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hit>) {
        self.objects.push(object);
    }
}

impl Hit for ObjectList {
    fn test(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t = t_max;

        for object in &self.objects[..] {
            if let Some(hit) = object.test(ray, t_min, closest_t) {
                closest_t = hit.t;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}
