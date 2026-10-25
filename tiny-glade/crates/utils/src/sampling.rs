//! Pure game-owned sampling used by original glade startup.
use glam::Vec2;

/// Original140c909a0. The next digit index is formed by a float product and
/// saturating cast, not integer division; large indices depend on that order.
/// Zero index returns before division, including base0. Nonzero/base0 panics.
pub fn radical_inverse(mut index: u32, base: u32) -> f32 {
    if index == 0 {
        return 0.0;
    }
    let inverse = 1.0 / base as f32;
    let mut weight = inverse;
    let mut result = 0.0;
    while index != 0 {
        let digit = index % base;
        index = (index as f32 * inverse) as u32;
        result += digit as f32 * weight;
        weight *= inverse;
    }
    result
}

/// Original140c90700: radial uniform coordinate first, angular coordinate
/// second. Separate cos/sin calls and multiplications preserve the operation
/// order. Invalid inputs retain the original floating-point behavior.
pub fn uniform_sample_disk(sample: Vec2) -> Vec2 {
    let radius = sample.x.sqrt();
    let angle = sample.y * std::f32::consts::TAU;
    Vec2::new(angle.cos() * radius, radius * angle.sin())
}
