use std::sync::Arc;

use crate::material::Material;
use crate::vec4::{Point4, Vec4};

use super::{ObjectList, Plane};

pub fn make_box(center: Point4, size: Vec4, material: Arc<dyn Material>) -> ObjectList {
    let mut sides = ObjectList::new();

    let half_size = size / 2.0;
    let dx = Vec4::vec(half_size.x(), 0.0, 0.0);
    let dy = Vec4::vec(0.0, half_size.y(), 0.0);
    let dz = Vec4::vec(0.0, 0.0, half_size.z());

    sides.add(Box::new(Plane::new(
        center + dy,
        (dx, -dz),
        Arc::clone(&material),
    )));
    sides.add(Box::new(Plane::new(
        center - dy,
        (-dx, -dz),
        Arc::clone(&material),
    )));
    sides.add(Box::new(Plane::new(
        center - dx,
        (dy, -dz),
        Arc::clone(&material),
    )));
    sides.add(Box::new(Plane::new(
        center + dx,
        (dy, dz),
        Arc::clone(&material),
    )));
    sides.add(Box::new(Plane::new(
        center - dz,
        (dy, dx),
        Arc::clone(&material),
    )));
    sides.add(Box::new(Plane::new(
        center + dz,
        (dy, -dx),
        Arc::clone(&material),
    )));

    sides
}
