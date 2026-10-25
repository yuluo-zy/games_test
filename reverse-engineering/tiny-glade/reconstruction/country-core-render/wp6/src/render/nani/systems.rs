//! systems.rs — nani GPU 世界的 ECS 维护/更新/冲刷系统（WP6 完整版）。
//!
//! 机器码锚点（B patched EXE，cf048e27...）：
//! - `nani_maintain_instances` 0x14206f300（行 36；closure$1 0x141565950 行 84）。
//! - `nani_update_*` 20 个系统：0x142070ac0..0x14207fcc0（地址表见 contracts/systems.md §3）。
//! - `nani_flush_instance_updates` 0x142080640（行 1011）——WP3 未列出的第 21 个系统
//!   （本包新恢复；见 contracts/systems.md §5）。
//!
//! ECS 系统签名完整来自 PDB `FunctionSystem<void(*)(...)>` 单态化
//! （证据 `wp3/evidence/ecs-system-signatures.txt`，24 条唯一签名；flush 的签名由
//! 反编译参数推断，见 §5）。
//!
//! 业务逻辑标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。
//! 完整契约见 contracts/systems.md。

#![allow(dead_code, unused_imports)]

use bevy_ecs::change_detection::{DetectChanges, Res, ResMut};
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Changed, Or, With, Without};
use bevy_ecs::removal_detection::RemovedComponents;
use bevy_ecs::system::{Commands, Local, Query};
use hashbrown::HashMap;

use crate::components::drawable::{NaniActiveInstance, NaniInstanceDesc};
use crate::render::nani::gpu_world::{
    GpuWorld, GpuWorldSubsetKind, InstanceDataMarker, InstanceHandle, InstanceKey,
    OwnedGpuWorldSubset,
};
use crate::render::nani::instance_data::*;
use crate::resources::render::mesh_atlas_library::MeshAtlasLibrary;
use crate::utils::components::transform::Transform;

/// u8 → `GpuWorldSubsetKind`（O：27 个变体 u8 判别；`GpuWorldSubsetKind::all` 同构）。
fn kind_from_u8(v: u8) -> GpuWorldSubsetKind {
    if v < 27 {
        unsafe { core::mem::transmute::<u8, GpuWorldSubsetKind>(v) }
    } else {
        todo!("invalid subset kind u8 {v}")
    }
}

// ===========================================================================
// `nani_maintain_instances`（0x14206f300，行 36）
// ===========================================================================

/// `nani_maintain_instances` 的 `Local` 状态（O）。
#[derive(Default)]
pub struct NaniActiveInstanceCollection {
    /// Entity → 实例句柄（O：`HashMap<Entity, InstanceHandle>::insert` /
    /// `remove_entry<Entity, InstanceHandle>`）。
    pub entity_to_handle: HashMap<Entity, InstanceHandle>,
    /// typed gap：每 kind 的变更 tick 切片/迭代缓存缓冲 U。
    pub _kind_cache: crate::Gap,
}

/// 维护系统（O，完整 ECS 签名）：
/// 新实例（无 NaniActiveInstance）→ create；NaniInstanceDesc 变更 → update_instance_mesh；
/// NaniActiveInstance 移除 → remove_instance。
///
/// 业务逻辑（O）：
/// 1. puffin 作用域（系统名经 rfind/UTF-8 解码，同 `nani_update_*` 模式）。
/// 2. **新实例**（`Without<NaniActiveInstance>` 查询）：每实体经
///    `MeshAtlasLibrary::get_by_name<NameHash>` 取 atlas part（缺失 → panic）；
///    计算 `InstanceKey`（含 f16 sort_key，`half::f16::from_f32`）；
///    `GpuWorld::owned_subset_mut(kind)` + `OwnedGpuWorldSubset::extend_no_data(keys,
///    callback, kind)` 建实例；`local.entity_to_handle.insert(entity, handle)`；
///    `Commands` push `Insert<NaniActiveInstance>`（28B command，O）。
/// 3. **删除**（`RemovedComponents<NaniActiveInstance>`）：`local.remove_entry` →
///    `GpuWorld::remove_instance(key)`。
/// 4. **变更**（`Changed<NaniInstanceDesc>`）：经
///    `Table::get_changed_ticks_slice_for` 遍历变更实体 →
///    `GpuWorld::update_instance_mesh(entity, desc, transform)`。
///
/// 上限（O）：`extend_no_data` 内部 `instance_count >= max_instances[kind]` →
/// panic "Max instance count reached for {kind}"；本系统容量用尽 → panic
/// "Instance limit reached while trying ..."（格式串经 `format_inner` + `log` 路径）。
pub fn nani_maintain_instances(
    new: Query<(Entity, &NaniInstanceDesc, &Transform), Without<NaniActiveInstance>>,
    changed: Query<(Entity, &NaniInstanceDesc, &NaniActiveInstance), Changed<NaniInstanceDesc>>,
    mut removed: RemovedComponents<NaniActiveInstance>,
    mesh_atlas: Res<MeshAtlasLibrary>,
    mut gpu_world: ResMut<GpuWorld>,
    mut local: Local<NaniActiveInstanceCollection>,
    mut commands: Commands,
) {
    let _scope = crate::puffin_contract::scope("nani_maintain_instances");

    // 2. 新实例：atlas part 查找 → InstanceKey → extend_no_data → Local 登记 → 插组件。
    for (entity, desc, transform) in new.iter() {
        // O：`MeshAtlasLibrary::get_by_name<NameHash>`；缺失 → `panic_fmt`。
        let part = mesh_atlas.get_by_name_hash(desc.mesh_name_hash());
        // O：InstanceKey 计算（chunk_id 来源 I；sort_key f16）。
        let key = crate::render::nani::systems::instance_key_from_desc(entity, desc, transform, part);
        let kind = kind_from_u8(desc.subset_kind);
        let mut guard = gpu_world.owned_subset_mut(kind).write();
        if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) =
            guard.as_mut()
        {
            // O：`extend_no_data(keys, callback, kind)`；callback 收 `(instance_index, kind)`。
            subset.extend_no_data(
                core::slice::from_ref(&key),
                kind,
                |packed: u64| {
                    let _ = packed;
                    // O：Local `HashMap<Entity, InstanceHandle>::insert`；U：packed 解码。
                    let _ = local.entity_to_handle.insert(
                        entity,
                        InstanceHandle {
                            kind,
                            index: key.instance_index,
                        },
                    );
                    // U：Commands `Insert<NaniActiveInstance>`（28B command；字段见 §2 契约）。
                    commands
                        .entity(entity)
                        .insert(crate::components::drawable::NaniActiveInstance::from_key(key, kind));
                },
            );
        }
        // U：上限 panic "Instance limit reached while trying ..."（extend_no_data 之外）。
    }

    // 3. 删除：Local 记录 → GpuWorld::remove_instance。
    for entity in removed.read() {
        if let Some((_, handle)) = local.entity_to_handle.remove_entry(&entity) {
            // O：`GpuWorld::remove_instance(key)`；key 由 handle 构造（U：chunk_id 来源）。
            let key = instance_key_from_handle(handle);
            gpu_world.remove_instance(key);
        }
    }

    // 4. 变更：`Table::get_changed_ticks_slice_for` → `GpuWorld::update_instance_mesh`。
    // 注：PDB 签名 changed 查询为 `(NaniInstanceDesc, NaniActiveInstance)`（无 Entity）；
    // 装配版补 Entity 以对接 `GpuWorld::update_instance_mesh(entity)`（I，见契约 §2）。
    for (entity, _desc, _active) in changed.iter() {
        gpu_world.update_instance_mesh(entity);
    }
}

/// 从 desc/transform/atlas-part 构造 `InstanceKey`（O：maintain 的 f16 sort_key +
/// chunk_id/instance_index 编码；字段语义 I）。
pub fn instance_key_from_desc(
    _entity: Entity,
    _desc: &NaniInstanceDesc,
    _transform: &Transform,
    _part: crate::render::nani::mesh_atlas::MeshAtlasPart,
) -> InstanceKey {
    todo!("recover InstanceKey derivation (chunk_id + instance_index + f16 sort_key); see contracts/systems.md §2")
}

/// 从 `InstanceHandle` 构造 `InstanceKey`（O：remove 路径；chunk_id 来源 U）。
pub fn instance_key_from_handle(_handle: InstanceHandle) -> InstanceKey {
    todo!("recover remove-instance key construction")
}

// ===========================================================================
// `nani_update_<T>` 系统族（×20，O，统一模式）
// ===========================================================================
//
// 统一模式（以 `nani_update_plant` 0x142074740 为 O 代表）：
// 1. puffin 作用域。
// 2. `GpuWorld::owned_subset_mut(kind)` 拿写锁（每系统一个 kind）。
// 3. 经 `Table::get_changed_ticks_slice_for` 迭代已变更实例；每实例从 ECS 组件计算
//    `T` → `OwnedGpuWorldSubset::update_instance_data<T>(key, data)`。
// 4. 释放锁（parking_lot deadlock_impl release + RawRwLock unlock）；结束作用域。

/// `nani_update_solid_vertex_color` 0x142070ac0（行 119）。
/// ECS：`Query<(&NaniActiveInstance, &Transform, Option<&SemiTransparent>,
/// Option<&FlatLighting>), With<...SolidVertexColorInstanceData>>` + `Res<GpuWorld>`。
pub fn nani_update_solid_vertex_color(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            Option<&crate::components::SemiTransparent>,
            Option<&crate::components::FlatLighting>,
        ),
        With<InstanceDataMarker<SolidVertexColorInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_solid_vertex_color");
    let kind = kind_from_u8(0); // I: Main kind
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, transform, semi, flat) in q.iter() {
            let _ = (transform, semi, flat);
            let key = active.into_key();
            let data: SolidVertexColorInstanceData = todo!("component → 0x40 instance data");
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_solid_vertex_color_window` 0x142071590（行 152）。
pub fn nani_update_solid_vertex_color_window(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            &crate::resources::color_id::ColorId,
            Option<&crate::components::SemiTransparent>,
        ),
        With<InstanceDataMarker<SolidVertexColorWindowInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_solid_vertex_color_window");
    let kind = kind_from_u8(9); // U: kind 常量
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, _transform, _color_id, _semi) in q.iter() {
            let key = active.into_key();
            let data: SolidVertexColorWindowInstanceData = todo!();
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_solid_albedo_map` 0x142072010（行 185）。
pub fn nani_update_solid_albedo_map(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            &crate::components::drawable::NaniSolidTexture,
        ),
        With<InstanceDataMarker<SolidAlbedoMapInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_solid_albedo_map");
    let kind = kind_from_u8(9); // U
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, _transform, _texture) in q.iter() {
            let key = active.into_key();
            let data: SolidAlbedoMapInstanceData = todo!();
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_solid_albedo_map_emissive` 0x142072b50（行 223）。
pub fn nani_update_solid_albedo_map_emissive(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            &crate::components::drawable::NaniSolidTexture,
        ),
        With<InstanceDataMarker<SolidAlbedoMapEmissiveInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_solid_albedo_map_emissive");
    let kind = kind_from_u8(9); // U
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, _transform, _texture) in q.iter() {
            let key = active.into_key();
            let data: SolidAlbedoMapEmissiveInstanceData = todo!();
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_preplaced_flower` 0x142073690（行 261）。
///
/// O 业务逻辑（非统一模式）：
/// 1. 经 `AssetSsboLibrary::get<SsboId>` 取花位图 SSBO（BufferState；`assert_failed`
///    校验 name）。
/// 2. 每变更实例：从 Transform 位置算 512×512 网格 UV（`x/130 + 0.5` 双分量 → clamp →
///    `*512` → 格点）；查位图 bit（`uVar15 = (y<<9|x)>>5`，`bit = (word >> (x&31)) & 1`）。
/// 3. 若 bit 为 0 且 `bilinear_sample_at_world_xz<RasterBufferRef<512>>` 4 邻居高度和
///    `<= 0.01`（水阈值）→ 写 `PreplacedFlowerInstanceData`（真实 transform）；
///    否则写 `Transform::default()`（隐藏，scale=0）。
pub fn nani_update_preplaced_flower(
    q: Query<(&NaniActiveInstance, &Transform), With<InstanceDataMarker<PreplacedFlowerInstanceData>>>,
    gpu_world: Res<GpuWorld>,
    ssbo: Res<crate::asset_libraries::ssbo_library::AssetSsboLibrary>,
    path: crate::systems::paths::draw_path::PathRastersResources,
    water: Res<crate::resources::world_raster_defs::WaterRaster>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_preplaced_flower");
    let kind = kind_from_u8(3); // I: Flowers kind（cull shader `_nani_cull_and_bucket_flowers`）
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, transform) in q.iter() {
            let _ = (active, transform);
            // O 路径：花位图 bit 检查 + 水高度检查（≤0.01）→ 显示；否则隐藏。
            let key = active.into_key();
            let data = preplaced_flower_instance_data(&ssbo, &path, &water, transform);
            subset.update_instance_data(key, data);
        }
    }
}

/// 花实例数据计算（O 结构与阈值；精确字段 U）。
fn preplaced_flower_instance_data(
    _ssbo: &crate::asset_libraries::ssbo_library::AssetSsboLibrary,
    _path: &crate::systems::paths::draw_path::PathRastersResources,
    _water: &crate::resources::world_raster_defs::WaterRaster,
    _transform: &Transform,
) -> PreplacedFlowerInstanceData {
    // O：`x/130.0 + 0.5` → clamp[0,1] → `*512.0` → 格点；bit 掩码与高度采样。
    // U：花位图 SSBO 句柄与 RasterBufferRef<512> 的精确采样字段。
    todo!("recover flower bitmap + water-height gating; see contracts/systems.md §3")
}

/// `nani_update_plant` 0x142074740（行 313）—— O 代表。
pub fn nani_update_plant(
    q: Query<(&NaniActiveInstance, &Transform), With<InstanceDataMarker<PlantInstanceData>>>,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_plant");
    let kind = kind_from_u8(9); // U
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, transform) in q.iter() {
            let _ = transform;
            let key = active.into_key();
            let data: PlantInstanceData = todo!("transform → PlantInstanceData (0x30)");
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_solid_vertex_color_window_glass` 0x142075070（行 337）。
pub fn nani_update_solid_vertex_color_window_glass(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            &crate::resources::window_lights::WindowLight,
            &crate::resources::color_id::ColorId,
            Option<&crate::components::SemiTransparent>,
            Option<&crate::components::drawable::AlbedoMap>,
        ),
        With<InstanceDataMarker<SolidVertexColorWindowGlassInstanceData>>,
    >,
    window_lights: Res<crate::resources::window_lights::GlobalWindowLightsControl>,
    textures: Res<crate::asset_libraries::texture_library::AssetTextureLibrary>,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_solid_vertex_color_window_glass");
    let _ = (window_lights, textures);
    let kind = kind_from_u8(9); // U
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, _t, _light, _color, _semi, _albedo) in q.iter() {
            let key = active.into_key();
            let data: SolidVertexColorWindowGlassInstanceData = todo!();
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_solid_vertex_color_bricks` 0x142076230（行 393）。
pub fn nani_update_solid_vertex_color_bricks(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            &crate::components::BrickColor,
        ),
        With<InstanceDataMarker<SolidVertexColorBricksInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_solid_vertex_color_bricks");
    let kind = kind_from_u8(9); // U
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, _transform, _brick_color) in q.iter() {
            let key = active.into_key();
            let data: SolidVertexColorBricksInstanceData = todo!();
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_gothic_window_bricks` 0x142076e40（行 425）。
pub fn nani_update_gothic_window_bricks(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            &crate::components::BrickColor,
            &crate::components::GothicWindowBrickDeformation,
        ),
        With<InstanceDataMarker<GothicWindowBricksInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_gothic_window_bricks");
    let kind = kind_from_u8(9); // U
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, _t, _brick, _deform) in q.iter() {
            let key = active.into_key();
            let data: GothicWindowBricksInstanceData = todo!();
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_curved_halftimber` 0x142077d20（&mut 实例数据直写；唯一非复制式 update）。
///
/// O 业务逻辑：读 `CurvedHalfTimber`（墙几何）、`PublicWalls`、`HalftimberColorIds`、
/// `Query<&Roof>`；经 16 段 cap 掩码（u128 位提取，`auVar45[i] = -(byte==x)`）更新
/// `CurvedHalfTimberInstanceData` 的端帽标志并写回（`&mut` 直写）。
pub fn nani_update_curved_halftimber(
    mut q: Query<
        (
            &NaniActiveInstance,
            &mut CurvedHalfTimberInstanceData,
            &crate::resources::walls::curved_halftimber::CurvedHalfTimber,
        ),
        With<InstanceDataMarker<CurvedHalfTimberInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
    public_walls: Res<crate::resources::walls::public_walls::PublicWalls>,
    color_ids: Res<crate::resources::color_id::HalftimberColorIds>,
    roofs: Query<&crate::resources::roofs::roof_shape::Roof>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_curved_halftimber");
    let _ = (public_walls, color_ids, roofs);
    let kind = GpuWorldSubsetKind::CurvedHalftimber;
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, mut instance_data, curved) in q.iter_mut() {
            let _ = (active, curved);
            // O：16 段 cap 掩码位提取 → 更新 instance_data（packed 字段）。
            update_curved_halftimber_caps(&mut *instance_data);
            let key = active.into_key();
            let data = *instance_data;
            subset.update_instance_data(key, data);
        }
    }
}

/// 端帽掩码更新（O：0x142077d20 的 u128 位提取 `auVar45[i]=-(byte==x)`；精确字段 U）。
fn update_curved_halftimber_caps(_data: &mut CurvedHalfTimberInstanceData) {
    todo!("recover cap-mask update; see contracts/systems.md §3")
}

/// `nani_update_stone_floor` 0x142078780（O；`Option<&Roof>`/`Option<&RoofAnimation>`/
/// `Option<&Floor>`）。
///
/// O 业务逻辑：动态组件位（+0x120/0x128/0x130）检测 Roof/RoofAnimation/Floor 存在性；
/// 读屋顶数据（stride 0x58）、floor 数据（stride 0x28）、roof_animation（stride 0x8）；
/// `+8` 字节 bit0 判定是否走 `Circle2d::get_tangent_at_u()`（弯曲屋檐切线）；
/// 之后 `update_instance_data<StoneFloorInstanceData>`。
pub fn nani_update_stone_floor(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            Option<&crate::resources::roofs::roof_shape::Roof>,
            Option<&crate::resources::roofs::roof_animation::RoofAnimation>,
            Option<&crate::resources::walls::Floor>,
        ),
        With<InstanceDataMarker<StoneFloorInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_stone_floor");
    let kind = kind_from_u8(9); // U
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, _transform, _roof, _anim, _floor) in q.iter() {
            let key = active.into_key();
            let data: StoneFloorInstanceData = todo!("roof tangent + animation → stone floor");
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_wren` 0x1420797b0（行 671）。
pub fn nani_update_wren(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            &crate::components::drawable::PrevTransform,
            &crate::resources::critters::Wren,
        ),
        With<InstanceDataMarker<WrenInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_wren");
    let kind = kind_from_u8(9); // U
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, _t, _prev, _wren) in q.iter() {
            let key = active.into_key();
            let data: WrenInstanceData = todo!("wren pose (prev/current + critter params)");
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_lantern_emissive` 0x14207a4f0（行 700）。
pub fn nani_update_lantern_emissive(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            &crate::components::LanternLightParams,
            Option<&crate::components::SemiTransparent>,
            Option<&crate::resources::color_id::ColorId>,
        ),
        With<InstanceDataMarker<LanternEmissiveInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_lantern_emissive");
    let kind = kind_from_u8(9); // U
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, _t, _params, _semi, _color) in q.iter() {
            let key = active.into_key();
            let data: LanternEmissiveInstanceData = todo!();
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_plaster` 0x14207b290（行 750）。
/// ECS：`Query<(&NaniActiveInstance, &Plaster), Or<(Changed<Plaster>, Changed<NaniActiveInstance>)>>`
/// + `Res<PublicWalls>` + `Res<GpuWorld>` + `Res<AnimatedPlasterPeel>`。
pub fn nani_update_plaster(
    q: Query<
        (
            &NaniActiveInstance,
            &crate::resources::walls::plaster::Plaster,
        ),
        (
            With<InstanceDataMarker<PlasterInstanceData>>,
            Or<(
                Changed<crate::resources::walls::plaster::Plaster>,
                Changed<NaniActiveInstance>,
            )>,
        ),
    >,
    public_walls: Res<crate::resources::walls::public_walls::PublicWalls>,
    gpu_world: Res<GpuWorld>,
    animated: Res<crate::resources::walls::plaster::AnimatedPlasterPeel>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_plaster");
    let _ = (public_walls, animated);
    let kind = GpuWorldSubsetKind::Plaster;
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, _plaster) in q.iter() {
            let key = active.into_key();
            let data: PlasterInstanceData = todo!("plaster peel animation → 0x30");
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_cloth` 0x14207c0a0（行 800）。
pub fn nani_update_cloth(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            &crate::components::ClothlineCloth,
            &crate::components::NormalMap,
        ),
        With<InstanceDataMarker<ClothInstanceData>>,
    >,
    textures: Res<crate::asset_libraries::texture_library::AssetTextureLibrary>,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_cloth");
    let _ = textures;
    let kind = kind_from_u8(9); // U
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, _t, _cloth, _normal) in q.iter() {
            let key = active.into_key();
            let data: ClothInstanceData = todo!();
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_smoke` 0x14207cd60（行 835）。
pub fn nani_update_smoke(
    q: Query<(&NaniActiveInstance, &Transform, &crate::components::Smoke), With<InstanceDataMarker<SmokeInstanceData>>>,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_smoke");
    let kind = GpuWorldSubsetKind::Smoke;
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, _transform, _smoke) in q.iter() {
            let key = active.into_key();
            let data: SmokeInstanceData = todo!();
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_balcony_door` 0x14207d770（行 868）。
pub fn nani_update_balcony_door(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            &crate::resources::color_id::ColorId,
            Option<&crate::components::SemiTransparent>,
        ),
        With<InstanceDataMarker<BalconyDoorInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_balcony_door");
    let kind = kind_from_u8(9); // U
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, _t, _color, _semi) in q.iter() {
            let key = active.into_key();
            let data: BalconyDoorInstanceData = todo!();
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_uber` 0x14207e1f0（行 903）。
pub fn nani_update_uber(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            Option<&crate::components::SemiTransparent>,
            Option<&crate::components::drawable::AlbedoMap>,
        ),
        With<InstanceDataMarker<UberInstanceData>>,
    >,
    textures: Res<crate::asset_libraries::texture_library::AssetTextureLibrary>,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_uber");
    let _ = textures;
    let kind = kind_from_u8(9); // U
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, _t, _semi, _albedo) in q.iter() {
            let key = active.into_key();
            let data: UberInstanceData = todo!();
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_velocity_only` 0x14207f060（行 954）。
pub fn nani_update_velocity_only(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            &crate::components::drawable::PrevTransform,
        ),
        With<InstanceDataMarker<VelocityOnlyInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_velocity_only");
    let kind = kind_from_u8(9); // U
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, _t, _prev) in q.iter() {
            let key = active.into_key();
            let data: VelocityOnlyInstanceData = todo!("prev/current transform → 0x60");
            subset.update_instance_data(key, data);
        }
    }
}

/// `nani_update_stairs_floor` 0x14207fcc0（行 979）。
/// ECS：`Query<(&NaniActiveInstance, &StairsFloorMeshChunk),
/// Or<(Changed<StairsFloorMeshChunk>, Changed<NaniActiveInstance>)>>` + `Res<GpuWorld>`。
pub fn nani_update_stairs_floor(
    q: Query<
        (
            &NaniActiveInstance,
            &crate::resources::stairs::visual::StairsFloorMeshChunk,
        ),
        (
            With<InstanceDataMarker<StairsFloorInstanceData>>,
            Or<(
                Changed<crate::resources::stairs::visual::StairsFloorMeshChunk>,
                Changed<NaniActiveInstance>,
            )>,
        ),
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_stairs_floor");
    let kind = GpuWorldSubsetKind::StairsFloor;
    let mut guard = gpu_world.owned_subset_mut(kind).write();
    if let Some(crate::render::nani::gpu_world::GpuWorldSubset::Owned(subset)) = guard.as_mut() {
        for (active, _chunk) in q.iter() {
            let key = active.into_key();
            let data: StairsFloorInstanceData = todo!("mesh chunk → 0x30");
            subset.update_instance_data(key, data);
        }
    }
}

// ===========================================================================
// `nani_flush_instance_updates`（0x142080640，行 1011）— WP6 新恢复
// ===========================================================================

/// GPU 冲刷系统（O，行 1011；WP3 未列出）。
///
/// 反编译参数推断（I）：`fn nani_flush_instance_updates(mut gpu_world: ResMut<GpuWorld>)`
/// （无 Query；不是 `FunctionSystem<void(*)(Query, ...)>`，故不在 24 条 PDB 签名中）。
///
/// 业务逻辑（O）：
/// 1. puffin 作用域。
/// 2. 遍历 GpuWorld 的脏槽（30 个槽，stride 0xc0，从 +0x8 起；`for (0x1440; !=0; -=0xc0)`）：
///    若 `*(槽 -0x48)` 的脏标志通过哨兵判定 → `ShadowedBuffer<GpuInstance>::gpu_flush()`
///    + vtable +0x30 的"冲刷脏区间"回调（`(handle, base+0x10)`）。
/// 3. `*(+0x1440) = *( +0x1448)`；`*(+0x1440) = 0`——把本帧实例变更计数转移到
///    "已冲刷"计数并清零（对应 maintain 每建实例 `*(+0x1440)+=1`）。
pub fn nani_flush_instance_updates(mut gpu_world: ResMut<GpuWorld>) {
    let _scope = crate::puffin_contract::scope("nani_flush_instance_updates");
    let _ = &mut *gpu_world;
    // O：逐 kind 脏槽 gpu_flush + 帧计数搬移。
    todo!("recover per-kind ShadowedBuffer<GpuInstance> gpu_flush + counter swap; see contracts/systems.md §5")
}
