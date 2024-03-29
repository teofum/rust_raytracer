use crate::mat4::Mat4;
use crate::vec4::{Color, Vec4};

// sRGB => XYZ => D65_2_D60 => AP1 => RRT_SAT
const ACES_INPUT: Mat4 = Mat4([
    0.59719, 0.35458, 0.04823, 0.0, //
    0.07600, 0.90834, 0.01566, 0.0, //
    0.02840, 0.13383, 0.83777, 0.0, //
    0.0, 0.0, 0.0, 0.1
]);

// ODT_SAT => XYZ => D60_2_D65 => sRGB
const ACES_OUTPUT: Mat4 = Mat4([
    1.60475, -0.53108, -0.07367, 0.0, //
    -0.10208, 1.10813, -0.00605, 0.0, //
    -0.00327, -0.07276, 1.07602, 0.0, //
    0.0, 0.0, 0.0, 0.1
]);

fn rrt_and_odt_fit(v: Color) -> Color {
    let a = v * (v + 0.0245786) - 0.000090537;
    let b = v * (v * 0.983729 + 0.4329510) + 0.238081;
    Vec4::vec(a[0] / b[0], a[1] / b[1], a[2] / b[2])
}

/// ACES filmic tonemapping.
pub fn tonemap_aces(color: Color) -> Color {
    let mut color = ACES_INPUT * color;
    color = rrt_and_odt_fit(color);
    color = ACES_OUTPUT * color;

    color.map_components(|x| x.clamp(0.0, 1.0))
}
