use std::{error::Error, f64::consts::PI, fmt::Display};

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

    Mat4::from_columns(u, v, w, Vec4([0.0, 0.0, 0.0, 1.0]))
}

// Schlick's approximation for reflectance
pub fn reflectance(cos_theta: f64, ior_ratio: f64) -> f64 {
    let r0 = (1.0 - ior_ratio) / (1.0 + ior_ratio);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}

/// Parse a 3d vector from a string of format x,y,z, can panic
pub fn parse_vec(str: &str) -> Result<[f64; 3], ParseError> {
    let components: Vec<_> = str
        .split(",")
        .map(|x| x.parse::<f64>().expect("Vector component must be a number"))
        .collect();
    if components.len() != 3 {
        return Err(ParseError::new("Vector must have three components"));
    }

    Ok([components[0], components[1], components[2]])
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn new(message: &str) -> Self {
        ParseError {
            message: message.to_owned(),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }
}

impl Error for ParseError {}
