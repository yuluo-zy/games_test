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
