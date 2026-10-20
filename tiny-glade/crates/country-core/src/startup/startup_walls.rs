//! Original B glade_startup_walls@140a92670/2320B. The seven global wall
//! drawables consume the shared brick SSBO through their indirect callbacks;
//! per-wall InstancedWall entities own allocations, not these drawables.
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
    systems::render_loop::draw_passes::{
        draw_callbacks_a::WallBrickInstancedDrawProxyView, draw_lists::OcclusionCulled,
    },
    utils::components::transform::Transform,
};
use bevy_ecs::{entity::Entity, system::Commands};
use std::path::Path;

pub const WALL_SHADERS: [(&str, &str); 2] = [
    ("/wall/wall_brick_lod0.raster.hlsl", "wall_brick_lod0"),
    ("/wall/wall_brick_lod1.raster.hlsl", "wall_brick_lod1"),
];
pub const BRICK_MESH_NAME: &str = "brick";
pub const BRICK_MESH_PATH: &str = "assets/meshes/brick.json";
pub const HALF_CUBE_NAME: &str = "half_cube";
pub const WALL_DRAWABLE_RECORDS: usize = 7;
/// Native stores four16B constants@142ad9a40, then u64(3 | 1<<32).
/// No vertex attributes are present; do not generate a generic cube mesh.
pub const HALF_CUBE_INDICES: [u32; 18] = [7, 5, 4, 4, 6, 7, 3, 7, 6, 6, 2, 3, 1, 5, 7, 7, 3, 1];
pub fn create_half_cube_mesh() -> Mesh {
    let mut mesh = Mesh::new("brick half cube");
    mesh.set_indices(HALF_CUBE_INDICES.to_vec());
    mesh
}

/// Byte policy comes from native command payload writes@140a92913..92d7d.
/// The third proxy byte of regular records is inactive; use zero in safe Rust.
/// First four tuples contain OcclusionCulled (PDB1412d6640); last three do not
/// (PDB1412dd190). No NaniInstanceDesc, KeepOnLevelUnload or pass override.
pub const WALL_DRAWABLE_POLICY: [(usize, usize, [u8; 3], bool); 7] = [
    (0, 0, [0, 0, 0], true),
    (1, 1, [0, 0, 0], true),
    (0, 0, [1, 0, 0], true),
    (1, 1, [1, 0, 0], true),
    (1, 1, [0, 1, 0], false),
    (1, 1, [0, 1, 1], false),
    (1, 1, [0, 1, 2], false),
];

fn spawn_wall_drawables(
    meshes: [Handle<Mesh>; 2],
    shaders: [Handle<RasterShaderProgram>; 2],
    commands: &mut Commands<'_, '_>,
) -> [Entity; 7] {
    WALL_DRAWABLE_POLICY.map(|(mesh, shader, bytes, culled)| {
        let bundle = DrawableMeshBundle {
            mesh: meshes[mesh],
            shader: shaders[shader],
            transform: Transform::identity(),
        };
        let proxy = WallBrickInstancedDrawProxyView {
            b0: bytes[0],
            b1: bytes[1],
            b2: bytes[2],
        };
        if culled {
            commands.spawn((bundle, proxy, OcclusionCulled)).id()
        } else {
            commands.spawn((bundle, proxy)).id()
        }
    })
}

/// Current device/asset owners are passed by reference. Neither a second
/// shader registry nor a separate sparse buffer is created by wall startup.
/// Original order: load/register brick, construct/register half_cube, compile
/// LOD0 then LOD1, append all seven entity commands. Failure remains fatal.
pub fn glade_startup_walls(
    meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary,
    backend: &dyn ShaderBackend,
    watch: &MeshWatch,
    installed: &Path,
    commands: &mut Commands<'_, '_>,
) -> [Entity; 7] {
    let path = installed.join(BRICK_MESH_PATH);
    let path = path.to_str().expect("installed brick path must be UTF-8");
    let brick = crate::utils::load_json::load_json_as_mesh(path);
    watch.watch(&brick);
    let brick = meshes.add(MeshAsset::new(brick, Some(BRICK_MESH_NAME.into())));
    let half = create_half_cube_mesh();
    watch.watch(&half);
    let half = meshes.add(MeshAsset::new(half, Some(HALF_CUBE_NAME.into())));
    let shaders = WALL_SHADERS.map(|(path, name)| {
        super::load_shaders::load_shader_into_library(shaders, backend, path, name)
    });
    spawn_wall_drawables([brick, half], shaders, commands)
}

/// Existing mesh-only callers are explicitly partial; device gameplay uses
/// glade_startup_walls, which includes shaders, procedural index mesh and ECS.
pub fn startup_walls_meshes(meshes: bevy_ecs::prelude::Res<AssetMeshLibrary>) {
    super::register_mesh_file(&meshes, BRICK_MESH_PATH, Some(BRICK_MESH_NAME));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        asset_libraries::Handle,
        components::KeepOnLevelUnload,
        render::{
            test_device::TestDevice,
            vao::{Vao, build_vao},
        },
        systems::render_loop::draw_passes::draw_lists::{self, DrawLists},
    };
    use bevy_ecs::{prelude::*, system::SystemState};
    #[test]
    fn original_half_cube_is_index_only_not_an_empty_mesh_or_six_face_box() {
        let mesh = create_half_cube_mesh();
        assert_eq!(mesh.label, "brick half cube");
        assert!(mesh.attributes.is_empty());
        assert!(mesh.source_file.is_none());
        assert_eq!(mesh.indices, HALF_CUBE_INDICES);
        let vao = build_vao(&TestDevice(1), &mesh);
        assert!(vao.bindings.is_empty());
        assert_eq!(vao.index_buffer.byte_size, 72);
    }
    #[test]
    fn seven_original_drawables_reach_real_draw_lists_and_keep_proxy_gates() {
        let mut world = World::new();
        let shaders = AssetShaderLibrary::new();
        let handles = WALL_SHADERS.map(|(path, name)| {
            super::super::load_shaders::load_shader_into_library(
                &shaders,
                &TestDevice(1),
                path,
                name,
            )
        });
        let meshes = [Handle::<Mesh>::new(), Handle::<Mesh>::new()];
        let mut state = SystemState::<Commands>::new(&mut world);
        let ids = spawn_wall_drawables(meshes, handles, &mut state.get_mut(&mut world));
        state.apply(&mut world);
        let vaos = [Handle::<Vao>::new(), Handle::<Vao>::new()];
        for (i, id) in ids.iter().copied().enumerate() {
            let (mesh, shader, bytes, culled) = WALL_DRAWABLE_POLICY[i];
            assert_eq!(world.get::<Handle<Mesh>>(id), Some(&meshes[mesh]));
            assert_eq!(
                world.get::<Handle<RasterShaderProgram>>(id),
                Some(&handles[shader])
            );
            assert_eq!(world.get::<Transform>(id), Some(&Transform::identity()));
            assert_eq!(
                world.get::<WallBrickInstancedDrawProxyView>(id),
                Some(&WallBrickInstancedDrawProxyView {
                    b0: bytes[0],
                    b1: bytes[1],
                    b2: bytes[2]
                })
            );
            assert_eq!(world.get::<OcclusionCulled>(id).is_some(), culled);
            assert!(world.get::<KeepOnLevelUnload>(id).is_none());
            // CPU list construction admission only. Physical VAO creation and
            // GPU indirect command/readback are separate device acceptance.
            world.entity_mut(id).insert(vaos[mesh]);
        }
        let mut lists = DrawLists::new();
        draw_lists::build_draw_lists(&mut world, &shaders, &mut lists);
        lists.sort();
        assert_eq!(lists.list(draw_lists::list::SHADOW).len(), 7);
        assert_eq!(lists.list(draw_lists::list::GBUFFER).len(), 7);
        assert_eq!(lists.list(draw_lists::list::GBUFFER_OC2).len(), 4);
        assert_eq!(lists.total_len(), 18);
        let proxy: Vec<_> = ids
            .iter()
            .map(|id| *world.get::<WallBrickInstancedDrawProxyView>(*id).unwrap())
            .collect();
        use crate::systems::render_loop::draw_passes::draw_shaders::draw_bricks::{
            self, BrickGate, BrickProxy,
        };
        for mode in 2..=4 {
            for (i, p) in proxy.iter().enumerate().skip(4) {
                let result = draw_bricks::gate::<1>(
                    mode,
                    BrickProxy {
                        b0: p.b0,
                        b1: p.b1,
                        b2: p.b2,
                    },
                );
                assert_eq!(
                    matches!(result, BrickGate::Pass { .. }),
                    i - 4 == usize::from(mode - 2)
                );
            }
        }
    }
}
