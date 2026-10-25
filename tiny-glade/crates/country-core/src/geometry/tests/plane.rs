use super::*;

#[test]
fn corners_are_half_the_size_on_the_xz_plane() {
    let mesh = plane_mesh(4.0);
    let p = mesh.positions_without_indices();
    assert_eq!(p[0], Vec3::new(2.0, 0.0, -2.0));
    assert_eq!(p[1], Vec3::new(2.0, 0.0, 2.0));
    assert_eq!(p[2], Vec3::new(-2.0, 0.0, 2.0));
    assert_eq!(p[3], Vec3::new(-2.0, 0.0, -2.0));
    assert!(p.iter().all(|c| c.y == 0.0));
}

#[test]
fn normals_point_up_and_uvs_cover_the_unit_square() {
    let mesh = plane_mesh(1.0);
    assert!(mesh.normals_without_indices().iter().all(|n| *n == Vec3::Y));
    let uvs = mesh.uvs_without_indices();
    assert_eq!(uvs[0], Vec2::new(1.0, 1.0));
    assert_eq!(uvs[2], Vec2::new(0.0, 0.0));
    assert_eq!(mesh.label, "Plane");
}

#[test]
fn indices_make_two_upward_facing_triangles() {
    let mesh = plane_mesh(2.0);
    assert_eq!(mesh.indices, [0, 2, 1, 0, 3, 2]);
    let p = mesh.positions_without_indices();
    for tri in mesh.indices.chunks_exact(3) {
        let (a, b, c) = (p[tri[0] as usize], p[tri[1] as usize], p[tri[2] as usize]);
        assert!((b - a).cross(c - a).y > 0.0, "{tri:?}");
    }
}

#[test]
fn colours_are_white_per_vertex() {
    let mesh = plane_mesh(1.0);
    assert_eq!(mesh.attributes[ATTR_COLOR].array_length(), 4);
}

#[test]
fn zero_size_collapses_to_a_point_but_still_has_four_vertices() {
    let mesh = plane_mesh(0.0);
    assert_eq!(mesh.attributes[ATTR_POSITION].array_length(), 4);
}
