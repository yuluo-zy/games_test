//! Iterative polyline simplification with a finite-segment distance bound.
use glam::{Vec2, Vec3};
pub trait RdpPoint: Copy {
    fn as_vec3(self) -> Vec3;
}
impl RdpPoint for Vec3 {
    fn as_vec3(self) -> Vec3 {
        self
    }
}
impl RdpPoint for Vec2 {
    fn as_vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, 0.0)
    }
}
#[derive(Clone, Debug)]
pub struct Rdp<T> {
    pub indices: Vec<usize>,
    pub points: Vec<T>,
}
impl<T: RdpPoint> Rdp<T> {
    /// Simplify with a distance tolerance of 0.1 and no endpoint spacing.
    pub fn new(points: &[T]) -> Self {
        Self::new_with_opts(points, 0.1, 0.0)
    }

    /// Keep the old result through `at`, drop the new result's first seam point,
    /// then offset its remaining source indices by the old seam source index.
    /// The first point is dropped unconditionally; no distance-based dedup is
    /// introduced. Inputs must contain the seam and matching points/indices.
    pub fn stitch_with(&self, at: usize, other: &Self) -> Self {
        assert_eq!(
            self.points.len(),
            self.indices.len(),
            "invalid source indices"
        );
        assert_eq!(
            other.points.len(),
            other.indices.len(),
            "invalid source indices"
        );
        assert!(!other.points.is_empty(), "missing seam point");
        let mut points = self.points[..=at].to_vec();
        points.extend_from_slice(&other.points[1..]);
        let base = self.indices[at];
        let mut indices = self.indices[..=at].to_vec();
        indices.extend(other.indices[1..].iter().map(|&index| {
            index
                .checked_add(base)
                .expect("stitched source index overflow")
        }));
        Self { indices, points }
    }
    /// Every omitted vertex lies within `epsilon` of its retained finite segment.
    /// Prefer split points at least `min_endpoint_distance` from both endpoints;
    /// if none can resolve an error, the tolerance takes precedence over spacing.
    /// Both options and all input points must be finite; distances are nonnegative.
    pub fn new_with_opts(points: &[T], epsilon: f32, min_endpoint_distance: f32) -> Self {
        assert!(
            epsilon.is_finite() && epsilon >= 0.0,
            "invalid RDP tolerance"
        );
        assert!(
            min_endpoint_distance.is_finite() && min_endpoint_distance >= 0.0,
            "invalid RDP endpoint distance"
        );
        assert!(
            points.iter().all(|point| point.as_vec3().is_finite()),
            "nonfinite RDP point"
        );
        if points.len() < 3 {
            return Self {
                indices: (0..points.len()).collect(),
                points: points.to_vec(),
            };
        }
        // Double precision also handles finite f32 coordinates whose differences
        // or squared distances overflow f32. Compare squared distances throughout.
        let tolerance_squared = f64::from(epsilon).powi(2);
        let spacing_squared = f64::from(min_endpoint_distance).powi(2);
        let mut indices = vec![0];
        let mut pending = vec![(0, points.len() - 1)];
        while let Some((first, last)) = pending.pop() {
            let start = points[first].as_vec3().as_dvec3();
            let end = points[last].as_vec3().as_dvec3();
            let segment = end - start;
            let length_squared = segment.length_squared();
            let mut furthest = (0.0, first);
            let mut spaced = (0.0, first);
            for (offset, point) in points[first + 1..last].iter().enumerate() {
                let index = first + offset + 1;
                let point = point.as_vec3().as_dvec3();
                let from_start = point - start;
                let parameter = if length_squared == 0.0 {
                    0.0
                } else {
                    (from_start.dot(segment) / length_squared).clamp(0.0, 1.0)
                };
                let mut distance_squared = (from_start - segment * parameter).length_squared();
                if distance_squared <= tolerance_squared && point != start && point != end {
                    // Certify points we might omit. Translating very small points
                    // by large endpoints can lose their entire off-line component.
                    distance_squared = distance_squared
                        .max(segment_distance_upper_bound(point, start, end, parameter));
                }
                if distance_squared > furthest.0 {
                    furthest = (distance_squared, index);
                }
                if spacing_squared > 0.0
                    && distance_squared > spaced.0
                    && from_start.length_squared() >= spacing_squared
                    && (point - end).length_squared() >= spacing_squared
                {
                    spaced = (distance_squared, index);
                }
            }
            if furthest.0 > tolerance_squared {
                // Spacing is a preference, never permission to violate epsilon.
                let split = if spaced.0 > tolerance_squared {
                    spaced.1
                } else {
                    furthest.1
                };
                pending.push((split, last));
                pending.push((first, split));
            } else {
                indices.push(last);
            }
        }
        let simplified = indices.iter().map(|&i| points[i]).collect();
        Self {
            indices,
            points: simplified,
        }
    }
}

/// An upper bound on distance to a point on the exact finite segment.
///
/// The parameter need not be the exact closest-point parameter: every point on
/// the segment provides a valid upper bound. Error-free sum/difference transforms
/// and an FMA residual bound interpolation error without extra allocations. This
/// slower path runs only when the ordinary distance would permit omitting a point.
fn segment_distance_upper_bound(
    point: glam::DVec3,
    start: glam::DVec3,
    end: glam::DVec3,
    t: f64,
) -> f64 {
    // Exact axis-aligned membership requires no interpolation, including when
    // zero tolerance would otherwise conservatively retain a rounded quotient.
    let mut changed = (0..3).filter(|&axis| start[axis] != end[axis]);
    if let Some(axis) = changed.next()
        && changed.next().is_none()
        && point[axis] >= start[axis].min(end[axis])
        && point[axis] <= start[axis].max(end[axis])
        && (0..3).all(|other| other == axis || point[other] == start[other])
    {
        return 0.0;
    }
    fn sum_with_tail(a: f64, b: f64) -> (f64, f64) {
        let sum = a + b;
        let virtual_b = sum - a;
        let tail = (a - (sum - virtual_b)) + (b - virtual_b);
        (sum, tail)
    }
    fn up(value: f64) -> f64 {
        if value == 0.0 { 0.0 } else { value.next_up() }
    }
    let mut squared_bound = 0.0;
    for axis in 0..3 {
        let (delta, delta_tail) = sum_with_tail(end[axis], -start[axis]);
        let product = delta * t;
        let product_tail = delta.mul_add(t, -product);
        let (nearest, nearest_tail) = sum_with_tail(start[axis], product);
        let (residual, residual_tail) = sum_with_tail(point[axis], -nearest);
        let error = up(
            up(up(up(delta_tail.abs() * t) + product_tail.abs()) + nearest_tail.abs())
                + residual_tail.abs(),
        );
        let coordinate_bound = up(residual.abs() + error);
        squared_bound = up(squared_bound + up(coordinate_bound * coordinate_bound));
    }
    squared_bound
}

#[cfg(test)]
mod completion_tests {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/oracle/foundation-completion-2026-10-13/base-assets/rdp_tests.rs"
    ));
}

#[cfg(test)]
#[allow(clippy::type_complexity)]
#[path = "rdp/native_tests.rs"]
mod native_oracle;

#[cfg(test)]
fn assert_simplification<T: RdpPoint>(source: &[T], result: &Rdp<T>, epsilon: f32) {
    assert_eq!(result.points.len(), result.indices.len());
    assert!(result.indices.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(
        result.indices.first().copied(),
        (!source.is_empty()).then_some(0)
    );
    assert_eq!(result.indices.last().copied(), source.len().checked_sub(1));
    for (&index, point) in result.indices.iter().zip(&result.points) {
        assert_eq!(source[index].as_vec3(), point.as_vec3());
    }
    for pair in result.indices.windows(2) {
        let a = source[pair[0]].as_vec3().as_dvec3();
        let b = source[pair[1]].as_vec3().as_dvec3();
        for point in &source[pair[0]..=pair[1]] {
            let point = point.as_vec3().as_dvec3();
            let t = if a == b {
                0.0
            } else {
                ((point - a).dot(b - a) / (b - a).length_squared()).clamp(0.0, 1.0)
            };
            let error = (point - (a + (b - a) * t)).length();
            assert!(
                error <= f64::from(epsilon) + 1e-10,
                "error {error} exceeds epsilon {epsilon}"
            );
        }
    }
}
