//! B private polyline algorithms. Allocation identity and Rust ABI are not APIs.
//! Vec3 has both XYZ and projected-XZ arc-length semantics in the original.
pub mod curve_slice;
pub mod curve_u;
pub mod curve_u_circular;
mod resample;
pub use curve_u::CurveU;
use glam::{Vec2, Vec3};
use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};
#[derive(Clone, Copy, Debug)]
pub struct CurveCoordSemantic2D;
#[derive(Clone, Copy, Debug)]
pub struct CurveCoordSemantic3D;
/// The observed B Thresholded3D constructor uses full XYZ arc length.
/// Keep the semantic marker distinct; its name is not an XZ metric policy.
#[derive(Clone, Copy, Debug)]
pub struct CurveCoordSemanticThresholded3D;
#[derive(Clone, Debug, PartialEq)]
pub enum CurveError {
    TooFewPoints(u8),
    NonFiniteLength,
    DuplicateSegment(usize),
}
/// Point arithmetic uses glam; explicit metric sums preserve game operation order.
pub trait CurvePoint:
    Copy
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<f32, Output = Self>
    + Div<f32, Output = Self>
{
    fn metric_squared(self) -> f32;
    fn normalized(self) -> Self;
    fn projected(self) -> Vec2;
}
impl CurvePoint for Vec2 {
    fn metric_squared(self) -> f32 {
        self.length_squared()
    }
    fn normalized(self) -> Self {
        // Repeated points are permitted. Construct the undefined tangent
        // directly: arithmetic-generated NaN signs depend on the hardware.
        if self == Self::ZERO {
            Self::splat(f32::from_bits(0xffc0_0000))
        } else {
            self.normalize()
        }
    }
    fn projected(self) -> Vec2 {
        self
    }
}
impl CurvePoint for Vec3 {
    fn metric_squared(self) -> f32 {
        self.length_squared()
    }
    fn normalized(self) -> Self {
        // Repeated points are permitted. Construct the undefined tangent
        // directly: arithmetic-generated NaN signs depend on the hardware.
        if self == Self::ZERO {
            Self::splat(f32::from_bits(0xffc0_0000))
        } else {
            self.normalize()
        }
    }
    fn projected(self) -> Vec2 {
        Vec2::new(self.x, self.z)
    }
}
pub trait CurveMetric<T: CurvePoint> {
    fn segment_length(delta: T) -> f32;
}
impl<T: CurvePoint> CurveMetric<T> for CurveCoordSemantic2D {
    fn segment_length(delta: T) -> f32 {
        let p = delta.projected();
        p.length()
    }
}
impl<T: CurvePoint> CurveMetric<T> for CurveCoordSemantic3D {
    fn segment_length(delta: T) -> f32 {
        delta.metric_squared().sqrt()
    }
}
impl<T: CurvePoint> CurveMetric<T> for CurveCoordSemanticThresholded3D {
    fn segment_length(delta: T) -> f32 {
        delta.metric_squared().sqrt()
    }
}
#[derive(Clone, Debug)]
pub struct Curve<T: CurvePoint, S = CurveCoordSemantic2D> {
    pub points: Vec<T>,
    pub points_u: Vec<f32>,
    pub length: f32,
    semantic: PhantomData<S>,
}
pub type Curve2 = Curve<Vec2, CurveCoordSemantic2D>;
pub type Curve3 = Curve<Vec3, CurveCoordSemantic3D>;
pub type Curve3XZ = Curve<Vec3, CurveCoordSemantic2D>;
pub type Curve3Thresholded = Curve<Vec3, CurveCoordSemanticThresholded3D>;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CurveInternalCoord {
    pub segment: usize,
    pub t: f32,
}
impl CurveInternalCoord {
    /// B 0x1411fb2b0: the last point at t=0 maps to previous segment at t=1.
    #[allow(clippy::manual_range_contains)] // Original internal-coordinate guard permits NaN t.
    pub fn new_for_point_count(segment: usize, t: f32, point_count: usize) -> Self {
        assert!(!(t < 0.0 || t > 1.0));
        assert!(segment < point_count);
        if segment + 1 < point_count {
            Self { segment, t }
        } else {
            assert!(t == 0.0);
            Self {
                segment: segment.checked_sub(1).expect("no segment"),
                t: 1.0,
            }
        }
    }
}
fn clamp01(value: f32) -> f32 {
    let lo = if 0.0 <= value { value } else { 0.0 };
    if lo <= 1.0 { lo } else { 1.0 }
}
impl<T: CurvePoint, S: CurveMetric<T>> Curve<T, S> {
    /// B 0x141469430 / 0x141269ce0 / 0x1412699d0.
    pub fn try_new_from_points(
        points: Vec<T>,
        reject_duplicates: bool,
    ) -> Result<Self, CurveError> {
        let (points_u, length) = Self::parameters_for_points(&points, reject_duplicates)?;
        Ok(Self {
            points,
            points_u,
            length,
            semantic: PhantomData,
        })
    }
    fn parameters_for_points(
        points: &[T],
        reject_duplicates: bool,
    ) -> Result<(Vec<f32>, f32), CurveError> {
        if points.len() < 2 {
            return Err(CurveError::TooFewPoints(points.len() as u8));
        }
        let mut previous = points[0];
        let mut length = 0.0;
        let mut points_u = Vec::with_capacity(points.len());
        for &point in points {
            length += S::segment_length(previous - point);
            points_u.push(length);
            previous = point;
        }
        if length == 0.0 {
            return Err(CurveError::NonFiniteLength);
        }
        for u in &mut points_u {
            *u /= length;
            if !u.is_finite() {
                return Err(CurveError::NonFiniteLength);
            }
        }
        if reject_duplicates {
            for i in 0..points.len() - 1 {
                if points[i] == points[i + 1] {
                    return Err(CurveError::DuplicateSegment(i));
                }
            }
        }
        Ok((points_u, length))
    }
    pub fn try_new(points: Vec<T>, reject_duplicates: bool) -> Result<Self, CurveError> {
        Self::try_new_from_points(points, reject_duplicates)
    }
    /// B inline retain predicate compares each point to the previously retained
    /// point. Repeated endpoints of a loop are not adjacent and are preserved.
    pub fn try_new_from_points_remove_duplicates(mut points: Vec<T>) -> Result<Self, CurveError> {
        points.dedup();
        Self::try_new_from_points(points, false)
    }
    pub fn crashy_new_from_points(points: Vec<T>, reject_duplicates: bool) -> Self {
        Self::try_new_from_points(points, reject_duplicates).unwrap()
    }
    /// Finite unbounded input, endpoint clamp, right-biased search (0x141467ce0).
    pub fn get_coord_at_u(&self, u: f32) -> CurveInternalCoord {
        assert!(u.is_finite());
        if u >= 1.0 {
            return CurveInternalCoord {
                segment: self.points.len() - 2,
                t: 1.0,
            };
        }
        if u <= 0.0 {
            return CurveInternalCoord { segment: 0, t: 0.0 };
        }
        // Keep the original right bias at repeated cumulative coordinates;
        // searching a sorted slice is provided by std, not a game algorithm.
        let next = self.points_u.partition_point(|&value| value <= u);
        assert!(next > 0 && next < self.points_u.len());
        let segment = next - 1;
        let a = self.points_u[segment];
        let b = self.points_u[next];
        assert!(a != b);
        CurveInternalCoord {
            segment,
            t: clamp01((u - a) / (b - a)),
        }
    }
    pub fn coord_at_u(&self, u: f32) -> (usize, f32) {
        let c = self.get_coord_at_u(u);
        (c.segment, c.t)
    }
    pub fn get_pos_at_coord(&self, c: CurveInternalCoord) -> T {
        self.points[c.segment + 1] * c.t + self.points[c.segment] * (1.0 - c.t)
    }
    pub fn get_pos_at_coord_unchecked(&self, c: CurveInternalCoord) -> T {
        // Unchecked means the parameter can extrapolate; ownership stays safe.
        self.get_pos_at_coord(c)
    }
    pub fn get_unnormalized_tangent_at_coord(&self, c: CurveInternalCoord) -> T {
        self.points[c.segment + 1] - self.points[c.segment]
    }
    /// B caller inline uses saturating_sub(i0,1) and min(i0+2,last).
    pub fn get_unnormalized_central_tangent_at_coord(&self, c: CurveInternalCoord) -> T {
        self.points[(c.segment + 2).min(self.points.len() - 1)]
            - self.points[c.segment.saturating_sub(1)]
    }
    pub fn get_pos_at_u(&self, u: f32) -> T {
        self.get_pos_at_coord(self.get_coord_at_u(u))
    }
    pub fn pos_at_u(&self, u: f32) -> T {
        self.get_pos_at_u(u)
    }
    pub fn get_tangent_at_coord(&self, c: CurveInternalCoord) -> T {
        (self.points[c.segment + 1] - self.points[c.segment]).normalized()
    }
    pub fn get_tangent_at_u(&self, u: f32) -> T {
        self.get_tangent_at_coord(self.get_coord_at_u(u))
    }
    pub fn tangent_at_u(&self, u: f32) -> T {
        self.get_tangent_at_u(u)
    }
    /// B 0x14146a060/0x141469e00 compare XZ for Vec3, XY for Vec2.
    /// Equal-distance ties retain the first segment; near-zero denominator 1e-8.
    pub fn get_approx_coord_from_pos(&self, point: Vec2) -> CurveInternalCoord {
        let mut best = CurveInternalCoord { segment: 0, t: 0.0 };
        let mut distance = f32::INFINITY;
        for (segment, pair) in self.points.windows(2).enumerate() {
            let a = pair[0].projected();
            let b = pair[1].projected();
            let delta = b - a;
            let d2 = delta.y * delta.y + delta.x * delta.x;
            let denominator = if d2 <= 1e-8 { 1e-8 } else { d2 };
            let t = clamp01(((point.y - a.y) * delta.y + (point.x - a.x) * delta.x) / denominator);
            let candidate = b * t + a * (1.0 - t);
            let d = point - candidate;
            let d = (d.y * d.y + d.x * d.x).sqrt();
            if d < distance {
                distance = d;
                best = CurveInternalCoord { segment, t };
            }
        }
        best
    }
    pub fn get_approx_u_from_pos(&self, point: Vec2) -> f32 {
        assert!(self.points.len() >= 2);
        let c = self.get_approx_coord_from_pos(point);
        let u = c.t * self.points_u[c.segment + 1] + (1.0 - c.t) * self.points_u[c.segment];
        assert!(u.is_finite());
        u
    }
    /// B reverse rebuilds the cumulative lengths instead of subtracting from 1.
    pub fn reverse(&mut self) {
        self.points.reverse();
        let (points_u, length) = Self::parameters_for_points(&self.points, false).unwrap();
        self.points_u = points_u;
        self.length = length;
    }
    /// The original appends before validation. On error, appended points remain
    /// in the donor while its old parameter cache remains unchanged.
    pub fn try_extend(&mut self, points: &[T]) -> Result<(), CurveError> {
        self.points.extend_from_slice(points);
        let (points_u, length) = Self::parameters_for_points(&self.points, true)?;
        self.points_u = points_u;
        self.length = length;
        Ok(())
    }
    pub fn translate(&mut self, delta: T) {
        for point in &mut self.points {
            *point = *point + delta;
        }
    }
    pub fn crashy_lerp(&self, other: &Self, t: f32) -> Self {
        assert_eq!(self.points.len(), other.points.len());
        let points = self
            .points
            .iter()
            .zip(&other.points)
            .map(|(&a, &b)| a * (1.0 - t) + b * t)
            .collect();
        Self::try_new_from_points(points, false).unwrap()
    }
    pub fn smooth(mut self, iterations: usize) -> Result<Self, CurveError> {
        smooth_in_place(&mut self.points, iterations, false);
        Self::try_new_from_points(self.points, false)
    }
    /// Weighted B smoothing gives each neighbor weight 0.5 and the center its
    /// supplied integer weight, with unchanged open endpoints.
    pub fn smooth_weighted(
        mut self,
        iterations: usize,
        weights: &[usize],
    ) -> Result<Self, CurveError> {
        assert_eq!(self.points.len(), weights.len());
        if self.points.len() > 2 {
            for _ in 0..iterations {
                let mut previous = self.points[0];
                for (i, &weight) in weights
                    .iter()
                    .enumerate()
                    .take(self.points.len() - 1)
                    .skip(1)
                {
                    let current = self.points[i];
                    let weight = weight as f32;
                    self.points[i] = (current * weight + previous * 0.5 + self.points[i + 1] * 0.5)
                        / (weight + 1.0);
                    previous = current;
                }
            }
        }
        Self::try_new_from_points(self.points, false)
    }
    pub fn smooth_in_place(&mut self, iterations: usize, closed: bool) -> Result<(), CurveError> {
        smooth_in_place(&mut self.points, iterations, closed);
        let (points_u, length) = Self::parameters_for_points(&self.points, false)?;
        self.points_u = points_u;
        self.length = length;
        Ok(())
    }
    /// Distance follows the mapped XY/XZ point set, independently of S's cache.
    pub fn get_coord_at_2d_ws_distance(&self, distance: f32) -> CurveInternalCoord {
        let mut view = curve_slice::CurveSlice::<T, _, S>::from_curve_range(self, 0..=0).remap_2d();
        view.extend_right(distance);
        view.end()
    }
    pub fn get_coord_at_wall_space(&self, distance: f32) -> CurveInternalCoord {
        self.get_coord_at_2d_ws_distance(distance.max(0.0))
    }
    /// B nearest segment selection and separately normalized vectors; zero
    /// vectors produce NaNs and consequently return false.
    pub fn is_on_right_side(&self, point: Vec2) -> bool {
        let coord = self.get_approx_coord_from_pos(point);
        let a = self.points[coord.segment].projected();
        let b = self.points[coord.segment + 1].projected();
        let from_start = (point - a).normalize();
        let reverse_segment = (a - b).normalize();
        reverse_segment.y * from_start.x < reverse_segment.x * from_start.y
    }
}
/// B free smoother: previous values are carried to prevent Gauss-Seidel bias.
/// For a closed donor the last point duplicates the first; update it each pass.
pub fn smooth_in_place<T: CurvePoint>(points: &mut [T], iterations: usize, closed: bool) {
    if points.len() <= 2 {
        return;
    }
    for _ in 0..iterations {
        let begin = if closed { 0 } else { 1 };
        let end = points.len() - 1;
        let mut previous = if closed { points[end - 1] } else { points[0] };
        for i in begin..end {
            let current = points[i];
            points[i] = ((previous + points[i + 1]) * 0.5 - current) * 0.5 + current;
            previous = current;
        }
        if closed {
            points[end] = points[0];
        }
    }
}
impl<S: CurveMetric<Vec3>> Curve<Vec3, S> {
    /// B inline projects point coordinates and copies parameter/length caches.
    /// This differs from try_to_curve2, which recalculates projected lengths.
    pub fn reinterpret_as_curve2(&self) -> Curve2 {
        Curve2 {
            points: crate::swizzlers::project_vec3_slice_xz(&self.points),
            points_u: self.points_u.clone(),
            length: self.length,
            semantic: PhantomData,
        }
    }
    pub fn try_to_curve2(&self) -> Result<Curve2, CurveError> {
        Curve2::try_new_from_points(crate::swizzlers::project_vec3_slice_xz(&self.points), false)
    }
    pub fn crashy_to_curve2(&self) -> Curve2 {
        self.try_to_curve2().unwrap()
    }
}
impl Curve3XZ {
    /// B 0x140c9cc60 moves caches unchanged; it does not rebuild XYZ lengths.
    pub fn into_curve3(self) -> Curve3 {
        Curve3 {
            points: self.points,
            points_u: self.points_u,
            length: self.length,
            semantic: PhantomData,
        }
    }
}
impl Curve2 {
    pub fn to_curve3(&self, height: f32) -> Curve3XZ {
        Curve3XZ {
            points: self
                .points
                .iter()
                .map(|p| Vec3::new(p.x, height, p.y))
                .collect(),
            points_u: self.points_u.clone(),
            length: self.length,
            semantic: PhantomData,
        }
    }
    pub fn project_point(&self, point: Vec2) -> Vec2 {
        self.get_pos_at_u(clamp01(self.get_approx_u_from_pos(point)))
    }
}
impl crate::wall_space::CurveSampling for Curve2 {
    fn length(&self) -> f32 {
        self.length
    }
    fn point_u(&self, index: usize) -> f32 {
        self.points_u[index]
    }
    fn position_at_u(&self, u: f32) -> Vec2 {
        self.get_pos_at_u(u)
    }
    fn tangent_at_u(&self, u: f32) -> Vec2 {
        self.get_tangent_at_u(u)
    }
    fn approximate_u(&self, point: Vec2) -> f32 {
        self.get_approx_u_from_pos(point)
    }
}
#[cfg(test)]
mod native_oracle {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/oracle/shared-foundation-2026-10-13/geometry-curves/native_tests.rs"
    ));
}
#[cfg(test)]
mod completion_oracle {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/oracle/foundation-completion-2026-10-13/curves/sampling_tests.rs"
    ));
}
#[cfg(test)]
mod completion_query_oracle {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/oracle/foundation-completion-2026-10-13/curves/query_tests.rs"
    ));
}
