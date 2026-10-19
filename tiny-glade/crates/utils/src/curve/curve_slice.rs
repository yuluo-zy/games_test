//! Borrowed, optionally remapped polyline intervals and owned clipping.
use super::{Curve, CurveError, CurveInternalCoord, CurveMetric, CurvePoint, CurveU};
use itertools::Itertools;
use std::{
    marker::PhantomData,
    ops::{Bound, RangeBounds},
};
use take_until::TakeUntilExt;

/// Point access shared by ordinary and logical-loop views.
pub trait CurvePoints<T> {
    fn len(&self) -> usize;
    fn point(&self, index: usize) -> &T;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn iter<'a>(&'a self) -> impl DoubleEndedIterator<Item = &'a T>
    where
        T: 'a,
    {
        (0..self.len()).map(|i| self.point(i))
    }
}
impl<T> CurvePoints<T> for &[T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
    fn point(&self, index: usize) -> &T {
        &self[index]
    }
    fn iter<'a>(&'a self) -> impl DoubleEndedIterator<Item = &'a T>
    where
        T: 'a,
    {
        <[T]>::iter(self)
    }
}
/// A logical loop range over a borrowed point set.
pub struct LoopCurvePoints<'a, T> {
    pub points: &'a [T],
    pub range: crate::loop_range::LoopRange,
}
impl<T> CurvePoints<T> for LoopCurvePoints<'_, T> {
    fn len(&self) -> usize {
        self.range.len()
    }
    fn point(&self, index: usize) -> &T {
        assert!(index < self.len());
        let logical = crate::loop_range::LoopIdx {
            idx: self.range.start_idx.wrapping_add(index as i32),
            period: self.range.period,
        };
        &self.points[logical.index()]
    }
    fn iter<'a>(&'a self) -> impl DoubleEndedIterator<Item = &'a T>
    where
        T: 'a,
    {
        self.range.map(|index| &self.points[index.index()])
    }
}
fn identity<T: Copy>(_: usize, point: &T) -> T {
    *point
}
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
        Self::from_coord_range(
            points,
            CurveInternalCoord::new_for_point_count(first, 0.0, count),
            CurveInternalCoord::new_for_point_count(last, 0.0, count),
        )
    }
    pub fn from_coord_range(
        points: Points,
        first: CurveInternalCoord,
        last: CurveInternalCoord,
    ) -> Self {
        Self::from_coord_range_with_remap(points, first, last, identity::<T>)
    }
}
impl<'a, T: CurvePoint, S: CurveMetric<T>> CurveSlice<T, &'a [T], S> {
    /// Point-index bounds over the curve's original vertices.
    pub fn from_curve_range(curve: &'a Curve<T, S>, range: impl RangeBounds<usize>) -> Self {
        Self::from_point_range(curve.points.as_slice(), range)
    }
}
impl<T, Points: CurvePoints<T>, S, R: CurvePoint, F: Fn(usize, &T) -> R>
    CurveSlice<T, Points, S, R, F>
{
    /// Build a checked view of the actual borrowed input points, mapping only
    /// when an existing sampler needs a point. Input T need not be CurvePoint;
    /// all curve arithmetic remains on the mapped R.
    pub fn from_coord_range_with_remap(
        points: Points,
        first: CurveInternalCoord,
        last: CurveInternalCoord,
        remap: F,
    ) -> Self {
        for coord in [first, last] {
            assert!(
                coord.segment < points.len().saturating_sub(1),
                "segment outside point set"
            );
            assert!((0.0..=1.0).contains(&coord.t), "invalid segment parameter");
        }
        assert!(
            first.segment < last.segment || (first.segment == last.segment && first.t <= last.t),
            "reversed or unordered curve slice"
        );
        Self {
            points,
            first,
            last,
            remap,
            marker: PhantomData,
        }
    }

    pub fn start(&self) -> CurveInternalCoord {
        self.first
    }
    pub fn end(&self) -> CurveInternalCoord {
        self.last
    }
    pub fn remap<U: CurvePoint, G: Fn(usize, &T) -> U>(
        self,
        remap: G,
    ) -> CurveSlice<T, Points, S, U, G> {
        CurveSlice {
            points: self.points,
            first: self.first,
            last: self.last,
            remap,
            marker: PhantomData,
        }
    }
    pub fn remap_2d(
        self,
    ) -> CurveSlice<T, Points, S, glam::Vec2, impl Fn(usize, &T) -> glam::Vec2>
    where
        T: CurvePoint,
    {
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
    fn mapped_point(&self, index: usize) -> R {
        (self.remap)(index, self.points.point(index))
    }
    pub fn sample_at_coord(&self, coord: CurveInternalCoord) -> R {
        self.mapped_point(coord.segment + 1) * coord.t
            + self.mapped_point(coord.segment) * (1.0 - coord.t)
    }
    pub fn measure_length(&self) -> f32 {
        self.iter_coords()
            .map(|coord| self.sample_at_coord(coord))
            .tuple_windows::<(_, _)>()
            .fold(0.0, |length, (a, b)| {
                length + b.metric_distance_f64(a) as f32
            })
    }
    pub fn extend_right(&mut self, distance: f32) {
        self.extend_right_until(distance, |_| false)
    }
    pub fn extend_left(&mut self, distance: f32) {
        self.extend_left_until(distance, |_| false)
    }
    /// Include the vertex selected by the stopping predicate.
    pub fn extend_right_until(&mut self, distance: f32, stop: impl FnMut(&usize) -> bool) {
        assert!(distance >= 0.0, "invalid extension distance");
        let old_t = self.last.t;
        // Borrow only the sampled fields: completed endpoints must remain
        // committed if a later remapping or stopping predicate panics.
        let points = &self.points;
        let remap = &self.remap;
        let mut windows = (self.last.segment..points.len())
            .take_until(stop)
            .map(|i| (i, remap(i, points.point(i))))
            .tuple_windows::<(_, _)>();
        let Some(((segment, a), (_, b))) = windows.next() else {
            return;
        };
        // B 1412898e0: keep the original f32 residual and cancellation order,
        // including zero-length division and non-finite endpoint outcomes.
        let length = (b - a).metric_squared().sqrt();
        let mut residual = (1.0 - old_t) * length - distance;
        if residual >= 0.0 {
            self.last = CurveInternalCoord {
                segment,
                t: 1.0 - residual / length,
            };
            return;
        }
        self.last = CurveInternalCoord { segment, t: 1.0 };
        for ((segment, a), (_, b)) in windows {
            let length = (b - a).metric_squared().sqrt();
            residual += length;
            if residual >= 0.0 {
                self.last = CurveInternalCoord {
                    segment,
                    t: 1.0 - residual / length,
                };
                return;
            }
            self.last = CurveInternalCoord { segment, t: 1.0 };
        }
    }
    pub fn extend_left_until(&mut self, distance: f32, stop: impl FnMut(&usize) -> bool) {
        assert!(distance >= 0.0, "invalid extension distance");
        if distance == 0.0 {
            return;
        }
        let mut remaining = f64::from(distance);
        let windows = (0..=self.first.segment + 1)
            .rev()
            .take_until(stop)
            .map(|i| (i, self.mapped_point(i)))
            .tuple_windows::<(_, _)>();
        let mut updated = None;
        for ((_, b), (i, a)) in windows {
            let length = b.metric_distance_f64(a);
            if length == 0.0 {
                if remaining == 0.0 {
                    break;
                }
                updated = Some(CurveInternalCoord { segment: i, t: 0.0 });
                continue;
            }
            let start_t = if updated.is_none() {
                f64::from(self.first.t)
            } else {
                1.0
            };
            let available = start_t * length;
            if remaining <= available {
                updated = Some(CurveInternalCoord {
                    segment: i,
                    t: (start_t - remaining / length).max(0.0) as f32,
                });
                break;
            }
            updated = Some(CurveInternalCoord { segment: i, t: 0.0 });
            remaining -= available;
        }
        if let Some(start) = updated {
            self.first = start;
        }
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
        if let Some(last) = last
            && first <= last
        {
            output.extend_from_slice(&self.points[first..=last]);
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
    /// Join the tail and head of a closed curve, removing the seam vertex.
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
        if end != 0.0
            && let Ok(right) = self.try_slice(0.0, end)
        {
            if !out.is_empty() {
                out.pop();
            }
            if out.last() == right.points.first() {
                out.pop();
            }
            out.extend(right.points);
        }
        Self::try_new_from_points(out, false)
    }
}

#[cfg(test)]
mod stateful_native_tests {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/oracle/shared-foundation-2026-10-13/geometry-curves/interface-batch-02/slice-native-tests.rs"
    ));
}

#[cfg(test)]
mod original_right_f32_regressions {
    use super::*;
    use glam::{Vec2, Vec3};

    #[test]
    fn zero_distance_keeps_original_f32_cancellation() {
        let points = [
            Vec3::new(f32::from_bits(3246861589), f32::from_bits(3236099130), f32::from_bits(1096230032)),
            Vec3::new(f32::from_bits(1089722329), f32::from_bits(1100558226), f32::from_bits(1097214363)),
            Vec3::new(f32::from_bits(3233710308), f32::from_bits(1093554716), f32::from_bits(1091714934)),
        ];
        let coordinate = CurveInternalCoord { segment: 0, t: f32::from_bits(1057632207) };
        let mut slice = CurveSlice::<Vec3, _, super::super::CurveCoordSemantic2D>::from_coord_range(
            points.as_slice(), coordinate, coordinate,
        ).remap_2d();
        slice.extend_right(0.0);
        assert_eq!(slice.start(), coordinate);
        assert_eq!(slice.end().t.to_bits(), 1057632208);
    }

    #[test]
    fn right_extension_uses_the_remapped_metric() {
        let points = [Vec3::ZERO, Vec3::new(3.0, 100.0, 4.0)];
        let mut slice = CurveSlice::<Vec3, _, super::super::CurveCoordSemantic2D>::from_point_range(
            points.as_slice(), 0..=0,
        ).remap_2d();
        slice.extend_right(2.5);
        assert_eq!(slice.end(), CurveInternalCoord { segment: 0, t: 0.5 });
    }

    #[test]
    fn stopping_vertex_is_included_without_sampling_the_next_vertex() {
        let points = [Vec2::ZERO, Vec2::X, Vec2::X * 2.0, Vec2::X * 3.0];
        let sampled = std::cell::RefCell::new(Vec::new());
        let mut stopped = Vec::new();
        let mut slice = CurveSlice::<Vec2, _, super::super::CurveCoordSemantic2D>::from_point_range(
            points.as_slice(), 0..=0,
        ).remap(|index, point| {
            sampled.borrow_mut().push(index);
            *point
        });
        slice.extend_right_until(100.0, |index| {
            stopped.push(*index);
            *index == 2
        });
        assert_eq!(*sampled.borrow(), [0, 1, 2]);
        assert_eq!(stopped, [0, 1, 2]);
        assert_eq!(slice.end(), CurveInternalCoord { segment: 1, t: 1.0 });
    }

    #[test]
    fn later_remap_failure_preserves_the_completed_endpoint() {
        let points = [Vec2::ZERO, Vec2::X, Vec2::X * 2.0];
        let mut slice = CurveSlice::<Vec2, _, super::super::CurveCoordSemantic2D>::from_point_range(
            points.as_slice(), 0..=0,
        ).remap(|index, point| {
            assert_ne!(index, 2, "late remap failure");
            *point
        });
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            slice.extend_right(10.0);
        })).is_err());
        assert_eq!(slice.end(), CurveInternalCoord { segment: 0, t: 1.0 });
    }
}

#[cfg(test)]
mod borrowed_point_constructor_tests {
    use super::*;
    use super::super::CurveCoordSemantic2D;
    use glam::Vec2;

    #[test]
    fn borrowed_xyz_is_lazy_and_uses_actual_xz_distance() {
        let points = [[0.0, 2.0, 0.0], [3.0, 100.0, 4.0], [6.0, -10.0, 4.0]];
        let sampled = std::cell::RefCell::new(Vec::new());
        let coordinate = CurveInternalCoord { segment: 0, t: 0.0 };
        let mut view = CurveSlice::<[f32; 3], _, CurveCoordSemantic2D, Vec2, _>::from_coord_range_with_remap(
            points.as_slice(), coordinate, coordinate, |index, point| {
                sampled.borrow_mut().push(index);
                Vec2::new(point[0], point[2])
            },
        );
        assert!(sampled.borrow().is_empty());
        view.extend_right(2.5);
        assert_eq!(*sampled.borrow(), [0, 1]);
        assert_eq!(view.end(), CurveInternalCoord { segment: 0, t: 0.5 });
        assert_eq!(points[0][1], 2.0);
        assert_eq!(points[1][1], 100.0);
    }

    #[test]
    fn borrowed_constructor_retains_coordinate_and_short_input_failures() {
        let points = [[0.0, 2.0, 0.0], [3.0, 100.0, 4.0]];
        let zero = CurveInternalCoord { segment: 0, t: 0.0 };
        for (first, last) in [
            (CurveInternalCoord { segment: 1, t: 0.0 }, zero),
            (zero, CurveInternalCoord { segment: 0, t: f32::NAN }),
            (zero, CurveInternalCoord { segment: 0, t: -0.1 }),
            (CurveInternalCoord { segment: 0, t: 0.5 }, CurveInternalCoord { segment: 0, t: 0.2 }),
        ] {
            assert!(std::panic::catch_unwind(|| {
                CurveSlice::<[f32; 3], _, CurveCoordSemantic2D, Vec2, _>::from_coord_range_with_remap(
                    points.as_slice(), first, last, |_, p| Vec2::new(p[0], p[2]),
                )
            }).is_err());
        }
        for short in [&points[..0], &points[..1]] {
            assert!(std::panic::catch_unwind(|| {
                CurveSlice::<[f32; 3], _, CurveCoordSemantic2D, Vec2, _>::from_coord_range_with_remap(
                    short, zero, zero, |_, p| Vec2::new(p[0], p[2]),
                )
            }).is_err());
        }
    }

    #[test]
    fn borrowed_late_mapper_failure_preserves_completed_native_endpoint() {
        let points = [[0.0, 0.0, 0.0], [1.0, 100.0, 0.0], [2.0, 200.0, 0.0]];
        let zero = CurveInternalCoord { segment: 0, t: 0.0 };
        let mut view = CurveSlice::<[f32; 3], _, CurveCoordSemantic2D, Vec2, _>::from_coord_range_with_remap(
            points.as_slice(), zero, zero, |index, p| {
                assert_ne!(index, 2, "later genuine mapper failure");
                Vec2::new(p[0], p[2])
            },
        );
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            view.extend_right(10.0);
        })).is_err());
        assert_eq!(view.end(), CurveInternalCoord { segment: 0, t: 1.0 });
    }
}
