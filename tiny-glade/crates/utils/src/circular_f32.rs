//! A circular angle in radians, normalized into a single turn.
use serde::{Deserialize, Serialize};
use std::f32::consts::{PI, TAU};
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CircularF32(f32);
fn wrap(value: f32) -> f32 {
    let r = value % TAU;
    if r < 0.0 { r + TAU } else { r }
}
impl CircularF32 {
    pub fn new(value: f32) -> Self {
        Self(wrap(value))
    }
    pub fn set_value(&mut self, value: f32) {
        self.0 = wrap(value);
    }
    pub fn as_f32(self) -> f32 {
        wrap(self.0)
    }
    pub fn distance(&self, other: f32) -> f32 {
        let d = (wrap(self.0) - wrap(other)).abs();
        d.min((TAU - d).abs())
    }
    pub fn lerp(&self, other: f32, factor: f32) -> Self {
        let a = wrap(self.0);
        let mut d = wrap(other) - a;
        if d < -PI {
            d += TAU;
        } else if d >= PI {
            d -= TAU;
        }
        Self::new(wrap(a + d * factor))
    }
}

#[cfg(test)]
#[path = "oracle/circular_f32_native_oracle.rs"]
mod native_oracle;
