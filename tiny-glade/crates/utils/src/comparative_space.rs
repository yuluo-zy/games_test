/// The sign of a displacement. Unordered (NaN) displacements are nonnegative.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComparativeSide {
    Negative,
    NonNegative,
}
pub trait ComparativeSpace {
    fn distance(&self, a: f32, b: f32) -> f32;
    fn project(&self, value: f32) -> Option<f32>;
    fn project_clamp(&self, value: f32) -> f32;
    fn project_unbounded(&self, value: f32) -> f32;
    fn subtract(&self, a: f32, b: f32) -> f32;
    fn lerp_clamp(&self, a: f32, b: f32, t: f32) -> f32 {
        self.project_clamp(a + self.subtract(b, a) * t)
    }
    /// Extents are radii or half-widths. Touching endpoints do not intersect.
    fn regions_intersect(
        &self,
        a_center: f32,
        a_half_extent: f32,
        b_center: f32,
        b_half_extent: f32,
    ) -> bool {
        self.distance(a_center, b_center) < a_half_extent + b_half_extent
    }
    fn classify_side(&self, a: f32, b: f32) -> ComparativeSide {
        if self.subtract(a, b) < 0.0 {
            ComparativeSide::Negative
        } else {
            ComparativeSide::NonNegative
        }
    }
}
/// A linear coordinate space bounded by zero and `upper_bound`.
#[derive(Clone, Copy, Debug)]
pub struct ZeroBoundedComparativeSpace {
    pub upper_bound: f32,
}
impl ComparativeSpace for ZeroBoundedComparativeSpace {
    fn distance(&self, a: f32, b: f32) -> f32 {
        (a - b).abs()
    }
    fn project(&self, value: f32) -> Option<f32> {
        if value >= 0.0 && value <= self.upper_bound {
            Some(value)
        } else {
            None
        }
    }
    fn project_clamp(&self, value: f32) -> f32 {
        value.clamp(0.0, self.upper_bound)
    }
    fn project_unbounded(&self, value: f32) -> f32 {
        value
    }
    fn subtract(&self, a: f32, b: f32) -> f32 {
        a - b
    }
}
/// A circular coordinate space with a finite, strictly positive period.
#[derive(Clone, Copy, Debug)]
pub struct CircularComparativeSpace {
    pub period: f32,
}
impl ComparativeSpace for CircularComparativeSpace {
    fn lerp_clamp(&self, a: f32, b: f32, t: f32) -> f32 {
        // Reduce the travel before adding the starting phase. Both the
        // unbounded coordinate and a large finite extrapolation can otherwise
        // absorb a short displacement, and f32 products may underflow.
        let period = f64::from(self.period);
        let start = f64::from(a) % period;
        let displacement = f64::from(self.subtract(b, a));
        let travel = (displacement * f64::from(t)) % period;
        let remainder = (start + travel) % period;
        let phase = if remainder < 0.0 {
            remainder + period
        } else {
            remainder
        };
        let projected = phase as f32;
        // A phase just below the period may round up to it in f32.
        if projected == self.period {
            0.0
        } else {
            projected
        }
    }
    fn distance(&self, a: f32, b: f32) -> f32 {
        self.subtract(a, b).abs()
    }
    fn project(&self, value: f32) -> Option<f32> {
        Some(self.project_clamp(value))
    }
    fn project_clamp(&self, value: f32) -> f32 {
        let r = value % self.period;
        if r < 0.0 { r + self.period } else { r }
    }
    fn project_unbounded(&self, value: f32) -> f32 {
        self.project_clamp(value)
    }
    fn subtract(&self, a: f32, b: f32) -> f32 {
        let d = a - b;
        // Projected coordinates are the hot path. Wider arithmetic is needed
        // for multi-turn inputs: subtracting first can lose the smaller phase
        // even when the f32 difference does not overflow.
        if a.abs() > self.period
            || b.abs() > self.period
            || d.abs() > self.period
            || d.abs() == self.period * 0.5
            || !d.is_finite()
            || self.period < f32::MIN_POSITIVE
        {
            let period = f64::from(self.period);
            let mut displacement = (f64::from(a) % period - f64::from(b) % period) % period;
            let half_period = period * 0.5;
            if displacement < -half_period {
                displacement += period;
            } else if displacement >= half_period {
                displacement -= period;
            }
            return displacement as f32;
        }

        if d < -0.5 * self.period {
            d + self.period
        } else if d >= 0.5 * self.period {
            d - self.period
        } else {
            d
        }
    }
}

#[cfg(test)]
mod completion_tests {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/oracle/foundation-completion-2026-10-13/base-assets/comparative_tests.rs"
    ));
}
