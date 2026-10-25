//! Minimal stub of utils (easing_functions only).
pub mod easing_functions {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum EasingFunction {
        CircularInOut,
        CubicInOut,
        SineInOut,
        Bias(f32),
        ElasticInOut,
        BackOut,
        CubicOut,
    }
    impl EasingFunction {
        pub fn apply(self, t: f32) -> f32 {
            cubic_in_out(t)
        }
    }
    fn cubic_in_out(t: f32) -> f32 {
        if t < 0.5 {
            ((4.0 * t) * t) * t
        } else {
            let t = t + t - 2.0;
            ((0.5 * t) * t) * t + 1.0
        }
    }
}
