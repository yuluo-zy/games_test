//! Circular curve parameters use a directed, increasing-parameter interval.
//! B inline evidence is recorded in geometry-curves/interface-batch-02.
use super::CurveU;
use crate::comparative_space::{CircularComparativeSpace, ComparativeSpace};
use lerp::Lerp;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct CurveUCircular<S = super::CurveCoordSemantic2D> {
    value: f32,
    semantic: PhantomData<S>,
}
impl<S> Copy for CurveUCircular<S> {}
impl<S> Clone for CurveUCircular<S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<S> PartialEq for CurveUCircular<S> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<S> CurveUCircular<S> {
    /// The original checks NaN before projection; infinity projects to NaN.
    pub fn new(value: f32) -> Self {
        assert!(!value.is_nan(), "NaN circular curve parameter");
        Self::wrap_around(value)
    }
    fn wrap_around(value: f32) -> Self {
        Self {
            value: CircularComparativeSpace { period: 1.0 }.project_unbounded(value),
            semantic: PhantomData,
        }
    }
    pub fn get(self) -> f32 {
        self.value
    }
    #[allow(clippy::should_implement_trait)] // Preserve the original scalar-delta method surface.
    pub fn add(self, delta: f32) -> Self {
        Self::wrap_around(self.value + delta)
    }
    pub fn subtract(self, delta: f32) -> Self {
        Self::wrap_around(self.value - delta)
    }
    /// Distance from this parameter to `end` along increasing parameters.
    pub fn distance(self, end: Self) -> f32 {
        self.forward_end(end) - self.value
    }
    fn forward_end(self, end: Self) -> f32 {
        if self.value <= end.value {
            end.value
        } else {
            end.value + 1.0
        }
    }
    /// `t` is deliberately unbounded, as in the original lerp dependency.
    pub fn wrapping_lerp(self, end: Self, t: f32) -> Self {
        Self::new(self.value.lerp(self.forward_end(end), t))
    }
    /// Test the inclusive forward interval from `self` to `end`.
    /// Equal endpoints describe one parameter, rather than a complete turn.
    pub fn wrapping_contains(self, end: Self, point: Self) -> bool {
        let within = |p: f32, end: f32| self.value <= p && p <= end;
        if end.value >= self.value {
            within(point.value, end.value)
        } else {
            let end = end.value + 1.0;
            within(point.value, end) || within(point.value + 1.0, end)
        }
    }
}
impl<S> From<CurveUCircular<S>> for CurveU<S> {
    fn from(value: CurveUCircular<S>) -> Self {
        Self::new(value.value)
    }
}

#[cfg(test)]
#[path = "../../../../../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-batch-02/circular-native-tests.rs"]
mod native_tests;
