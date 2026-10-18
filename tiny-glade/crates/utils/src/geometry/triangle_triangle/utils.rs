use glam::DVec3;

/// Unit normal and the longest edge. Relative arithmetic avoids the cancellation
/// in a homogeneous determinant when triangles are far from the origin.
pub(super) fn plane(triangle: [DVec3; 3]) -> Option<(DVec3, f64)> {
    let edges = [
        triangle[1] - triangle[0],
        triangle[2] - triangle[0],
        triangle[2] - triangle[1],
    ];
    let scale = edges.iter().map(|edge| edge.length()).fold(0.0, f64::max);
    let normal = edges[0].cross(edges[1]);
    let length = normal.length();
    if length == 0.0 {
        None
    } else {
        Some((normal / length, scale))
    }
}

/// Intersect a triangle with a plane, retaining vertices on the plane. There are
/// at most three distinct candidates; fixed storage keeps this query allocation free.
pub(super) fn plane_slice(
    triangle: [DVec3; 3],
    distances: [f64; 3],
    axis: DVec3,
) -> Option<[DVec3; 2]> {
    let mut points = [DVec3::ZERO; 3];
    let mut count = 0;
    for index in 0..3 {
        let next = (index + 1) % 3;
        let distance = distances[index];
        if distance == 0.0 {
            points[count] = triangle[index];
            count += 1;
        } else if (distance < 0.0 && distances[next] > 0.0)
            || (distance > 0.0 && distances[next] < 0.0)
        {
            let t = distance / (distance - distances[next]);
            points[count] = triangle[index] + (triangle[next] - triangle[index]) * t;
            count += 1;
        }
    }
    let first = *points.first().filter(|_| count > 0)?;
    // Compare in the section's own frame. A far-away triangle vertex can erase
    // a tiny but representable section when subtracted from both endpoints.
    let mut bounds = [(0.0, first); 2];
    for &point in &points[1..count] {
        let coordinate = (point - first).dot(axis);
        if coordinate < bounds[0].0 {
            bounds[0] = (coordinate, point);
        }
        if coordinate > bounds[1].0 {
            bounds[1] = (coordinate, point);
        }
    }
    Some(bounds.map(|(_, point)| point))
}

/// Signed distances with a roundoff bound for the scalar triple product. The
/// bound depends on the actual products, so a short but representable altitude
/// is not erased merely because another edge or triangle is much larger.
pub(super) fn plane_distances(triangle: [DVec3; 3], supporting: [DVec3; 3]) -> [f64; 3] {
    let edge_a = supporting[1] - supporting[0];
    let edge_b = supporting[2] - supporting[0];
    let normal = edge_a.cross(edge_b);
    let length = normal.length();
    let absolute_a = edge_a.abs();
    let absolute_b = edge_b.abs();
    let product_bounds = DVec3::new(
        absolute_a.y * absolute_b.z + absolute_a.z * absolute_b.y,
        absolute_a.z * absolute_b.x + absolute_a.x * absolute_b.z,
        absolute_a.x * absolute_b.y + absolute_a.y * absolute_b.x,
    );
    triangle.map(|point| {
        let delta = point - supporting[0];
        let determinant = normal.dot(delta);
        let error = 16.0 * f64::EPSILON * product_bounds.dot(delta.abs());
        if determinant.abs() <= error {
            0.0
        } else {
            determinant / length
        }
    })
}
