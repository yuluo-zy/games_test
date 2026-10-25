//! A four-sample wall-time filter with warmup and upward-spike suppression.
use std::collections::VecDeque;
#[derive(Clone, Debug)]
pub struct FilteredWallTime {
    pub warmup_remaining: i32,
    pub previous_delta: f32,
    samples: VecDeque<f32>,
}
impl Default for FilteredWallTime {
    fn default() -> Self {
        Self {
            warmup_remaining: 1,
            previous_delta: 1.0 / 60.0,
            samples: VecDeque::with_capacity(4),
        }
    }
}
impl FilteredWallTime {
    /// Restores filter state. Excess history is discarded on the next sample.
    pub fn from_observed_state(
        warmup_remaining: i32,
        previous_delta: f32,
        samples: VecDeque<f32>,
    ) -> Self {
        Self {
            warmup_remaining,
            previous_delta,
            samples,
        }
    }
    pub fn samples(&self) -> &VecDeque<f32> {
        &self.samples
    }
    pub fn delta_secs(&mut self, raw_delta: f32) -> f32 {
        let raw_delta = raw_delta.max(0.0);
        if raw_delta < 0.0005 {
            return raw_delta;
        }
        let mut value = raw_delta.min(self.previous_delta * 1.2);
        if self.warmup_remaining >= 0 {
            self.warmup_remaining -= 1;
            value = value.min(1.0 / 60.0);
        } else {
            while self.samples.len() >= 4 {
                self.samples.pop_front();
            }
            self.samples.push_back(value);
            value =
                self.samples.iter().fold(-0.0_f32, |acc, x| acc + x) / (self.samples.len() as f32);
        }
        self.previous_delta = value;
        value
    }
}

#[cfg(test)]
// Fixture tuples retain the captured native input/output record shape.
#[allow(clippy::type_complexity)]
#[path = "oracle/timer_native_oracle.rs"]
mod native_oracle;
