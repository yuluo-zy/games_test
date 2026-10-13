//! B 0x140154b60/0x14126a8f0 clip a polyline retaining interior vertices.
//! Stateful CurveSlice remap/extend iterator variants remain separately tracked.
use super::{Curve, CurveError, CurveInternalCoord, CurveMetric, CurvePoint, CurveU};
use itertools::Itertools;
use std::{marker::PhantomData, ops::{Bound, RangeBounds}};
use take_until::TakeUntilExt;

/// Original point access policy, with ordinary and logical-loop implementations.
pub trait CurvePoints<T> {
    fn len(&self) -> usize;
    fn point(&self, index: usize) -> &T;
    fn is_empty(&self) -> bool { self.len() == 0 }
    fn iter<'a>(&'a self) -> impl DoubleEndedIterator<Item = &'a T> where T: 'a {
        (0..self.len()).map(|i| self.point(i))
    }
}
impl<T> CurvePoints<T> for &[T] {
    fn len(&self) -> usize { <[T]>::len(self) }
    fn point(&self, index: usize) -> &T { &self[index] }
    fn iter<'a>(&'a self) -> impl DoubleEndedIterator<Item = &'a T> where T: 'a { <[T]>::iter(self) }
}
/// Reuse the game's restored LoopRange; logical enumeration is local to the view.
pub struct LoopCurvePoints<'a, T> {
    pub points: &'a [T],
    pub range: crate::loop_range::LoopRange,
}
impl<T> CurvePoints<T> for LoopCurvePoints<'_, T> {
    fn len(&self) -> usize { self.range.len() }
    fn point(&self, index: usize) -> &T {
        assert!(index < self.len());
        let logical = crate::loop_range::LoopIdx {
            idx: self.range.start_idx.wrapping_add(index as i32),
            period: self.range.period,
        };
        &self.points[logical.index()]
    }
    fn iter<'a>(&'a self) -> impl DoubleEndedIterator<Item = &'a T> where T: 'a {
        self.range.map(|index| &self.points[index.index()])
    }
}
fn identity<T: Copy>(_: usize, point: &T) -> T { *point }
/// A view of an original point set. Remapping preserves the endpoint coordinates.
pub struct CurveSlice<T, Points, S = super::CurveCoordSemantic2D, R = T, F = fn(usize, &T) -> T> {
    points: Points,
    first: CurveInternalCoord,
    last: CurveInternalCoord,
    remap: F,
    marker: PhantomData<(T, S, R)>,
}
impl<T: CurvePoint, Points: CurvePoints<T>, S> CurveSlice<T, Points, S> {
    pub fn from_point_range(points: Points, range: impl RangeBounds<usize>) -> Self {
        let count = points.len();
        let first = match range.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i.checked_add(1).expect("point index overflow"),
            Bound::Unbounded => 0,
        };
        assert!(first < count, "start point outside point set");
        let last = match range.end_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i.checked_sub(1).expect("empty point range"),
            Bound::Unbounded => count.checked_sub(1).expect("empty point set"),
        };
        Self::from_coord_range(points,
            CurveInternalCoord::new_for_point_count(first, 0.0, count),
            CurveInternalCoord::new_for_point_count(last, 0.0, count))
    }
    pub fn from_coord_range(points: Points, first: CurveInternalCoord, last: CurveInternalCoord) -> Self {
        let span = (last.t - first.t) + (last.segment as isize - first.segment as isize) as f32;
        assert!(span >= 0.0, "reversed or unordered curve slice");
        Self { points, first, last, remap: identity::<T>, marker: PhantomData }
    }
}
impl<'a, T: CurvePoint, S: CurveMetric<T>> CurveSlice<T, &'a [T], S> {
    /// Point-index bounds over the curve's original vertices.
    pub fn from_curve_range(curve: &'a Curve<T, S>, range: impl RangeBounds<usize>) -> Self {
        Self::from_point_range(curve.points.as_slice(), range)
    }
}
impl<T: CurvePoint, Points: CurvePoints<T>, S, R: CurvePoint, F: Fn(usize, &T) -> R> CurveSlice<T, Points, S, R, F> {
    pub fn start(&self) -> CurveInternalCoord { self.first }
    pub fn end(&self) -> CurveInternalCoord { self.last }
    pub fn remap<U: CurvePoint, G: Fn(usize, &T) -> U>(self, remap: G) -> CurveSlice<T, Points, S, U, G> {
        CurveSlice { points: self.points, first: self.first, last: self.last, remap, marker: PhantomData }
    }
    pub fn remap_2d(self) -> CurveSlice<T, Points, S, glam::Vec2, impl Fn(usize, &T) -> glam::Vec2> {
        self.remap(|_, p| p.projected())
    }
    /// Chain start, strict interior vertices and end. A zero span keeps both ends.
    pub fn iter_coords(&self) -> impl Iterator<Item = CurveInternalCoord> {
        let first_inner = self.first.segment + 1 + usize::from(self.first.t == 1.0);
        let end_inner = (self.last.segment + 1 - usize::from(self.last.t == 0.0)).max(first_inner);
        std::iter::once(self.first)
            .chain((first_inner..end_inner).map(|segment| CurveInternalCoord { segment, t: 0.0 }))
            .chain(std::iter::once(self.last))
    }
    fn mapped_point(&self, index: usize) -> R { (self.remap)(index, self.points.point(index)) }
    pub fn sample_at_coord(&self, coord: CurveInternalCoord) -> R {
        self.mapped_point(coord.segment + 1) * coord.t + self.mapped_point(coord.segment) * (1.0 - coord.t)
    }
    pub fn measure_length(&self) -> f32 {
        self.iter_coords().map(|coord| self.sample_at_coord(coord))
            .tuple_windows::<(_, _)>().fold(0.0, |length, (a, b)| length + (b - a).metric_squared().sqrt())
    }
    pub fn extend_right(&mut self, distance: f32) { self.extend_right_until(distance, |_| false) }
    pub fn extend_left(&mut self, distance: f32) { self.extend_left_until(distance, |_| false) }
    /// The original take-until dependency includes the stopping vertex.
    pub fn extend_right_until(&mut self, distance: f32, stop: impl FnMut(&usize) -> bool) {
        assert!(distance >= 0.0, "invalid extension distance");
        let mut remaining = distance;
        let windows = (self.last.segment..self.points.len()).take_until(stop)
            .map(|i| (i, self.mapped_point(i))).tuple_windows::<(_, _)>();
        let mut updated = None;
        for ((i, a), (_, b)) in windows {
            let length = (b - a).metric_squared().sqrt();
            let available = if updated.is_none() { (1.0 - self.last.t) * length } else { length };
            remaining = available - remaining;
            if remaining >= 0.0 {
                updated = Some(CurveInternalCoord { segment: i, t: 1.0 - remaining / length });
                break;
            }
            updated = Some(CurveInternalCoord { segment: i, t: 1.0 });
            remaining = -remaining;
        }
        if let Some(end) = updated { self.last = end; }
    }
    pub fn extend_left_until(&mut self, distance: f32, stop: impl FnMut(&usize) -> bool) {
        assert!(distance >= 0.0, "invalid extension distance");
        let mut remaining = distance;
        let windows = (0..=self.first.segment + 1).rev().take_until(stop)
            .map(|i| (i, self.mapped_point(i))).tuple_windows::<(_, _)>();
        let mut updated = None;
        for ((_, b), (i, a)) in windows {
            let length = (b - a).metric_squared().sqrt();
            // Original subtraction then addition round separately near t=0.
            let available = if updated.is_none() { ((self.first.t - 1.0) + 1.0) * length } else { length };
            remaining = available - remaining;
            if remaining >= 0.0 {
                updated = Some(CurveInternalCoord { segment: i, t: remaining / length });
                break;
            }
            updated = Some(CurveInternalCoord { segment: i, t: 0.0 });
            remaining = -remaining;
        }
        if let Some(start) = updated { self.first = start; }
    }
}
impl<T: CurvePoint, S: CurveMetric<T>> Curve<T, S> {
    pub fn slice_into(&self, start: f32, end: f32, output: &mut Vec<T>) {
        assert!(end > start);
        let left = self.get_coord_at_u(start);
        let right = self.get_coord_at_u(end);
        let a = self.get_pos_at_coord(left);
        let b = self.get_pos_at_coord(right);
        let first = left.segment
            + if a == self.points[left.segment + 1] {
                2
            } else {
                1
            };
        let last = if b == self.points[right.segment] {
            right.segment.checked_sub(1)
        } else {
            Some(right.segment)
        };
        output.push(a);
        if let Some(last) = last {
            if first <= last {
                output.extend_from_slice(&self.points[first..=last]);
            }
        }
        output.push(b);
    }
    pub fn try_slice(&self, start: f32, end: f32) -> Result<Self, CurveError> {
        let mut out = Vec::new();
        self.slice_into(start, end, &mut out);
        Self::try_new_from_points(out, false)
    }
    pub fn crashy_slice(&self, start: f32, end: f32) -> Self {
        self.try_slice(start, end).unwrap()
    }
    /// B circular_slice joins end to start, removing one junction point.
    pub fn circular_slice(&self, start: f32, end: f32) -> Result<Self, CurveError> {
        let _ = CurveU::<S>::new(start);
        let _ = CurveU::<S>::new(end);
        if start < end {
            return self.try_slice(start, end);
        }
        let mut out = Vec::new();
        if start != 1.0 {
            self.slice_into(start, 1.0, &mut out)
        }
        if end != 0.0 {
            if let Ok(right) = self.try_slice(0.0, end) {
                if !out.is_empty() {
                    out.pop();
                }
                if out.last() == right.points.first() {
                    out.pop();
                }
                out.extend(right.points);
            }
        }
        Self::try_new_from_points(out, false)
    }
}

#[cfg(test)]
mod stateful_native_tests {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../evidence/shared-foundation-2026-10-13/geometry-curves/interface-batch-02/slice-native-tests.rs"));
}
