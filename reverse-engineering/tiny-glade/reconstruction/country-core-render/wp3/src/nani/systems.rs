//! systems.rs — nani GPU 世界的 ECS 维护/更新系统（WP3）。
//!
//! 机器码锚点（B patched EXE）：
//! - `nani_maintain_instances` 0x14206f300（closure$1 0x141565950）。
//! - `nani_update_*` 20 个系统：0x142070ac0..0x14207fcc0（地址表见 contracts/systems.md §3）。
//!
//! ECS 系统签名完整来自 PDB `FunctionSystem<void(*)(...)>` 单态化
//! （证据 `evidence/ecs-system-signatures.txt`，24 条唯一签名）。
//! 完整契约见 contracts/systems.md。

#![allow(dead_code, unused_imports)]

use bevy_ecs::change_detection::{DetectChanges, Res, ResMut};
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Or, With, Without};
use bevy_ecs::removal_detection::RemovedComponents;
use bevy_ecs::system::{Local, Query};
use bevy_ecs::system_param::SystemParam;
use bevy_ecs::world::Commands;
use hashbrown::HashMap;
use utils::components::transform::Transform;

use crate::components::drawable::{NaniActiveInstance, NaniInstanceDesc};
use crate::render::nani::gpu_world::{
    GpuWorld, GpuWorldSubsetKind, InstanceDataMarker, InstanceHandle,
};
use crate::render::nani::instance_data::*;
use crate::resources::render::mesh_atlas_library::MeshAtlasLibrary;

/// `nani_maintain_instances` 的 `Local` 状态（O）。
#[derive(Default)]
pub struct NaniActiveInstanceCollection {
    /// Entity → 实例句柄。
    pub entity_to_handle: HashMap<Entity, InstanceHandle>,
    // typed gap：每 kind 的变更 tick 切片/迭代缓存缓冲 U。
}

/// 维护系统（0x14206f300，行 36，O）：
/// 新实例（无 NaniActiveInstance）→ create；NaniInstanceDesc 变更 → update_instance_mesh；
/// NaniActiveInstance 移除 → remove_instance。
/// PDB 签名见 contracts/systems.md §2。
pub fn nani_maintain_instances(
    new: Query<(Entity, &NaniInstanceDesc, &Transform), Without<NaniActiveInstance>>,
    changed: Query<(&NaniInstanceDesc, &NaniActiveInstance), Changed<NaniInstanceDesc>>,
    removed: RemovedComponents<NaniActiveInstance>,
    mesh_atlas: Res<MeshAtlasLibrary>,
    mut gpu_world: ResMut<GpuWorld>,
    mut local: Local<NaniActiveInstanceCollection>,
    mut commands: Commands,
) {
    // O 路径：
    // 1. puffin 作用域。
    // 2. 新实例：MeshAtlasLibrary::get_by_name<NameHash> 取 atlas part → InstanceKey（f16 sort_key）→
    //    owned_subset_mut(kind) + extend_no_data → local.entity_to_handle.insert → Commands 插 NaniActiveInstance。
    // 3. 移除：local.remove_entry → GpuWorld::remove_instance。
    // 4. 变更：Table::get_changed_ticks_slice_for → GpuWorld::update_instance_mesh。
    // 上限：extend_no_data panic "Max instance count reached for ..."；
    //        本系统 panic "Instance limit reached while trying ..."。
    let _ = (new, changed, removed, mesh_atlas, gpu_world, local, commands);
    todo!("recover nani_maintain_instances; see contracts/systems.md §2")
}

// --- 20 个 nani_update_<T> 系统（统一模式：锁 subset → 遍历变更 → update_instance_data<T>）---

/// `nani_update_solid_vertex_color` 0x142070ac0（行 119）。
pub fn nani_update_solid_vertex_color(
    q: Query<(&NaniActiveInstance, &Transform, Option<&crate::components::SemiTransparent>, Option<&crate::components::FlatLighting>), With<InstanceDataMarker<SolidVertexColorInstanceData>>>,
    gpu_world: Res<GpuWorld>,
) {
    let _ = (q, gpu_world);
    todo!()
}

/// `nani_update_solid_vertex_color_window` 0x142071590（行 152）。
pub fn nani_update_solid_vertex_color_window(
    q: Query<(&NaniActiveInstance, &Transform, &crate::resources::color_id::ColorId, Option<&crate::components::SemiTransparent>), With<InstanceDataMarker<SolidVertexColorWindowInstanceData>>>,
    gpu_world: Res<GpuWorld>,
) {
    let _ = (q, gpu_world);
    todo!()
}

/// `nani_update_solid_albedo_map` 0x142072010（行 185）。
pub fn nani_update_solid_albedo_map(
    q: Query<(&NaniActiveInstance, &Transform, &crate::components::drawable::NaniSolidTexture), With<InstanceDataMarker<SolidAlbedoMapInstanceData>>>,
    gpu_world: Res<GpuWorld>,
) {
    let _ = (q, gpu_world);
    todo!()
}

/// `nani_update_solid_albedo_map_emissive` 0x142072b50（行 223）。
pub fn nani_update_solid_albedo_map_emissive(
    q: Query<(&NaniActiveInstance, &Transform, &crate::components::drawable::NaniSolidTexture), With<InstanceDataMarker<SolidAlbedoMapEmissiveInstanceData>>>,
    gpu_world: Res<GpuWorld>,
) {
    let _ = (q, gpu_world);
    todo!()
}

/// `nani_update_preplaced_flower` 0x142073690（行 261）。
pub fn nani_update_preplaced_flower(
    q: Query<(&NaniActiveInstance, &Transform), With<InstanceDataMarker<PreplacedFlowerInstanceData>>>,
    gpu_world: Res<GpuWorld>,
    ssbo: Res<crate::asset_libraries::ssbo_library::AssetSsboLibrary>,
    path: crate::systems::paths::draw_path::PathRastersResources,
    water: Res<crate::resources::world_raster_defs::WaterRaster>,
) {
    let _ = (q, gpu_world, ssbo, path, water);
    todo!()
}

/// `nani_update_plant` 0x142074740（行 313）—— O 代表（见 contracts/systems.md §3）。
pub fn nani_update_plant(
    q: Query<(&NaniActiveInstance, &Transform), With<InstanceDataMarker<PlantInstanceData>>>,
    gpu_world: Res<GpuWorld>,
) {
    // puffin 作用域 → owned_subset_mut(kind) → 逐变更实例 update_instance_data<PlantInstanceData>
    // → 释放锁 → end_scope。
    let _ = (q, gpu_world);
    todo!()
}

/// `nani_update_solid_vertex_color_window_glass` 0x142075070（行 337）。
pub fn nani_update_solid_vertex_color_window_glass(
    q: Query<(&NaniActiveInstance, &Transform, &crate::resources::window_lights::WindowLight, &crate::resources::color_id::ColorId, Option<&crate::components::SemiTransparent>, Option<&crate::components::drawable::AlbedoMap>), With<InstanceDataMarker<SolidVertexColorWindowGlassInstanceData>>>,
    window_lights: Res<crate::resources::window_lights::GlobalWindowLightsControl>,
    textures: Res<crate::asset_libraries::texture_library::AssetTextureLibrary>,
    gpu_world: Res<GpuWorld>,
) {
    let _ = (q, window_lights, textures, gpu_world);
    todo!()
}

/// `nani_update_solid_vertex_color_bricks` 0x142076230（行 393）。
pub fn nani_update_solid_vertex_color_bricks(
    q: Query<(&NaniActiveInstance, &Transform, &crate::components::BrickColor), With<InstanceDataMarker<SolidVertexColorBricksInstanceData>>>,
    gpu_world: Res<GpuWorld>,
) {
    let _ = (q, gpu_world);
    todo!()
}

/// `nani_update_gothic_window_bricks` 0x142076e40（行 425）。
pub fn nani_update_gothic_window_bricks(
    q: Query<(&NaniActiveInstance, &Transform, &crate::components::BrickColor, &crate::components::GothicWindowBrickDeformation), With<InstanceDataMarker<GothicWindowBricksInstanceData>>>,
    gpu_world: Res<GpuWorld>,
) {
    let _ = (q, gpu_world);
    todo!()
}

/// `nani_update_curved_halftimber` 0x142077d20（&mut 实例数据直写；PDB 签名见 contracts/systems.md §3）。
pub fn nani_update_curved_halftimber(
    q: Query<(&NaniActiveInstance, &mut CurvedHalfTimberInstanceData, &crate::resources::walls::curved_halftimber::CurvedHalfTimber), With<InstanceDataMarker<CurvedHalfTimberInstanceData>>>,
    gpu_world: Res<GpuWorld>,
    public_walls: Res<crate::resources::walls::public_walls::PublicWalls>,
    color_ids: Res<crate::resources::color_id::HalftimberColorIds>,
    roofs: Query<&crate::resources::roofs::roof_shape::Roof>,
) {
    let _ = (q, gpu_world, public_walls, color_ids, roofs);
    todo!()
}

/// `nani_update_stone_floor` 0x142078780。
pub fn nani_update_stone_floor(
    q: Query<(&NaniActiveInstance, &Transform, Option<&crate::resources::roofs::roof_shape::Roof>, Option<&crate::resources::roofs::roof_animation::RoofAnimation>, Option<&crate::resources::walls::Floor>), With<InstanceDataMarker<StoneFloorInstanceData>>>,
    gpu_world: Res<GpuWorld>,
) {
    let _ = (q, gpu_world);
    todo!()
}

/// `nani_update_wren` 0x1420797b0（行 671）。
pub fn nani_update_wren(
    q: Query<(&NaniActiveInstance, &Transform, &crate::components::drawable::PrevTransform, &crate::resources::critters::Wren), With<InstanceDataMarker<WrenInstanceData>>>,
    gpu_world: Res<GpuWorld>,
) {
    let _ = (q, gpu_world);
    todo!()
}

/// `nani_update_lantern_emissive` 0x14207a4f0（行 700）。
pub fn nani_update_lantern_emissive(
    q: Query<(&NaniActiveInstance, &Transform, &crate::components::LanternLightParams, Option<&crate::components::SemiTransparent>, Option<&crate::resources::color_id::ColorId>), With<InstanceDataMarker<LanternEmissiveInstanceData>>>,
    gpu_world: Res<GpuWorld>,
) {
    let _ = (q, gpu_world);
    todo!()
}

/// `nani_update_plaster` 0x14207b290（行 750）。
pub fn nani_update_plaster(
    q: Query<(&NaniActiveInstance, &crate::resources::walls::plaster::Plaster), (With<InstanceDataMarker<PlasterInstanceData>>, Or<(Changed<crate::resources::walls::plaster::Plaster>, Changed<NaniActiveInstance>)>)>,
    public_walls: Res<crate::resources::walls::public_walls::PublicWalls>,
    gpu_world: Res<GpuWorld>,
    animated: Res<crate::resources::walls::plaster::AnimatedPlasterPeel>,
) {
    let _ = (q, public_walls, gpu_world, animated);
    todo!()
}

/// `nani_update_cloth` 0x14207c0a0（行 800）。
pub fn nani_update_cloth(
    q: Query<(&NaniActiveInstance, &Transform, &crate::components::ClothlineCloth, &crate::components::NormalMap), With<InstanceDataMarker<ClothInstanceData>>>,
    textures: Res<crate::asset_libraries::texture_library::AssetTextureLibrary>,
    gpu_world: Res<GpuWorld>,
) {
    let _ = (q, textures, gpu_world);
    todo!()
}

/// `nani_update_smoke` 0x14207cd60（行 835）。
pub fn nani_update_smoke(
    q: Query<(&NaniActiveInstance, &Transform, &crate::components::Smoke), With<InstanceDataMarker<SmokeInstanceData>>>,
    gpu_world: Res<GpuWorld>,
) {
    let _ = (q, gpu_world);
    todo!()
}

/// `nani_update_balcony_door` 0x14207d770（行 868）。
pub fn nani_update_balcony_door(
    q: Query<(&NaniActiveInstance, &Transform, &crate::resources::color_id::ColorId, Option<&crate::components::SemiTransparent>), With<InstanceDataMarker<BalconyDoorInstanceData>>>,
    gpu_world: Res<GpuWorld>,
) {
    let _ = (q, gpu_world);
    todo!()
}

/// `nani_update_uber` 0x14207e1f0（行 903）。
pub fn nani_update_uber(
    q: Query<(&NaniActiveInstance, &Transform, Option<&crate::components::SemiTransparent>, Option<&crate::components::drawable::AlbedoMap>), With<InstanceDataMarker<UberInstanceData>>>,
    textures: Res<crate::asset_libraries::texture_library::AssetTextureLibrary>,
    gpu_world: Res<GpuWorld>,
) {
    let _ = (q, textures, gpu_world);
    todo!()
}

/// `nani_update_velocity_only` 0x14207f060（行 954）。
pub fn nani_update_velocity_only(
    q: Query<(&NaniActiveInstance, &Transform, &crate::components::drawable::PrevTransform), With<InstanceDataMarker<VelocityOnlyInstanceData>>>,
    gpu_world: Res<GpuWorld>,
) {
    let _ = (q, gpu_world);
    todo!()
}

/// `nani_update_stairs_floor` 0x14207fcc0（行 979）。
pub fn nani_update_stairs_floor(
    q: Query<(&NaniActiveInstance, &crate::resources::stairs::visual::StairsFloorMeshChunk), (With<InstanceDataMarker<StairsFloorInstanceData>>, Or<(Changed<crate::resources::stairs::visual::StairsFloorMeshChunk>, Changed<NaniActiveInstance>)>)>,
    gpu_world: Res<GpuWorld>,
) {
    let _ = (q, gpu_world);
    todo!()
}
