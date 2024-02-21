use crate::vec3::Color;

/// The simplest tone-mapping function. Clamps RGB values to the \[0; 1\] range.
pub fn tonemap_clamp(color: Color) -> Color {
    color.map_components(|x| x.clamp(0.0, 1.0))
}
