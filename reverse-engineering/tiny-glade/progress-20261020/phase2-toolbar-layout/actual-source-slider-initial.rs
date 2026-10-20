mod photomode { pub mod slider_math { 
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
}}}
pub fn initial_slider_value_01(kind: u8) -> f32 {
    const DEFAULTS: [f32; 13] = [0., 1., 1., 0., 0., 0.015, 0.5, 0.5, 0.5, 1., 0.8, 2., 3.];
    let value = DEFAULTS[kind as usize];
    crate::photomode::slider_math::normalized(kind, value).expect("native UiSliderType")
}

fn main(){ let expected:[u32;13]=[1056964608, 1056964608, 1062557013, 0, 0, 1056964608, 1056964608, 1059061760, 1056964608, 1054003922, 1057727209, 1048576000, 1056964608]; for (k,b) in expected.into_iter().enumerate(){ assert_eq!(initial_slider_value_01(k as u8).to_bits(),b); } println!("13/13 actual-source slider initial values match original f32 bits"); }