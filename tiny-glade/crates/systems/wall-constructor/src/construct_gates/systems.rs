//! 木门系统（construct_gates/systems.rs）。
//!
//! - `construct_gates`（0x140756370，9 参 G06 冻结）。
//! - `update_gates_hashmap`（0x140755690，6 组合 despawn 顺序冻结）。
//! - `construct_wooden_gate_doors`（0x141210110，28 原返回/1886 条 96B/
//!   46 mesh/822 顶点冻结）。
//!
//! W-B1 轮 3 终验接线：ConstructGateCmd（construct_arches 的
//! ConstructGateCmd120 产出）→ 完整拥有的 materialized segment/wall/ordinal
//! → `construct_gate_cpu`（systems/wall/construction/gates.rs，内部按墙高比选
//! construct_wooden_gate_doors 或 nani 分支）→ GateCpuOutput：
//! - bricks（96B）→ WallConstructorOutput；
//! - leaves（WoodenGateProjection：mesh 顶点/top_positions/端点方向/
//!   wall_id/seed/29a）→ 136B spawn payload（TransientMesh 0..16、
//!   变换 16..56、mesh 64、shader 72、top-position 80..104、WallId 104、
//!   端点方向 112..128、seed 128、+29a 132）→ 门实体 + Gates 表；
//! - nanis → actual NaniBundle<SolidVertexColorInstanceData> entities.
//! seed = wrapping `WallId::get_inner() * ordinal + leaf.ordinal` cast u32；
//! shader 键 = `solid_vertex_color_wooden_gate`。
use bevy_ecs::{event::Event, prelude::*};
use country_core::asset_libraries::{
    mesh_library::AssetMeshLibrary, ssbo_library::AssetSsboLibrary,
};
use country_core::resources::walls::publication_fields::WallId as ObservedWallId;
use country_core::resources::walls::publication_source::TerrainHeightGrid512;
use country_core::resources::{
    roofs::commands::{CreateRoofCmd, SwitchRoofTypeCmd},
    terrain_height::TerrainHeightsData,
    walls::{
        WallId,
        public_walls::{OnWallChanged, OnWallDeleted, PublicWalls},
        wooden_gates::Gates,
    },
};
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::wall::construction::gates::GateConstructionInput;
use country_core::systems::wall::construction::gates::construct_gate_cpu;
use country_core::systems::wall::construction::structures::MaterializedWallSegment;

/// Owned semantic form of native120B {MaterializedWallSegment, WallId, ordinal}.
/// The three Vec fields stay owned values; no native pointers are serialized.
#[derive(Event, Clone, Debug)]
pub struct ConstructGateCmd {
    pub wall: u64,
    pub ordinal: u64,
    pub segment: MaterializedWallSegment,
}

/// Full leaf metadata retained independently of the renderer's two-field view.
/// Module ownership is an integration design choice; the original component
/// contains the top-position Vec, WallId, endpoint directions, seed and version.
pub use country_core::components::WoodenGate as WoodenGateState;

/// B 0x140756370：9 参数面（顺序按 G06 findings）。
pub fn construct_gates(
    mut requests: EventReader<ConstructGateCmd>,
    mut gates: ResMut<Gates>,
    mut commands: Commands,
    meshes: Res<AssetMeshLibrary>,
    shaders: Res<country_core::asset_libraries::shader_library::SharedAssetShaderLibrary>,
    walls: Res<PublicWalls>,
    terrain: Res<TerrainHeightsData>,
    mut ssbo: ResMut<AssetSsboLibrary>,
    _signifiers: Res<UiSignifierStream>,
    mut output: ResMut<crate::construct_arches::WallConstructorOutput>,
) {
    let mut exhausted = false;
    // TerrainHeightGrid512（512×512 u16 + 高度界；采样修复标志按证据登记）。
    let terrain_grid = TerrainHeightGrid512 {
        encoded: std::sync::Arc::from(terrain.heights()),
        minimum_y: terrain.min_height(),
        maximum_y: terrain.max_height(),
        sampling_fix: utils::float_grid::CENTERED_PATH.load(std::sync::atomic::Ordering::Relaxed),
    };
    for request in requests.read() {
        let wall = request.wall;
        let ordinal = request.ordinal;
        let id = WallId(wall);
        let Some(wall_state) = walls.get(id) else {
            continue;
        };
        let projection = wall_state.building_projection();
        let path = wall_state
            .prepared_path
            .as_ref()
            .expect("gate construction requires prepared wall path");
        let input = GateConstructionInput {
            segment: &request.segment,
            prepared_positions: &path.positions,
            prepared_u: &path.points_u,
            observed_public_288: projection.float_0x288(),
            observed_public_29a: projection.observed_29a(),
            wall_id: ObservedWallId(wall),
            ordinal,
        };
        let outcome = construct_gate_cpu(input, &terrain_grid)
            .expect("invalid or unrecovered gate construction input");
        let storage = ssbo
            .get_mut::<country_core::geometry::instanced_wall::BrickTransformSsbo>()
            .expect("canonical BrickTransformSsbo must exist before gate construction");
        exhausted |= output.gpu.append_bricks(
            storage,
            crate::gpu_output::ConstructorKey {
                wall,
                part: crate::gpu_output::ConstructorPart::Gates,
            },
            &outcome.bricks,
        ) != 0;
        for leaf in outcome.leaves {
            use country_core::asset_libraries::{
                mesh_library::MeshAsset, transient_mesh::TransientMesh,
            };
            use country_core::components::drawable::DrawableMeshBundle;
            use country_core::render::mesh::{
                ATTR_COLOR, ATTR_NORMAL, ATTR_POSITION, ATTR_UV, Mesh,
            };
            use country_core::utils::components::transform::Transform;
            let mut mesh = Mesh::new("wooden_gate");
            mesh.indices = leaf.mesh.indices;
            mesh.set_attribute(
                ATTR_POSITION,
                leaf.mesh
                    .positions
                    .into_iter()
                    .map(glam::Vec3::from_array)
                    .collect::<Vec<_>>(),
            );
            mesh.set_attribute(
                ATTR_NORMAL,
                leaf.mesh
                    .normals
                    .into_iter()
                    .map(glam::Vec3::from_array)
                    .collect::<Vec<_>>(),
            );
            mesh.set_attribute(
                ATTR_COLOR,
                leaf.mesh
                    .colors
                    .into_iter()
                    .map(glam::Vec3::from_array)
                    .collect::<Vec<_>>(),
            );
            mesh.set_attribute(
                ATTR_UV,
                leaf.mesh
                    .uvs
                    .into_iter()
                    .map(glam::Vec2::from_array)
                    .collect::<Vec<_>>(),
            );
            let handle = meshes.add(MeshAsset::new(mesh, None));
            let shader = shaders
                .0
                .get_raster_handle_by_name("solid_vertex_color_wooden_gate")
                .expect("wooden gate shader must be loaded");
            let entity = commands
                .spawn((
                    DrawableMeshBundle {
                        mesh: handle,
                        shader,
                        transform: Transform::identity(),
                    },
                    TransientMesh::new_with_registry(handle, meshes.transient_meshes()),
                    WoodenGateState {
                        top_positions: leaf.top_positions,
                        wall: id,
                        endpoint_directions: leaf.endpoint_directions,
                        seed: leaf.seed,
                        clutter_version: leaf.observed_public_29a,
                    },
                ))
                .id();
            gates.for_wall_mut(id).push(entity.to_bits());
        }
        for nani in outcome.nanis {
            use country_core::{
                components::drawable::NaniBundle,
                name_hash::NameHash,
                render::nani::{
                    gpu_world::{GpuWorldSubsetKind, InstanceDataMarker},
                    instance_data::SolidVertexColorInstanceData,
                    systems::{NaniInstanceDesc, NaniMeshSelector},
                },
                utils::components::transform::Transform,
            };
            assert_eq!(nani.observed_word_30, 0);
            assert_eq!(nani.subset_kind, 3);
            let entity = commands
                .spawn(NaniBundle::<SolidVertexColorInstanceData> {
                    desc: NaniInstanceDesc {
                        mesh: NaniMeshSelector::Named(NameHash(nani.definition_id)),
                        subset_kind: GpuWorldSubsetKind::SolidVertexColor,
                    },
                    transform: Transform {
                        rotation: glam::Quat::from_array(nani.rotation),
                        translation: glam::Vec3::from_array(nani.position),
                        scale: glam::Vec3::from_array(nani.scale),
                    },
                    marker: InstanceDataMarker::default(),
                })
                .id();
            gates.for_wall_mut(id).push(entity.to_bits());
        }
        let _ = &ssbo;
    }
    if exhausted {
        log::error!(target:"system_wall_constructor::construct_gates",
            "Gate constructor ran out of bricks");
    }
}

/// B 0x140755690：6 组合 despawn 顺序（changed/create/switch rebuild +
/// wall/roof removal）；重建保留空条目。
pub fn update_gates_hashmap(
    mut changed: EventReader<OnWallChanged>,
    mut create_roof: EventReader<CreateRoofCmd>,
    mut switch_type: EventReader<SwitchRoofTypeCmd>,
    mut deleted: EventReader<OnWallDeleted>,
    mut delete_roof: EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,
    mut gates: ResMut<Gates>,
    mut commands: Commands,
) {
    let rebuild = changed
        .read()
        .map(|e| e.0)
        .chain(create_roof.read().map(|e| WallId(e.0.wall_id().0)))
        .chain(switch_type.read().map(|e| e.wall));
    for wall in rebuild {
        for entity in gates.for_wall_mut(wall).drain(..) {
            commands.entity(Entity::from_bits(entity)).despawn();
        }
    }
    for wall in deleted
        .read()
        .map(|e| e.0)
        .chain(delete_roof.read().map(|e| e.0))
    {
        if let Some(entities) = gates.per_wall.remove(&wall) {
            for entity in entities {
                commands.entity(Entity::from_bits(entity)).despawn();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::roofs::commands::DeleteRoofCmd;
    #[test]
    fn gate_rebuild_and_roof_delete_despawn_tracked_entities_in_real_world() {
        let mut world = World::new();
        world.init_resource::<Gates>();
        world.init_resource::<Events<OnWallChanged>>();
        world.init_resource::<Events<CreateRoofCmd>>();
        world.init_resource::<Events<SwitchRoofTypeCmd>>();
        world.init_resource::<Events<OnWallDeleted>>();
        world.init_resource::<Events<DeleteRoofCmd>>();
        let wall = WallId(12345);
        let leaves = [world.spawn_empty().id(), world.spawn_empty().id()];
        world
            .resource_mut::<Gates>()
            .per_wall
            .insert(wall, leaves.iter().map(|e| e.to_bits()).collect());
        world
            .resource_mut::<Events<OnWallChanged>>()
            .send(OnWallChanged(wall));
        let mut schedule = bevy_ecs::schedule::Schedule::default();
        schedule.add_systems(update_gates_hashmap);
        schedule.run(&mut world);
        assert!(leaves.iter().all(|&e| world.get_entity(e).is_err()));
        assert!(world.resource::<Gates>().per_wall[&wall].is_empty());
        let replacement = world.spawn_empty().id();
        world
            .resource_mut::<Gates>()
            .for_wall_mut(wall)
            .push(replacement.to_bits());
        world
            .resource_mut::<Events<DeleteRoofCmd>>()
            .send(DeleteRoofCmd(wall));
        schedule.run(&mut world);
        assert!(world.get_entity(replacement).is_err());
        assert!(!world.resource::<Gates>().per_wall.contains_key(&wall));
    }
}
