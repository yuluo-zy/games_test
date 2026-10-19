//! Chord-length natural cubic splines with optional endpoint derivatives.
//! Fallible constructors reject degenerate knots before solving the system.
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
fn knots<const N: usize>(
    points: &[[f32; N]],
    periodic: bool,
) -> Result<(Vec<f32>, Vec<f32>), SplineError> {
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
        let next_t = ts.last().unwrap() + length;
        if !length.is_finite()
            || length <= 0.0
            || !next_t.is_finite()
            || next_t <= *ts.last().unwrap()
        {
            return Err(SplineError::InvalidSegment(i));
        }
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
    let (knots, lengths) = knots(points, periodic)?;
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
        assert!(
            span.is_finite() && span > 0.0,
            "invalid spline segment span"
        );
        let count = (span + span).ceil();
        assert!(
            count <= (1 << f32::MANTISSA_DIGITS) as f32,
            "too many spline samples"
        );
        let count = (count as usize).max(2);
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

    /// Construct an open spline, panicking if the input cannot define finite coefficients.
    pub fn from_control_points_with_tangents(
        points: &[Vec2],
        start: Option<Vec2>,
        end: Option<Vec2>,
    ) -> Self {
        Self::try_from_control_points_with_tangents(points, start, end)
            .expect("invalid spline control points")
    }

    /// Construct a loop without a repeated closing point.
    pub fn try_from_periodic_loop(points: &[Vec2]) -> Result<Self, SplineError> {
        let points: Vec<_> = points.iter().map(|point| point.to_array()).collect();
        let data = build_spline(&points, None, None, true)?;
        Ok(Self::from_data(data, true))
    }

    pub fn from_periodic_loop(points: &[Vec2]) -> Self {
        Self::try_from_periodic_loop(points).expect("invalid periodic spline control points")
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
