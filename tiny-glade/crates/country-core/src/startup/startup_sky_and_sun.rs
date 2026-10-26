//! Original B sky startup140864430/636B. Host asset compilation and commands
//! use the active App owners; the complete original tuple has one drawable.
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
    utils::components::transform::Transform,
};
use bevy_ecs::{entity::Entity, system::Commands};
use std::path::Path;

pub const SKYDOME_MESH: (&str, &str) = ("assets/meshes/skydome.json", "skydome");
pub const SKYDOME_SHADER: (&str, &str) = ("/skydome.raster.hlsl", "skydome_shader");

/// Native default Transform then overrides translation with(-0,-2,-0).
/// Preserve both negative-zero signs; no extra sun entity or pass flags exist.
fn sky_transform() -> Transform {
    Transform::from_translation(glam::Vec3::new(
        f32::from_bits(0x80000000),
        -2.0,
        f32::from_bits(0x80000000),
    ))
}
fn spawn_skydome(
    mesh: Handle<Mesh>,
    shader: Handle<RasterShaderProgram>,
    commands: &mut Commands<'_, '_>,
) -> Entity {
    commands
        .spawn(DrawableMeshBundle {
            mesh,
            shader,
            transform: sky_transform(),
        })
        .id()
}

pub fn startup_sky_and_sun(
    meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary,
    backend: &dyn ShaderBackend,
    watch: &MeshWatch,
    installed: &Path,
    commands: &mut Commands<'_, '_>,
) -> Entity {
    let path = installed.join(SKYDOME_MESH.0);
    let mesh = crate::utils::load_json::load_json_as_mesh(
        path.to_str().expect("installed skydome path must be UTF-8"),
    );
    watch.watch(&mesh);
    let mesh = meshes.add(MeshAsset::new(mesh, Some(SKYDOME_MESH.1.into())));
    let shader = super::load_shaders::load_shader_into_library(
        shaders,
        backend,
        SKYDOME_SHADER.0,
        SKYDOME_SHADER.1,
    );
    spawn_skydome(mesh, shader, commands)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        components::KeepOnLevelUnload,
        render::{test_device::TestDevice, vao::Vao},
        systems::render_loop::draw_passes::draw_lists::{
            self, DesignatedRenderPass, DontCastShadow, DrawLists, OcclusionCulled,
        },
    };
    use bevy_ecs::{prelude::*, system::SystemState};
    #[test]
    fn original_single_sky_drawable_keeps_transform_and_reaches_forward_and_shadow_lists() {
        let mut world = World::new();
        let shaders = AssetShaderLibrary::new();
        let shader = super::super::load_shaders::load_shader_into_library(
            &shaders,
            &TestDevice(1),
            SKYDOME_SHADER.0,
            SKYDOME_SHADER.1,
        );
        let mesh = Handle::<Mesh>::new();
        let mut state = SystemState::<Commands>::new(&mut world);
        let id = spawn_skydome(mesh, shader, &mut state.get_mut(&mut world));
        state.apply(&mut world);
        assert_eq!(world.get::<Handle<Mesh>>(id), Some(&mesh));
        assert_eq!(world.get::<Handle<RasterShaderProgram>>(id), Some(&shader));
        let t = world.get::<Transform>(id).unwrap();
        assert_eq!(
            t.translation.to_array().map(f32::to_bits),
            [0x80000000, 0xc0000000, 0x80000000]
        );
        assert_eq!(t.rotation, glam::Quat::IDENTITY);
        assert_eq!(t.scale, glam::Vec3::ONE);
        assert!(world.get::<OcclusionCulled>(id).is_none());
        assert!(world.get::<KeepOnLevelUnload>(id).is_none());
        assert!(world.get::<DontCastShadow>(id).is_none());
        assert!(world.get::<DesignatedRenderPass>(id).is_none());
        // CPU admission only; the device runtime must build the actual VAO.
        world.entity_mut(id).insert(Handle::<Vao>::new());
        let mut lists = DrawLists::new();
        draw_lists::build_draw_lists(&mut world, &shaders, &mut lists);
        assert_eq!(lists.list(draw_lists::list::FORWARD).len(), 1);
        assert_eq!(lists.list(draw_lists::list::SHADOW).len(), 1);
        assert_eq!(lists.total_len(), 2);
        assert_eq!(
            lists.list(draw_lists::list::FORWARD)[0].0.sort_key,
            10_000_000
        );
    }
}
