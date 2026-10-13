use glam::{Vec2, Vec3};
/// B inlinee 0x12a38: first triangle wins; NaN weights are not rejected by an
/// invented finite guard. Indices refer to input triangles, not vertex indices.
pub fn get_triangle_coords_from_mesh_uv(
    uv_triangles: &[[Vec2; 3]],
    uv: Vec2,
) -> Option<(usize, Vec3)> {
    for (index, &[a, b, c]) in uv_triangles.iter().enumerate() {
        let weights = super::barycentric::barycentric2(uv, a, b, c);
        if weights.z < 0.0 || weights.y < 0.0 || weights.x < 0.0 {
            continue;
        }
        return Some((index, weights));
    }
    None
}
/// B 0x1416f34a0 visits UV triangles in input order and returns first match.
/// UV and position triangle arrays are parallel; degenerates preserve IEEE NaNs.
pub fn get_position_from_mesh_uv(
    position_triangles: &[[Vec3; 3]],
    uv_triangles: &[[Vec2; 3]],
    uv: Vec2,
) -> Option<Vec3> {
    let (index, weights) = get_triangle_coords_from_mesh_uv(uv_triangles, uv)?;
    let p = position_triangles[index];
    Some(p[1] * weights.y + p[0] * weights.x + p[2] * weights.z)
}
