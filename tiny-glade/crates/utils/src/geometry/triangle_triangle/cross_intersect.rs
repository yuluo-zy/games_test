use super::{
    TriangleTriangleIntersection,
    utils::{plane, plane_distances, plane_slice},
};
use glam::Vec3;

/// Intersect the closed plane sections of two triangles without depending on
/// vertex winding. Double precision protects small triangles and large offsets;
/// the public coordinates and result remain single precision.
pub fn cross_intersect(a: [Vec3; 3], b: [Vec3; 3]) -> TriangleTriangleIntersection {
    use TriangleTriangleIntersection::{Coplanar, Disjoint, Point, Segment};
    if a.iter().chain(&b).any(|point| !point.is_finite()) {
        return Disjoint;
    }
    let a = a.map(|point| point.as_dvec3());
    let b = b.map(|point| point.as_dvec3());
    let Some((normal_a, scale_a)) = plane(a) else {
        return Disjoint;
    };
    let Some((normal_b, scale_b)) = plane(b) else {
        return Disjoint;
    };
    let line = normal_a.cross(normal_b);
    let distances_a = plane_distances(a, b);
    let distances_b = plane_distances(b, a);
    // Each contour supplies its own tolerance. A huge opposing triangle must
    // not absorb a small triangle's entire cross section into its plane.
    if line.length_squared() <= 1e-20
        && distances_a
            .iter()
            .all(|distance| distance.abs() <= 1e-10 * scale_a)
        && distances_b
            .iter()
            .all(|distance| distance.abs() <= 1e-10 * scale_b)
    {
        return Coplanar;
    }
    if [distances_a, distances_b]
        .iter()
        .any(|distances| distances.iter().all(|&d| d > 0.0) || distances.iter().all(|&d| d < 0.0))
    {
        return Disjoint;
    }
    if line.length_squared() == 0.0 {
        return Disjoint;
    }
    let axis = line.normalize();
    let Some(section_a) = plane_slice(a, distances_a, axis) else {
        return Disjoint;
    };
    let Some(section_b) = plane_slice(b, distances_b, axis) else {
        return Disjoint;
    };
    // Project both intervals relative to the shorter section, rather than an
    // arbitrary input vertex. This retains tiny sections when the opposing
    // contour or its offset is many orders of magnitude larger.
    let origin = if section_a[0].distance_squared(section_a[1])
        <= section_b[0].distance_squared(section_b[1])
    {
        section_a[0]
    } else {
        section_b[0]
    };
    let section_a = section_a.map(|point| ((point - origin).dot(axis), point));
    let section_b = section_b.map(|point| ((point - origin).dot(axis), point));
    let start = if section_a[0].0 >= section_b[0].0 {
        section_a[0]
    } else {
        section_b[0]
    };
    let end = if section_a[1].0 <= section_b[1].0 {
        section_a[1]
    } else {
        section_b[1]
    };
    // Merge only differences within the rounding precision of the selected
    // endpoints; edge-length tolerances would collapse real, thin intersections.
    let interval_error = 32.0 * f64::EPSILON * start.0.abs().max(end.0.abs());
    if start.0 > end.0 + interval_error {
        Disjoint
    } else if end.0 - start.0 <= interval_error {
        Point(((start.1 + end.1) * 0.5).as_vec3())
    } else {
        Segment(start.1.as_vec3(), end.1.as_vec3())
    }
}
