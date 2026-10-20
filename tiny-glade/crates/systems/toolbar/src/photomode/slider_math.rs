//! Original B slider parameter tables and arithmetic (142349de0..142349fd4).
//! The public-symbol body was incorrectly bounded to 28 bytes; all 13 jump
//! targets belong to this getter. Keep the original multiply/add ordering.

pub const RANGES: [(f32, f32); 13] = [
    (-2.5, 2.5),
    (0.6, 1.4),
    (0.0, 1.2),
    (0.0, 1.0),
    (0.0, 0.11),
    (0.0, 0.03),
    (0.0, 1.0),
    (-2.0, 2.0),
    (0.0, 1.0),
    (0.3, 2.0),
    (f32::from_bits(0x3ea3d70b), 1.2),
    (1.0, 5.0),
    (2.0, 4.0),
];

pub fn inverse_lerp(lower: f32, upper: f32, value: f32) -> f32 {
    (value - lower) / (upper - lower)
}

/// x86 MAXSS/MINSS source-operand NaN and signed-zero behavior.
fn maxss(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}
fn minss(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn clamp_01(value: f32) -> f32 {
    minss(1.0, maxss(0.0, value))
}

pub fn focus_parameter(distance: f32) -> f32 {
    minss(2.0, maxss(-2.0, distance.log2() + 3.0))
}

pub fn normalized(kind: u8, value: f32) -> Option<f32> {
    let &(lower, upper) = RANGES.get(kind as usize)?;
    Some(inverse_lerp(lower, upper, value))
}

pub fn parameter(kind: u8, fraction: f32) -> Option<f32> {
    let &(lower, upper) = RANGES.get(kind as usize)?;
    Some((1.0 - fraction) * lower + fraction * upper)
}

/// Lens FOV/aperture getters clamp the lens-relative result before applying
/// the common 0..1 table. Formation and brush getters deliberately do not.
pub fn normalized_lens(lower: f32, upper: f32, value: f32) -> f32 {
    let fraction = inverse_lerp(lower, upper, value);
    inverse_lerp(0.0, 1.0, minss(1.0, maxss(0.0, fraction)))
}

pub fn lens_parameter(lower: f32, upper: f32, fraction: f32) -> f32 {
    (1.0 - fraction) * lower + fraction * upper
}
