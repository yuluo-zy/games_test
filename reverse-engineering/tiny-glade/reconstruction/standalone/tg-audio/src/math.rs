//! `inverse_lerp` (0x140c904a0) + clamp, as used for FMOD parameter normalisation (A §3b/C).

/// `clamp((v - a) / (b - a), 0, 1)`.
pub fn inverse_lerp_clamped(a: f32, b: f32, v: f32) -> f32 {
    ((v - a) / (b - a)).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maps_and_clamps() {
        assert_eq!(inverse_lerp_clamped(0.5, 14.0, 0.5), 0.0);
        assert_eq!(inverse_lerp_clamped(0.5, 14.0, 14.0), 1.0);
        assert_eq!(inverse_lerp_clamped(8.0, 50.0, 29.0), 0.5);
        assert_eq!(inverse_lerp_clamped(8.0, 50.0, -1.0), 0.0);
        assert_eq!(inverse_lerp_clamped(8.0, 50.0, 99.0), 1.0);
    }
}
