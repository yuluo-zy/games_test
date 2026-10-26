//! Complete original B startup_roof1414547d0/1680B, current physical owners.
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
pub const ROOF_MESHES: [(&str, &str); 4] = [
    ("assets/meshes/roof_spire.json", "roof_spire"),
    ("assets/meshes/roof_support_beam.json", "roof_support_beam"),
    ("assets/meshes/roof_button.json", "roof_button"),
    (
        "assets/meshes/roof_delete_scribble.json",
        "roof_delete_scribble",
    ),
];
pub const ROOF_SHADERS: [(&str, &str); 2] = [
    (
        "/instanced_roof_spire.raster.hlsl",
        "instanced_roof_spire_shader",
    ),
    (
        "/instanced_roof_beams.raster.hlsl",
        "support_beam_roof_shader",
    ),
];
pub const ROOF_DRAWABLES: [(&str, &str); 2] = [
    ("roof_spire", "instanced_roof_spire_shader"),
    ("roof_support_beam", "support_beam_roof_shader"),
];
fn spawn_roof_drawable(
    mesh: Handle<Mesh>,
    shader: Handle<RasterShaderProgram>,
    commands: &mut Commands<'_, '_>,
) -> Entity {
    commands
        .spawn(DrawableMeshBundle {
            mesh,
            shader,
            transform: Transform::identity(),
        })
        .id()
}
/// Preserve interleaved host calls, command writes and failure side effects:
/// spire mesh/shader/spawn, beam mesh/shader/spawn, button mesh, scribble mesh.
/// Both native command executors1412dd460 insert only DrawableMeshBundle.
pub fn startup_roof(
    meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary,
    backend: &dyn ShaderBackend,
    watch: &MeshWatch,
    installed: &Path,
    commands: &mut Commands<'_, '_>,
) -> [Entity; 2] {
    let mut entities = Vec::with_capacity(2);
    for (i, (file, name)) in ROOF_MESHES.into_iter().enumerate() {
        let path = installed.join(file);
        let mesh = crate::utils::load_json::load_json_as_mesh(
            path.to_str()
                .expect("installed roof mesh path must be UTF-8"),
        );
        watch.watch(&mesh);
        let mesh = meshes.add(MeshAsset::new(mesh, Some(name.into())));
        if let Some(&(path, name)) = ROOF_SHADERS.get(i) {
            let shader =
                super::load_shaders::load_shader_into_library(shaders, backend, path, name);
            entities.push(spawn_roof_drawable(mesh, shader, commands));
        }
    }
    entities
        .try_into()
        .expect("original roof startup has exactly two drawables")
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
    fn two_original_roof_globals_are_real_drawables_with_expected_draw_callbacks() {
        let mut world = World::new();
        let shaders = AssetShaderLibrary::new();
        let mut ids = Vec::new();
        for (path, name) in ROOF_SHADERS {
            let shader = super::super::load_shaders::load_shader_into_library(
                &shaders,
                &TestDevice(1),
                path,
                name,
            );
            let mesh = Handle::<Mesh>::new();
            let mut state = SystemState::<Commands>::new(&mut world);
            let id = spawn_roof_drawable(mesh, shader, &mut state.get_mut(&mut world));
            state.apply(&mut world);
            assert_eq!(world.get::<Handle<Mesh>>(id), Some(&mesh));
            assert_eq!(world.get::<Handle<RasterShaderProgram>>(id), Some(&shader));
            assert_eq!(world.get::<Transform>(id), Some(&Transform::identity()));
            assert!(world.get::<OcclusionCulled>(id).is_none());
            assert!(world.get::<KeepOnLevelUnload>(id).is_none());
            assert!(world.get::<DontCastShadow>(id).is_none());
            assert!(world.get::<DesignatedRenderPass>(id).is_none());
            world.entity_mut(id).insert(Handle::<Vao>::new());
            ids.push(id);
        }
        // CPU list admission, not a substitute for real SSBO/indirect uploads.
        let mut lists = DrawLists::new();
        draw_lists::build_draw_lists(&mut world, &shaders, &mut lists);
        lists.sort();
        assert_eq!(lists.list(draw_lists::list::SHADOW).len(), 2);
        assert_eq!(lists.list(draw_lists::list::GBUFFER).len(), 2);
        assert_eq!(lists.total_len(), 4);
        assert_eq!(
            lists
                .list(draw_lists::list::GBUFFER)
                .iter()
                .map(|x| x.0.sort_key)
                .collect::<Vec<_>>(),
            vec![3100, 3500]
        );
        assert_eq!(
            lists
                .list(draw_lists::list::GBUFFER)
                .iter()
                .map(|x| x.0.params.entity)
                .collect::<Vec<_>>(),
            ids
        );
    }
}
