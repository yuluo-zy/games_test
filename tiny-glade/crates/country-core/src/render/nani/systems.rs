//! `render::nani::systems` — nani 的 ECS 系统层：`nani_maintain_instances`、20 个 `nani_update_*`、
//! `nani_flush_instance_updates`（B 构建 0x14206f300 / 0x142070ac0..0x14207fcc0 / 0x142080640）。
//!
//! 本工作区的系统都写成"行视图 + 纯函数"（`systems/render_loop` 同款），不依赖真实的 bevy `Query`：
//! 每个系统函数收一组行（原版 `Query` 的一项），行里的字段就是该系统读取的组件值。载荷的数值全部
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
//! 未恢复（U）：19 个 gap 组件（`Wren`、`Plaster`、`Smoke`……）的真实字段布局；这里的行视图只放
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

/// 网格选择器（O：`NaniInstanceDesc` 的 `+0` 判别位：1 = `AtlasPart`，0 = `Named`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NaniMeshSelector {
    AtlasPart(u32),
    Named(NameHash),
}

/// 实例描述组件（O：24 B）。
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
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

/// `nani_maintain_instances`（O 0x14206f300）。返回本帧要压入 `Commands` 的 `Insert<NaniActiveInstance>`。
pub fn nani_maintain_instances(
    removed: impl IntoIterator<Item = Entity>,
    new: &[NewInstanceRow<'_>],
    changed: &mut [ChangedInstanceRow<'_>],
    atlas: &MeshAtlasLibrary,
    gpu_world: &mut GpuWorld,
    local: &mut NaniActiveInstanceCollection,
) -> Vec<(Entity, InstanceKey)> {
    // 1. 删除。
    for entity in removed {
        let (_, mut key) = local
            .active
            .remove_entry(&entity)
            .expect("called `Option::unwrap()` on a `None` value");
        gpu_world.remove_instance(&mut key);
    }

    // 2. 新建。
    let mut inserts = Vec::new();
    for row in new {
        let kind = row.desc.subset_kind;
        let part = match row.desc.mesh {
            NaniMeshSelector::AtlasPart(part) => part,
            NaniMeshSelector::Named(name) => {
                atlas
                    .get_by_name(name)
                    .iter()
                    .find(|part| part.subset == kind)
                    .unwrap_or_else(|| describe_part_mismatch(name, kind))
                    .part_index
            }
        };
        let compact = compact_key_for(row.transform, part);
        let mut record = None;
        gpu_world
            .owned_subset_mut(kind)
            .extend_no_data(kind, &[compact], |key| record = Some(key));
        gpu_world.total_instance_count += 1;
        match record {
            Some(key) => {
                local.active.insert(row.entity, key);
                inserts.push((row.entity, key));
            }
            None => {
                nani_warn_once!(
                    LOG_TARGET,
                    format!(
                        "Instance limit reached while trying to spawn {:?}",
                        row.desc
                    )
                );
            }
        }
    }

    // 3. 变更。
    for row in changed {
        let part = match row.desc.mesh {
            NaniMeshSelector::AtlasPart(part) => part,
            NaniMeshSelector::Named(name) => {
                let parts = atlas.get_by_name(name);
                assert_eq!(parts.len(), 1);
                assert_eq!(parts[0].subset, row.desc.subset_kind);
                parts[0].part_index
            }
        };
        gpu_world.update_instance_mesh(row.active, part as u16);
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::mesh::Mesh;
    use crate::render::nani::gpu_world::{GpuWorldSubset, OwnedGpuWorldSubset};
    use crate::render::ssbo::tests::MockDevice;
    use bevy_ecs::world::World;
    use glam::Vec3;

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
    #[should_panic(expected = "is not")]
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
