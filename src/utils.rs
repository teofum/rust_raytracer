use std::f64::consts::PI;

use crate::{mat4::Mat4, vec4::Vec4};

pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees / 180.0 * PI
}

/// Build an othonormal basis (ONB) from a vector.
///
/// # Params
/// `w`: vector to use as local z-axis. Assumed to be a unit vector.
///
/// # Return
/// Returns a 4x4 matrix representing the transform from the canonical basis
/// to the local ONB.
pub fn onb_from_vec(w: Vec4) -> Mat4 {
    let a = if w.x().abs() > 0.9 {
        Vec4::vec(0.0, 1.0, 0.0)
    } else {
        Vec4::vec(1.0, 0.0, 0.0)
    };

    let v = w.cross(&a).to_unit();
    let u = w.cross(&v);

    Mat4::from_columns(u, v, w, Vec4(0.0, 0.0, 0.0, 1.0))
}
