//! Polyline construction, queries, smoothing and coordinate conversion.
//! The semantic marker selects full spatial or projected arc length.
pub mod curve_slice;
pub mod curve_u;
pub mod curve_u_circular;
mod resample;
use crate::scalar::clamp01;
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
/// Full XYZ arc length with a distinct marker for thresholded consumers.
#[derive(Clone, Copy, Debug)]
pub struct CurveCoordSemanticThresholded3D;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CurveError {
    TooFewPoints(u8),
    NonFiniteLength,
    DuplicateSegment(usize),
    InvalidSpacing,
    InvalidSampleCount,
    AllocationFailed,
}
impl std::fmt::Display for CurveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewPoints(count) => write!(f, "curve needs at least two points, got {count}"),
            Self::NonFiniteLength => f.write_str("curve length must be finite and positive"),
            Self::DuplicateSegment(index) => write!(f, "duplicate points at segment {index}"),
            Self::InvalidSpacing => f.write_str("sample spacing must be finite and positive"),
            Self::InvalidSampleCount => {
                f.write_str("sample count is outside the representable range")
            }
            Self::AllocationFailed => f.write_str("unable to allocate curve samples"),
        }
    }
}
impl std::error::Error for CurveError {}

/// Vector operations needed by polyline algorithms.
pub trait CurvePoint:
    Copy
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<f32, Output = Self>
    + Div<f32, Output = Self>
{
    /// 已核原构造单态的点维度；自定义Point默认保留现稳健参数策略。
    const ORIGINAL_PARAMETER_DIMENSION: u8 = 0;

    fn metric_squared(self) -> f32;
    /// Euclidean length, with a default for existing custom point types.
    fn metric_length(self) -> f32 {
        self.metric_squared().sqrt()
    }
    /// Wider distance for slices whose finite endpoints can span more than f32::MAX.
    fn metric_distance_f64(self, other: Self) -> f64 {
        f64::from((self - other).metric_length())
    }
    fn is_finite(self) -> bool {
        self.metric_squared().is_finite()
    }
    fn normalized(self) -> Self;
    fn projected(self) -> Vec2;
}
impl CurvePoint for Vec2 {
    const ORIGINAL_PARAMETER_DIMENSION: u8 = 2;

    fn is_finite(self) -> bool {
        self.is_finite()
    }
    fn metric_squared(self) -> f32 {
        self.length_squared()
    }
    fn metric_length(self) -> f32 {
        let squared = self.length_squared();
        if squared.is_normal() || self == Self::ZERO {
            squared.sqrt()
        } else {
            self.as_dvec2().length() as f32
        }
    }
    fn metric_distance_f64(self, other: Self) -> f64 {
        let squared = (self - other).length_squared();
        if squared.is_normal() || self == other {
            f64::from(squared.sqrt())
        } else {
            self.as_dvec2().distance(other.as_dvec2())
        }
    }
    fn normalized(self) -> Self {
        let norm_squared = self.length_squared();
        if norm_squared.is_normal() {
            return self.normalize();
        }
        // Scale only extreme magnitudes; squaring may underflow or overflow.
        let scale = self.abs().max_element();
        if scale.is_finite() && scale > 0.0 {
            Vec2::new(self.x / scale, self.y / scale).normalize()
        } else {
            Self::ZERO
        }
    }
    fn projected(self) -> Vec2 {
        self
    }
}
impl CurvePoint for Vec3 {
    const ORIGINAL_PARAMETER_DIMENSION: u8 = 3;

    fn is_finite(self) -> bool {
        self.is_finite()
    }
    fn metric_squared(self) -> f32 {
        self.length_squared()
    }
    fn metric_length(self) -> f32 {
        let squared = self.length_squared();
        if squared.is_normal() || self == Self::ZERO {
            squared.sqrt()
        } else {
            self.as_dvec3().length() as f32
        }
    }
    fn metric_distance_f64(self, other: Self) -> f64 {
        let squared = (self - other).length_squared();
        if squared.is_normal() || self == other {
            f64::from(squared.sqrt())
        } else {
            self.as_dvec3().distance(other.as_dvec3())
        }
    }
    fn normalized(self) -> Self {
        let norm_squared = self.length_squared();
        if norm_squared.is_normal() {
            return self.normalize();
        }
        // Scale only extreme magnitudes; squaring may underflow or overflow.
        let scale = self.abs().max_element();
        if scale.is_finite() && scale > 0.0 {
            Vec3::new(self.x / scale, self.y / scale, self.z / scale).normalize()
        } else {
            Self::ZERO
        }
    }
    fn projected(self) -> Vec2 {
        Vec2::new(self.x, self.z)
    }
}
pub trait CurveMetric<T: CurvePoint> {
    /// 只有已证的Point/semantic组合使用原f32构造域。
    const ORIGINAL_PARAMETER_DOMAIN: bool = false;

    fn segment_length(delta: T) -> f32;

    /// 构造参数专用的编译期策略；既有稳健测距API保持不变。
    fn constructor_segment_length(delta: T) -> f32 {
        Self::segment_length(delta)
    }
}
impl<T: CurvePoint> CurveMetric<T> for CurveCoordSemantic2D {
    // 原Vec2构造与1412699d0的Vec3XZ构造共用纯f32参数流程。
    // XZ只度量x/z，不提前要求Y有限；重复点仍按完整XYZ有序比较。
    const ORIGINAL_PARAMETER_DOMAIN: bool =
        T::ORIGINAL_PARAMETER_DIMENSION == 2 || T::ORIGINAL_PARAMETER_DIMENSION == 3;

    fn segment_length(delta: T) -> f32 {
        let p = delta.projected();
        p.metric_length()
    }

    fn constructor_segment_length(delta: T) -> f32 {
        let p = delta.projected();
        (p.y * p.y + p.x * p.x).sqrt()
    }
}
impl<T: CurvePoint> CurveMetric<T> for CurveCoordSemantic3D {
    const ORIGINAL_PARAMETER_DOMAIN: bool = T::ORIGINAL_PARAMETER_DIMENSION == 3;

    fn segment_length(delta: T) -> f32 {
        delta.metric_length()
    }

    fn constructor_segment_length(delta: T) -> f32 {
        // Vec3::metric_squared是pinned glam的纯f32先xy后z，无f64补偿。
        delta.metric_squared().sqrt()
    }
}
impl<T: CurvePoint> CurveMetric<T> for CurveCoordSemanticThresholded3D {
    fn segment_length(delta: T) -> f32 {
        delta.metric_length()
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
    /// Represent the final vertex as the preceding segment at `t = 1`.
    pub fn new_for_point_count(segment: usize, t: f32, point_count: usize) -> Self {
        assert!((0.0..=1.0).contains(&t), "invalid segment parameter");
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
impl<T: CurvePoint, S: CurveMetric<T>> Curve<T, S> {
    /// Takes an authoritative serialized curve, including its cached parameters.
    /// No recomputation or geometry policy is applied by this ownership constructor.
    pub fn from_cached_parts(points: Vec<T>, points_u: Vec<f32>, length: f32) -> Self {
        Self {
            points,
            points_u,
            length,
            semantic: PhantomData,
        }
    }
    /// Build normalized cumulative arc lengths, optionally rejecting repeated points.
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
        if !S::ORIGINAL_PARAMETER_DOMAIN && points.iter().any(|point| !point.is_finite()) {
            return Err(CurveError::NonFiniteLength);
        }
        // 原已证单态先分配，再读取first并逐项累计（包含first自身）。
        let mut points_u = Vec::with_capacity(points.len());
        let mut previous = points[0];
        let mut length = 0.0;
        for &point in points {
            let delta = previous - point;
            let distance = if S::ORIGINAL_PARAMETER_DOMAIN {
                S::constructor_segment_length(delta)
            } else {
                S::segment_length(delta)
            };
            length += distance;
            points_u.push(length);
            previous = point;
        }
        let invalid_length = if S::ORIGINAL_PARAMETER_DOMAIN {
            // 原NaN/Inf继续到逐项规范化检查，不能提前扫描输入或改为<=0。
            length == 0.0
        } else {
            !length.is_finite() || length <= 0.0
        };
        if invalid_length {
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
    /// Remove adjacent duplicates while preserving repeated endpoints of a loop.
    pub fn try_new_from_points_remove_duplicates(mut points: Vec<T>) -> Result<Self, CurveError> {
        points.dedup();
        Self::try_new_from_points(points, false)
    }
    #[track_caller]
    pub fn crashy_new_from_points(points: Vec<T>, reject_duplicates: bool) -> Self {
        Self::try_new_from_points(points, reject_duplicates).unwrap()
    }
    /// Clamp a finite coordinate and select the segment after repeated knots.
    pub fn get_coord_at_u(&self, u: f32) -> CurveInternalCoord {
        coord_at_cached_u(self.points.len(), &self.points_u, u)
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
    /// Estimate the tangent using the outer neighboring vertices.
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
    /// Degenerate segments have a zero tangent.
    pub fn get_tangent_at_coord(&self, c: CurveInternalCoord) -> T {
        (self.points[c.segment + 1] - self.points[c.segment]).normalized()
    }
    pub fn get_tangent_at_u(&self, u: f32) -> T {
        self.get_tangent_at_coord(self.get_coord_at_u(u))
    }
    pub fn tangent_at_u(&self, u: f32) -> T {
        self.get_tangent_at_u(u)
    }
    /// Find the nearest segment in XZ for Vec3 or XY for Vec2.
    /// Equal-distance ties retain the first segment; near-zero denominator 1e-8.
    pub fn get_approx_coord_from_pos(&self, point: Vec2) -> CurveInternalCoord {
        approx_curve_coord(&self.points, point, |p| p.projected())
    }
    pub fn get_approx_u_from_pos(&self, point: Vec2) -> f32 {
        approx_curve_u(&self.points, &self.points_u, point, |p| p.projected())
    }
    /// Reverse the points and rebuild cumulative lengths to avoid subtraction drift.
    pub fn reverse(&mut self) {
        self.points.reverse();
        let (points_u, length) = Self::parameters_for_points(&self.points, false).unwrap();
        self.points_u = points_u;
        self.length = length;
    }
    /// Append points atomically: a validation error preserves the curve and its cache.
    pub fn try_extend(&mut self, points: &[T]) -> Result<(), CurveError> {
        let original_len = self.points.len();
        self.points.extend_from_slice(points);
        let (points_u, length) = match Self::parameters_for_points(&self.points, true) {
            Ok(parameters) => parameters,
            Err(error) => {
                self.points.truncate(original_len);
                return Err(error);
            }
        };
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
    /// Weighted smoothing gives each neighbor weight 0.5 and the center its
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
        if iterations == 0 {
            return Ok(());
        }
        let mut points = self.points.clone();
        smooth_in_place(&mut points, iterations, closed);
        let (points_u, length) = Self::parameters_for_points(&points, false)?;
        self.points = points;
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
    /// Nearest segment selection and separately normalized vectors; zero
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
/// Carry previous values so each pass uses the original neighbors.
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
    /// Project point coordinates while preserving parameter and length caches.
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
    /// Change the marker while retaining the existing projected-length cache.
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
    fn raw_tangent_at_u(&self, u: f32) -> Vec2 {
        self.get_unnormalized_tangent_at_coord(self.get_coord_at_u(u))
    }
    fn approximate_u(&self, point: Vec2) -> f32 {
        self.get_approx_u_from_pos(point)
    }
    fn start_tangent(&self) -> Vec2 {
        self.points
            .windows(2)
            .map(|pair| (pair[1] - pair[0]).normalized())
            .find(|&tangent| tangent != Vec2::ZERO)
            .unwrap_or(Vec2::ZERO)
    }
    fn end_tangent(&self) -> Vec2 {
        self.points
            .windows(2)
            .rev()
            .map(|pair| (pair[1] - pair[0]).normalized())
            .find(|&tangent| tangent != Vec2::ZERO)
            .unwrap_or(Vec2::ZERO)
    }
}
/// Native572/595B nearest-coordinate kernel shared by owned and borrowed points.
/// Projection is lazy per actual pair; no new curve/cache or point allocation.
pub fn approx_curve_coord<P>(
    points: &[P],
    point: Vec2,
    project: impl Fn(&P) -> Vec2,
) -> CurveInternalCoord {
    let mut best = CurveInternalCoord { segment: 0, t: 0.0 };
    let mut distance = f32::MAX;
    let epsilon = f32::from_bits(0x322bcc77);
    for (segment, pair) in points.windows(2).enumerate() {
        let a = project(&pair[0]);
        let b = project(&pair[1]);
        let delta = b - a;
        let d2 = delta.y * delta.y + delta.x * delta.x;
        // Native MAXSS(d2,epsilon) selects its second operand on NaN.
        let denominator = if d2 > epsilon { d2 } else { epsilon };
        let t = ((point.y - a.y) * delta.y + (point.x - a.x) * delta.x) / denominator;
        // MAXSS(0,t), MINSS(1,t): preserve an unordered t, unlike global clamp01.
        let t = if 0.0 > t { 0.0 } else { t };
        let t = if 1.0 < t { 1.0 } else { t };
        let candidate = b * t + a * (1.0 - t);
        let d = point - candidate;
        let d = (d.y * d.y + d.x * d.x).sqrt();
        if d < distance {
            assert!((0.0..=1.0).contains(&t), "nearest coordinate outside [0,1]");
            distance = d;
            best = CurveInternalCoord { segment, t };
        }
    }
    best
}

/// 借用权威曲线的参数缓存，保持原 get_coord_at_u 的右偏与边界行为。
/// 上层切片不必为复用曲线算法逐帧克隆整条点列/参数列。
pub fn coord_at_cached_u(point_count: usize, points_u: &[f32], u: f32) -> CurveInternalCoord {
    assert!(u.is_finite());
    if u >= 1.0 {
        return CurveInternalCoord {
            segment: point_count - 2,
            t: 1.0,
        };
    }
    if u <= 0.0 {
        return CurveInternalCoord { segment: 0, t: 0.0 };
    }
    // Keep the original right bias at repeated cumulative coordinates;
    // searching a sorted slice is provided by std, not a game algorithm.
    let next = points_u.partition_point(|&value| value <= u);
    assert!(next > 0 && next < points_u.len());
    let segment = next - 1;
    let a = points_u[segment];
    let b = points_u[next];
    assert!(a != b);
    CurveInternalCoord {
        segment,
        t: clamp01((u - a) / (b - a)),
    }
}

/// Native195B approximate-U wrapper over the same actual cached coordinate data.
pub fn approx_curve_u<P>(
    points: &[P],
    points_u: &[f32],
    point: Vec2,
    project: impl Fn(&P) -> Vec2,
) -> f32 {
    assert!(points.len() > 1);
    let c = approx_curve_coord(points, point, project);
    // Native bounds checks/read order is start then end; retain final add roles.
    let start = points_u[c.segment];
    let end = points_u[c.segment + 1];
    let u = (c.t * end) + ((1.0 - c.t) * start);
    assert!(u.is_finite());
    u
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

#[cfg(test)]
mod borrowed_nearest_original_tests {
    use super::*;

    fn fixture_curve(points: Vec<Vec2>, points_u: Vec<f32>) -> Curve2 {
        // Explicit leaf fixture, not native constructor or loaded-world evidence.
        Curve {
            points,
            points_u,
            length: 1.0,
            semantic: PhantomData,
        }
    }

    #[test]
    fn original_overflow_skips_nan_segments_in_borrowed_and_owned_queries() {
        let points = [[-1e20, 0.0], [0.0, 0.0], [1e22, 0.0]];
        let cached_u = [0.0, 0.25, 1.0];
        let query = Vec2::new(1e18, 0.0);
        let curve = fixture_curve(points.map(Vec2::from_array).to_vec(), cached_u.to_vec());
        let expected = CurveInternalCoord { segment: 0, t: 0.0 };
        assert_eq!(
            approx_curve_coord(&points, query, |p| Vec2::from_array(*p)),
            expected
        );
        assert_eq!(curve.get_approx_coord_from_pos(query), expected);
        assert_eq!(
            approx_curve_u(&points, &cached_u, query, |p| Vec2::from_array(*p)),
            0.0
        );
        assert_eq!(curve.get_approx_u_from_pos(query), 0.0);
    }

    #[test]
    fn ordinary_endpoints_interior_and_first_tie_use_actual_cached_u() {
        let points = [[0.0, 0.0], [2.0, 0.0], [2.0, 2.0]];
        let cached_u = [0.0, 0.2, 1.0];
        let curve = fixture_curve(points.map(Vec2::from_array).to_vec(), cached_u.to_vec());
        for (query, segment, t, u) in [
            (Vec2::new(-1.0, -1.0), 0, 0.0, 0.0),
            (Vec2::new(1.0, 0.5), 0, 0.5, 0.1),
            (Vec2::new(2.0, 1.0), 1, 0.5, 0.6),
            (Vec2::new(3.0, 3.0), 1, 1.0, 1.0),
            (Vec2::new(1.0, 1.0), 0, 0.5, 0.1),
        ] {
            let expected = CurveInternalCoord { segment, t };
            assert_eq!(
                approx_curve_coord(&points, query, |p| Vec2::from_array(*p)),
                expected
            );
            assert_eq!(curve.get_approx_coord_from_pos(query), expected);
            assert_eq!(
                approx_curve_u(&points, &cached_u, query, |p| Vec2::from_array(*p)),
                u
            );
            assert_eq!(curve.get_approx_u_from_pos(query), u);
        }
        let repeated = [[0.0, 0.0], [0.0, 0.0], [2.0, 0.0]];
        assert_eq!(
            approx_curve_coord(&repeated, Vec2::X, |p| Vec2::from_array(*p)),
            CurveInternalCoord { segment: 1, t: 0.5 }
        );
    }

    #[test]
    fn actual_vec3_family_projects_xz_and_retains_its_cached_u() {
        let points = [
            Vec3::new(0.0, 100.0, 0.0),
            Vec3::new(2.0, -100.0, 0.0),
            Vec3::new(2.0, 500.0, 2.0),
        ];
        let curve: Curve3 = Curve {
            points: points.to_vec(),
            points_u: vec![0.0, 0.2, 1.0],
            length: 1.0,
            semantic: PhantomData,
        };
        let query = Vec2::new(2.0, 1.0);
        assert_eq!(
            curve.get_approx_coord_from_pos(query),
            CurveInternalCoord { segment: 1, t: 0.5 }
        );
        assert_eq!(curve.get_approx_u_from_pos(query), 0.6);
        assert_eq!(
            approx_curve_u(&points, &curve.points_u, query, |p| Vec2::new(p.x, p.z)),
            0.6
        );
    }

    #[test]
    fn coordinate_defaults_and_u_failures_preserve_distinct_boundaries() {
        let points = [Vec2::ZERO, Vec2::X];
        let zero = CurveInternalCoord { segment: 0, t: 0.0 };
        for short in [&points[..0], &points[..1]] {
            assert_eq!(approx_curve_coord(short, Vec2::ZERO, |p| *p), zero);
            let calls = std::cell::Cell::new(0);
            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    approx_curve_u(short, &[], Vec2::ZERO, |p| {
                        calls.set(calls.get() + 1);
                        *p
                    })
                }))
                .is_err()
            );
            assert_eq!(calls.get(), 0);
        }
        for cached_u in [
            &[][..],
            &[0.0][..],
            &[f32::NAN, 1.0][..],
            &[f32::INFINITY, 1.0][..],
        ] {
            assert!(
                std::panic::catch_unwind(|| {
                    approx_curve_u(&points, cached_u, Vec2::new(0.5, 0.0), |p| *p)
                })
                .is_err()
            );
        }
    }
}

#[cfg(test)]
mod native_constructor_tests {
    use super::*;

    #[test]
    fn original_f32_lengths_keep_point_order_and_the_underflow_overflow_domain() {
        let curve = Curve2::try_new(
            vec![Vec2::ZERO, Vec2::new(3.0, 4.0), Vec2::new(6.0, 8.0)],
            false,
        )
        .unwrap();
        assert_eq!(curve.length, 10.0);
        assert_eq!(curve.points_u, [0.0, 0.5, 1.0]);
        let curve = Curve3::try_new(
            vec![
                Vec3::ZERO,
                Vec3::new(0.0, 3.0, 4.0),
                Vec3::new(12.0, 3.0, 4.0),
            ],
            false,
        )
        .unwrap();
        assert_eq!(curve.length, 17.0);
        assert_eq!(curve.points_u, [0.0, 5.0 / 17.0, 1.0]);
        for length in [1e-30_f32, f32::MIN_POSITIVE, 1e20, f32::MAX] {
            assert!(matches!(
                Curve2::try_new(vec![Vec2::ZERO, Vec2::new(length, 0.0)], false),
                Err(CurveError::NonFiniteLength)
            ));
            assert!(matches!(
                Curve3::try_new(vec![Vec3::ZERO, Vec3::new(0.0, 0.0, length)], false),
                Err(CurveError::NonFiniteLength)
            ));
        }
        for nonfinite in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert!(matches!(
                Curve2::try_new(vec![Vec2::ZERO, Vec2::new(nonfinite, 0.0)], false),
                Err(CurveError::NonFiniteLength)
            ));
            assert!(matches!(
                Curve3::try_new(vec![Vec3::ZERO, Vec3::new(0.0, nonfinite, 0.0)], false),
                Err(CurveError::NonFiniteLength)
            ));
        }
    }

    #[test]
    fn original_normalization_errors_precede_optional_duplicate_rejection() {
        assert!(matches!(
            Curve2::try_new(Vec::new(), true),
            Err(CurveError::TooFewPoints(0))
        ));
        assert!(matches!(
            Curve3::try_new(vec![Vec3::ZERO], true),
            Err(CurveError::TooFewPoints(1))
        ));
        let points = vec![Vec2::ZERO, Vec2::ZERO, Vec2::X];
        let curve = Curve2::try_new(points.clone(), false).unwrap();
        assert_eq!(curve.points, points);
        assert_eq!(curve.points_u, [0.0, 0.0, 1.0]);
        assert!(matches!(
            Curve2::try_new(points, true),
            Err(CurveError::DuplicateSegment(0))
        ));
        assert!(matches!(
            Curve2::try_new(vec![Vec2::ZERO; 3], true),
            Err(CurveError::NonFiniteLength)
        ));
        assert!(matches!(
            Curve2::try_new(vec![Vec2::ZERO, Vec2::ZERO, Vec2::new(f32::NAN, 0.0)], true),
            Err(CurveError::NonFiniteLength)
        ));
        assert!(matches!(
            Curve3::try_new(vec![Vec3::ZERO, Vec3::ZERO, Vec3::X], true),
            Err(CurveError::DuplicateSegment(0))
        ));
    }

    #[test]
    fn original_crashy_false_and_existing_boolean_compatibility_share_the_same_constructor() {
        let points = vec![Vec2::ZERO, Vec2::ZERO, Vec2::X];
        assert_eq!(
            Curve2::crashy_new_from_points(points.clone(), false).points,
            points
        );
        assert!(std::panic::catch_unwind(|| Curve2::crashy_new_from_points(points, true)).is_err());
        assert!(
            std::panic::catch_unwind(|| Curve2::crashy_new_from_points(
                vec![Vec2::ZERO, Vec2::new(1e-30, 0.0)],
                false
            ))
            .is_err()
        );
    }

    #[test]
    fn engineering_metrics_keep_their_host_policy_separate_from_original_constructors() {
        struct Engineering;
        impl CurveMetric<Vec2> for Engineering {
            fn segment_length(delta: Vec2) -> f32 {
                delta.metric_length()
            }
        }
        let tiny2 = Vec2::new(1e-30, 0.0);
        let tiny3 = Vec3::new(1e-30, 0.0, 0.0);
        assert_eq!(tiny2.metric_length(), 1e-30);
        assert_eq!(tiny3.metric_length(), 1e-30);
        assert_eq!(
            <CurveCoordSemantic2D as CurveMetric<Vec2>>::segment_length(tiny2),
            1e-30
        );
        assert_eq!(
            Curve::<Vec2, Engineering>::try_new(vec![Vec2::ZERO, tiny2], false)
                .unwrap()
                .length,
            1e-30
        );
        assert_eq!(
            Curve3Thresholded::try_new(vec![Vec3::ZERO, tiny3], false)
                .unwrap()
                .length,
            1e-30
        );
        // 已证XZ原构造域也保留f32平方下溢失败，工程测距方法仍保持上面的稳健结果。
        assert!(matches!(
            Curve3XZ::try_new(vec![Vec3::ZERO, tiny3], false),
            Err(CurveError::NonFiniteLength)
        ));
    }
    #[test]
    fn original_xz_parameters_preserve_nonfinite_y_but_xyz_output_still_fails() {
        for y in [
            f32::from_bits(0x7fc1_2345),
            f32::INFINITY,
            f32::NEG_INFINITY,
        ] {
            let points = vec![Vec3::new(0.0, y, 0.0), Vec3::new(3.0, y, 4.0)];
            let old_xz = Curve3XZ::try_new_from_points(points.clone(), true).unwrap();
            assert_eq!(old_xz.length, 5.0);
            assert_eq!(old_xz.points_u, [0.0, 1.0]);
            for point in &old_xz.points {
                assert_eq!(point.y.to_bits(), y.to_bits());
            }
            assert_eq!(old_xz.points[0].projected(), Vec2::ZERO);
            assert_eq!(old_xz.points[1].projected(), Vec2::new(3.0, 4.0));
            // 原history先用XZ判有效，再独立构造XYZ输出；不能把两个Result合并。
            assert!(matches!(
                Curve3::try_new_from_points(points, true),
                Err(CurveError::NonFiniteLength)
            ));
        }
    }

    #[test]
    fn original_xz_duplicate_gate_compares_full_xyz_with_ordered_nan_and_signed_zero() {
        let vertical_then_horizontal = vec![Vec3::ZERO, Vec3::new(0.0, 7.0, 0.0), Vec3::X];
        let curve = Curve3XZ::try_new_from_points(vertical_then_horizontal, true).unwrap();
        assert_eq!(curve.length, 1.0);
        assert_eq!(curve.points_u, [0.0, 0.0, 1.0]);

        let nan_y = f32::from_bits(0x7fc1_2345);
        let same_bits_but_unordered = vec![
            Vec3::new(0.0, nan_y, 0.0),
            Vec3::new(0.0, nan_y, 0.0),
            Vec3::new(1.0, nan_y, 0.0),
        ];
        let curve = Curve3XZ::try_new_from_points(same_bits_but_unordered, true).unwrap();
        assert_eq!(curve.points_u, [0.0, 0.0, 1.0]);
        // XZ参数重复不是完整XYZ相等；NaN也不能被按位相同判为重复。
        for second in [Vec3::ZERO, Vec3::new(-0.0, -0.0, -0.0)] {
            assert!(matches!(
                Curve3XZ::try_new_from_points(vec![Vec3::ZERO, second, Vec3::X], true),
                Err(CurveError::DuplicateSegment(0))
            ));
        }
    }

    #[test]
    fn original_xz_length_and_normalization_errors_precede_duplicate_rejection() {
        assert!(matches!(
            Curve3XZ::try_new_from_points(Vec::new(), true),
            Err(CurveError::TooFewPoints(0))
        ));
        assert!(matches!(
            Curve3XZ::try_new_from_points(vec![Vec3::splat(f32::NAN)], true),
            Err(CurveError::TooFewPoints(1))
        ));
        assert!(matches!(
            Curve3XZ::try_new_from_points(vec![Vec3::ZERO, Vec3::new(0.0, 7.0, 0.0)], true),
            Err(CurveError::NonFiniteLength)
        ));
        assert!(matches!(
            Curve3XZ::try_new_from_points(
                vec![Vec3::ZERO, Vec3::ZERO, Vec3::new(f32::INFINITY, 0.0, 0.0)],
                true,
            ),
            Err(CurveError::NonFiniteLength)
        ));
        for step in [1e-30_f32, f32::MIN_POSITIVE, 1e20, f32::MAX] {
            assert!(matches!(
                Curve3XZ::try_new_from_points(vec![Vec3::ZERO, Vec3::new(step, 0.0, 0.0)], false),
                Err(CurveError::NonFiniteLength)
            ));
        }
    }
}
