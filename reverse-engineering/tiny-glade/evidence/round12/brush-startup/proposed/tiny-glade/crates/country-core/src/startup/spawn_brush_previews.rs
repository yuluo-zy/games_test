//! B spawn_brush_previews140924b40/8361B: original22 mesh registrations,
//!18 explicit color overwrites,17 typed entities and three shader lookups.
//! Asset failures propagate to the startup owner; no missing-file continuation.
use crate::{
    asset_libraries::{
        Handle,
        mesh_library::{AssetMeshLibrary, MeshAsset},
        meshwatch::MeshWatch,
        shader_library::AssetShaderLibrary,
    },
    components::drawable::DrawableMeshBundle,
    components::{FollowMouse, KeepOnLevelUnload, SnappingToEndPreview, SnappingToSegmentPreview},
    geometry::sphere::Sphere,
    render::{
        mesh::Mesh,
        shader::{RasterShaderProgram, boundary::ShaderBackend},
    },
    startup::load_shaders::load_shader_into_library,
    systems::{
        brush_preview::BrushPreview,
        history::signifier_replay_active::SignifierReplayIndex,
        look_at_camera::LookAtCamera,
        paths::SetSizeBasedOnAppModePath,
        render_loop::draw_passes::{
            draw_callbacks_a::ArrowGizmoView,
            draw_lists::{DontCastShadow, UiPass},
        },
    },
    utils::{components::transform::Transform, load_json::try_load_json_as_mesh},
};
use anyhow::Context;
use bevy_ecs::system::Commands;
use glam::{Quat, Vec3};
use std::path::Path;
pub const PREVIEW_SHADER: (&str, &str) = (
    "/timed_terrain_gizmo.raster.hlsl",
    "timed_terrain_gizmo_shader",
);
/// Native dynamically formats these exact stems, including original `foward`.
pub const EXTRA_PREVIEW_MESHES: [&str; 10] = [
    "brush_arrow_flat_single_up",
    "brush_arrow_flat_single_foward",
    "brush_arrow_flat_outline",
    "flat_arrow",
    "flat_arrow_outline",
    "corner_arrow",
    "corner_arrow_outline",
    "corner_arrow_switch_ridge_dir",
    "move_shape",
    "move_shape_outline",
];
fn mesh(
    root: &Path,
    stem: &str,
    name: &str,
    color: Option<Vec3>,
    meshes: &AssetMeshLibrary,
    watch: &MeshWatch,
) -> anyhow::Result<Handle<Mesh>> {
    let path = root.join(format!("assets/meshes/{stem}.json"));
    let mut value = try_load_json_as_mesh(&path)
        .with_context(|| format!("loading brush preview {}", path.display()))?;
    if let Some(color) = color {
        value.add_color(color);
    }
    watch.watch(&value);
    Ok(meshes.add(MeshAsset::new(value, Some(name.to_owned()))))
}
fn color_shader(shaders: &AssetShaderLibrary) -> anyhow::Result<Handle<RasterShaderProgram>> {
    shaders
        .get_raster_handle_by_name("vertex_color_shader")
        .context("original vertex_color_shader is required for brush startup")
}
fn snapping_transform() -> Transform {
    Transform {
        rotation: Quat::IDENTITY,
        translation: Vec3::new(-0.0, -0.5, -0.0),
        scale: Vec3::ZERO,
    }
}
/// Source adapter takes the actual asset root/backend owners directly. Native
/// six ECS params are device,meshes,meshwatch,shaders,shaderwatch,Commands; the
/// host owns compiler/watch wiring. This function preserves original admission
/// and all partial-side-effect boundaries if a later asset fails.
#[allow(clippy::too_many_arguments)]
pub fn spawn_brush_previews(
    root: &Path,
    backend: &dyn ShaderBackend,
    meshes: &AssetMeshLibrary,
    watch: &MeshWatch,
    shaders: &AssetShaderLibrary,
    commands: &mut Commands,
) -> anyhow::Result<()> {
    let timed = load_shader_into_library(shaders, backend, PREVIEW_SHADER.0, PREVIEW_SHADER.1);
    let arrow = mesh(
        root,
        "brush_arrow",
        "brush_arrow",
        Some(Vec3::ONE),
        meshes,
        watch,
    )?;
    let circle = mesh(
        root,
        "brush_circle",
        "brush_circle",
        Some(Vec3::ONE),
        meshes,
        watch,
    )?;
    let cross = mesh(
        root,
        "brush_circle_cross",
        "brush_circle_cross",
        None,
        meshes,
        watch,
    )?;
    let scribble = mesh(
        root,
        "brush_circle_scribble",
        "brush_circle_scribble",
        Some(Vec3::new(0.5, 0., 0.)),
        meshes,
        watch,
    )?;
    let flat = mesh(
        root,
        "brush_arrow_flat",
        "brush_arrow_flat",
        Some(Vec3::ONE),
        meshes,
        watch,
    )?;
    let terrain = mesh(
        root,
        "brush_4_arrows",
        "brush_terrain_edit",
        Some(Vec3::ONE),
        meshes,
        watch,
    )?;
    let roof = mesh(
        root,
        "brush_2_arrows",
        "brush_flat_roof_edit",
        Some(Vec3::ONE),
        meshes,
        watch,
    )?;
    let replay = mesh(
        root,
        "brush_5_replay",
        "brush_replay",
        Some(Vec3::ONE),
        meshes,
        watch,
    )?;
    for stem in EXTRA_PREVIEW_MESHES {
        mesh(root, stem, stem, Some(Vec3::ONE), meshes, watch)?;
    }
    let color = color_shader(shaders)?;
    commands.spawn((
        KeepOnLevelUnload,
        DrawableMeshBundle {
            mesh: arrow,
            shader: color,
            transform: Transform::identity(),
        },
        BrushPreview(1),
        FollowMouse(false),
    ));
    commands.spawn((
        KeepOnLevelUnload,
        circle,
        color,
        BrushPreview(3),
        FollowMouse(false),
        SetSizeBasedOnAppModePath,
    ));
    commands.spawn((
        KeepOnLevelUnload,
        circle,
        color,
        BrushPreview(4),
        FollowMouse(false),
    ));
    commands.spawn((
        KeepOnLevelUnload,
        circle,
        color,
        BrushPreview(5),
        FollowMouse(false),
    ));
    commands.spawn((
        KeepOnLevelUnload,
        cross,
        color,
        BrushPreview(6),
        DontCastShadow,
        FollowMouse(false),
    ));
    for kind in 7..=10 {
        commands.spawn((
            KeepOnLevelUnload,
            scribble,
            color,
            BrushPreview(kind),
            DontCastShadow,
            FollowMouse(false),
        ));
    }
    commands.spawn((
        KeepOnLevelUnload,
        flat,
        color,
        BrushPreview(2),
        LookAtCamera,
        UiPass,
    ));
    for index in [1, 2] {
        commands.spawn((
            KeepOnLevelUnload,
            replay,
            color,
            BrushPreview(13),
            SignifierReplayIndex(index),
            UiPass,
        ));
    }
    commands.spawn((
        KeepOnLevelUnload,
        terrain,
        timed,
        BrushPreview(11),
        ArrowGizmoView { f0: 0., f4: 0. },
        LookAtCamera,
        FollowMouse(true),
        UiPass,
    ));
    commands.spawn((
        KeepOnLevelUnload,
        roof,
        timed,
        BrushPreview(12),
        ArrowGizmoView { f0: 0., f4: 0. },
        LookAtCamera,
        FollowMouse(true),
        UiPass,
    ));
    // Native constructs this named sphere even though this startup does not
    // attach its handle to an entity. Other real consumers resolve the name.
    let sphere = Mesh::try_from(Sphere::new(0, 1.0))
        .context("creating temp_sphere brush mesh")?
        .add_color_self(Vec3::ONE);
    watch.watch(&sphere);
    meshes.add(MeshAsset::new(sphere, Some("temp_sphere".to_owned())));
    let hand = mesh(root, "hand", "pet_hand", None, meshes, watch)?;
    commands.spawn((
        KeepOnLevelUnload,
        hand,
        color,
        LookAtCamera,
        FollowMouse(false),
        UiPass,
        BrushPreview(14),
    ));
    let dashed_circle = mesh(root, "dashed_circle", "circle", None, meshes, watch)?;
    let color = color_shader(shaders)?;
    commands.spawn((
        KeepOnLevelUnload,
        DrawableMeshBundle {
            mesh: dashed_circle,
            shader: color,
            transform: snapping_transform(),
        },
        DontCastShadow,
        SnappingToEndPreview,
        UiPass,
    ));
    let dashed_line = mesh(root, "dashed_lines", "snapping_line", None, meshes, watch)?;
    let color = color_shader(shaders)?;
    commands.spawn((
        KeepOnLevelUnload,
        DrawableMeshBundle {
            mesh: dashed_line,
            shader: color,
            transform: snapping_transform(),
        },
        DontCastShadow,
        SnappingToSegmentPreview,
        UiPass,
    ));
    Ok(())
}

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
