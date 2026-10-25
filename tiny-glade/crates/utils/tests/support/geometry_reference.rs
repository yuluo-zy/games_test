use glam::{DVec3, Vec3};

#[derive(Debug)]
pub enum ReferenceIntersection {
    Disjoint,
    Coplanar,
    Points(Vec<Vec3>),
}

/// Independent segment/triangle barycentric oracle. Unlike production, this
/// enumerates all six boundary edges instead of overlapping two plane sections.
pub fn triangle_reference(a: [Vec3; 3], b: [Vec3; 3]) -> ReferenceIntersection {
    use ReferenceIntersection::{Coplanar, Disjoint, Points};
    if a.iter().chain(&b).any(|p| !p.is_finite()) {
        return Disjoint;
    }
    let a = a.map(|point| point.as_dvec3());
    let b = b.map(|point| point.as_dvec3());
    let longest_edge = |t: [DVec3; 3]| {
        (0..3)
            .map(|i| (t[(i + 1) % 3] - t[i]).length())
            .fold(0.0, f64::max)
    };
    let scale_a = longest_edge(a);
    let scale_b = longest_edge(b);
    let normal_a = (a[1] - a[0]).cross(a[2] - a[0]);
    let normal_b = (b[1] - b[0]).cross(b[2] - b[0]);
    if normal_a.length() == 0.0 || normal_b.length() == 0.0 {
        return Disjoint;
    }
    let tolerance_a = 1e-10 * scale_a;
    let tolerance_b = 1e-10 * scale_b;
    if normal_a
        .normalize()
        .cross(normal_b.normalize())
        .length_squared()
        <= 1e-20
        && a.iter()
            .all(|&p| (p - b[0]).dot(normal_b).abs() <= tolerance_a * normal_b.length())
        && b.iter()
            .all(|&p| (p - a[0]).dot(normal_a).abs() <= tolerance_b * normal_a.length())
    {
        return Coplanar;
    }
    let mut hits = Vec::new();
    for (edges, triangle) in [(a, b), (b, a)] {
        let edge1 = triangle[1] - triangle[0];
        let edge2 = triangle[2] - triangle[0];
        for i in 0..3 {
            let origin = edges[i];
            let direction = edges[(i + 1) % 3] - origin;
            let p = direction.cross(edge2);
            let determinant = edge1.dot(p);
            if determinant == 0.0 {
                continue;
            }
            let delta = origin - triangle[0];
            let u = delta.dot(p) / determinant;
            let q = delta.cross(edge1);
            let v = direction.dot(q) / determinant;
            let t = edge2.dot(q) / determinant;
            let epsilon = 1e-9;
            if u >= -epsilon
                && v >= -epsilon
                && u + v <= 1.0 + epsilon
                && t >= -epsilon
                && t <= 1.0 + epsilon
            {
                hits.push((origin + direction * t.clamp(0.0, 1.0)).as_vec3());
            }
        }
    }
    if hits.is_empty() {
        Disjoint
    } else {
        Points(hits)
    }
}
