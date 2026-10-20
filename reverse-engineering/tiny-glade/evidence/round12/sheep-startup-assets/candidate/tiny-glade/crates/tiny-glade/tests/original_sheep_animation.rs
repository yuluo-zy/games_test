//! Original instruction goldens, using already compiled production libraries.
use country_core::{
    render::mesh::{ATTR_NORMAL, ATTR_POSITION, Mesh, VertexAttributeValues},
    startup::sheep_assets::build_sheep_animation,
};
use glam::Vec3;
fn vectors(k: u32) -> Vec<Vec3> {
    vec![
        Vec3::new(
            f32::from_bits(0x3f800000 + k),
            f32::from_bits(0x80000000),
            f32::from_bits(0x7fc01234),
        ),
        Vec3::new(f32::NEG_INFINITY, 0.0, f32::INFINITY),
    ]
}
fn mesh(positions: Vec<Vec3>, normals: Vec<Vec3>) -> Mesh {
    let mut m = Mesh::new("original-sheep-fixture");
    m.set_attribute(ATTR_POSITION, VertexAttributeValues::Float3(positions));
    m.set_attribute(ATTR_NORMAL, VertexAttributeValues::Float3(normals));
    m
}
fn frames() -> [Mesh; 30] {
    std::array::from_fn(|i| mesh(vectors(i as u32), vectors(60 + i as u32)))
}
#[test]
fn original_992b_records_preserve_every_frame_channel_and_special_float_payload() {
    let base = mesh(vectors(0), vectors(0));
    let frames = frames();
    let delighted = mesh(vectors(90), vectors(91));
    let rows = build_sheep_animation(&base, &frames, &delighted);
    assert_eq!(rows.len(), 2);
    assert_eq!(
        bytemuck::cast_slice::<_, u8>(&rows),
        include_bytes!("fixtures/sheep_animation_synthetic.original.bin")
    );
}
#[test]
fn original_missing_wrong_type_short_and_zero_vertex_policies() {
    let cases: Vec<serde_json::Value> =
        serde_json::from_str(include_str!("fixtures/sheep_animation_original_cases.json")).unwrap();
    for case in cases {
        let name = case["name"].as_str().unwrap();
        let count = case["vertex_count"].as_u64().unwrap() as usize;
        let mut base = mesh(vectors(0), vectors(0));
        // Base variant is deliberately Float2: original only takes its array
        // length. Each requested frame/delighted attribute has a Float3 guard.
        base.set_attribute(
            ATTR_POSITION,
            VertexAttributeValues::Float2(vec![glam::Vec2::ZERO; count]),
        );
        let mut frames = frames();
        let mut delighted = mesh(vectors(90), vectors(91));
        if let Some(variant) = case["variant"].as_object() {
            if let Some(index) = variant.get("frame").and_then(|v| v.as_u64()) {
                let m = &mut frames[index as usize];
                if let Some(attr) = variant.get("missing").and_then(|v| v.as_str()) {
                    m.attributes.remove(attr);
                }
                if let Some(attr) = variant.get("wrong").and_then(|v| v.as_str()) {
                    m.set_attribute(
                        attr,
                        VertexAttributeValues::Float2(vec![glam::Vec2::ONE; 2]),
                    );
                }
            }
            if let Some(attr) = variant.get("delighted_missing").and_then(|v| v.as_str()) {
                delighted.attributes.remove(attr);
            }
        }
        if name.starts_with("short-frame-") {
            let frame = name.rsplit('-').next().unwrap().parse::<usize>().unwrap();
            frames[frame].set_attribute(
                ATTR_POSITION,
                VertexAttributeValues::Float3(vectors(frame as u32)[..1].to_vec()),
            );
        }
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            build_sheep_animation(&base, &frames, &delighted)
        }));
        assert_eq!(result.is_err(), !case["panic"].is_null(), "{name}");
        if let Ok(rows) = result {
            assert_eq!(
                rows.len(),
                case["native_completed_rows"].as_u64().unwrap() as usize,
                "{name}"
            );
        }
    }
}
#[test]
fn longer_frame_data_is_ignored_beyond_the_base_vertex_count() {
    let mut base = mesh(vectors(0), vectors(0));
    base.set_attribute(
        ATTR_POSITION,
        VertexAttributeValues::Float3(vectors(0)[..1].to_vec()),
    );
    let rows = build_sheep_animation(&base, &frames(), &mesh(vectors(90), vectors(91)));
    assert_eq!(
        bytemuck::cast_slice::<_, u8>(&rows),
        &include_bytes!("fixtures/sheep_animation_synthetic.original.bin")[..992]
    );
}
#[test]
#[ignore = "run the compiled test executable directly from the original distribution directory"]
fn original_all_860_asset_vertices_match_native_bytes() {
    let read = |name: &str| {
        country_core::utils::load_json::try_load_json_as_mesh(&format!(
            "assets/meshes/sheep_animation/{name}.json"
        ))
        .unwrap()
    };
    let base = read("1");
    let delighted = read("delighted");
    let frames: [Mesh; 30] = std::array::from_fn(|i| read(&(i + 1).to_string()));
    let rows = build_sheep_animation(&base, &frames, &delighted);
    assert_eq!(rows.len(), 860);
    assert_eq!(
        bytemuck::cast_slice::<_, u8>(&rows),
        include_bytes!("fixtures/sheep_animation_860.original.bin")
    );
}
