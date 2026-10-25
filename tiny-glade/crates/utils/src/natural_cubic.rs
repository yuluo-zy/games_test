//! Chord-length natural cubic splines with optional endpoint derivatives.
//! 现代 try 入口校验退化节点；二维原版入口保留求解后由消费者决定失败的语义。
use glam::{Vec2, Vec3};
#[derive(Clone, Copy, Debug)]
pub struct SplineSegment2D {
    pub start_t: f32,
    pub coefficients: [Vec2; 4],
}
impl SplineSegment2D {
    pub fn position_at_local_t(self, t: f32) -> Vec2 {
        ((self.coefficients[3] * t + self.coefficients[2]) * t + self.coefficients[1]) * t
            + self.coefficients[0]
    }
    pub fn tangent_at_local_t(self, t: f32) -> Vec2 {
        (self.coefficients[3] * 3.0 * t + self.coefficients[2] * 2.0) * t + self.coefficients[1]
    }
}
#[derive(Clone, Copy, Debug)]
pub struct SplineSegment3D {
    pub start_t: f32,
    pub coefficients: [Vec3; 4],
}
impl SplineSegment3D {
    pub fn position_at_local_t(self, t: f32) -> Vec3 {
        ((self.coefficients[3] * t + self.coefficients[2]) * t + self.coefficients[1]) * t
            + self.coefficients[0]
    }
    pub fn tangent_at_local_t(self, t: f32) -> Vec3 {
        (self.coefficients[3] * 3.0 * t + self.coefficients[2] * 2.0) * t + self.coefficients[1]
    }
}
#[derive(Clone, Debug)]
pub struct Spline2D {
    pub segments: Vec<SplineSegment2D>,
    pub min_t: f32,
    pub max_t: f32,
    pub periodic: bool,
}
/// Segment samples with outward-facing endpoint tangents.
#[derive(Clone, Debug)]
pub struct DenseSplineSegment2D {
    pub points: Vec<Vec2>,
    pub start_tangent: Vec2,
    pub end_tangent: Vec2,
}
#[derive(Clone, Debug)]
pub struct Spline3D {
    pub segments: Vec<SplineSegment3D>,
    pub min_t: f32,
    pub max_t: f32,
    pub periodic: bool,
}
/// Failures that would otherwise introduce divisions by zero or nonfinite coefficients.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SplineError {
    TooFewPoints { required: usize, actual: usize },
    NonFinitePoint(usize),
    InvalidSegment(usize),
    NonFiniteTangent,
    NonFiniteCoefficients,
}
impl std::fmt::Display for SplineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooFewPoints { required, actual } => {
                write!(f, "spline needs at least {required} points, got {actual}")
            }
            Self::NonFinitePoint(index) => write!(f, "nonfinite spline control point {index}"),
            Self::InvalidSegment(index) => {
                write!(f, "segment {index} cannot define a distinct finite knot")
            }
            Self::NonFiniteTangent => f.write_str("spline tangents must be finite"),
            Self::NonFiniteCoefficients => {
                f.write_str("spline coefficients exceed the representable range")
            }
        }
    }
}
impl std::error::Error for SplineError {}

fn select_scalar_knot<T>(
    segments: &[T],
    min_t: f32,
    max_t: f32,
    t: f32,
    start_t: impl Fn(&T) -> f32,
) -> (usize, f32) {
    assert!(!t.is_nan(), "NaN spline parameter");
    let t = t.clamp(min_t, max_t);
    let index = segments
        .partition_point(|s| start_t(s) <= t)
        .saturating_sub(1);
    (index, t - start_t(&segments[index]))
}
fn knots<const N: usize, E>(
    points: &[[f32; N]],
    periodic: bool,
    mut check: impl FnMut(usize, f32, f32, f32) -> Result<(), E>,
) -> Result<(Vec<f32>, Vec<f32>), E> {
    let count = points.len() - usize::from(!periodic);
    let mut lengths = Vec::with_capacity(count);
    let mut ts = Vec::with_capacity(count + 1);
    ts.push(0.0);
    for i in 0..count {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        // Use the same geometric norm for knot spacing and spline coefficients.
        let length = match N {
            2 => (Vec2::new(b[0], b[1]) - Vec2::new(a[0], a[1])).length(),
            3 => (Vec3::new(b[0], b[1], b[2]) - Vec3::new(a[0], a[1], a[2])).length(),
            _ => unreachable!("only 2D and 3D game splines are constructed"),
        };
        let previous_t = *ts.last().unwrap();
        let next_t = previous_t + length;
        check(i, length, previous_t, next_t)?;
        lengths.push(length);
        ts.push(next_t);
    }
    Ok((ts, lengths))
}
fn quadratic_coefficients(
    values: &[f32],
    ts: &[f32],
    lengths: &[f32],
    start: Option<f32>,
    end: Option<f32>,
) -> Vec<f32> {
    let n = values.len();
    let mut upper = vec![0.0; n];
    let mut rhs = vec![0.0; n];
    if let Some(tangent) = start {
        upper[0] = 0.5;
        rhs[0] = (((values[1] - values[0]) * 3.0) * (1.0 / lengths[0]) - tangent * 3.0)
            / (lengths[0] + lengths[0]);
    }
    for i in 1..n - 1 {
        let diagonal = 2.0 * (ts[i + 1] - ts[i - 1]) - upper[i - 1] * lengths[i - 1];
        upper[i] = lengths[i] / diagonal;
        // Preserve operation order to minimize changes from rounding.
        let difference = ((values[i + 1] - values[i]) * 3.0) * (1.0 / lengths[i])
            - ((values[i] - values[i - 1]) * 3.0) * (1.0 / lengths[i - 1]);
        rhs[i] = (difference - lengths[i - 1] * rhs[i - 1]) / diagonal;
    }
    if let Some(tangent) = end {
        let h = lengths[n - 2];
        rhs[n - 1] =
            (tangent * 3.0 - ((values[n - 1] - values[n - 2]) * 3.0) * (1.0 / h) - h * rhs[n - 2])
                / ((2.0 - upper[n - 2]) * h);
    }
    for i in (0..n - 1).rev() {
        rhs[i] -= upper[i] * rhs[i + 1];
    }
    rhs
}
fn tridiagonal(diagonal: &[f32], lengths: &[f32], mut rhs: Vec<f32>) -> Vec<f32> {
    let n = diagonal.len();
    let mut upper = vec![0.0; n];
    upper[0] = lengths[0] / diagonal[0];
    rhs[0] /= diagonal[0];
    for i in 1..n {
        let divisor = diagonal[i] - lengths[i - 1] * upper[i - 1];
        if i + 1 < n {
            upper[i] = lengths[i] / divisor;
        }
        rhs[i] = (rhs[i] - lengths[i - 1] * rhs[i - 1]) / divisor;
    }
    for i in (0..n - 1).rev() {
        rhs[i] -= upper[i] * rhs[i + 1];
    }
    rhs
}
fn periodic_quadratic_coefficients(values: &[f32], lengths: &[f32]) -> Vec<f32> {
    let n = values.len();
    let mut diagonal = vec![0.0; n];
    let mut rhs = vec![0.0; n];
    for i in 0..n {
        let previous = (i + n - 1) % n;
        let next = (i + 1) % n;
        diagonal[i] = 2.0 * (lengths[previous] + lengths[i]);
        rhs[i] = ((values[next] - values[i]) * 3.0) * (1.0 / lengths[i])
            - ((values[i] - values[previous]) * 3.0) * (1.0 / lengths[previous]);
    }
    // Cyclic boundary couples first and last derivatives. Eliminate that one
    // coupling with a rank-one correction; both solves remain linear in n.
    let gamma = -diagonal[0];
    let corner = lengths[n - 1];
    diagonal[0] -= gamma;
    diagonal[n - 1] -= corner * corner / gamma;
    let mut result = tridiagonal(&diagonal, lengths, rhs);
    let mut correction = vec![0.0; n];
    correction[0] = gamma;
    correction[n - 1] = corner;
    let correction = tridiagonal(&diagonal, lengths, correction);
    let factor = (result[0] + corner * result[n - 1] / gamma)
        / (1.0 + correction[0] + corner * correction[n - 1] / gamma);
    for (v, correction) in result.iter_mut().zip(correction) {
        *v -= factor * correction;
    }
    result
}
fn coefficients<const N: usize>(
    points: &[[f32; N]],
    lengths: &[f32],
    ts: &[f32],
    start: Option<[f32; N]>,
    end: Option<[f32; N]>,
    periodic: bool,
) -> Vec<[[f32; N]; 4]> {
    let quadratic: [Vec<f32>; N] = std::array::from_fn(|axis| {
        let values: Vec<_> = points.iter().map(|v| v[axis]).collect();
        if periodic {
            periodic_quadratic_coefficients(&values, lengths)
        } else {
            quadratic_coefficients(
                &values,
                ts,
                lengths,
                start.map(|v| v[axis]),
                end.map(|v| v[axis]),
            )
        }
    });
    lengths
        .iter()
        .enumerate()
        .map(|(i, &h)| {
            let next = (i + 1) % points.len();
            [
                points[i],
                std::array::from_fn(|axis| {
                    (points[next][axis] - points[i][axis]) * (1.0 / h)
                        - h * (2.0 * quadratic[axis][i] + quadratic[axis][next]) / 3.0
                }),
                std::array::from_fn(|axis| quadratic[axis][i]),
                std::array::from_fn(|axis| {
                    (quadratic[axis][next] - quadratic[axis][i]) / (3.0 * h)
                }),
            ]
        })
        .collect()
}
struct SplineData<const N: usize> {
    knots: Vec<f32>,
    coefficients: Vec<[[f32; N]; 4]>,
}

fn build_spline<const N: usize>(
    points: &[[f32; N]],
    start: Option<[f32; N]>,
    end: Option<[f32; N]>,
    periodic: bool,
) -> Result<SplineData<N>, SplineError> {
    let required = if periodic { 3 } else { 2 };
    if points.len() < required {
        return Err(SplineError::TooFewPoints {
            required,
            actual: points.len(),
        });
    }
    if let Some(index) = points
        .iter()
        .position(|point| point.iter().any(|value| !value.is_finite()))
    {
        return Err(SplineError::NonFinitePoint(index));
    }
    if start
        .iter()
        .chain(end.iter())
        .flatten()
        .any(|value| !value.is_finite())
    {
        return Err(SplineError::NonFiniteTangent);
    }
    let (knots, lengths) = knots(points, periodic, |i, length, previous_t, next_t| {
        if !length.is_finite()
            || length <= 0.0
            || !next_t.is_finite()
            || next_t <= previous_t
        {
            return Err(SplineError::InvalidSegment(i));
        }
        Ok(())
    })?;
    let coefficients = coefficients(points, &lengths, &knots, start, end, periodic);
    if coefficients
        .iter()
        .flatten()
        .flatten()
        .any(|value| !value.is_finite())
    {
        return Err(SplineError::NonFiniteCoefficients);
    }
    Ok(SplineData {
        knots,
        coefficients,
    })
}

// B 140c926b0/140c930f0：最小点数由原入口先断言；零弦长和非有限值
// 按同一求解内核传播，最终交给楼梯请求的 Curve2::try_new_from_points 判定。
// 泛型回调可单态化，不为原版路径增加运行时校验或另建一套样条算法。
fn native_spline_2d_data(
    points: &[Vec2],
    start: Option<Vec2>,
    end: Option<Vec2>,
    periodic: bool,
) -> SplineData<2> {
    let points: Vec<_> = points.iter().map(|point| point.to_array()).collect();
    let (knots, lengths) = knots(&points, periodic, |_, _, _, _| {
        Ok::<(), std::convert::Infallible>(())
    })
    .unwrap_or_else(|never| match never {});
    let coefficients = coefficients(
        &points,
        &lengths,
        &knots,
        start.map(|v| v.to_array()),
        end.map(|v| v.to_array()),
        periodic,
    );
    SplineData {
        knots,
        coefficients,
    }
}

impl Spline2D {
    /// Sample at approximately half-unit spacing, including both endpoints.
    /// The end tangent faces back into the segment.
    pub fn densely_sample_segment(&self, index: usize) -> DenseSplineSegment2D {
        let segment = self.segments[index];
        let end_t = self
            .segments
            .get(index + 1)
            .map_or(self.max_t, |s| s.start_t);
        let span = end_t - segment.start_t;
        // B 140809b2f..b84：先 ceil，再按 Rust 饱和转换，最后至少采两点。
        // 原流程让零/负/NaN span 继续采样，超大数量由真实 Vec 容量/分配失败处理。
        let count = ((span + span).ceil() as usize).max(2);
        let step = span / (count - 1) as f32;
        DenseSplineSegment2D {
            points: (0..count)
                .map(|i| segment.position_at_local_t(i as f32 * step))
                .collect(),
            start_tangent: segment.tangent_at_local_t(0.0),
            end_tangent: -segment.tangent_at_local_t(span),
        }
    }
    /// Construct an open spline. Adjacent control points must be distinct.
    pub fn try_from_control_points_with_tangents(
        points: &[Vec2],
        start: Option<Vec2>,
        end: Option<Vec2>,
    ) -> Result<Self, SplineError> {
        let points: Vec<_> = points.iter().map(|point| point.to_array()).collect();
        let data = build_spline(
            &points,
            start.map(|v| v.to_array()),
            end.map(|v| v.to_array()),
            false,
        )?;
        Ok(Self::from_data(data, false))
    }

    /// 原版二维入口只要求至少两个控制点；退化输入保留真实求解结果。
    pub fn from_control_points_with_tangents(
        points: &[Vec2],
        start: Option<Vec2>,
        end: Option<Vec2>,
    ) -> Self {
        assert!(points.len() >= 2);
        Self::from_data(native_spline_2d_data(points, start, end, false), false)
    }

    /// Construct a loop without a repeated closing point.
    pub fn try_from_periodic_loop(points: &[Vec2]) -> Result<Self, SplineError> {
        let points: Vec<_> = points.iter().map(|point| point.to_array()).collect();
        let data = build_spline(&points, None, None, true)?;
        Ok(Self::from_data(data, true))
    }

    /// 原版周期入口要求至少三个点，闭合边的零长度同样进入周期求解。
    pub fn from_periodic_loop(points: &[Vec2]) -> Self {
        assert!(points.len() >= 3);
        Self::from_data(native_spline_2d_data(points, None, None, true), true)
    }

    fn from_data(data: SplineData<2>, periodic: bool) -> Self {
        let max_t = *data.knots.last().expect("validated knot sequence");
        let segments = data
            .coefficients
            .into_iter()
            .zip(data.knots)
            .map(|(coefficients, start_t)| SplineSegment2D {
                start_t,
                coefficients: coefficients.map(Vec2::from_array),
            })
            .collect();
        Self {
            segments,
            min_t: 0.0,
            max_t,
            periodic,
        }
    }
}

impl Spline3D {
    /// Construct an open spline. Adjacent control points must be distinct.
    pub fn try_from_control_points_with_tangents(
        points: &[Vec3],
        start: Option<Vec3>,
        end: Option<Vec3>,
    ) -> Result<Self, SplineError> {
        let points: Vec<_> = points.iter().map(|point| point.to_array()).collect();
        let data = build_spline(
            &points,
            start.map(|v| v.to_array()),
            end.map(|v| v.to_array()),
            false,
        )?;
        Ok(Self::from_data(data, false))
    }

    /// Construct an open spline, panicking if the input cannot define finite coefficients.
    pub fn from_control_points_with_tangents(
        points: &[Vec3],
        start: Option<Vec3>,
        end: Option<Vec3>,
    ) -> Self {
        Self::try_from_control_points_with_tangents(points, start, end)
            .expect("invalid spline control points")
    }

    /// Construct a loop without a repeated closing point.
    pub fn try_from_periodic_loop(points: &[Vec3]) -> Result<Self, SplineError> {
        let points: Vec<_> = points.iter().map(|point| point.to_array()).collect();
        let data = build_spline(&points, None, None, true)?;
        Ok(Self::from_data(data, true))
    }

    pub fn from_periodic_loop(points: &[Vec3]) -> Self {
        Self::try_from_periodic_loop(points).expect("invalid periodic spline control points")
    }

    fn from_data(data: SplineData<3>, periodic: bool) -> Self {
        let max_t = *data.knots.last().expect("validated knot sequence");
        let segments = data
            .coefficients
            .into_iter()
            .zip(data.knots)
            .map(|(coefficients, start_t)| SplineSegment3D {
                start_t,
                coefficients: coefficients.map(Vec3::from_array),
            })
            .collect();
        Self {
            segments,
            min_t: 0.0,
            max_t,
            periodic,
        }
    }
    pub fn find_segment_and_dt(&self, t: f32) -> (SplineSegment3D, f32) {
        let (index, dt) =
            select_scalar_knot(&self.segments, self.min_t, self.max_t, t, |s| s.start_t);
        (self.segments[index], dt)
    }
    pub fn position_at_t(&self, t: f32) -> Vec3 {
        let (segment, t) = self.find_segment_and_dt(t);
        segment.position_at_local_t(t)
    }
    pub fn tangent_at_t(&self, t: f32) -> Vec3 {
        let (segment, t) = self.find_segment_and_dt(t);
        segment.tangent_at_local_t(t)
    }
    pub fn to_owned(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
#[allow(clippy::type_complexity)]
#[path = "natural_cubic/native_tests.rs"]
mod native_oracle;

#[cfg(test)]
mod native_domain_tests {
    use super::*;
    use crate::curve::{Curve2, CurveError};

    #[test]
    fn repeated_control_points_reach_the_real_curve_failure_boundary() {
        let points = [Vec2::new(3.0, -2.0); 2];
        assert_eq!(
            Spline2D::try_from_control_points_with_tangents(&points, None, None).unwrap_err(),
            SplineError::InvalidSegment(0)
        );
        let spline = Spline2D::from_control_points_with_tangents(&points, None, None);
        assert_eq!(spline.segments.len(), 1);
        assert_eq!(spline.max_t, 0.0);
        let sampled = spline.densely_sample_segment(0);
        assert_eq!(sampled.points.len(), 2);
        assert!(sampled.points.iter().all(|point| point.is_nan()));
        assert_eq!(
            Curve2::try_new_from_points(sampled.points, true).unwrap_err(),
            CurveError::NonFiniteLength
        );
    }

    #[test]
    fn repeated_closing_point_reaches_periodic_solver_and_curve_consumer() {
        let points = [Vec2::ZERO, Vec2::X, Vec2::ZERO];
        assert_eq!(
            Spline2D::try_from_periodic_loop(&points).unwrap_err(),
            SplineError::InvalidSegment(2)
        );
        let spline = Spline2D::from_periodic_loop(&points);
        assert!(spline.periodic);
        assert_eq!(spline.segments.len(), 3);
        assert_eq!(spline.max_t, 2.0);
        let sampled = spline.densely_sample_segment(2);
        assert_eq!(sampled.points.len(), 2);
        assert_eq!(
            Curve2::try_new_from_points(sampled.points, true).unwrap_err(),
            CurveError::NonFiniteLength
        );
    }

    #[test]
    fn native_entry_preserves_nonfinite_data_for_the_existing_consumer() {
        let points = [Vec2::ZERO, Vec2::splat(f32::NAN)];
        assert_eq!(
            Spline2D::try_from_control_points_with_tangents(&points, None, None).unwrap_err(),
            SplineError::NonFinitePoint(1)
        );
        let spline = Spline2D::from_control_points_with_tangents(&points, None, None);
        assert!(spline.max_t.is_nan());
        let sampled = spline.densely_sample_segment(0);
        assert_eq!(sampled.points.len(), 2);
        assert_eq!(
            Curve2::try_new_from_points(sampled.points, true).unwrap_err(),
            CurveError::NonFiniteLength
        );

        let points = [Vec2::ZERO, Vec2::X];
        for tangent in [f32::INFINITY, f32::MAX] {
            let start = Some(Vec2::splat(tangent));
            let expected_error = if tangent.is_finite() {
                SplineError::NonFiniteCoefficients
            } else {
                SplineError::NonFiniteTangent
            };
            assert_eq!(
                Spline2D::try_from_control_points_with_tangents(&points, start, None).unwrap_err(),
                expected_error
            );
            let spline = Spline2D::from_control_points_with_tangents(&points, start, None);
            let sampled = spline.densely_sample_segment(0);
            assert_eq!(sampled.points.len(), 2);
            assert_eq!(
                Curve2::try_new_from_points(sampled.points, true).unwrap_err(),
                CurveError::NonFiniteLength
            );
        }
    }

    fn segment_with_span(span: f32) -> Spline2D {
        Spline2D {
            segments: vec![SplineSegment2D {
                start_t: 0.0,
                coefficients: [
                    Vec2::new(2.0, -3.0),
                    Vec2::new(1.0, 4.0),
                    Vec2::ZERO,
                    Vec2::ZERO,
                ],
            }],
            min_t: 0.0,
            max_t: span,
            periodic: false,
        }
    }

    #[test]
    fn dense_zero_negative_and_nan_spans_keep_two_samples_and_outgoing_tangents() {
        let zero = segment_with_span(0.0).densely_sample_segment(0);
        assert_eq!(zero.points, vec![Vec2::new(2.0, -3.0); 2]);
        assert_eq!(zero.start_tangent, Vec2::new(1.0, 4.0));
        assert_eq!(zero.end_tangent, Vec2::new(-1.0, -4.0));
        assert_eq!(
            Curve2::try_new_from_points(zero.points, true).unwrap_err(),
            CurveError::NonFiniteLength
        );

        let reversed = segment_with_span(-1.0).densely_sample_segment(0);
        assert_eq!(
            reversed.points,
            vec![Vec2::new(2.0, -3.0), Vec2::new(1.0, -7.0)]
        );
        assert_eq!(reversed.start_tangent, Vec2::new(1.0, 4.0));
        assert_eq!(reversed.end_tangent, Vec2::new(-1.0, -4.0));
        for span in [f32::NAN, f32::NEG_INFINITY] {
            let sampled = segment_with_span(span).densely_sample_segment(0);
            assert_eq!(sampled.points.len(), 2);
            assert!(sampled.points.iter().all(|point| point.is_nan()));
            assert_eq!(sampled.start_tangent, Vec2::new(1.0, 4.0));
            assert!(sampled.end_tangent.is_nan());
        }
    }

    #[test]
    fn infinite_dense_count_reaches_real_vec_capacity_failure_before_sampling() {
        // usize::MAX 个 Vec2 必定先溢出布局容量，不会向分配器申请巨量内存。
        let panic =
            std::panic::catch_unwind(|| segment_with_span(f32::INFINITY).densely_sample_segment(0))
                .unwrap_err();
        let message = panic
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| panic.downcast_ref::<String>().map(String::as_str))
            .unwrap_or("");
        assert!(message.contains("capacity overflow"), "{message}");
    }

    #[test]
    #[should_panic(expected = "assertion failed: points.len() >= 2")]
    fn native_open_checks_original_minimum_before_point_data() {
        Spline2D::from_control_points_with_tangents(&[Vec2::splat(f32::NAN)], None, None);
    }

    #[test]
    #[should_panic(expected = "assertion failed: points.len() >= 3")]
    fn native_periodic_checks_original_minimum_before_point_data() {
        Spline2D::from_periodic_loop(&[Vec2::splat(f32::NAN), Vec2::X]);
    }
}
