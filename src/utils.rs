use std::f64::consts::PI;

pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees / 180.0 * PI
}