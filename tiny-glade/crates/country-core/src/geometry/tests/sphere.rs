use super::*;

#[test]
fn invalid_radius_and_extreme_subdivisions_fail_before_generation() {
    for radius in [-1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert_eq!(
            Sphere::new(1, radius).to_mesh().unwrap_err(),
            SphereError::InvalidRadius
        );
    }
    assert_eq!(icosphere_vertex_count(usize::MAX), usize::MAX);
    assert!(Sphere::new(usize::MAX, 1.0).to_mesh().is_err());
    let uv = sphere_uv(Vec3::new(0.0, 1.0 + f32::EPSILON, 0.0));
    assert!(uv.into_iter().all(f32::is_finite));
}

#[test]
fn vertex_counts_follow_the_icosphere_formula() {
    assert_eq!(icosphere_vertex_count(0), 12);
    assert_eq!(icosphere_vertex_count(1), 42);
    assert_eq!(icosphere_vertex_count(79), 64_002);
    assert_eq!(icosphere_vertex_count(80), 65_612);
}

#[test]
fn limit_is_79_and_message_matches_the_recovered_strings() {
    assert!(Sphere::new(79, 1.0).to_mesh().is_ok());
    let err = Sphere::new(80, 1.0).to_mesh().unwrap_err();
    assert_eq!(
        err.to_string(),
        "Cannot create an icosphere of 80 subdivisions due to there being too many vertices being generated: 65612. (Limited to 65535 vertices or 79 subdivisions)"
    );
}

#[test]
fn subdivision_zero_is_the_plain_icosahedron() {
    let mesh = Sphere::new(0, 2.0).to_mesh().unwrap();
    assert_eq!(mesh.label, "Sphere");
    assert_eq!(mesh.indices.len(), 60);
    for name in [ATTR_POSITION, ATTR_NORMAL, ATTR_UV] {
        assert_eq!(mesh.attributes[name].array_length(), 12, "{name}");
    }
    assert_eq!(mesh.attributes.len(), 3, "no colour attribute");
}

#[test]
fn index_count_is_60_per_n_plus_one_squared() {
    for n in [1usize, 2, 5] {
        let mesh = Sphere::new(n, 1.0).to_mesh().unwrap();
        assert_eq!(mesh.indices.len(), 60 * (n + 1) * (n + 1));
        assert_eq!(
            mesh.attributes[ATTR_POSITION].array_length(),
            icosphere_vertex_count(n)
        );
        assert!(
            mesh.indices
                .iter()
                .all(|&i| (i as usize) < icosphere_vertex_count(n))
        );
    }
}

#[test]
fn positions_are_normals_scaled_by_the_radius() {
    let mesh = Sphere::new(2, 3.5).to_mesh().unwrap();
    let crate::render::mesh::VertexAttributeValues::Float3(p) = &mesh.attributes[ATTR_POSITION]
    else {
        panic!("positions are Float3")
    };
    let crate::render::mesh::VertexAttributeValues::Float3(n) = &mesh.attributes[ATTR_NORMAL]
    else {
        panic!("normals are Float3")
    };
    for (p, n) in p.iter().zip(n) {
        assert_eq!(*p, *n * 3.5);
        assert!((n.length() - 1.0).abs() < 1e-5);
    }
}

#[test]
fn uv_formula_matches_the_closure() {
    assert_eq!(sphere_uv(Vec3::Y), [0.5, 0.0]);
    // 南极点：`acos(-1.0f32)/PI` 在 libm 中比 f32 π 低 1 ulp（0.99999994），
    // 按浮点容差断言（文件头已注明与 Windows CRT 末位可能差 1 ulp）。
    let [u, v] = sphere_uv(Vec3::NEG_Y);
    assert!((u - 0.5).abs() < 1e-5, "u={u}");
    assert!((v - 1.0).abs() < 1e-5, "v={v}");
    let [u, v] = sphere_uv(Vec3::X);
    assert_eq!((u, v), (0.5, 0.5));
    let [u, _] = sphere_uv(Vec3::Z);
    assert!((u - 0.25).abs() < 1e-6);
}

#[test]
fn try_from_matches_to_mesh() {
    let a = Mesh::try_from(Sphere::new(1, 1.0)).unwrap();
    let b = Sphere::new(1, 1.0).to_mesh().unwrap();
    assert_eq!(a.indices, b.indices);
}
