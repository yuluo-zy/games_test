//! `render::nani::systems` — nani 的 ECS 系统层：`nani_maintain_instances`、20 个 `nani_update_*`、
//! `nani_flush_instance_updates`（B 构建 0x14206f300 / 0x142070ac0..0x14207fcc0 / 0x142080640）。
//!
//! 原有行视图 API 与下方真实 ECS 适配器共用维护算法。保留九类既有组件映射，另接
//! canonical Cloth 绑定与限定 Roof-bearing RoofFloor 支线；绑定本身不证明独立场景 producer
//! 可达。Floor-only 与其余未闭合输入在发布前明确报错。载荷数值全部
//! 来自 [`super::instance_updates`] 的 `build_*`（逐机器码核对），这里负责三件事：
//! 1. 选 subset（每个系统固定一个 kind，见 [`GpuWorldSubsetKind`] 的判别值）；
//! 2. 用 `GpuWorld::owned_subset_mut(kind)` 取写锁，`update_instance_data` 写 `position / sort_key / 载荷`；
//! 3. 载荷字按该 kind 的实例数据类型（`instance_data.rs`）装箱。
//!
//! `nani_maintain_instances` 的关键事实（O，`B_n_sys.md §2` + 本次核对的字符串）：
//! * 先删除、后新建、最后变更；
//! * 删除：`Local` 表 `remove_entry` 取不到 → `Option::unwrap` panic（源码 51:41）；
//! * 新建：`NaniInstanceDesc` 的网格选择器为 `AtlasPart(u32)` 或 `Named(NameHash)`；`Named` 经
//!   `get_by_name` 线性找 `part.subset == subset_kind`，找不到 → panic
//!   `"NaniInstanceDesc requested mesh {} and shader {:?},\nbut this mesh is not registered for that shader in nani_meshes.ron"`
//!   （三段文本在 0x142fd3430 起的数据区，实读）；
//! * 紧凑 key = `{translation, f16(max|scale|), mesh_part as u16}`（[`compact_key_for`]）；
//!   `extend_no_data` 回调给出 12 B 记录；解锁后 `total_instance_count += 1`（**无条件**）；
//!   有记录 → `Local` 表登记并压 `Insert<NaniActiveInstance>`；没有（容量满）→
//!   `warn_once("Instance limit reached while trying to spawn {desc:?}")`，目标 `country_core::render::nani::systems`；
//! * 变更：`Named` 要求 `parts.len() == 1`（源码 106:17）且 `part.subset == kind`（108:17），
//!   然后 `update_instance_mesh(&mut key, part_index)`。
//!
//! 未闭合（U）：Floor-only 支线和 `Wren`、`Plaster`、`Smoke` 等组件/producer 的完整契约；行视图只放
//! `build_*` 已经需要的原始字。`NaniInstanceDesc` 在原版里是 24 B（`+0` 判别位、`AtlasPart` u32@+4、`Named` u64@+8、
//! `subset_kind` u8@+0x10），Rust 侧用枚举表达，不复刻内存布局。

use std::collections::HashMap;

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use glam::{Quat, Vec3};

use crate::name_hash::NameHash;
use crate::render::nani::gpu_world::{GpuWorld, GpuWorldSubsetKind, InstanceElement, InstanceKey};
use crate::render::nani::instance_data::*;
use crate::render::nani::instance_updates::*;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::resources::render::mesh_atlas_library::MeshAtlasLibrary;
use crate::utils::components::transform::Transform;

const LOG_TARGET: &str = "country_core::render::nani::systems";

/// 与 components::drawable 共用网格选择器，保持已有导入路径。
pub use crate::components::drawable::NaniInstanceMeshDesc as NaniMeshSelector;

/// 实例描述组件（O：24 B）。
#[derive(Component, Debug, PartialEq, Eq)]
pub struct NaniInstanceDesc {
    pub mesh: NaniMeshSelector,
    pub subset_kind: GpuWorldSubsetKind,
}

/// `maintain` 的 `Local` 状态（O：`Local<NaniActiveInstanceCollection>`，内部是 `HashMap<Entity, 12 B 记录>`）。
#[derive(Default, Debug)]
pub struct NaniActiveInstanceCollection {
    pub active: HashMap<Entity, InstanceKey>,
}

/// 新建查询的一项：`Query<(Entity, &NaniInstanceDesc, &Transform), Without<NaniActiveInstance>>`。
#[derive(Clone, Copy, Debug)]
pub struct NewInstanceRow<'a> {
    pub entity: Entity,
    pub desc: &'a NaniInstanceDesc,
    pub transform: &'a Transform,
}

/// 变更查询的一项：`Query<(&NaniInstanceDesc, &NaniActiveInstance), Changed<NaniInstanceDesc>>`（PDB 里没有 `Entity`）。
pub struct ChangedInstanceRow<'a> {
    pub desc: &'a NaniInstanceDesc,
    pub active: &'a mut InstanceKey,
}

fn describe_part_mismatch(name: NameHash, kind: GpuWorldSubsetKind) -> ! {
    panic!(
        "NaniInstanceDesc requested mesh {:?} and shader {:?},\nbut this mesh is not registered for that shader in nani_meshes.ron",
        name, kind
    )
}

fn remove_active_instance(
    entity: Entity, gpu_world: &GpuWorld, local: &mut NaniActiveInstanceCollection,
) {
    let (_, mut key) = local.active.remove_entry(&entity)
        .expect("called `Option::unwrap()` on a `None` value");
    gpu_world.remove_instance(&mut key);
}

fn create_active_instance(
    row: NewInstanceRow<'_>, atlas: &MeshAtlasLibrary, gpu_world: &mut GpuWorld,
    local: &mut NaniActiveInstanceCollection,
) -> Option<InstanceKey> {
    let kind = row.desc.subset_kind;
    let part = match row.desc.mesh {
        NaniMeshSelector::AtlasPart(part) => part,
        NaniMeshSelector::Named(name) => atlas.get_by_name(name).iter()
            .find(|part| part.subset == kind)
            .unwrap_or_else(|| describe_part_mismatch(name, kind)).part_index,
    };
    let compact = compact_key_for(row.transform, part);
    let mut record = None;
    gpu_world.owned_subset_mut(kind).extend_no_data(kind, &[compact], |key| record = Some(key));
    // Original counts the request, even if capacity prevents allocation.
    gpu_world.total_instance_count += 1;
    if let Some(key) = record {
        local.active.insert(row.entity, key);
    } else {
        nani_warn_once!(LOG_TARGET,
            format!("Instance limit reached while trying to spawn {:?}", row.desc));
    }
    record
}

fn change_active_instance_mesh(
    desc: &NaniInstanceDesc, active: &mut InstanceKey,
    atlas: &MeshAtlasLibrary, gpu_world: &mut GpuWorld,
) {
    let part = match desc.mesh {
        NaniMeshSelector::AtlasPart(part) => part,
        NaniMeshSelector::Named(name) => {
            let parts = atlas.get_by_name(name);
            assert_eq!(parts.len(), 1);
            assert_eq!(parts[0].subset, desc.subset_kind);
            parts[0].part_index
        }
    };
    gpu_world.update_instance_mesh(active, part as u16);
}

/// `nani_maintain_instances`（O 0x14206f300）. The slice adapter and real ECS
/// entry below share allocation/removal/mesh-change behavior.
pub fn nani_maintain_instances(
    removed: impl IntoIterator<Item = Entity>,
    new: &[NewInstanceRow<'_>], changed: &mut [ChangedInstanceRow<'_>],
    atlas: &MeshAtlasLibrary, gpu_world: &mut GpuWorld,
    local: &mut NaniActiveInstanceCollection,
) -> Vec<(Entity, InstanceKey)> {
    for entity in removed { remove_active_instance(entity, gpu_world, local); }
    let mut inserts = Vec::new();
    for row in new {
        if let Some(key) = create_active_instance(*row, atlas, gpu_world, local) {
            inserts.push((row.entity, key));
        }
    }
    for row in changed { change_active_instance_mesh(row.desc, row.active, atlas, gpu_world); }
    inserts
}

/// 把 `build_*` 的字载荷写进 `kind` 的 owned subset。
fn write_update<T: InstanceElement, const N: usize>(
    gpu_world: &GpuWorld,
    kind: GpuWorldSubsetKind,
    key: &mut InstanceKey,
    update: InstanceUpdate<N>,
    make: impl FnOnce([u32; N]) -> T,
) {
    gpu_world.owned_subset_mut(kind).update_instance_data(
        key,
        update.position,
        update.sort_key,
        make(update.words),
    );
}

fn trs(t: &Transform) -> (Vec3, Quat, Vec3) {
    (t.translation, t.rotation, t.scale)
}

/// 通用行：`(&NaniActiveInstance, &Transform)`。
pub struct TransformRow<'a> {
    pub key: &'a mut InstanceKey,
    pub transform: &'a Transform,
}

/// kind 3（0x142070ac0）：`SemiTransparent` / `FlatLighting` 是否在场。
pub struct SolidVertexColorRow<'a> {
    pub key: &'a mut InstanceKey,
    pub transform: &'a Transform,
    pub semi_transparent: bool,
    pub flat_lighting: bool,
}

pub fn nani_update_solid_vertex_color(gpu_world: &GpuWorld, rows: &mut [SolidVertexColorRow<'_>]) {
    for row in rows {
        let (p, r, s) = trs(row.transform);
        let update = build_solid_vertex_color(p, r, s, row.semi_transparent, row.flat_lighting);
        write_update(
            gpu_world,
            GpuWorldSubsetKind::SolidVertexColor,
            row.key,
            update,
            |raw| SolidVertexColorInstanceData { raw },
        );
    }
}

/// kind 4（0x142071590）、kind 0x16（阳台门 0x14207d770）共用：`ColorId` + `SemiTransparent`。
pub struct ColorIdRow<'a> {
    pub key: &'a mut InstanceKey,
    pub transform: &'a Transform,
    pub color_id: u32,
    pub semi_transparent: bool,
}

pub fn nani_update_solid_vertex_color_window(gpu_world: &GpuWorld, rows: &mut [ColorIdRow<'_>]) {
    for row in rows {
        let (p, r, s) = trs(row.transform);
        let update = build_solid_vertex_color_window(p, r, s, row.color_id, row.semi_transparent);
        write_update(
            gpu_world,
            GpuWorldSubsetKind::SolidVertexColorWindow,
            row.key,
            update,
            |raw| SolidVertexColorWindowInstanceData { raw },
        );
    }
}

pub fn nani_update_balcony_door(gpu_world: &GpuWorld, rows: &mut [ColorIdRow<'_>]) {
    for row in rows {
        let (p, r, s) = trs(row.transform);
        let update = build_balcony_door(r, p, s, row.color_id, row.semi_transparent);
        write_update(
            gpu_world,
            GpuWorldSubsetKind::BalconyDoor,
            row.key,
            update,
            |raw| BalconyDoorInstanceData { raw },
        );
    }
}

/// kind 0xf / 0x10（两个系统机器码同体）：`NaniSolidTexture` 的字节。
pub struct AlbedoMapRow<'a> {
    pub key: &'a mut InstanceKey,
    pub transform: &'a Transform,
    pub texture: u8,
}

pub fn nani_update_solid_albedo_map(gpu_world: &GpuWorld, rows: &mut [AlbedoMapRow<'_>]) {
    for row in rows {
        let (p, r, s) = trs(row.transform);
        let update = build_solid_albedo_map(p, r, s, row.texture);
        write_update(
            gpu_world,
            GpuWorldSubsetKind::SolidAlbedoMap,
            row.key,
            update,
            |raw| SolidAlbedoMapInstanceData { raw },
        );
    }
}

pub fn nani_update_solid_albedo_map_emissive(gpu_world: &GpuWorld, rows: &mut [AlbedoMapRow<'_>]) {
    for row in rows {
        let (p, r, s) = trs(row.transform);
        let update = build_solid_albedo_map(p, r, s, row.texture);
        write_update(
            gpu_world,
            GpuWorldSubsetKind::SolidAlbedoMapEmissive,
            row.key,
            update,
            |raw| SolidAlbedoMapEmissiveInstanceData { raw },
        );
    }
}

/// kind 0xe（0x142073690）。`rock_mask` 是 `TerrainRockMaskSsbo` 的字，`sample` 依次采水与四张路径栅格。
pub fn nani_update_preplaced_flower(
    gpu_world: &GpuWorld,
    rock_mask: &[u32],
    mut sample: impl FnMut(FlowerRaster, f32, f32) -> f32,
    rows: &mut [TransformRow<'_>],
) {
    for row in rows {
        let (p, r, s) = trs(row.transform);
        let visible = flower_visible(p.x, p.z, rock_mask, &mut sample);
        let update = build_preplaced_flower(r, p, s, visible);
        write_update(
            gpu_world,
            GpuWorldSubsetKind::PreplacedFlower,
            row.key,
            update,
            |raw| PreplacedFlowerInstanceData { raw },
        );
    }
}

/// kind 0xa（0x142074740）。
pub fn nani_update_plant(gpu_world: &GpuWorld, rows: &mut [TransformRow<'_>]) {
    for row in rows {
        let (p, r, s) = trs(row.transform);
        let update = build_plant(p, r, s);
        write_update(
            gpu_world,
            GpuWorldSubsetKind::Plant,
            row.key,
            update,
            |raw| PlantInstanceData { raw },
        );
    }
}

/// kind 5（0x142075070）。
pub struct WindowGlassRow<'a> {
    pub key: &'a mut InstanceKey,
    pub transform: &'a Transform,
    pub color_id: u32,
    pub semi_transparent: bool,
    pub window_light_value: f32,
    pub albedo_bindless: Option<u32>,
}

pub fn nani_update_solid_vertex_color_window_glass(
    gpu_world: &GpuWorld,
    rows: &mut [WindowGlassRow<'_>],
) {
    for row in rows {
        let (p, r, s) = trs(row.transform);
        let update = build_window_glass(
            p,
            r,
            s,
            row.color_id,
            row.semi_transparent,
            row.window_light_value,
            row.albedo_bindless,
        );
        write_update(
            gpu_world,
            GpuWorldSubsetKind::SolidVertexColorWindowGlass,
            row.key,
            update,
            |raw| SolidVertexColorWindowGlassInstanceData { raw },
        );
    }
}

/// kind 6（0x142076230）、kind 7（0x142076e40）：砖颜色的 6 个字，哥特窗另带 15 个字的窗定义。
pub struct BricksRow<'a> {
    pub key: &'a mut InstanceKey,
    pub transform: &'a Transform,
    pub brick: [u32; 6],
    pub window_def: [u32; 15],
}

pub fn nani_update_solid_vertex_color_bricks(gpu_world: &GpuWorld, rows: &mut [BricksRow<'_>]) {
    for row in rows {
        let (p, r, s) = trs(row.transform);
        let update = build_solid_vertex_color_bricks(p, r, s, row.brick);
        write_update(
            gpu_world,
            GpuWorldSubsetKind::SolidVertexColorBricks,
            row.key,
            update,
            |raw| SolidVertexColorBricksInstanceData { raw },
        );
    }
}

pub fn nani_update_gothic_window_bricks(gpu_world: &GpuWorld, rows: &mut [BricksRow<'_>]) {
    for row in rows {
        let (p, r, s) = trs(row.transform);
        let update = build_gothic_window_bricks(r, p, s, row.brick, row.window_def);
        write_update(
            gpu_world,
            GpuWorldSubsetKind::GothicWindowBricks,
            row.key,
            update,
            |raw| GothicWindowBricksInstanceData { raw },
        );
    }
}

/// kind 0x18（0x142077d20）：唯一 `&mut` 直写的系统。`data` 是实例数据组件本身。
pub struct CurvedHalfTimberRow<'a> {
    pub key: &'a mut InstanceKey,
    pub data: &'a mut CurvedHalfTimberInstanceData,
    /// 组件里的 `u64 @+0`。
    pub colour_key: u64,
    /// 组件里的 `u8 @+8`。
    pub direct: bool,
    /// `PublicWalls` 取到的 `Roof` 行（已解析）。
    pub roof: Option<&'a RoofWords>,
}

pub fn nani_update_curved_halftimber(
    gpu_world: &GpuWorld,
    halftimber_colour_ids: impl Fn(u64) -> Option<u32>,
    rows: &mut [CurvedHalfTimberRow<'_>],
) {
    for row in rows {
        let roof_colour = row.roof.and_then(roof_curved_colour_from_words);
        let colour = curved_halftimber_colour(
            row.colour_key,
            row.direct,
            roof_colour,
            &halftimber_colour_ids,
        );
        row.data.pad_0c = colour;
        let update = build_curved_halftimber(curved_halftimber_words(row.data), colour);
        gpu_world
            .owned_subset_mut(GpuWorldSubsetKind::CurvedHalfTimber)
            .update_instance_data(row.key, update.position, update.sort_key, *row.data);
    }
}

/// kind 9（0x142078780）。
pub struct StoneFloorRow<'a> {
    pub key: &'a mut InstanceKey,
    pub transform: &'a Transform,
    pub roof: Option<&'a RoofWords>,
    pub roof_animation: Option<f32>,
    pub floor: Option<&'a FloorWords>,
}

pub fn nani_update_stone_floor(
    gpu_world: &GpuWorld,
    tangent: TangentAtU0<'_>,
    rows: &mut [StoneFloorRow<'_>],
) {
    for row in rows {
        let update = build_stone_floor(
            row.transform.translation,
            row.roof,
            row.roof_animation,
            row.floor,
            tangent,
        );
        write_update(
            gpu_world,
            GpuWorldSubsetKind::RoofFloor,
            row.key,
            update,
            |raw| StoneFloorInstanceData { raw },
        );
    }
}

/// kind 0x11（0x1420797b0）。
pub struct WrenRow<'a> {
    pub key: &'a mut InstanceKey,
    pub transform: &'a Transform,
    pub prev_transform: &'a Transform,
    pub wren_44: [u32; 2],
    pub wren_4c: u32,
}

pub fn nani_update_wren(gpu_world: &GpuWorld, rows: &mut [WrenRow<'_>]) {
    for row in rows {
        let cur = (
            row.transform.rotation,
            row.transform.translation,
            row.transform.scale,
        );
        let prev = (
            row.prev_transform.rotation,
            row.prev_transform.translation,
            row.prev_transform.scale,
        );
        let update = build_wren(cur, prev, row.wren_44, row.wren_4c);
        write_update(
            gpu_world,
            GpuWorldSubsetKind::Wren,
            row.key,
            update,
            |raw| WrenInstanceData { raw },
        );
    }
}

/// kind 0x12（0x14207a4f0）。
pub struct LanternRow<'a> {
    pub key: &'a mut InstanceKey,
    pub transform: &'a Transform,
    pub params: LanternParamsView,
    pub semi_transparent: bool,
    pub color_id: Option<u32>,
}

pub fn nani_update_lantern_emissive(gpu_world: &GpuWorld, rows: &mut [LanternRow<'_>]) {
    for row in rows {
        let (p, r, s) = trs(row.transform);
        let update =
            build_lantern_emissive(r, p, s, row.params, row.semi_transparent, row.color_id);
        write_update(
            gpu_world,
            GpuWorldSubsetKind::LanternEmissive,
            row.key,
            update,
            |raw| LanternEmissiveInstanceData { raw },
        );
    }
}

/// kind 0x13（0x14207b290）。`Or<(Changed<Plaster>, Changed<NaniActiveInstance>)>` 的过滤由调用方完成。
pub struct PlasterRow<'a> {
    pub key: &'a mut InstanceKey,
    pub plaster: PlasterView,
    pub wall: PlasterWallInputs,
    pub animated_peel: Option<f32>,
}

pub fn nani_update_plaster(gpu_world: &GpuWorld, rows: &mut [PlasterRow<'_>]) {
    for row in rows {
        let update = build_plaster(row.plaster, row.wall, row.animated_peel);
        write_update(
            gpu_world,
            GpuWorldSubsetKind::Plaster,
            row.key,
            update,
            |raw| PlasterInstanceData { raw },
        );
    }
}

/// kind 0x14（0x14207c0a0）。
pub struct ClothRow<'a> {
    pub key: &'a mut InstanceKey,
    pub transform: &'a Transform,
    pub cloth: [u32; 3],
    pub normal_map_bindless_index: u32,
}

pub fn nani_update_cloth(gpu_world: &GpuWorld, rows: &mut [ClothRow<'_>]) {
    for row in rows {
        let (p, r, s) = trs(row.transform);
        let update = build_cloth(r, p, s, row.cloth, row.normal_map_bindless_index);
        write_update(
            gpu_world,
            GpuWorldSubsetKind::Cloth,
            row.key,
            update,
            |raw| ClothInstanceData { raw },
        );
    }
}

/// kind 0x15（0x14207cd60）。
pub struct SmokeRow<'a> {
    pub key: &'a mut InstanceKey,
    pub transform: &'a Transform,
    pub smoke: SmokeView,
}

pub fn nani_update_smoke(gpu_world: &GpuWorld, rows: &mut [SmokeRow<'_>]) {
    for row in rows {
        let (p, r, s) = trs(row.transform);
        let update = build_smoke(r, p, s, row.smoke);
        write_update(
            gpu_world,
            GpuWorldSubsetKind::Smoke,
            row.key,
            update,
            |raw| SmokeInstanceData { raw },
        );
    }
}

/// kind 0x17（0x14207e1f0）。
pub struct UberRow<'a> {
    pub key: &'a mut InstanceKey,
    pub transform: &'a Transform,
    pub semi_transparent: bool,
    pub albedo_bindless_index: Option<u32>,
}

pub fn nani_update_uber(gpu_world: &GpuWorld, rows: &mut [UberRow<'_>]) {
    for row in rows {
        let update = build_uber(
            row.transform,
            row.semi_transparent,
            row.albedo_bindless_index,
        );
        write_update(
            gpu_world,
            GpuWorldSubsetKind::Uber,
            row.key,
            update,
            |raw| UberInstanceData { raw },
        );
    }
}

/// kind 0x19（0x14207f060）。
pub struct VelocityOnlyRow<'a> {
    pub key: &'a mut InstanceKey,
    pub transform: &'a Transform,
    pub prev_transform: &'a Transform,
}

pub fn nani_update_velocity_only(gpu_world: &GpuWorld, rows: &mut [VelocityOnlyRow<'_>]) {
    for row in rows {
        let update = build_velocity_only(row.transform, row.prev_transform);
        write_update(
            gpu_world,
            GpuWorldSubsetKind::VelocityOnly,
            row.key,
            update,
            |raw| VelocityOnlyInstanceData { raw },
        );
    }
}

/// kind 0x1a（0x14207fcc0）。`chunk_words` 是 `StairsFloorMeshChunk + 8` 起的 9 个字。
pub struct StairsFloorRow<'a> {
    pub key: &'a mut InstanceKey,
    pub chunk_words: [u32; 9],
}

pub fn nani_update_stairs_floor(gpu_world: &GpuWorld, rows: &mut [StairsFloorRow<'_>]) {
    for row in rows {
        let update = build_stairs_floor(row.chunk_words);
        write_update(
            gpu_world,
            GpuWorldSubsetKind::StairsFloor,
            row.key,
            update,
            |raw| StairsFloorInstanceData { raw },
        );
    }
}

/// `nani_flush_instance_updates`（O 0x142080640）：27 槽里 Owned 的做 GPU 冲刷，然后滚动实例计数。
pub fn nani_flush_instance_updates(gpu_world: &mut GpuWorld, device: &dyn GraphicsDevice) {
    gpu_world.flush_instance_buffers(device);
    gpu_world.roll_instance_counts();
}

// Actual ECS adapter for the component contracts that currently exist in this
// workspace. Remaining unavailable payload/producer contracts stay explicit;
// a new descriptor outside these bindings fails before an uninitialized instance can
// be published. This completeness guard is a host boundary, not a native branch.
use bevy_ecs::{
    change_detection::DetectChangesMut,
    prelude::{Changed, Commands, Local, ParamSet, Query, Res, ResMut, With, Without},
    query::Has,
    removal_detection::RemovedComponents,
    schedule::{ApplyDeferred, IntoScheduleConfigs, Schedule, SystemSet},
};
use crate::{
    asset_libraries::{ssbo_library::AssetSsboLibrary, texture_library::AssetTextureLibrary},
    components::{FlatLighting, NormalMap, SemiTransparent,
        drawable::{AlbedoMap, ClothlineCloth, PrevTransform, NaniSolidTexture}},
    render::{device::GraphicsDeviceResource, nani::gpu_world::{InstanceDataMarker, OwnedGpuWorldSubset}},
    resources::{color_id::ColorId, roofs::{Roof, RoofAnimation},
        terrain_rock_mask::TerrainRockMaskSsbo, world_raster_defs::WaterRaster,
        window_lights::{GlobalWindowLightsControl, WindowLight}},
    systems::paths::PathRastersResources,
};

type MappedNaniMarkers = (
    Has<InstanceDataMarker<SolidVertexColorInstanceData>>,
    Has<InstanceDataMarker<SolidVertexColorWindowInstanceData>>,
    Has<InstanceDataMarker<PlantInstanceData>>,
    Has<InstanceDataMarker<PreplacedFlowerInstanceData>>,
    Has<InstanceDataMarker<BalconyDoorInstanceData>>,
    Has<InstanceDataMarker<UberInstanceData>>,
    Has<InstanceDataMarker<VelocityOnlyInstanceData>>,
    Has<InstanceDataMarker<SolidAlbedoMapInstanceData>>,
    Has<InstanceDataMarker<SolidAlbedoMapEmissiveInstanceData>>,
    Has<InstanceDataMarker<StoneFloorInstanceData>>,
    Has<InstanceDataMarker<ClothInstanceData>>,
    Has<InstanceDataMarker<SolidVertexColorWindowGlassInstanceData>>,
);

type MappedNaniMarkerPresence = (bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool);
type MappedNaniInputs = (MappedNaniMarkers, Has<ColorId>, Has<PrevTransform>, Has<Transform>,
    Has<NaniSolidTexture>, Has<Roof>, Has<ClothlineCloth>, Has<NormalMap>, Has<WindowLight>);
fn require_mapped_nani_payload(
    kind: GpuWorldSubsetKind,
    (markers, has_color, has_previous, has_transform, has_selector, has_roof, has_cloth, has_normal, has_light):
        (MappedNaniMarkerPresence, bool, bool, bool, bool, bool, bool, bool, bool),
) {
    let mapped = match kind {
        GpuWorldSubsetKind::SolidVertexColor => markers.0,
        GpuWorldSubsetKind::SolidVertexColorWindow => markers.1 && has_color,
        GpuWorldSubsetKind::Plant => markers.2,
        GpuWorldSubsetKind::PreplacedFlower => markers.3,
        GpuWorldSubsetKind::BalconyDoor => markers.4 && has_color,
        GpuWorldSubsetKind::Uber => markers.5,
        GpuWorldSubsetKind::VelocityOnly => markers.6 && has_previous,
        GpuWorldSubsetKind::SolidAlbedoMap => markers.7 && has_selector,
        GpuWorldSubsetKind::SolidAlbedoMapEmissive => markers.8 && has_selector,
        // Restricted original kind9 producer domain. Floor-only entities
        // remain unsupported; do not publish a key without a real Roof.
        GpuWorldSubsetKind::RoofFloor => markers.9 && has_roof,
        GpuWorldSubsetKind::Cloth => markers.10 && has_cloth && has_normal,
        GpuWorldSubsetKind::SolidVertexColorWindowGlass => markers.11 && has_color && has_light,
        _ => false,
    };
    let marker_count = [markers.0, markers.1, markers.2, markers.3,
        markers.4, markers.5, markers.6, markers.7, markers.8, markers.9, markers.10, markers.11]
        .into_iter().filter(|present| *present).count();
    assert!(mapped && has_transform && marker_count == 1,
        "Nani ECS payload mapping is unavailable or canonical inputs are incomplete: {kind:?}");
}

/// Original B14206f300 producer contracts, now reading the real same-World
/// components. No row Vec, duplicate key owner or per-frame scratch allocation.
/// Dense-index hint repair bypasses change detection: the native Query reads
/// &NaniActiveInstance, so repairing its cache must not fabricate Changed<Key>.
pub fn nani_maintain_instances_ecs(
    mut removed: RemovedComponents<InstanceKey>,
    new: Query<(Entity, &NaniInstanceDesc, &Transform), Without<InstanceKey>>,
    mut keys: ParamSet<(
        Query<(&NaniInstanceDesc, Option<&InstanceKey>, MappedNaniInputs)>,
        Query<(&NaniInstanceDesc, &mut InstanceKey), Changed<NaniInstanceDesc>>,
    )>,
    atlas: Res<MeshAtlasLibrary>, mut gpu_world: ResMut<GpuWorld>,
    mut local: Local<NaniActiveInstanceCollection>, mut commands: Commands,
) {
    // Host completeness preflight touches only actual descriptor-bearing
    // entities. A registered empty subset remains a real zero-count subset.
    for (desc, key, inputs) in &keys.p0() {
        require_mapped_nani_payload(desc.subset_kind, inputs);
        if let Some(key) = key {
            assert_eq!(key.kind, desc.subset_kind,
                "Nani descriptor changed the canonical active instance kind");
        }
    }
    for entity in removed.read() {
        remove_active_instance(entity, &gpu_world, &mut local);
    }
    for (entity, desc, transform) in &new {
        if let Some(key) = create_active_instance(
            NewInstanceRow { entity, desc, transform }, &atlas, &mut gpu_world, &mut local,
        ) {
            commands.entity(entity).insert(key);
        }
    }
    for (desc, mut key) in &mut keys.p1() {
        change_active_instance_mesh(desc, key.bypass_change_detection(), &atlas, &mut gpu_world);
    }
}

fn write_mapped_update<T: InstanceElement, const N: usize>(
    subset: &mut OwnedGpuWorldSubset, key: &mut InstanceKey,
    update: InstanceUpdate<N>, make: impl FnOnce([u32; N]) -> T,
) {
    subset.update_instance_data(key, update.position, update.sort_key, make(update.words));
}

/// Each original update takes one subset lock before iterating its query,
/// including an empty query. Keep that failure order and lock once per system.
pub fn nani_update_solid_vertex_color_ecs(
    mut rows: Query<(&mut InstanceKey, &Transform, Option<&SemiTransparent>, Option<&FlatLighting>),
        With<InstanceDataMarker<SolidVertexColorInstanceData>>>,
    gpu: Res<GpuWorld>,
) {
    let mut subset = gpu.owned_subset_mut(GpuWorldSubsetKind::SolidVertexColor);
    for (mut key, transform, semi, flat) in &mut rows {
        let (p, r, s) = trs(transform);
        write_mapped_update(&mut subset, key.bypass_change_detection(),
            build_solid_vertex_color(p, r, s, semi.is_some(), flat.is_some()),
            |raw| SolidVertexColorInstanceData { raw });
    }
}

pub fn nani_update_solid_vertex_color_window_ecs(
    mut rows: Query<(&mut InstanceKey, &Transform, &ColorId, Option<&SemiTransparent>),
        With<InstanceDataMarker<SolidVertexColorWindowInstanceData>>>,
    gpu: Res<GpuWorld>,
) {
    let mut subset = gpu.owned_subset_mut(GpuWorldSubsetKind::SolidVertexColorWindow);
    for (mut key, transform, color, semi) in &mut rows {
        let (p, r, s) = trs(transform);
        write_mapped_update(&mut subset, key.bypass_change_detection(),
            build_solid_vertex_color_window(p, r, s, color.0, semi.is_some()),
            |raw| SolidVertexColorWindowInstanceData { raw });
    }
}

/// Native tuple5 / kind9, restricted to actual Roof-bearing entities. Keep
/// the original With-only marker domain even if NaniInstanceDesc is absent:
/// requiring &Roof in the Query would silently skip an unsupported active row.
/// The original Floor-only component contract is not yet recovered.
pub fn nani_update_roof_floor_for_roofs_ecs(
    mut rows: Query<(&mut InstanceKey, &Transform, Option<&Roof>, Option<&RoofAnimation>),
        With<InstanceDataMarker<StoneFloorInstanceData>>>,
    gpu: Res<GpuWorld>,
) {
    let mut subset = gpu.owned_subset_mut(GpuWorldSubsetKind::RoofFloor);
    for (mut key, transform, roof, animation) in &mut rows {
        let roof = roof.expect(
            "RoofFloor ECS support requires a real Roof; Floor-only or missing Roof inputs are unsupported");
        let words: RoofWords = std::array::from_fn(|index| {
            let offset = index * 4;
            u32::from_le_bytes(roof.observed()[offset..offset + 4].try_into().unwrap())
        });
        write_mapped_update(&mut subset, key.bypass_change_detection(),
            build_stone_floor(transform.translation, Some(&words),
                animation.map(|animation| animation.values()[0]), None, &circle_tangent_u0),
            |raw| StoneFloorInstanceData { raw });
    }
}

pub fn nani_update_plant_ecs(
    mut rows: Query<(&mut InstanceKey, &Transform), With<InstanceDataMarker<PlantInstanceData>>>,
    gpu: Res<GpuWorld>,
) {
    let mut subset = gpu.owned_subset_mut(GpuWorldSubsetKind::Plant);
    for (mut key, transform) in &mut rows {
        let (p, r, s) = trs(transform);
        write_mapped_update(&mut subset, key.bypass_change_detection(), build_plant(p, r, s),
            |raw| PlantInstanceData { raw });
    }
}

pub fn nani_update_preplaced_flower_ecs(
    mut rows: Query<(&mut InstanceKey, &Transform), With<InstanceDataMarker<PreplacedFlowerInstanceData>>>,
    gpu: Res<GpuWorld>, ssbos: Res<AssetSsboLibrary>, paths: PathRastersResources,
    water: Res<WaterRaster>,
    mut completed_mask: Local<Vec<u32>>,
) {
    // Original14207396a obtains the owned subset before142073982 reads
    // TerrainRockMask, even when the component query is empty.
    let mut subset = gpu.owned_subset_mut(GpuWorldSubsetKind::PreplacedFlower);
    let rock = ssbos.get::<TerrainRockMaskSsbo>()
        .expect("called `Option::unwrap()` on a `None` value: TerrainRockMaskSsbo");
    rock.try_cpu_read(|samples| {
        completed_mask.clear();
        completed_mask.extend_from_slice(bytemuck::cast_slice(samples));
    }).expect("flower rock-mask readback failed");
    // Keep the last completed mask. Release the read guard before publishing
    // instance uploads; never hold one mapped allocation while writing another.
    if completed_mask.is_empty() {
        return;
    }
    let mask = completed_mask.as_slice();
    let path_grids = paths.as_float_grid();
    let water_grid = (**water).as_ref();
    for (mut key, transform) in &mut rows {
        let (p, r, s) = trs(transform);
        let visible = flower_visible(p.x, p.z, mask, |which, x, z| {
            let grid = match which {
                FlowerRaster::Water => &water_grid,
                FlowerRaster::Path0 => &path_grids[0],
                FlowerRaster::Path1 => &path_grids[1],
                FlowerRaster::Path2 => &path_grids[2],
                FlowerRaster::Path3 => &path_grids[3],
            };
            grid.bilinear_sample_at_world_xz(x, z, FLOWER_WORLD_SIZE, FLOWER_WORLD_SIZE)
        });
        write_mapped_update(&mut subset, key.bypass_change_detection(),
            build_preplaced_flower(r, p, s, visible), |raw| PreplacedFlowerInstanceData { raw });
    }
}

/// Original tuple8/9: With-only updates read the same canonical selector
/// emitted by the real decorator builder. No per-entity texture lookup.
pub fn nani_update_solid_albedo_map_ecs(
    mut rows: Query<(&mut InstanceKey, &Transform, &NaniSolidTexture),
        With<InstanceDataMarker<SolidAlbedoMapInstanceData>>>,
    gpu: Res<GpuWorld>,
) {
    let mut subset = gpu.owned_subset_mut(GpuWorldSubsetKind::SolidAlbedoMap);
    for (mut key, transform, selector) in &mut rows {
        let (p, r, s) = trs(transform);
        write_mapped_update(&mut subset, key.bypass_change_detection(),
            build_solid_albedo_map(p, r, s, selector.encoded()),
            |raw| SolidAlbedoMapInstanceData { raw });
    }
}

pub fn nani_update_solid_albedo_map_emissive_ecs(
    mut rows: Query<(&mut InstanceKey, &Transform, &NaniSolidTexture),
        With<InstanceDataMarker<SolidAlbedoMapEmissiveInstanceData>>>,
    gpu: Res<GpuWorld>,
) {
    let mut subset = gpu.owned_subset_mut(GpuWorldSubsetKind::SolidAlbedoMapEmissive);
    for (mut key, transform, selector) in &mut rows {
        let (p, r, s) = trs(transform);
        write_mapped_update(&mut subset, key.bypass_change_detection(),
            build_solid_albedo_map(p, r, s, selector.encoded()),
            |raw| SolidAlbedoMapEmissiveInstanceData { raw });
    }
}

pub fn nani_update_balcony_door_ecs(
    mut rows: Query<(&mut InstanceKey, &Transform, &ColorId, Option<&SemiTransparent>),
        With<InstanceDataMarker<BalconyDoorInstanceData>>>,
    gpu: Res<GpuWorld>,
) {
    let mut subset = gpu.owned_subset_mut(GpuWorldSubsetKind::BalconyDoor);
    for (mut key, transform, color, semi) in &mut rows {
        let (p, r, s) = trs(transform);
        write_mapped_update(&mut subset, key.bypass_change_detection(),
            build_balcony_door(r, p, s, color.0, semi.is_some()),
            |raw| BalconyDoorInstanceData { raw });
    }
}

/// Original tuple13/kind20: the same canonical12-byte cloth words and
/// disk-texture name owner emitted by the real decorator builder. With-only
/// required payload Query; no Changed gate or parallel instance owner.
pub fn nani_update_cloth_ecs(
    mut rows: Query<(&mut InstanceKey, &Transform, &ClothlineCloth, &NormalMap),
        With<InstanceDataMarker<ClothInstanceData>>>,
    textures: Res<AssetTextureLibrary>, gpu: Res<GpuWorld>,
) {
    let mut subset = gpu.owned_subset_mut(GpuWorldSubsetKind::Cloth);
    for (mut key, transform, cloth, normal_map) in &mut rows {
        let asset = textures.get_handle_and_texture_by_name(NameHash(normal_map.0))
            .expect("called `Option::unwrap()` on a `None` value").0;
        let normal_index = textures.handle_to_bindless_index(asset).0;
        let (p, r, s) = trs(transform);
        write_mapped_update(&mut subset, key.bypass_change_detection(),
            build_cloth(r, p, s, cloth.words(), normal_index), |raw| ClothInstanceData { raw });
    }
}

/// B142075070：借用真实窗灯状态与可选磁盘贴图。
/// 未找到灯键时原版亮度为零；指定了不存在的贴图则保留原失败边界。
pub fn nani_update_solid_vertex_color_window_glass_ecs(
    mut rows: Query<(&mut InstanceKey, &Transform, &WindowLight, &ColorId,
        Option<&SemiTransparent>, Option<&AlbedoMap>),
        With<InstanceDataMarker<SolidVertexColorWindowGlassInstanceData>>>,
    control: Res<GlobalWindowLightsControl>, textures: Res<AssetTextureLibrary>,
    gpu: Res<GpuWorld>,
) {
    let mut subset = gpu.owned_subset_mut(GpuWorldSubsetKind::SolidVertexColorWindowGlass);
    for (mut key, transform, light, color, semi, albedo) in &mut rows {
        let brightness = control.lights.get(&light.key).map_or(0.0, |value| value.brightness);
        let albedo_index = albedo.map(|albedo| {
            let asset = textures.get_handle_and_texture_by_name(NameHash(albedo.0))
                .expect("called `Option::unwrap()` on a `None` value").0;
            textures.handle_to_bindless_index(asset).0
        });
        let (p, r, s) = trs(transform);
        write_mapped_update(&mut subset, key.bypass_change_detection(),
            build_window_glass(p, r, s, color.0, semi.is_some(), brightness, albedo_index),
            |raw| SolidVertexColorWindowGlassInstanceData { raw });
    }
}

pub fn nani_update_uber_ecs(
    mut rows: Query<(&mut InstanceKey, &Transform, Option<&SemiTransparent>, Option<&AlbedoMap>),
        With<InstanceDataMarker<UberInstanceData>>>,
    textures: Res<AssetTextureLibrary>, gpu: Res<GpuWorld>,
) {
    let mut subset = gpu.owned_subset_mut(GpuWorldSubsetKind::Uber);
    for (mut key, transform, semi, albedo) in &mut rows {
        let albedo_index = albedo.map(|albedo| {
            let asset = textures.get_handle_and_texture_by_name(NameHash(albedo.0))
                .expect("called `Option::unwrap()` on a `None` value").0;
            textures.handle_to_bindless_index(asset).0
        });
        write_mapped_update(&mut subset, key.bypass_change_detection(),
            build_uber(transform, semi.is_some(), albedo_index), |raw| UberInstanceData { raw });
    }
}

pub fn nani_update_velocity_only_ecs(
    mut rows: Query<(&mut InstanceKey, &Transform, &PrevTransform),
        With<InstanceDataMarker<VelocityOnlyInstanceData>>>,
    gpu: Res<GpuWorld>,
) {
    let mut subset = gpu.owned_subset_mut(GpuWorldSubsetKind::VelocityOnly);
    for (mut key, transform, previous) in &mut rows {
        let previous = Transform { rotation: previous.rotation,
            translation: previous.translation, scale: previous.scale };
        write_mapped_update(&mut subset, key.bypass_change_detection(),
            build_velocity_only(transform, &previous), |raw| VelocityOnlyInstanceData { raw });
    }
}

pub fn nani_flush_instance_updates_ecs(
    device: Res<GraphicsDeviceResource>, mut gpu: ResMut<GpuWorld>,
) {
    // GraphicsDeviceResource carries this App's same real BufferHost. Flush
    // actual dirty ranges; roll the actual request count only after success.
    nani_flush_instance_updates(&mut gpu, device.device());
}

/// Original backend44 fragment: maintain -> explicit ApplyDeferred ->
/// unordered20-update tuple -> flush, outer chain_ignore_deferred.
/// 当前保留九类既有映射、Cloth 与限定 Roof-bearing kind9 分支，新增共享窗灯玻璃。
/// Floor-only 及其他未闭合输入继续明确失败；绑定不代表独立场景生产者已可达。
pub fn add_mapped_nani_instance_systems<S: SystemSet>(schedule: &mut Schedule, set: S) {
    schedule.add_systems((
        nani_maintain_instances_ecs,
        ApplyDeferred,
        (nani_update_solid_vertex_color_ecs,
         nani_update_solid_vertex_color_window_ecs,
         nani_update_roof_floor_for_roofs_ecs,
         nani_update_plant_ecs,
         nani_update_preplaced_flower_ecs,
         nani_update_solid_albedo_map_ecs,
         nani_update_solid_albedo_map_emissive_ecs,
         nani_update_balcony_door_ecs,
         nani_update_cloth_ecs,
         nani_update_solid_vertex_color_window_glass_ecs,
         nani_update_uber_ecs,
         nani_update_velocity_only_ecs),
        nani_flush_instance_updates_ecs,
    ).chain_ignore_deferred().in_set(set));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::mesh::Mesh;
    use crate::render::nani::gpu_world::{GpuWorldSubset, OwnedGpuWorldSubset};
    use crate::render::ssbo::tests::MockDevice;
    use bevy_ecs::world::World;
    use glam::Vec3;

    fn glass_world(dev: &MockDevice) -> World {
        use crate::render::test_device::TestDevice;
        use crate::rhapsody_types::Texture;
        let mut gpu = GpuWorld::new(dev);
        gpu.assign_subset(GpuWorldSubsetKind::SolidVertexColorWindowGlass, GpuWorldSubset::Owned(
            OwnedGpuWorldSubset::new::<SolidVertexColorWindowGlassInstanceData>(
                dev, GpuWorldSubsetKind::SolidVertexColorWindowGlass)));
        let mut atlas = MeshAtlasLibrary::default();
        let mut mesh = Mesh::new("窗玻璃 CPU 集成测试三角形");
        mesh.set_attribute(crate::render::mesh::ATTR_POSITION,
            vec![Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 0.0, 3.0)]);
        mesh.set_indices(vec![0, 1, 2]);
        atlas.add(GpuWorldSubsetKind::SolidVertexColorWindowGlass, "window_glass_cpu", &mesh);
        let mut textures = AssetTextureLibrary::new(&TestDevice(91));
        let tex = |id| Texture { id, width: 4, height: 4, format: 0x25 };
        let first = textures.add(Some("glass_cpu_first".into()), tex(91));
        textures.add(Some("glass_cpu_unused".into()), tex(92));
        let actual = textures.add(Some("glass_cpu_albedo".into()), tex(93));
        assert_eq!(textures.handle_to_bindless_index(first).0, 0);
        // 真实资产句柄为2；它的描述符索引为1，两者不能混用。
        assert_eq!(actual.0, 2);
        let mut world = World::new();
        world.insert_resource(gpu); world.insert_resource(atlas); world.insert_resource(textures);
        world.init_resource::<GlobalWindowLightsControl>();
        world
    }

    fn spawn_cpu_glass(world: &mut World) -> Entity {
        world.spawn((crate::components::drawable::NaniBundle::<SolidVertexColorWindowGlassInstanceData> {
            desc: NaniInstanceDesc { mesh: NaniMeshSelector::Named(NameHash::compute("window_glass_cpu")),
                subset_kind: GpuWorldSubsetKind::SolidVertexColorWindowGlass },
            transform: Transform { rotation: Quat::from_rotation_y(0.37),
                translation: Vec3::new(2.0, 3.0, 4.0), scale: Vec3::new(1.2, 0.8, 2.5) },
            marker: InstanceDataMarker::default(),
        }, WindowLight { key: 0x1_0000_0061, decorator_id: 71 }, ColorId(3))).id()
    }

    fn actual_glass_payload(world: &World, entity: Entity) -> [u32; 16] {
        let key = world.get::<InstanceKey>(entity).unwrap();
        world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::SolidVertexColorWindowGlass)
            .data().as_any().downcast_ref::<
                crate::render::nani::gpu_world::InstanceDataBuffer<SolidVertexColorWindowGlassInstanceData>>()
            .unwrap().shadow.data[key.index as usize].raw
    }

    #[test]
    fn glass_uses_shared_light_actual_albedo_barrier_upload_and_removal() {
        let dev = MockDevice::default();
        let mut world = glass_world(&dev); let entity = spawn_cpu_glass(&mut world);
        world.entity_mut(entity).insert((SemiTransparent,
            AlbedoMap(NameHash::compute("glass_cpu_albedo").0)));
        world.resource_mut::<GlobalWindowLightsControl>().lights.insert(0x1_0000_0061,
            crate::resources::window_lights::LightInfo { brightness: 0.625, ..Default::default() });
        let mut schedule = Schedule::default();
        schedule.add_systems((nani_maintain_instances_ecs, ApplyDeferred,
            nani_update_solid_vertex_color_window_glass_ecs).chain_ignore_deferred());
        schedule.run(&mut world);
        let key = *world.get::<InstanceKey>(entity).unwrap();
        assert_eq!(key.kind, GpuWorldSubsetKind::SolidVertexColorWindowGlass);
        assert_eq!(&actual_glass_payload(&world, entity)[12..],
            &[0x8000_0003, 0.625f32.to_bits(), 1, 1]);
        let textures = world.resource::<AssetTextureLibrary>();
        let asset = textures.get_handle_and_texture_by_name(NameHash::compute("glass_cpu_albedo")).unwrap().0;
        assert_eq!(asset.0, 2); assert_eq!(textures.handle_to_bindless_index(asset).0, 1);
        assert_eq!(textures.pending_bindless().len(), 2);
        nani_flush_instance_updates(&mut world.resource_mut::<GpuWorld>(), &dev);
        assert!(dev.writes.borrow().iter().any(|(ranges, offset, bytes)|
            ranges == &[0..64] && *offset == 0 && *bytes == 64));
        assert_eq!(world.resource::<GpuWorld>().previous_instance_count(), 1);
        world.clear_trackers();
        world.resource_mut::<GlobalWindowLightsControl>().lights.get_mut(&0x1_0000_0061)
            .unwrap().brightness = -0.0;
        schedule.run(&mut world);
        assert_eq!(actual_glass_payload(&world, entity)[13], 0x8000_0000);
        assert_eq!(world.get::<InstanceKey>(entity).unwrap().id, key.id);
        world.clear_trackers();
        world.resource_mut::<GlobalWindowLightsControl>().lights.clear();
        world.entity_mut(entity).remove::<SemiTransparent>();
        world.entity_mut(entity).remove::<AlbedoMap>();
        schedule.run(&mut world);
        assert_eq!(&actual_glass_payload(&world, entity)[12..], &[3, 0, 0, 0]);
        world.clear_trackers(); world.despawn(entity); schedule.run(&mut world);
        assert_eq!(world.resource::<GpuWorld>().owned_subset_mut(
            GpuWorldSubsetKind::SolidVertexColorWindowGlass).instance_count(), 0);
    }

    #[test]
    fn glass_missing_shared_inputs_fail_before_instance_publication() {
        for missing in ["light", "color", "transform", "marker", "ambiguous"] {
            let dev = MockDevice::default();
            let mut world = glass_world(&dev); let entity = spawn_cpu_glass(&mut world);
            match missing {
                "light" => { world.entity_mut(entity).remove::<WindowLight>(); }
                "color" => { world.entity_mut(entity).remove::<ColorId>(); }
                "transform" => { world.entity_mut(entity).remove::<Transform>(); }
                "marker" => { world.entity_mut(entity).remove::<InstanceDataMarker<SolidVertexColorWindowGlassInstanceData>>(); }
                _ => { world.entity_mut(entity).insert(InstanceDataMarker::<PlantInstanceData>::default()); }
            }
            let mut schedule = Schedule::default(); schedule.add_systems(nani_maintain_instances_ecs);
            assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                schedule.run(&mut world))).is_err(), "缺少{missing}却发布了实例");
            assert!(world.get::<InstanceKey>(entity).is_none());
            let gpu = world.resource::<GpuWorld>(); assert_eq!(gpu.total_instance_count, 0);
            assert_eq!(gpu.owned_subset_mut(GpuWorldSubsetKind::SolidVertexColorWindowGlass).instance_count(), 0);
        }
    }

    #[test]
    fn glass_present_unknown_albedo_fails_in_actual_update() {
        let dev = MockDevice::default();
        let mut world = glass_world(&dev); let entity = spawn_cpu_glass(&mut world);
        world.entity_mut(entity).insert(AlbedoMap(NameHash::compute("missing_glass_albedo").0));
        let mut allocation = Schedule::default();
        allocation.add_systems((nani_maintain_instances_ecs, ApplyDeferred).chain_ignore_deferred());
        allocation.run(&mut world);
        let before_writes = dev.writes.borrow().len();
        let mut update = Schedule::default(); update.add_systems(nani_update_solid_vertex_color_window_glass_ecs);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| update.run(&mut world))).is_err());
        assert_eq!(dev.writes.borrow().len(), before_writes);
        assert_eq!(world.resource::<AssetTextureLibrary>().pending_bindless().len(), 1);
    }

    #[test]
    fn empty_glass_query_still_requires_its_actual_owned_subset() {
        let dev = MockDevice::default(); let mut world = glass_world(&dev);
        let before_writes = dev.writes.borrow().len();
        let mut update = Schedule::default(); update.add_systems(nani_update_solid_vertex_color_window_glass_ecs);
        update.run(&mut world);
        assert_eq!(world.resource::<GpuWorld>().owned_subset_mut(
            GpuWorldSubsetKind::SolidVertexColorWindowGlass).instance_count(), 0);
        assert_eq!(dev.writes.borrow().len(), before_writes);
        world.insert_resource(GpuWorld::new(&dev));
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| update.run(&mut world))).is_err());
    }

    fn cloth_world(dev: &MockDevice) -> World {
        use crate::render::test_device::TestDevice;
        use crate::rhapsody_types::Texture;
        let mut gpu = GpuWorld::new(dev);
        gpu.assign_subset(GpuWorldSubsetKind::Cloth, GpuWorldSubset::Owned(
            OwnedGpuWorldSubset::new::<ClothInstanceData>(dev, GpuWorldSubsetKind::Cloth)));
        let mut atlas = MeshAtlasLibrary::default();
        let mut mesh = Mesh::new("cloth CPU boundary triangle");
        mesh.set_attribute(crate::render::mesh::ATTR_POSITION,
            vec![Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), Vec3::new(0.0, 0.0, 3.0)]);
        mesh.set_indices(vec![0, 1, 2]);
        atlas.add(GpuWorldSubsetKind::Cloth, "decorators/setdressing_clothline_cloth", &mesh);
        let mut textures = AssetTextureLibrary::new(&TestDevice(91));
        let tex = |id| Texture { id, width: 4, height: 4, format: 0x25 };
        let first = textures.add(Some("cloth_cpu_first".into()), tex(91));
        textures.add(Some("cloth_cpu_unused".into()), tex(92));
        let actual = textures.add(Some("cloth_cpu_normal".into()), tex(93));
        assert_eq!(textures.handle_to_bindless_index(first).0, 0);
        assert_eq!(actual.0, 2); // actual descriptor below is1, not this asset index
        let mut world = World::new();
        world.insert_resource(gpu);
        world.insert_resource(atlas);
        world.insert_resource(textures);
        world
    }

    fn spawn_cpu_cloth(world: &mut World) -> Entity {
        world.spawn((crate::components::drawable::NaniBundle::<ClothInstanceData> {
            desc: NaniInstanceDesc { mesh: NaniMeshSelector::Named(
                NameHash::compute("decorators/setdressing_clothline_cloth")), subset_kind: GpuWorldSubsetKind::Cloth },
            transform: Transform { rotation: glam::Quat::from_rotation_y(0.37),
                translation: Vec3::new(2.0, 3.0, 4.0), scale: Vec3::new(1.2, 0.8, 2.5) },
            marker: InstanceDataMarker::default(),
        }, ClothlineCloth::from_words([0x3e80_0000, 0xbe00_0000, 37]),
            NormalMap(NameHash::compute("cloth_cpu_normal").0))).id()
    }

    fn actual_cloth_payload(world: &World, entity: Entity) -> [u32; 16] {
        let key = world.get::<InstanceKey>(entity).unwrap();
        world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::Cloth)
            .data().as_any().downcast_ref::<
                crate::render::nani::gpu_world::InstanceDataBuffer<ClothInstanceData>>()
            .unwrap().shadow.data[key.index as usize].raw
    }

    #[test]
    fn cloth_uses_canonical_payload_actual_texture_owner_barrier_flush_and_removal() {
        let dev = MockDevice::default();
        let mut world = cloth_world(&dev);
        let entity = spawn_cpu_cloth(&mut world);
        let mut schedule = Schedule::default();
        schedule.add_systems((nani_maintain_instances_ecs, ApplyDeferred,
            nani_update_cloth_ecs).chain_ignore_deferred());
        schedule.run(&mut world);
        let key = *world.get::<InstanceKey>(entity).unwrap();
        assert_eq!(key.kind, GpuWorldSubsetKind::Cloth);
        assert_eq!(&actual_cloth_payload(&world, entity)[12..], &[0x3e80_0000, 0xbe00_0000, 37, 1]);
        let textures = world.resource::<AssetTextureLibrary>();
        let asset = textures.get_handle_and_texture_by_name(NameHash::compute("cloth_cpu_normal")).unwrap().0;
        assert_eq!(asset.0, 2);
        assert_eq!(textures.handle_to_bindless_index(asset).0, 1);
        assert_eq!(textures.pending_bindless().len(), 2);
        {
            let subset = world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::Cloth);
            assert_eq!(subset.instance_count(), 1);
            assert_eq!(subset.positions()[key.index as usize].position, Vec3::new(2.0, 3.0, 4.0));
            assert_eq!(subset.positions_dirty().to_range(), Some(0..1));
        }
        nani_flush_instance_updates(&mut world.resource_mut::<GpuWorld>(), &dev);
        assert!(dev.writes.borrow().iter().any(|(ranges, offset, bytes)|
            ranges == &[0..64] && *offset == 0 && *bytes == 64));
        assert_eq!(world.resource::<GpuWorld>().previous_instance_count(), 1);
        world.clear_trackers();
        world.entity_mut(entity).insert(ClothlineCloth::from_words([0x8000_0000, 0x7fc0_0123, 99]));
        schedule.run(&mut world);
        assert_eq!(world.get::<InstanceKey>(entity).unwrap().id, key.id);
        assert_eq!(&actual_cloth_payload(&world, entity)[12..], &[0x8000_0000, 0x7fc0_0123, 99, 1]);
        assert_eq!(world.resource::<AssetTextureLibrary>().pending_bindless().len(), 2);
        world.clear_trackers();
        world.despawn(entity);
        schedule.run(&mut world);
        assert_eq!(world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::Cloth).instance_count(), 0);
    }

    #[test]
    fn cloth_missing_canonical_inputs_fail_before_key_and_count_publication() {
        for missing in ["cloth", "normal", "transform", "marker", "ambiguous"] {
            let dev = MockDevice::default();
            let mut world = cloth_world(&dev);
            let entity = spawn_cpu_cloth(&mut world);
            match missing {
                "cloth" => { world.entity_mut(entity).remove::<ClothlineCloth>(); }
                "normal" => { world.entity_mut(entity).remove::<NormalMap>(); }
                "transform" => { world.entity_mut(entity).remove::<Transform>(); }
                "marker" => { world.entity_mut(entity).remove::<InstanceDataMarker<ClothInstanceData>>(); }
                _ => { world.entity_mut(entity).insert(InstanceDataMarker::<PlantInstanceData>::default()); }
            }
            let mut schedule = Schedule::default();
            schedule.add_systems(nani_maintain_instances_ecs);
            assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                schedule.run(&mut world))).is_err(), "accepted missing {missing}");
            assert!(world.get::<InstanceKey>(entity).is_none());
            let gpu = world.resource::<GpuWorld>();
            assert_eq!(gpu.total_instance_count, 0);
            assert_eq!(gpu.owned_subset_mut(GpuWorldSubsetKind::Cloth).instance_count(), 0);
        }
    }

    #[test]
    fn cloth_unknown_texture_name_fails_in_actual_update() {
        let dev = MockDevice::default();
        let mut world = cloth_world(&dev);
        let entity = spawn_cpu_cloth(&mut world);
        world.entity_mut(entity).insert(NormalMap(0x9abc_def0_1234_5678));
        let mut allocation = Schedule::default();
        allocation.add_systems((nani_maintain_instances_ecs, ApplyDeferred).chain_ignore_deferred());
        allocation.run(&mut world);
        let before_writes = dev.writes.borrow().len();
        let mut update = Schedule::default();
        update.add_systems(nani_update_cloth_ecs);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
            update.run(&mut world))).is_err());
        assert_eq!(dev.writes.borrow().len(), before_writes); // no flush/success after failure
        assert_eq!(world.resource::<AssetTextureLibrary>().pending_bindless().len(), 1);
    }

    #[test]
    fn empty_cloth_query_still_requires_owned_subset_and_preserves_zero_count() {
        let dev = MockDevice::default();
        let mut world = cloth_world(&dev);
        let before_writes = dev.writes.borrow().len();
        let mut update = Schedule::default();
        update.add_systems(nani_update_cloth_ecs);
        update.run(&mut world);
        assert_eq!(world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::Cloth).instance_count(), 0);
        assert_eq!(dev.writes.borrow().len(), before_writes);
        world.insert_resource(GpuWorld::new(&dev));
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
            update.run(&mut world))).is_err());
    }

    fn world_with(kind: GpuWorldSubsetKind, dev: &MockDevice) -> GpuWorld {
        let mut w = GpuWorld::new(dev);
        let subset = match kind {
            GpuWorldSubsetKind::Plant => OwnedGpuWorldSubset::new::<PlantInstanceData>(dev, kind),
            GpuWorldSubsetKind::SolidVertexColor => {
                OwnedGpuWorldSubset::new::<SolidVertexColorInstanceData>(dev, kind)
            }
            _ => unreachable!(),
        };
        w.assign_subset(kind, GpuWorldSubset::Owned(subset));
        w
    }

    fn desc(part: u32, kind: GpuWorldSubsetKind) -> NaniInstanceDesc {
        NaniInstanceDesc {
            mesh: NaniMeshSelector::AtlasPart(part),
            subset_kind: kind,
        }
    }

    fn entities(n: usize) -> Vec<Entity> {
        let mut world = World::new();
        (0..n).map(|_| world.spawn_empty().id()).collect()
    }

    // Actual canonical component constructors with explicit nonzero CPU
    // boundary inputs. This does not claim a complete gameplay/asset startup.
    fn cpu_boundary_roof(curved: bool) -> Roof {
        let shape_words = [u32::from(!curved), 2.5f32.to_bits(), (-3.25f32).to_bits(),
            4.5f32.to_bits(), if curved { 0.73f32.to_bits() } else { 1.25f32.to_bits() },
            6.0f32.to_bits(), 7.0f32.to_bits()];
        let parameter_words = [1.125f32.to_bits(), 2.125f32.to_bits(), 1.5f32.to_bits(),
            0.05f32.to_bits(), 1.0f32.to_bits(), 3.75f32.to_bits(), 1];
        let mut shape = [0; 28];
        let mut parameters = [0; 28];
        let mut flat_style = [0; 12];
        for (bytes, word) in shape.chunks_exact_mut(4).zip(shape_words) {
            bytes.copy_from_slice(&word.to_le_bytes());
        }
        for (bytes, word) in parameters.chunks_exact_mut(4).zip(parameter_words) {
            bytes.copy_from_slice(&word.to_le_bytes());
        }
        for (bytes, word) in flat_style.chunks_exact_mut(4).zip([0x1002u32, 111, 222]) {
            bytes.copy_from_slice(&word.to_le_bytes());
        }
        Roof::new(7u64.to_le_bytes(), parameters, shape, 1.25f32.to_le_bytes(),
            13u32.to_le_bytes(), 5, flat_style)
    }

    fn cpu_boundary_roof_animation(first: f32, second: f32) -> RoofAnimation {
        let mut bytes = [0; 8];
        bytes[..4].copy_from_slice(&first.to_le_bytes());
        bytes[4..].copy_from_slice(&second.to_le_bytes());
        RoofAnimation::from_inner(crate::resources::roofs::components::RoofAnimation::from_observed(bytes))
    }

    fn roof_world(dev: &MockDevice) -> World {
        let mut gpu = GpuWorld::new(dev);
        gpu.assign_subset(GpuWorldSubsetKind::RoofFloor, GpuWorldSubset::Owned(
            OwnedGpuWorldSubset::new::<StoneFloorInstanceData>(dev, GpuWorldSubsetKind::RoofFloor)));
        let mut world = World::new();
        world.insert_resource(gpu);
        world.insert_resource(MeshAtlasLibrary::default());
        world
    }

    fn spawn_cpu_roof(world: &mut World, curved: bool) -> Entity {
        world.spawn((crate::components::drawable::NaniBundle::<StoneFloorInstanceData> {
            desc: desc(0, GpuWorldSubsetKind::RoofFloor),
            transform: Transform::from_translation(Vec3::new(12.0, 13.0, 14.0)),
            marker: InstanceDataMarker::default(),
        }, cpu_boundary_roof(curved))).id()
    }

    fn actual_roof_payload(world: &World, entity: Entity) -> [u32; 16] {
        let key = world.get::<InstanceKey>(entity).unwrap();
        let subset = world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::RoofFloor);
        subset.data().as_any().downcast_ref::<
            crate::render::nani::gpu_world::InstanceDataBuffer<StoneFloorInstanceData>>()
            .unwrap().shadow.data[key.index as usize].raw
    }

    #[test]
    fn roof_branch_uses_real_components_barrier_nonzero_payload_and_dirty_flush() {
        let dev = MockDevice::default();
        let mut world = roof_world(&dev);
        let curved = spawn_cpu_roof(&mut world, true);
        let straight = spawn_cpu_roof(&mut world, false);
        world.entity_mut(curved).insert(cpu_boundary_roof_animation(0.375, 0.875));
        let mut schedule = Schedule::default();
        schedule.add_systems((nani_maintain_instances_ecs, ApplyDeferred,
            nani_update_roof_floor_for_roofs_ecs).chain_ignore_deferred());
        schedule.run(&mut world);
        for (entity, is_curved, animation) in [(curved, true, 0.375f32), (straight, false, 1.0)] {
            let key = *world.get::<InstanceKey>(entity).unwrap();
            assert_eq!(key.kind, GpuWorldSubsetKind::RoofFloor);
            let words = actual_roof_payload(&world, entity);
            let position = if is_curved {
                Vec3::new(2.5, 1.25 - 0.69, -3.25)
            } else {
                Vec3::new(4.5, 1.25 - 0.69, 1.25)
            };
            assert_eq!(&words[..4], &[position.x.to_bits(), position.y.to_bits(), position.z.to_bits(), 7]);
            if is_curved {
                let angle = 0.73 + std::f32::consts::TAU;
                assert!((f32::from_bits(words[4]) + angle.sin()).abs() < 1e-6);
                assert!((f32::from_bits(words[5]) - angle.cos()).abs() < 1e-6);
                assert_eq!(&words[6..8], &[9.0f32.to_bits(), 9.0f32.to_bits()]);
            } else {
                assert_eq!(&words[4..8], &[2.5f32.to_bits(), (-3.25f32).to_bits(),
                    6.0f32.to_bits(), 7.0f32.to_bits()]);
            }
            assert_eq!(words[8], animation.to_bits()); // channel0, or original absent-animation1.0
            assert_eq!(&words[9..], &[u32::from(!is_curved), 0, 0, 0x12, 111, 222, 0]);
            let subset = world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::RoofFloor);
            assert_eq!(subset.instance_count(), 2);
            assert_eq!(subset.positions()[key.index as usize].position, position);
            assert_eq!(subset.positions()[key.index as usize].radius.to_f32(), if is_curved { 9.0 } else { 7.0 });
            let payload = subset.data().as_any().downcast_ref::<
                crate::render::nani::gpu_world::InstanceDataBuffer<StoneFloorInstanceData>>().unwrap();
            assert_eq!(payload.shadow.dirty.to_range(), Some(0..2));
        }
        nani_flush_instance_updates(&mut world.resource_mut::<GpuWorld>(), &dev);
        assert_eq!(world.resource::<GpuWorld>().previous_instance_count(), 2);
        assert_eq!(world.resource::<GpuWorld>().total_instance_count, 0);
        assert!(dev.writes.borrow().iter().any(|(ranges, offset, bytes)|
            ranges == &[0..128] && *offset == 0 && *bytes == 128));
        world.clear_trackers();
        // Original With-only update domain also contains active keys without
        // a descriptor. A real Roof still updates through that exact domain.
        world.entity_mut(curved).remove::<NaniInstanceDesc>();
        world.entity_mut(curved).insert(cpu_boundary_roof_animation(0.625, 0.125));
        let mut update_only = Schedule::default();
        update_only.add_systems(nani_update_roof_floor_for_roofs_ecs);
        update_only.run(&mut world);
        assert_eq!(actual_roof_payload(&world, curved)[8], 0.625f32.to_bits());
        assert_eq!(world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::RoofFloor).instance_count(), 2);
    }

    #[test]
    fn new_roof_descriptor_requires_roof_transform_marker_and_unique_mapped_marker() {
        for missing in ["Roof", "Transform", "marker", "ambiguous marker"] {
            let dev = MockDevice::default();
            let mut world = roof_world(&dev);
            let entity = spawn_cpu_roof(&mut world, true);
            match missing {
                "Roof" => { world.entity_mut(entity).remove::<Roof>(); }
                "Transform" => { world.entity_mut(entity).remove::<Transform>(); }
                "marker" => { world.entity_mut(entity).remove::<InstanceDataMarker<StoneFloorInstanceData>>(); }
                _ => { world.entity_mut(entity).insert(InstanceDataMarker::<PlantInstanceData>::default()); }
            }
            let mut maintain = Schedule::default();
            maintain.add_systems(nani_maintain_instances_ecs);
            assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                maintain.run(&mut world))).is_err(), "accepted missing {missing}");
            assert!(world.get::<InstanceKey>(entity).is_none());
            let gpu = world.resource::<GpuWorld>();
            assert_eq!(gpu.total_instance_count, 0);
            assert_eq!(gpu.owned_subset_mut(GpuWorldSubsetKind::RoofFloor).instance_count(), 0);
        }
    }

    #[test]
    fn missing_roof_is_rejected_for_existing_descriptor_and_descriptorless_active_marker() {
        for keep_descriptor in [true, false] {
            let dev = MockDevice::default();
            let mut world = roof_world(&dev);
            let entity = spawn_cpu_roof(&mut world, true);
            let mut allocation = Schedule::default();
            allocation.add_systems((nani_maintain_instances_ecs, ApplyDeferred,
                nani_update_roof_floor_for_roofs_ecs).chain_ignore_deferred());
            allocation.run(&mut world);
            let key = *world.get::<InstanceKey>(entity).unwrap();
            let before_payload = actual_roof_payload(&world, entity);
            nani_flush_instance_updates(&mut world.resource_mut::<GpuWorld>(), &dev);
            let before_writes = dev.writes.borrow().len();
            world.clear_trackers();
            world.entity_mut(entity).remove::<Roof>();
            if !keep_descriptor { world.entity_mut(entity).remove::<NaniInstanceDesc>(); }
            let mut update_only = Schedule::default();
            update_only.add_systems(nani_update_roof_floor_for_roofs_ecs);
            let error = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                if keep_descriptor { allocation.run(&mut world); }
                else { update_only.run(&mut world); }
            })).expect_err("unsupported active row was silently filtered");
            if !keep_descriptor {
                let message = error.downcast_ref::<&str>().copied()
                    .or_else(|| error.downcast_ref::<String>().map(String::as_str)).unwrap_or("");
                assert!(message.contains("RoofFloor ECS support requires a real Roof"));
            }
            assert_eq!(world.get::<InstanceKey>(entity).unwrap(), &key);
            assert_eq!(actual_roof_payload(&world, entity), before_payload);
            let gpu = world.resource::<GpuWorld>();
            assert_eq!(gpu.total_instance_count, 0);
            assert_eq!(gpu.previous_instance_count(), 1);
            let subset = gpu.owned_subset_mut(GpuWorldSubsetKind::RoofFloor);
            assert_eq!(subset.instance_count(), 1);
            assert!(subset.positions_dirty().is_empty());
            let payload = subset.data().as_any().downcast_ref::<
                crate::render::nani::gpu_world::InstanceDataBuffer<StoneFloorInstanceData>>().unwrap();
            assert!(payload.shadow.dirty.is_empty());
            assert_eq!(dev.writes.borrow().len(), before_writes);
        }
    }

    #[test]
    fn empty_registered_roof_subset_stays_a_real_zero_count() {
        let dev = MockDevice::default();
        let mut world = roof_world(&dev);
        let before_writes = dev.writes.borrow().len();
        let mut schedule = Schedule::default();
        schedule.add_systems((nani_maintain_instances_ecs, ApplyDeferred,
            nani_update_roof_floor_for_roofs_ecs).chain_ignore_deferred());
        schedule.run(&mut world);
        let gpu = world.resource::<GpuWorld>();
        assert_eq!(gpu.total_instance_count, 0);
        let subset = gpu.owned_subset_mut(GpuWorldSubsetKind::RoofFloor);
        assert_eq!(subset.instance_count(), 0);
        assert!(subset.positions_dirty().is_empty());
        assert_eq!(dev.writes.borrow().len(), before_writes);
    }

    fn world_with_solid_pair(dev: &MockDevice) -> GpuWorld {
        let mut gpu = GpuWorld::new(dev);
        gpu.assign_subset(GpuWorldSubsetKind::SolidAlbedoMap, GpuWorldSubset::Owned(
            OwnedGpuWorldSubset::new::<SolidAlbedoMapInstanceData>(dev,
                GpuWorldSubsetKind::SolidAlbedoMap)));
        gpu.assign_subset(GpuWorldSubsetKind::SolidAlbedoMapEmissive, GpuWorldSubset::Owned(
            OwnedGpuWorldSubset::new::<SolidAlbedoMapEmissiveInstanceData>(dev,
                GpuWorldSubsetKind::SolidAlbedoMapEmissive)));
        gpu
    }

    #[test]
    fn solid_pair_uses_canonical_nonzero_codes_through_barrier_payload_and_flush() {
        let dev = MockDevice::default();
        let mut world = World::new();
        world.insert_resource(world_with_solid_pair(&dev));
        world.insert_resource(MeshAtlasLibrary::default());
        let plain_transform = Transform::from_translation(Vec3::new(2.0, 3.0, 4.0));
        let emissive_transform = Transform::from_translation(Vec3::new(-5.0, 6.0, 7.0));
        let plain = world.spawn((desc(0, GpuWorldSubsetKind::SolidAlbedoMap),
            plain_transform, InstanceDataMarker::<SolidAlbedoMapInstanceData>::default(),
            NaniSolidTexture::try_from(2).unwrap())).id();
        let emissive = world.spawn((desc(0, GpuWorldSubsetKind::SolidAlbedoMapEmissive),
            emissive_transform, InstanceDataMarker::<SolidAlbedoMapEmissiveInstanceData>::default(),
            NaniSolidTexture::try_from(8).unwrap())).id();
        let mut schedule = Schedule::default();
        schedule.add_systems((nani_maintain_instances_ecs, ApplyDeferred,
            (nani_update_solid_albedo_map_ecs, nani_update_solid_albedo_map_emissive_ecs))
            .chain_ignore_deferred());
        schedule.run(&mut world);
        for (entity, kind, position, expected) in [
            (plain, GpuWorldSubsetKind::SolidAlbedoMap, plain_transform.translation, 8),
            (emissive, GpuWorldSubsetKind::SolidAlbedoMapEmissive, emissive_transform.translation, 7),
        ] {
            let key = *world.get::<InstanceKey>(entity).unwrap();
            assert_eq!(key.kind, kind);
            let gpu = world.resource::<GpuWorld>();
            let subset = gpu.owned_subset_mut(kind);
            assert_eq!(subset.instance_count(), 1);
            assert_eq!(subset.positions()[key.index as usize].position, position);
            assert_eq!(subset.positions_dirty().to_range(), Some(0..1));
            let words = if kind == GpuWorldSubsetKind::SolidAlbedoMap {
                let data = subset.data().as_any().downcast_ref::<
                    crate::render::nani::gpu_world::InstanceDataBuffer<SolidAlbedoMapInstanceData>>()
                    .unwrap();
                assert_eq!(data.shadow.dirty.to_range(), Some(0..1));
                data.shadow.data[key.index as usize].raw
            } else {
                let data = subset.data().as_any().downcast_ref::<
                    crate::render::nani::gpu_world::InstanceDataBuffer<SolidAlbedoMapEmissiveInstanceData>>()
                    .unwrap();
                assert_eq!(data.shadow.dirty.to_range(), Some(0..1));
                data.shadow.data[key.index as usize].raw
            };
            assert_eq!(words[12], expected); // code2 maps to8, not identity2
            assert_eq!(&words[13..16], &[0, 0, 0]);
        }
        nani_flush_instance_updates(&mut world.resource_mut::<GpuWorld>(), &dev);
        assert_eq!(world.resource::<GpuWorld>().previous_instance_count(), 2);
        assert_eq!(world.resource::<GpuWorld>().total_instance_count, 0);
        assert_eq!(dev.writes.borrow().iter().filter(|(r, o, n)|
            r == &[0..64] && *o == 0 && *n == 64).count(), 2);
        world.clear_trackers();
        schedule.run(&mut world); // original With-only updates run again unchanged
        let gpu = world.resource::<GpuWorld>();
        let subset = gpu.owned_subset_mut(GpuWorldSubsetKind::SolidAlbedoMap);
        let data = subset.data().as_any().downcast_ref::<
            crate::render::nani::gpu_world::InstanceDataBuffer<SolidAlbedoMapInstanceData>>()
            .unwrap();
        assert_eq!(data.shadow.dirty.to_range(), Some(0..1));
        assert_eq!(subset.instance_count(), 1);
    }

    #[test]
    fn solid_descriptor_missing_canonical_selector_fails_before_allocation() {
        let dev = MockDevice::default();
        let mut world = World::new();
        world.insert_resource(world_with_solid_pair(&dev));
        world.insert_resource(MeshAtlasLibrary::default());
        let entity = world.spawn((desc(0, GpuWorldSubsetKind::SolidAlbedoMap),
            Transform::default(), InstanceDataMarker::<SolidAlbedoMapInstanceData>::default())).id();
        let mut schedule = Schedule::default();
        schedule.add_systems(nani_maintain_instances_ecs);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
            schedule.run(&mut world))).is_err());
        assert!(world.get::<InstanceKey>(entity).is_none());
        let gpu = world.resource::<GpuWorld>();
        assert_eq!(gpu.total_instance_count, 0);
        assert_eq!(gpu.owned_subset_mut(GpuWorldSubsetKind::SolidAlbedoMap).instance_count(), 0);
    }

    // Exercise actual component insertion and the explicit schedule barrier;
    // the recording CPU device checks the upload boundary, not GPU execution.
    #[test]
    fn ecs_maintain_applies_keys_before_payload_update_and_flush() {
        let dev = MockDevice::default();
        let mut world = World::new();
        world.insert_resource(world_with(GpuWorldSubsetKind::Plant, &dev));
        world.insert_resource(MeshAtlasLibrary::default());
        let transform = Transform {
            translation: Vec3::new(2.5, -1.25, 6.0),
            scale: Vec3::new(2.0, 3.0, 4.0),
            ..Default::default()
        };
        let entity = world.spawn((
            desc(7, GpuWorldSubsetKind::Plant), transform,
            InstanceDataMarker::<PlantInstanceData>::default(),
        )).id();
        let mut schedule = Schedule::default();
        schedule.add_systems((nani_maintain_instances_ecs, ApplyDeferred,
            nani_update_plant_ecs).chain_ignore_deferred());
        schedule.run(&mut world);
        let key = *world.get::<InstanceKey>(entity).unwrap();
        let gpu = world.resource::<GpuWorld>();
        {
            let subset = gpu.owned_subset_mut(GpuWorldSubsetKind::Plant);
            assert_eq!(subset.instance_count(), 1);
            assert_eq!(subset.positions()[key.index as usize].position, transform.translation);
            let payload = subset.data().as_any()
                .downcast_ref::<crate::render::nani::gpu_world::InstanceDataBuffer<PlantInstanceData>>()
                .unwrap();
            assert_eq!(payload.shadow.data[key.index as usize].raw,
                build_plant(transform.translation, transform.rotation, transform.scale).words);
            assert_eq!(payload.shadow.dirty.to_range(), Some(0..1));
        }
        nani_flush_instance_updates(&mut world.resource_mut::<GpuWorld>(), &dev);
        let gpu = world.resource::<GpuWorld>();
        assert_eq!(gpu.previous_instance_count(), 1);
        assert_eq!(gpu.total_instance_count, 0);
        let subset = gpu.owned_subset_mut(GpuWorldSubsetKind::Plant);
        assert!(subset.positions_dirty().is_empty());
        let payload = subset.data().as_any()
            .downcast_ref::<crate::render::nani::gpu_world::InstanceDataBuffer<PlantInstanceData>>()
            .unwrap();
        assert!(payload.shadow.dirty.is_empty());
        assert!(dev.writes.borrow().iter().any(|(ranges, offset, bytes)|
            ranges == &[0..16] && *offset == 0 && *bytes == 16));
        assert!(dev.writes.borrow().iter().any(|(ranges, offset, bytes)|
            ranges == &[0..48] && *offset == 0 && *bytes == 48));
    }

    #[test]
    fn ecs_removal_repairs_live_dense_key_and_changed_desc_uses_same_allocation() {
        let dev = MockDevice::default();
        let mut world = World::new();
        world.insert_resource(world_with(GpuWorldSubsetKind::Plant, &dev));
        world.insert_resource(MeshAtlasLibrary::default());
        let first = world.spawn((desc(1, GpuWorldSubsetKind::Plant), Transform::default(),
            InstanceDataMarker::<PlantInstanceData>::default())).id();
        let second = world.spawn((desc(2, GpuWorldSubsetKind::Plant), Transform::default(),
            InstanceDataMarker::<PlantInstanceData>::default())).id();
        let mut schedule = Schedule::default();
        schedule.add_systems((nani_maintain_instances_ecs, ApplyDeferred,
            nani_update_plant_ecs).chain_ignore_deferred());
        schedule.run(&mut world);
        let second_id = world.get::<InstanceKey>(second).unwrap().id;
        world.clear_trackers();
        world.despawn(first);
        world.get_mut::<NaniInstanceDesc>(second).unwrap().mesh = NaniMeshSelector::AtlasPart(19);
        schedule.run(&mut world);
        let key = *world.get::<InstanceKey>(second).unwrap();
        assert_eq!(key.id, second_id);
        assert_eq!(key.index, 0);
        let gpu = world.resource::<GpuWorld>();
        let subset = gpu.owned_subset_mut(GpuWorldSubsetKind::Plant);
        assert_eq!(subset.instance_count(), 1);
        assert_eq!(subset.positions()[0].mesh_index, 19);
        assert_eq!(gpu.total_instance_count, 2);
    }

    #[test]
    fn incomplete_mapping_is_rejected_before_key_or_count_publication() {
        let dev = MockDevice::default();
        let mut world = World::new();
        world.insert_resource(world_with(GpuWorldSubsetKind::Plant, &dev));
        world.insert_resource(MeshAtlasLibrary::default());
        let entity = world.spawn((desc(0, GpuWorldSubsetKind::Wren), Transform::default())).id();
        let mut schedule = Schedule::default();
        schedule.add_systems(nani_maintain_instances_ecs);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| schedule.run(&mut world)));
        assert!(result.is_err());
        assert!(world.get::<InstanceKey>(entity).is_none());
        assert_eq!(world.resource::<GpuWorld>().total_instance_count, 0);
    }

    #[test]
    fn registered_unmapped_subset_with_no_descriptor_keeps_real_zero_count() {
        let dev = MockDevice::default();
        let mut gpu = world_with(GpuWorldSubsetKind::Plant, &dev);
        gpu.assign_subset(GpuWorldSubsetKind::Wren, GpuWorldSubset::Owned(
            OwnedGpuWorldSubset::new::<WrenInstanceData>(&dev, GpuWorldSubsetKind::Wren)));
        let mut world = World::new();
        world.insert_resource(gpu);
        world.insert_resource(MeshAtlasLibrary::default());
        let mut schedule = Schedule::default();
        schedule.add_systems(nani_maintain_instances_ecs);
        schedule.run(&mut world);
        let gpu = world.resource::<GpuWorld>();
        assert_eq!(gpu.total_instance_count, 0);
        assert_eq!(gpu.owned_subset_mut(GpuWorldSubsetKind::Wren).instance_count(), 0);
    }

    #[test]
    fn existing_descriptor_cannot_silently_keep_payload_after_marker_removal() {
        let dev = MockDevice::default();
        let mut world = World::new();
        world.insert_resource(world_with(GpuWorldSubsetKind::Plant, &dev));
        world.insert_resource(MeshAtlasLibrary::default());
        let entity = world.spawn((desc(0, GpuWorldSubsetKind::Plant), Transform::default(),
            InstanceDataMarker::<PlantInstanceData>::default())).id();
        let mut schedule = Schedule::default();
        schedule.add_systems((nani_maintain_instances_ecs, ApplyDeferred,
            nani_update_plant_ecs).chain_ignore_deferred());
        schedule.run(&mut world);
        let key = *world.get::<InstanceKey>(entity).unwrap();
        world.clear_trackers();
        world.entity_mut(entity).remove::<InstanceDataMarker<PlantInstanceData>>();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| schedule.run(&mut world)));
        assert!(result.is_err());
        assert_eq!(world.get::<InstanceKey>(entity).unwrap(), &key);
        assert_eq!(world.resource::<GpuWorld>().total_instance_count, 1);
    }

    #[test]
    fn maintain_creates_then_updates_then_removes() {
        let dev = MockDevice::default();
        let mut gpu = world_with(GpuWorldSubsetKind::Plant, &dev);
        let atlas = MeshAtlasLibrary::default();
        let mut local = NaniActiveInstanceCollection::default();
        let e = entities(1)[0];
        let d = desc(7, GpuWorldSubsetKind::Plant);
        let t = Transform {
            translation: Vec3::new(1.0, 2.0, 3.0),
            scale: Vec3::new(1.0, -4.0, 2.0),
            ..Default::default()
        };

        let rows = [NewInstanceRow {
            entity: e,
            desc: &d,
            transform: &t,
        }];
        let inserts = nani_maintain_instances([], &rows, &mut [], &atlas, &mut gpu, &mut local);
        assert_eq!(inserts.len(), 1);
        assert_eq!(gpu.total_instance_count, 1);
        let mut key = inserts[0].1;
        {
            let subset = gpu.owned_subset_mut(GpuWorldSubsetKind::Plant);
            assert_eq!(subset.instance_count(), 1);
            let cull = subset.positions()[0];
            assert_eq!(cull.position, t.translation);
            assert_eq!(cull.mesh_index, 7);
            assert_eq!(cull.radius.to_f32(), 4.0);
        }

        // 变更：只改 mesh 序号。
        let d2 = desc(9, GpuWorldSubsetKind::Plant);
        let mut changed = [ChangedInstanceRow {
            desc: &d2,
            active: &mut key,
        }];
        nani_maintain_instances([], &[], &mut changed, &atlas, &mut gpu, &mut local);
        assert_eq!(
            gpu.owned_subset_mut(GpuWorldSubsetKind::Plant).positions()[0].mesh_index,
            9
        );

        // 删除。
        nani_maintain_instances([e], &[], &mut [], &atlas, &mut gpu, &mut local);
        assert_eq!(
            gpu.owned_subset_mut(GpuWorldSubsetKind::Plant)
                .instance_count(),
            0
        );
        assert!(local.active.is_empty());
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn removing_an_unknown_entity_panics_like_the_original() {
        let dev = MockDevice::default();
        let mut gpu = world_with(GpuWorldSubsetKind::Plant, &dev);
        let e = entities(1)[0];
        nani_maintain_instances(
            [e],
            &[],
            &mut [],
            &MeshAtlasLibrary::default(),
            &mut gpu,
            &mut NaniActiveInstanceCollection::default(),
        );
    }

    #[test]
    fn named_mesh_resolves_through_the_atlas_for_that_shader() {
        let dev = MockDevice::default();
        let mut gpu = world_with(GpuWorldSubsetKind::Plant, &dev);
        let mut atlas = MeshAtlasLibrary::default();
        let mut mesh = Mesh::new("m");
        mesh.set_attribute("Vertex_Position", vec![Vec3::ZERO, Vec3::X, Vec3::Y]);
        mesh.set_indices(vec![0, 1, 2]);
        let name = NameHash::compute("bush");
        atlas.add(GpuWorldSubsetKind::Plant, name, &mesh);
        let e = entities(1)[0];
        let d = NaniInstanceDesc {
            mesh: NaniMeshSelector::Named(name),
            subset_kind: GpuWorldSubsetKind::Plant,
        };
        let t = Transform::default();
        let rows = [NewInstanceRow {
            entity: e,
            desc: &d,
            transform: &t,
        }];
        let mut local = NaniActiveInstanceCollection::default();
        nani_maintain_instances([], &rows, &mut [], &atlas, &mut gpu, &mut local);
        assert_eq!(
            gpu.owned_subset_mut(GpuWorldSubsetKind::Plant)
                .instance_count(),
            1
        );
    }

    #[test]
    #[should_panic(expected = "but this mesh is not registered for that shader in nani_meshes.ron")]
    fn named_mesh_for_another_shader_panics() {
        let dev = MockDevice::default();
        let mut gpu = world_with(GpuWorldSubsetKind::Plant, &dev);
        let mut atlas = MeshAtlasLibrary::default();
        let mut mesh = Mesh::new("m");
        mesh.set_attribute("Vertex_Position", vec![Vec3::ZERO, Vec3::X, Vec3::Y]);
        mesh.set_indices(vec![0, 1, 2]);
        let name = NameHash::compute("bush");
        atlas.add(GpuWorldSubsetKind::Flower, name, &mesh);
        let e = entities(1)[0];
        let d = NaniInstanceDesc {
            mesh: NaniMeshSelector::Named(name),
            subset_kind: GpuWorldSubsetKind::Plant,
        };
        let t = Transform::default();
        let rows = [NewInstanceRow {
            entity: e,
            desc: &d,
            transform: &t,
        }];
        nani_maintain_instances(
            [],
            &rows,
            &mut [],
            &atlas,
            &mut gpu,
            &mut NaniActiveInstanceCollection::default(),
        );
    }

    #[test]
    fn update_wrapper_writes_the_builder_payload() {
        let dev = MockDevice::default();
        let mut gpu = world_with(GpuWorldSubsetKind::Plant, &dev);
        let atlas = MeshAtlasLibrary::default();
        let mut local = NaniActiveInstanceCollection::default();
        let e = entities(1)[0];
        let d = desc(1, GpuWorldSubsetKind::Plant);
        let t = Transform {
            translation: Vec3::new(5.0, 0.0, -2.0),
            ..Default::default()
        };
        let rows = [NewInstanceRow {
            entity: e,
            desc: &d,
            transform: &t,
        }];
        let mut key =
            nani_maintain_instances([], &rows, &mut [], &atlas, &mut gpu, &mut local)[0].1;
        let t2 = Transform {
            translation: Vec3::new(6.0, 1.0, 0.0),
            ..Default::default()
        };
        nani_update_plant(
            &gpu,
            &mut [TransformRow {
                key: &mut key,
                transform: &t2,
            }],
        );
        let subset = gpu.owned_subset_mut(GpuWorldSubsetKind::Plant);
        assert_eq!(subset.positions()[0].position, t2.translation);
        // 数据缓冲类型不符会 panic；这里同类型，写入成功即说明 kind↔类型对应。
        assert_eq!(subset.instance_count(), 1);
    }

    #[test]
    #[should_panic(expected = "The instance data for Plant is not")]
    fn writing_with_the_wrong_instance_type_panics() {
        let dev = MockDevice::default();
        // 在 Plant 槽里放 SolidVertexColor 的数据缓冲，再用 plant 系统写 → 类型不符。
        let mut gpu = GpuWorld::new(&dev);
        gpu.assign_subset(
            GpuWorldSubsetKind::Plant,
            GpuWorldSubset::Owned(OwnedGpuWorldSubset::new::<SolidVertexColorInstanceData>(
                &dev,
                GpuWorldSubsetKind::Plant,
            )),
        );
        let mut local = NaniActiveInstanceCollection::default();
        let e = entities(1)[0];
        let d = desc(1, GpuWorldSubsetKind::Plant);
        let t = Transform::default();
        let rows = [NewInstanceRow {
            entity: e,
            desc: &d,
            transform: &t,
        }];
        let mut key = nani_maintain_instances(
            [],
            &rows,
            &mut [],
            &MeshAtlasLibrary::default(),
            &mut gpu,
            &mut local,
        )[0]
        .1;
        nani_update_plant(
            &gpu,
            &mut [TransformRow {
                key: &mut key,
                transform: &t,
            }],
        );
    }

    #[test]
    fn flush_rolls_the_instance_counter() {
        let dev = MockDevice::default();
        let mut gpu = world_with(GpuWorldSubsetKind::Plant, &dev);
        gpu.total_instance_count = 5;
        nani_flush_instance_updates(&mut gpu, &dev);
        assert_eq!(gpu.total_instance_count, 0);
        assert_eq!(gpu.previous_instance_count(), 5);
    }
}
