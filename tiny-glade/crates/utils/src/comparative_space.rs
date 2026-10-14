/// Semantic side chosen by the original ordered `< 0` branch. This safe enum
/// expresses the recovered decision; it does not claim the author's enum ABI.
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
    /// Extents are radii/half-widths. Original callers use a strict comparison:
    /// touching endpoints do not intersect. Full-width conversion is caller policy.
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
/// The original zero-bounded linear strategy, observed in WallSpace's non-loop
/// specializations. Field spelling is reconstructed; no native layout is implied.
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
#[derive(Clone, Copy, Debug)]
pub struct CircularComparativeSpace {
    pub period: f32,
}
impl ComparativeSpace for CircularComparativeSpace {
    fn distance(&self, a: f32, b: f32) -> f32 {
        let d = (a - b).abs();
        d.min((self.period - d).abs())
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
