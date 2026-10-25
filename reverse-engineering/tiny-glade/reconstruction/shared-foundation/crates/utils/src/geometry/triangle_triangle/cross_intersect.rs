use super::{
    utils::{canonicalize, determinant, edge_plane_intersection, orient, side, PLANE_EPS},
    TriangleTriangleIntersection,
};
use glam::Vec3;
/// B chooses two edges after tolerant tetrahedron-side classification.
/// The tolerance selects edges; it does not snap nearby vertices onto a plane
/// or clamp edge parameters. That distinction matters for small triangles.
pub fn cross_intersect(a: [Vec3; 3], b: [Vec3; 3]) -> TriangleTriangleIntersection {
    let sa = a.map(|p| side(b, p));
    let sb = b.map(|p| side(a, p));
    if sb[0] == sb[1] && sb[0] == sb[2] {
        return TriangleTriangleIntersection::Disjoint;
    }
    let mut a = canonicalize(a, sa);
    let mut b = canonicalize(b, sb);
    orient(&mut b, a[0]);
    orient(&mut a, b[0]);
    if determinant([a[0], a[1], b[0], b[1]]) > PLANE_EPS
        || determinant([a[0], a[2], b[2], b[0]]) > PLANE_EPS
    {
        return TriangleTriangleIntersection::Disjoint;
    }
    let first = if determinant([a[0], a[2], b[1], b[0]]) <= PLANE_EPS {
        edge_plane_intersection(b[0], b[1], a)
    } else {
        edge_plane_intersection(a[0], a[2], b)
    };
    let last = if determinant([a[0], a[1], b[2], b[0]]) > PLANE_EPS {
        edge_plane_intersection(b[0], b[2], a)
    } else {
        edge_plane_intersection(a[0], a[1], b)
    };
    if !first.is_finite() || !last.is_finite() {
        return TriangleTriangleIntersection::Disjoint;
    }
    let d = first - last;
    if d.x * d.x + d.y * d.y + d.z * d.z < 1e-20 {
        TriangleTriangleIntersection::Point(first)
    } else {
        TriangleTriangleIntersection::Segment(first, last)
    }
}
