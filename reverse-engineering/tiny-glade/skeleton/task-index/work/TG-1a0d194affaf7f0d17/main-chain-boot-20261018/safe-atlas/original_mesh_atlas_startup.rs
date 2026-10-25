//! B shipped inputs through the actual atlas producers; CPU asset registration.
//! No mock Mesh/empty success substitute and no optional skip on missing assets.
use country_core::{
    asset_libraries::mesh_library::AssetMeshLibrary,
    render::{mesh::VertexAttributeValues, nani::gpu_world::GpuWorldSubsetKind as Kind},
    resources::render::mesh_atlas_library::MeshAtlasLibrary,
    startup::load_mesh_atlases::{
        NaniWorldSubsetConfig, generate_procedural_meshes, load_nani_meshes,
    },
};
use glam::Vec3;
use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

fn original_installation() -> PathBuf {
    let path = std::env::var_os("TG_ORIGINAL_INSTALLATION")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .ancestors()
                .nth(4)
                .expect("country-core manifest must be inside game workspace")
                .join("ljxsj_92385/Tiny Glade")
        });
    assert!(
        path.join("assets/nani_meshes.ron").is_file(),
        "missing original installation: {path:?}"
    );
    path
}

#[test]
fn shipped_ron_and_json_reach_real_atlas_and_asset_registration() -> anyhow::Result<()> {
    let root = original_installation();
    let configs: Vec<NaniWorldSubsetConfig> = ron::de::from_reader(BufReader::new(File::open(
        root.join("assets/nani_meshes.ron"),
    )?))?;
    // Counts are from the identity-bound B file, not a synthetic empty fixture.
    assert_eq!(configs.len(), 19);
    assert_eq!(configs.iter().map(|c| c.meshes.len()).sum::<usize>(), 193);
    let mut atlases = MeshAtlasLibrary::default();
    load_nani_meshes(&root, &mut atlases, None, false)?;
    let mut source_vertex_offsets = HashMap::<Kind, u32>::new();
    let mut total_source_indices = 0usize;
    for config in &configs {
        for mesh in &config.meshes {
            let json: serde_json::Value = serde_json::from_reader(BufReader::new(File::open(
                root.join("assets/meshes")
                    .join(format!("{}.json", mesh.name)),
            )?))?;
            let source_indices = json["indices"]["buffer"]
                .as_array()
                .expect("original u32 indices");
            let vertex_count = json["Vertex_Position"]["buffer"]
                .as_array()
                .expect("original positions")
                .len();
            let mapping = atlases.get_by_name(mesh.name.as_str());
            let part = mapping
                .iter()
                .find(|part| part.subset == config.subset)
                .expect("every original RON entry must have its actual subset mapping");
            let atlas = &atlases.get_by_world_subset(config.subset).atlas;
            let layout = &atlas.layouts[part.part_index as usize];
            assert_eq!(layout.count as usize, source_indices.len(), "{}", mesh.name);
            let base = source_vertex_offsets.entry(config.subset).or_default();
            let fused = &atlas.mesh.indices[layout.base_index as usize..][..source_indices.len()];
            for (&actual, source) in fused.iter().zip(source_indices) {
                assert_eq!(
                    actual,
                    source.as_u64().expect("u32 index") as u32 + *base,
                    "{}",
                    mesh.name
                );
            }
            *base += vertex_count as u32;
            total_source_indices += source_indices.len();
            for removed in &config.attribs.remove {
                assert!(
                    !atlas.mesh.attributes.contains_key(removed),
                    "removed {removed} still exists"
                );
            }
            for added in &config.attribs.add {
                assert!(
                    atlas.mesh.attributes.contains_key(&added.name),
                    "missing added {}",
                    added.name
                );
            }
        }
    }
    assert_eq!(total_source_indices, 585_609);
    generate_procedural_meshes(&mut atlases);
    let mut meshes = AssetMeshLibrary::new();
    atlases.register_meshes(&mut meshes);
    let mut registered = 0usize;
    let mut fused_index_total = 0usize;
    for subset in Kind::ALL {
        let world = atlases.get_by_world_subset(subset);
        if world.atlas.mesh.indices.is_empty() {
            assert!(world.registered.is_none());
            continue;
        }
        let handle = world.registered.expect("nonempty atlas must be registered");
        let asset = meshes
            .get(handle)
            .expect("same real AssetMeshLibrary owns the handle");
        assert_eq!(asset.indices, world.atlas.mesh.indices);
        assert_eq!(asset.attributes, world.atlas.mesh.attributes);
        registered += 1;
        fused_index_total += asset.indices.len();
    }
    // Three tree subsets belong to the separate prefab loader, excluded here.
    assert_eq!(registered, 24);
    assert_eq!(fused_index_total, 592_419);
    Ok(())
}

fn assert_layouts(atlases: &MeshAtlasLibrary, subset: Kind, names: &[String], counts: &[u32]) {
    let atlas = &atlases.get_by_world_subset(subset).atlas;
    assert_eq!(atlas.layouts.len(), names.len());
    let mut offset = 0;
    for (index, (name, &count)) in names.iter().zip(counts).enumerate() {
        let mapping = atlases.get_by_name(name.as_str());
        assert_eq!(mapping.len(), 1);
        assert_eq!(mapping[0].subset, subset);
        assert_eq!(mapping[0].part_index, index as u32);
        assert_eq!(atlas.layouts[index].base_index, offset);
        assert_eq!(atlas.layouts[index].count, count);
        offset += count;
    }
    assert_eq!(atlas.mesh.indices.len(), offset as usize);
}

#[test]
fn all_nineteen_procedural_meshes_preserve_original_layouts_and_encoded_indices() {
    let mut atlases = MeshAtlasLibrary::default();
    generate_procedural_meshes(&mut atlases);
    assert_layouts(
        &atlases,
        Kind::RoofFloor,
        &(0..6)
            .map(|n| format!("stone_floor_{n}"))
            .collect::<Vec<_>>(),
        &[6, 12, 24, 48, 96, 192],
    );
    assert_layouts(
        &atlases,
        Kind::CurvedHalfTimber,
        &(0..6).map(|n| format!("timber_{n}")).collect::<Vec<_>>(),
        &[36, 72, 144, 288, 576, 1152],
    );
    assert_layouts(&atlases, Kind::Plaster, &["plaster_strip".into()], &[192]);
    assert_layouts(
        &atlases,
        Kind::StairsFloor,
        &[4, 8, 16, 32, 64].map(|n| format!("stairs_floor_chunk_{n}")),
        &[12, 24, 48, 96, 192],
    );
    assert_layouts(&atlases, Kind::Smoke, &["smoke".into()], &[3600]);
    let roof = &atlases.get_by_world_subset(Kind::RoofFloor).atlas.mesh;
    let points = match &roof.attributes["Vertex_Position"] {
        VertexAttributeValues::Float3(points) => points,
        _ => panic!("roof positions must remain Float3"),
    };
    assert_eq!(points.len(), 138);
    assert_eq!(
        &points[..4],
        &[
            Vec3::new(-0.5, 0., -0.5),
            Vec3::new(0.5, 0., -0.5),
            Vec3::new(-0.5, 0., 0.5),
            Vec3::new(0.5, 0., 0.5),
        ]
    );
    // Fixed native-order samples. These distinguish sin from the old wrong cos
    // candidate and ensure sqrt uses the TRANSFORMED z, not its original value.
    for (actual, expected) in points[4..10].iter().zip([
        Vec3::new(0., 0., -0.5),
        Vec3::new(0., 0., -0.5),
        Vec3::new(-0.5, 0., 0.),
        Vec3::new(0.5, 0., 0.),
        Vec3::new(0., 0., 0.5),
        Vec3::new(0., 0., 0.5),
    ]) {
        assert!(
            actual.abs_diff_eq(expected, 1.0e-6),
            "{actual:?} != {expected:?}"
        );
    }
    let diagonal = std::f32::consts::FRAC_1_SQRT_2 * 0.5;
    assert!(points[12].abs_diff_eq(Vec3::new(-diagonal, 0., -diagonal), 1.0e-6));
    let curved = &atlases
        .get_by_world_subset(Kind::CurvedHalfTimber)
        .atlas
        .mesh;
    assert_eq!(&curved.indices[..6], &[0, 1, 7, 7, 1, 8]);
    assert_eq!(&roof.indices[..6], &[0, 2, 1, 1, 2, 3]);
    let plaster = &atlases.get_by_world_subset(Kind::Plaster).atlas.mesh;
    assert!(plaster.attributes.is_empty());
    assert_eq!(&plaster.indices[..6], &[65536, 0, 1, 1, 65537, 65536]);
    assert_eq!(&plaster.indices[186..], &[65567, 31, 32, 32, 65568, 65567]);
    let stairs = &atlases.get_by_world_subset(Kind::StairsFloor).atlas.mesh;
    assert!(stairs.attributes.is_empty());
    for layout in &atlases.get_by_world_subset(Kind::StairsFloor).atlas.layouts {
        let actual = &stairs.indices[layout.base_index as usize..][..layout.count as usize];
        assert!(actual.iter().copied().eq(0..layout.count));
    }
    let smoke = &atlases.get_by_world_subset(Kind::Smoke).atlas.mesh;
    assert!(smoke.attributes.is_empty());
    assert_eq!(&smoke.indices[..6], &[0, 13, 1, 1, 13, 14]);
    assert_eq!(smoke.indices.iter().max(), Some(&662));
}
