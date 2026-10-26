//! Original1409d49a0 asset and global-drawable policy. Species births live in
//! system-critters so their canonical behavior/audio components retain identity.
use crate::{
    asset_libraries::{
        Handle,
        mesh_library::{AssetMeshLibrary, MeshAsset},
        meshwatch::MeshWatch,
        shader_library::AssetShaderLibrary,
    },
    components::drawable::DrawableMeshBundle,
    render::{
        mesh::Mesh,
        shader::{RasterShaderProgram, boundary::ShaderBackend},
    },
    systems::render_loop::draw_passes::draw_lists::{DesignatedRenderPass, DontCastShadow},
    utils::components::transform::Transform,
};
use bevy_ecs::prelude::*;
use glam::Vec3;
use std::path::Path;

pub const BUTTERFLY_SHOOSH_EVENT: &str = "event:/SFX/Butterfly/Shoosh";
pub const BUTTERFLY_COUNT: usize = 24;
pub const FIREFLY_COUNT: usize = 96;

fn publish_mesh(
    mesh: Mesh,
    name: &str,
    meshes: &AssetMeshLibrary,
    watch: &MeshWatch,
) -> Handle<Mesh> {
    watch.watch(&mesh);
    meshes.add(MeshAsset::new(mesh, Some(name.to_owned())))
}
fn file_mesh(
    root: &Path,
    path: &str,
    name: &str,
    meshes: &AssetMeshLibrary,
    watch: &MeshWatch,
) -> Handle<Mesh> {
    let path = root.join(path);
    let mesh = crate::utils::load_json::load_json_as_mesh(
        path.to_str().expect("critter mesh path must be UTF-8"),
    );
    publish_mesh(mesh, name, meshes, watch)
}
fn load_raster(
    library: &AssetShaderLibrary,
    backend: &dyn ShaderBackend,
    path: &str,
    name: &'static str,
) -> Handle<RasterShaderProgram> {
    super::load_shaders::load_shader_into_library(library, backend, path, name)
}

/// Called after the complete24-butterfly/audio birth loop. Original drawable
/// has identity rotation/translation, scale0.6 and no extra render flags.
pub fn startup_butterfly_drawable(
    meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary,
    backend: &dyn ShaderBackend,
    watch: &MeshWatch,
    root: &Path,
    commands: &mut Commands,
) {
    let mesh = file_mesh(
        root,
        "assets/meshes/butterfly.json",
        "butterfly",
        meshes,
        watch,
    );
    let shader = load_raster(
        shaders,
        backend,
        "/critters/butterfly.raster.hlsl",
        "butterfly_shader",
    );
    commands.spawn(DrawableMeshBundle {
        mesh,
        shader,
        transform: Transform::from_scale(Vec3::splat(0.6)),
    });
}

/// Called after all96-firefly births, only outside Winter. Existing full
/// geometry owners produce the actual meshes; no invented firefly asset path.
pub fn startup_firefly_drawables(
    meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary,
    backend: &dyn ShaderBackend,
    watch: &MeshWatch,
    _root: &Path,
    commands: &mut Commands,
) {
    let sphere = Mesh::try_from(crate::geometry::sphere::Sphere::new(0, 1.0))
        .expect("original firefly sphere")
        .add_color_self(Vec3::ONE);
    let mesh = publish_mesh(sphere, "firefly", meshes, watch);
    let shader = load_raster(
        shaders,
        backend,
        "/critters/firefly.raster.hlsl",
        "firefly_shader",
    );
    commands.spawn((
        DrawableMeshBundle {
            mesh,
            shader,
            transform: Transform::identity(),
        },
        DontCastShadow,
    ));
    let plane = crate::geometry::plane::plane_mesh(1.0).add_color_self(Vec3::ONE);
    let mesh = publish_mesh(plane, "firefly_glow", meshes, watch);
    let shader = load_raster(
        shaders,
        backend,
        "/critters/firefly_glow.raster.hlsl",
        "firefly_glow_shader",
    );
    commands.spawn((
        DrawableMeshBundle {
            mesh,
            shader,
            transform: Transform::identity(),
        },
        DesignatedRenderPass(17),
    ));
}

/// Unconditional original tail. These are assets for subsequent live duck /
/// heart / splash producers, not three global draw entities.
pub fn startup_duck_assets(
    meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary,
    backend: &dyn ShaderBackend,
    watch: &MeshWatch,
    root: &Path,
) {
    file_mesh(
        root,
        "assets/meshes/duck_flying.json",
        "duck",
        meshes,
        watch,
    );
    load_raster(
        shaders,
        backend,
        "/critters/duck_animated.raster.hlsl",
        "duck_shader",
    );
    file_mesh(
        root,
        "assets/meshes/duck_heart.json",
        "duck_heart",
        meshes,
        watch,
    );
    file_mesh(root, "assets/meshes/splash.json", "splash", meshes, watch);
    load_raster(
        shaders,
        backend,
        "/splash_particle.raster.hlsl",
        "splash_particle",
    );
}

