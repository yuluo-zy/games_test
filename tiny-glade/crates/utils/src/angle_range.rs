use crate::scalar::wrap_angle;
use std::f32::consts::TAU;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AngleRange {
    pub min: f32,
    pub len: f32,
}
impl AngleRange {
    /// Constructs a finite directed arc of length `0..=TAU`.
    ///
    /// # Panics
    /// Panics for a nonfinite start or a negative, nonfinite or oversized length.
    pub fn from_min_and_len(min: f32, len: f32) -> Self {
        assert!(min.is_finite(), "angle range start must be finite");
        assert!(
            (0.0..=TAU).contains(&len),
            "angle range length must be within 0..=TAU"
        );
        Self {
            min: wrap_angle(min),
            len,
        }
    }
    pub fn clamp(self, angle: f32) -> f32 {
        let angle = wrap_angle(angle);
        let end = self.min + self.len;
        if angle < self.min {
            let left = self.min - angle;
            let right = (TAU - self.len) - left;
            if right <= 0.0 {
                angle
            } else if left > right {
                wrap_angle(end)
            } else {
                self.min
            }
        } else if angle > end {
            let right = angle - end;
            let left = (TAU - self.len) - right;
            if left > right { wrap_angle(end) } else { self.min }
        } else {
            angle
        }
    }
}

#[cfg(test)]
#[path = "oracle/angle_range_native_oracle.rs"]
mod native_oracle;
