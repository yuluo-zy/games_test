//! Animation easing functions. Overshooting curves intentionally exceed `[0, 1]`.
use std::f32::consts::PI;
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
pub fn cubic_in_out(t: f32) -> f32 {
    if t < 0.5 {
        ((4.0 * t) * t) * t
    } else {
        let t = t + t - 2.0;
        ((0.5 * t) * t) * t + 1.0
    }
}
impl EasingFunction {
    // Bias accepts NaN, so ordinary range containment would change its policy.
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    pub fn apply(self, t: f32) -> f32 {
        match self {
            Self::CircularInOut => {
                if t < 0.5 {
                    (1.0 - (1.0 + (-4.0 * t) * t).sqrt()) * 0.5
                } else {
                    let twice = t + t;
                    (((twice - 1.0) * (3.0 - twice)).sqrt() + 1.0) * 0.5
                }
            }
            Self::CubicInOut => cubic_in_out(t),
            Self::SineInOut => ((PI * t).cos() - 1.0) * -0.5,
            Self::Bias(bias) => {
                assert!(!(t < 0.0));
                assert!(!(t > 1.0));
                // Endpoints are fixed even when bias=1 would make the formula 0/0.
                if t == 0.0 || t == 1.0 {
                    return t;
                }
                if t < bias {
                    (t + t) / (bias + 1.0)
                } else {
                    ((-2.0 + t) * t + bias * bias) / (bias * bias - 1.0)
                }
            }
            Self::ElasticInOut => {
                if t == 0.0 || t == 1.0 {
                    return t;
                }
                let scaled = 20.0 * t;
                let wave = ((scaled - 11.125) * f32::from_bits(0x3fb2_b8c3)).sin();
                if t < 0.5 {
                    (wave * (scaled - 10.0).exp2()) * -0.5
                } else {
                    ((10.0 - scaled).exp2() * wave) * 0.5 + 1.0
                }
            }
            Self::BackOut => {
                let q = (1.0 - t).clamp(0.0, 1.0);
                let cubic = (q * q) * q;
                (PI * q).sin() * q - cubic + 1.0
            }
            Self::CubicOut => {
                let q = 1.0 - t;
                1.0 - (q * q) * q
            }
        }
    }
}

#[cfg(test)]
#[path = "oracle/easing_functions_native_oracle.rs"]
mod native_oracle;
