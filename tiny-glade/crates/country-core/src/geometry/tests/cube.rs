use super::*;

#[test]
fn reversed_bounds_keep_all_faces_outward() {
    let mesh = cube_mesh(&CubeBox {
        x: [2.0, -2.0],
        y: [-3.0, 3.0],
        z: [4.0, -4.0],
    });
    let positions = mesh.positions_without_indices();
    let normals = mesh.normals_without_indices();
    for triangle in mesh.indices.chunks_exact(3) {
        let [first, second, third] = [
            triangle[0] as usize,
            triangle[1] as usize,
            triangle[2] as usize,
        ];
        assert!(
            (positions[second] - positions[first])
                .cross(positions[third] - positions[first])
                .dot(normals[first])
                > 0.0
        );
    }
}

fn unit() -> CubeBox {
    CubeBox {
        x: [-1.0, 1.0],
        y: [-2.0, 2.0],
        z: [-3.0, 3.0],
    }
}

#[test]
fn has_24_vertices_36_indices_and_four_attributes() {
    let mesh = cube_mesh(&unit());
    assert_eq!(mesh.label, "Box");
    assert_eq!(mesh.indices.len(), 36);
    for name in [ATTR_POSITION, ATTR_NORMAL, ATTR_UV, ATTR_COLOR] {
        assert_eq!(mesh.attributes[name].array_length(), 24, "{name}");
    }
    assert!(mesh.indices.iter().all(|i| *i < 24));
}

#[test]
fn every_face_has_one_axis_aligned_unit_normal_and_matching_plane() {
    let mesh = cube_mesh(&unit());
    let positions = mesh.positions_without_indices();
    let normals = mesh.normals_without_indices();
    let bounds = [[-1.0, 1.0], [-2.0, 2.0], [-3.0, 3.0]];
    for face in 0..6 {
        let n = normals[face * 4];
        assert!(normals[face * 4..face * 4 + 4].iter().all(|m| *m == n));
        let axis = (0..3).find(|a| n[*a] != 0.0).unwrap();
        assert_eq!(n[axis].abs(), 1.0);
        let plane = if n[axis] > 0.0 {
            bounds[axis][1]
        } else {
            bounds[axis][0]
        };
        assert!(
            positions[face * 4..face * 4 + 4]
                .iter()
                .all(|p| p[axis] == plane)
        );
    }
}

#[test]
fn faces_are_two_triangles_over_their_own_four_vertices() {
    let mesh = cube_mesh(&unit());
    for face in 0..6u32 {
        let tri = &mesh.indices[face as usize * 6..face as usize * 6 + 6];
        assert!(tri.iter().all(|i| *i / 4 == face));
        assert_eq!(tri[0], tri[5]);
        assert_eq!(tri[2], tri[3]);
    }
}

#[test]
fn winding_follows_the_stored_normals() {
    let mesh = cube_mesh(&unit());
    let p = mesh.positions_without_indices();
    let n = mesh.normals_without_indices();
    for tri in mesh.indices.chunks_exact(3) {
        let (a, b, c) = (p[tri[0] as usize], p[tri[1] as usize], p[tri[2] as usize]);
        let geometric = (b - a).cross(c - a);
        assert!(geometric.dot(n[tri[0] as usize]) > 0.0, "{tri:?}");
    }
}

#[test]
fn uvs_cover_the_unit_square_on_every_face() {
    let mesh = cube_mesh(&unit());
    let uvs = mesh.uvs_without_indices();
    for face in 0..6 {
        let mut face_uvs: Vec<[i32; 2]> = uvs[face * 4..face * 4 + 4]
            .iter()
            .map(|u| [u.x as i32, u.y as i32])
            .collect();
        face_uvs.sort();
        assert_eq!(face_uvs, [[0, 0], [0, 1], [1, 0], [1, 1]]);
    }
}

#[test]
fn box_corners_come_from_the_selected_ranges() {
    let c = CubeBox {
        x: [10.0, 20.0],
        y: [30.0, 40.0],
        z: [50.0, 60.0],
    };
    let mesh = cube_mesh(&c);
    let p = mesh.positions_without_indices();
    assert_eq!(p[0], Vec3::new(10.0, 30.0, 60.0));
    assert_eq!(p[1], Vec3::new(20.0, 30.0, 60.0));
}
