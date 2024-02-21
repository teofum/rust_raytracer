use crate::vec3::Color;

pub mod aces;
pub use aces::tonemap_aces;
pub mod clamp;
pub use clamp::tonemap_clamp;

/// A tone mapping function.
///
/// A valid tonemap function is expected to take an HDR value
/// in the range [0; ∞), and return an SDR value in the range \[0; 1\].
pub type TonemapFn = fn(color: Color) -> Color;
