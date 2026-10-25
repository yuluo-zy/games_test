#[cfg(test)]
mod original_startup_tests {
    use super::*;
    use crate::render::{
        mesh::{ATTR_COLOR, VertexAttributeValues},
        test_device::TestDevice,
    };
    use bevy_ecs::world::{CommandQueue, World};
    fn evidence(name: &str) -> serde_json::Value {
        let p = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../reverse-engineering/tiny-glade/evidence/round12/brush-startup")
            .join(name);
        serde_json::from_slice(&std::fs::read(p).unwrap()).unwrap()
    }
    fn assets() -> std::path::PathBuf {
        std::env::var_os("TG_INSTALL_ROOT")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../ljxsj_92385/Tiny Glade")
            })
    }
    fn common_shader(shaders: &AssetShaderLibrary, device: &TestDevice) {
        load_shader_into_library(
            shaders,
            device,
            "/vertex_color.raster.hlsl",
            "vertex_color_shader",
        );
    }
    #[test]
    fn actual_original_assets_match_all22_native_registrations_and17_typed_spawn_payloads() {
        let native = evidence("native-startup-receipt.json");
        let expected = evidence("native-semantic-spawn-receipts.json");
        let meshes = AssetMeshLibrary::new();
        let watch = MeshWatch::new(false);
        let shaders = AssetShaderLibrary::new();
        let device = TestDevice::default();
        common_shader(&shaders, &device);
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        spawn_brush_previews(
            &assets(),
            &device,
            &meshes,
            &watch,
            &shaders,
            &mut Commands::new(&mut queue, &world),
        )
        .unwrap();
        assert_eq!(world.entities().len(), 0);
        queue.apply(&mut world);
        assert_eq!(meshes.mesh_entries.len(), 22);
        for r in native["registrations"].as_array().unwrap() {
            let name = r["name"].as_str().unwrap();
            let mesh = meshes.get(meshes.get_handle_by_name(name)).unwrap();
            assert!(!mesh.indices.is_empty(), "{name}");
            let ordinal = r["mesh_ordinal"].as_u64().unwrap();
            if let Some(call) = native["color_calls"]
                .as_array()
                .unwrap()
                .iter()
                .find(|c| c["mesh_ordinal"].as_u64() == Some(ordinal))
            {
                let expected_bits: Vec<u32> = call["color_bits"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_u64().unwrap() as u32)
                    .collect();
                let VertexAttributeValues::Float3(colors) = mesh.attr(ATTR_COLOR) else {
                    panic!("native color overwrite must be Float3")
                };
                assert_eq!(colors.len(), mesh.positions().len());
                for color in colors {
                    assert_eq!(color.to_array().map(f32::to_bits).as_slice(), expected_bits);
                }
            }
        }
        assert_eq!(
            meshes
                .get(meshes.get_handle_by_name("temp_sphere"))
                .unwrap()
                .positions()
                .len(),
            12
        );
        let mut entities: Vec<_> = world
            .query_filtered::<bevy_ecs::entity::Entity, bevy_ecs::query::With<KeepOnLevelUnload>>()
            .iter(&world)
            .collect();
        entities.sort_by_key(|e| e.index());
        assert_eq!(entities.len(), 17);
        for (entity, expected) in entities.iter().zip(expected.as_array().unwrap()) {
            let row = world.entity(*entity);
            let mesh = *row.get::<Handle<Mesh>>().unwrap();
            let shader = *row.get::<Handle<RasterShaderProgram>>().unwrap();
            let name = native["registrations"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| meshes.get_handle_by_name(r["name"].as_str().unwrap()) == mesh)
                .unwrap()["name"]
                .as_str()
                .unwrap();
            let transform = row.get::<Transform>().map(|t| {
                t.rotation
                    .to_array()
                    .into_iter()
                    .chain(t.translation.to_array())
                    .chain(t.scale.to_array())
                    .map(f32::to_bits)
                    .collect::<Vec<_>>()
            });
            let actual = serde_json::json!({
                "mesh":name,"shader":shaders.get_raster(shader).unwrap().name,"transform_bits":transform,
                "brush":row.get::<BrushPreview>().map(|p|p.0),"follow_mouse":row.get::<FollowMouse>().map(|p|p.0),
                "ui":row.contains::<UiPass>(),"no_shadow":row.contains::<DontCastShadow>(),"look_at":row.contains::<LookAtCamera>(),
                "path_size":row.contains::<SetSizeBasedOnAppModePath>(),"replay_index":row.get::<SignifierReplayIndex>().map(|p|p.0),
                "arrow_bits":row.get::<ArrowGizmoView>().map(|p|[p.f0.to_bits(),p.f4.to_bits()]),
                "snap_end":row.contains::<SnappingToEndPreview>(),"snap_segment":row.contains::<SnappingToSegmentPreview>(),
            });
            assert_eq!(&actual, expected, "native deferred entity {entity:?}");
        }
    }
    #[test]
    fn every_original_file_failure_preserves_its_native_partial_side_effect_boundary() {
        let native = evidence("native-startup-receipt.json");
        let temp = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(temp.path().join("assets/meshes")).unwrap();
        for file in native["meshes"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|p| p.as_str())
            .filter(|p| !p.starts_with('<'))
        {
            std::fs::copy(assets().join(file), temp.path().join(file)).unwrap();
        }
        for case in native["failure_cases"].as_array().unwrap() {
            let ordinal = case["mesh_ordinal"].as_u64().unwrap() as usize;
            let relative = native["meshes"][ordinal - 1].as_str().unwrap();
            if relative.starts_with('<') {
                continue;
            }
            let file = temp.path().join(relative);
            let removed = file.with_extension("json.held");
            std::fs::rename(&file, &removed).unwrap();
            let meshes = AssetMeshLibrary::new();
            let watch = MeshWatch::new(false);
            let shaders = AssetShaderLibrary::new();
            let device = TestDevice::default();
            common_shader(&shaders, &device);
            let mut world = World::new();
            let mut queue = CommandQueue::default();
            assert!(
                spawn_brush_previews(
                    temp.path(),
                    &device,
                    &meshes,
                    &watch,
                    &shaders,
                    &mut Commands::new(&mut queue, &world)
                )
                .is_err(),
                "missing {relative} must fail"
            );
            queue.apply(&mut world);
            assert_eq!(
                world.entities().len() as u64,
                case["spawns"].as_u64().unwrap()
            );
            assert_eq!(
                meshes.mesh_entries.len() as u64,
                case["registered"].as_u64().unwrap()
            );
            std::fs::rename(removed, file).unwrap();
        }
    }
    #[test]
    fn missing_shared_vertex_shader_fails_before_any_entity_is_queued() {
        let meshes = AssetMeshLibrary::new();
        let watch = MeshWatch::new(false);
        let shaders = AssetShaderLibrary::new();
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        assert!(
            spawn_brush_previews(
                &assets(),
                &TestDevice::default(),
                &meshes,
                &watch,
                &shaders,
                &mut Commands::new(&mut queue, &world)
            )
            .is_err()
        );
        queue.apply(&mut world);
        assert_eq!(world.entities().len(), 0);
        assert_eq!(meshes.mesh_entries.len(), 18);
    }
}
