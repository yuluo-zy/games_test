//! Scalar helpers with std float semantics.

use std::f32::consts::TAU;

pub fn min_f32(a: f32, b: f32) -> f32 {
    a.min(b)
}

pub fn max_f32(a: f32, b: f32) -> f32 {
    a.max(b)
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.clamp(min, max)
}

pub fn clamp01(value: f32) -> f32 {
    clamp(value, 0.0, 1.0)
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn inverse_lerp(a: f32, b: f32, t: f32) -> f32 {
    (t - a) / (b - a)
}

pub fn smoothstep(t: f32) -> f32 {
    let t2 = t * t;
    (3.0 - 2.0 * t) * t2
}

pub fn smoothstep_clamped(t: f32) -> f32 {
    smoothstep(clamp01(t))
}

pub(crate) fn wrap_angle(value: f32) -> f32 {
    let r = value % TAU;
    if r < 0.0 { r + TAU } else { r }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_preserves_nan_and_signed_zero() {
        assert!(clamp01(f32::NAN).is_nan());
        assert_eq!(clamp01(-0.0).to_bits(), (-0.0f32).to_bits());
        assert_eq!(clamp01(-1.0), 0.0);
        assert_eq!(clamp01(2.0), 1.0);
    }

    #[test]
    fn min_max_follow_std_nan_policy() {
        assert_eq!(min_f32(f32::NAN, 1.0), 1.0);
        assert_eq!(min_f32(1.0, f32::NAN), 1.0);
        assert_eq!(max_f32(f32::NAN, 1.0), 1.0);
        assert_eq!(max_f32(1.0, f32::NAN), 1.0);
    }

    #[test]
    fn smoothstep_contracts_differ_outside_unit_range() {
        assert_eq!(smoothstep(2.0), -4.0);
        assert_eq!(smoothstep_clamped(2.0), 1.0);
        assert_eq!(smoothstep(-1.0), 5.0);
        assert_eq!(smoothstep_clamped(-1.0), 0.0);
    }

    #[test]
    fn lerp_and_inverse_lerp_round_trip() {
        let value = lerp(2.0, 6.0, 0.25);
        assert_eq!(value, 3.0);
        assert_eq!(inverse_lerp(2.0, 6.0, value), 0.25);
    }

    #[test]
    fn wrap_angle_maps_negative_values_into_one_turn() {
        assert!((wrap_angle(-0.1) - (TAU - 0.1)).abs() < 1e-6);
        assert_eq!(wrap_angle(0.0), 0.0);
    }
}
