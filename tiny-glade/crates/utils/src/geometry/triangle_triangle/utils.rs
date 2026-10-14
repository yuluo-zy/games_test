use glam::{Mat4, Vec3};
pub(crate) const PLANE_EPS: f32 = 1e-10;
pub(crate) fn degenerate(t: [Vec3; 3]) -> bool {
    let n = (t[0] - t[1]).cross(t[0] - t[2]);
    n.x.abs() < PLANE_EPS && n.y.abs() < PLANE_EPS && n.z.abs() < PLANE_EPS
}
/// Original calls glam Mat4::determinant; reuse its dependency implementation.
pub(crate) fn side(triangle: [Vec3; 3], point: Vec3) -> i8 {
    let d = determinant([triangle[0], triangle[1], triangle[2], point]);
    if d > PLANE_EPS {
        1
    } else if d < -PLANE_EPS {
        -1
    } else {
        0
    }
}
pub(crate) fn determinant(points: [Vec3; 4]) -> f32 {
    Mat4::from_cols(
        points[0].extend(1.0),
        points[1].extend(1.0),
        points[2].extend(1.0),
        points[3].extend(1.0),
    )
    .determinant()
}
/// Put the isolated side vertex first without changing winding. The original
/// has a separate rule for the (-1,0,+1) case: put the positive vertex first.
pub(crate) fn canonicalize(mut triangle: [Vec3; 3], signs: [i8; 3]) -> [Vec3; 3] {
    if signs[0] == signs[1] {
        triangle.rotate_right(1);
    } else if signs[0] == signs[2] {
        triangle.rotate_left(1);
    } else if signs[1] != signs[2] {
        if signs[1] > 0 {
            triangle.rotate_left(1);
        } else if signs[2] > 0 {
            triangle.rotate_right(1);
        }
    }
    triangle
}
/// B private helper 0x142418dc0 swaps vertices 1/2 when the opposite first
/// vertex is on the negative determinant side; glam computes the determinant.
pub(crate) fn orient(triangle: &mut [Vec3; 3], point: Vec3) {
    if side(*triangle, point) < 0 {
        triangle.swap(1, 2);
    }
}
pub(crate) fn edge_plane_intersection(a: Vec3, b: Vec3, plane: [Vec3; 3]) -> Vec3 {
    let n = (plane[1] - plane[0]).cross(plane[2] - plane[1]).normalize();
    let delta = b - a;
    delta * (-n.dot(a - plane[0]) / n.dot(delta)) + a
}
