use super::{TriangleTriangleIntersection, cross_intersect::cross_intersect};
use glam::Vec3;

/// Intersect two nondegenerate triangles, including their boundaries.
///
/// `Coplanar` describes their supporting planes; the finite triangles need not
/// overlap. Invalid or zero-area triangles return `Disjoint`. Coplanarity requires
/// unit-normal cross-product length at most `1e-10` and each triangle's plane distances within
/// `1e-10` of its own longest edge. Noncoplanar sections use arithmetic roundoff
/// bounds instead of this geometric tolerance, retaining short intersections.
pub fn triangles_intersect(a: [Vec3; 3], b: [Vec3; 3]) -> TriangleTriangleIntersection {
    cross_intersect(a, b)
}
