//! systems.rs — nani GPU 世界的 ECS 维护/更新/冲刷系统（B 阶段按机器码重写）。
//!
//! 机器码锚点（B patched EXE，cf048e27…；地址表见 `evidence/country-core-audit/B_n_sys.md`）：
//! - `nani_maintain_instances` 0x14206f300（6068 B；closure$1 0x141565950）。
//! - `nani_update_*` 21 个系统：0x142070ac0..0x14207fcc0。
//! - `nani_flush_instance_updates` 0x142080640（1053 B）。
//!
//! 标注：**O**=机器码观测；**I**=推断；**U**=未知（typed gap，不编造）。
//!
//! 共同模式（O，所有 `nani_update_*`）：puffin scope → `GpuWorld::owned_subset_mut(kind)` 写锁 →
//! 对 query 的每一行计算 `T` → `OwnedGpuWorldSubset::update_instance_data<T>(subset, key,
//! &position, sort_key: f32, &data)`（0x14156fff0 系）→ 释放锁。`position`/`sort_key` 由 `Transform`
//! 得出（`sort_key = max(|sx|,|sy|,|sz|)`）；实例数据 `T` 的 12 字（0x30）或 16 字（0x40）等按各系统。
//!
//! 与 gpu_world.rs 的接口：直接调用 n_world 重写后的真实签名（`owned_subset_mut -> MappedRwLockWriteGuard`、
//! `extend_no_data(kind, &[InstanceCullData], cb(InstanceKey))`、`update_instance_data<T>(&mut InstanceKey, ..)`、
//! `update_instance_mesh(&mut InstanceKey, u16)`、`remove_instance(&mut InstanceKey)`）；`NaniActiveInstance` 即
//! 12B 的 `InstanceKey`。

#![allow(dead_code, unused_imports, clippy::too_many_arguments)]

use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

use bevy_ecs::change_detection::{DetectChanges, Res, ResMut};
use bevy_ecs::entity::Entity;
use bevy_ecs::query::{Changed, Or, With, Without};
use bevy_ecs::removal_detection::RemovedComponents;
use bevy_ecs::system::{Commands, Local, Query};
use glam::{Mat3, Quat, Vec2, Vec3, Vec4};
use hashbrown::HashMap;

use crate::asset_libraries::texture_library::AssetTextureLibrary;
use crate::components::drawable::{AlbedoMap, NaniSolidTexture, PrevTransform};
use crate::components::drawable::{NaniActiveInstance, NaniInstanceDesc, NaniInstanceMeshDesc};
use crate::components::{
    BrickColor, ClothlineCloth, FlatLighting, GothicWindowBrickDeformation, LanternLightParams,
    NormalMap, SemiTransparent, Smoke,
};
use crate::name_hash::NameHash;
use crate::render::nani::gpu_world::{
    GpuWorld, GpuWorldSubset, GpuWorldSubsetKind, InstanceCullData, InstanceDataMarker,
    InstanceElement, InstanceKey, NaniInstanceData, OwnedGpuWorldSubset,
};
use crate::render::nani::instance_data::*;
use crate::resources::color_id::{ColorId, HalftimberColorIds};
use crate::resources::critters::Wren;
use crate::resources::render::mesh_atlas_library::MeshAtlasLibrary;
use crate::resources::stairs::visual::StairsFloorMeshChunk;
use crate::resources::walls::curved_halftimber::CurvedHalfTimber;
use crate::resources::walls::plaster::{AnimatedPlasterPeel, Plaster};
use crate::resources::walls::public_walls::PublicWalls;
use crate::resources::walls::{Floor, WallId};
use crate::resources::window_lights::{GlobalWindowLightsControl, WindowLight};
use crate::utils::components::transform::Transform;

// ===========================================================================
// 共享辅助
// ===========================================================================

/// u8 → `GpuWorldSubsetKind`（O：27 个变体；各系统的 kind 是 `mov dl, K` 立即数）。
pub fn kind_from_u8(v: u8) -> GpuWorldSubsetKind {
    GpuWorldSubsetKind::all()
        .nth(v as usize)
        .unwrap_or_else(|| panic!("invalid GpuWorldSubsetKind u8 {v}"))
}

/// 一次 `update_instance_data<T>(subset, key, &position, sort_key, &data)` 调用的全部参数（O）：
/// 写 16B 紧凑 key `{position: Vec3 @0, f16(sort_key) @0xc}`，并复制 `size_of::<T>()` 字节实例数据。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InstanceUpdate<const N: usize> {
    pub position: Vec3,
    pub sort_key: f32,
    /// 实例数据载荷（小端 32 位字；`N*4 == size_of::<T>()`）。
    pub words: [u32; N],
}

#[inline]
pub fn f32w(x: f32) -> u32 {
    x.to_bits()
}

/// 把字载荷重解释为实例数据结构体（`instance_data.rs` 里各类型当前是 `[u32; N]` 占位，尺寸来自机器码）。
pub fn pack_instance<T: Copy, const N: usize>(words: [u32; N]) -> T {
    assert_eq!(
        core::mem::size_of::<T>(),
        4 * N,
        "instance data size mismatch"
    );
    // SAFETY：尺寸相等且 `T` 为 `#[repr(C)]` 的纯数据 GPU 布局类型（Copy）。
    unsafe { core::mem::transmute_copy(&words) }
}

/// 每行的出口：`OwnedGpuWorldSubset::update_instance_data<T>`（0x14156fff0 系）。
/// 机器码的 key 指针来自 `&NaniActiveInstance` 查询项；真实签名取 `&mut InstanceKey`（会回写稠密下标提示），
/// 查询只给 `&`，所以这里在栈上拷一份（提示不回写进组件；仅影响快路径命中率，不影响结果）。
pub fn push_update<T: InstanceElement + Copy, const N: usize>(
    subset: &mut OwnedGpuWorldSubset,
    active: &NaniActiveInstance,
    u: InstanceUpdate<N>,
) {
    let mut key: InstanceKey = *active;
    subset.update_instance_data::<T>(
        &mut key,
        u.position,
        u.sort_key,
        pack_instance::<T, N>(u.words),
    );
}

/// U 型间隙组件（`crate::Gap` 单元结构，布局归 components/* 所有者）的字访问：在其所有者恢复布局前
/// 返回全 0（与"组件缺省"不同，纯占位；各系统注释里写明了机器码读取的偏移）。
pub fn gap_words<C, const N: usize>(_c: &C) -> [u32; N] {
    [0; N]
}

/// 16B 紧凑 key（O：`extend_no_data` 的 items 元素；`update_instance_data` 写同一布局的前 14 字节）：
/// `{position: Vec3 @0, sort_key: f16 @0xc, mesh_part: u16 @0xe}` —— 即 gpu_world 的 [`InstanceCullData`]
/// （`radius` = f16 sort_key，`mesh_index` = mesh_part）。
pub type NaniCompactKey = InstanceCullData;

/// 把紧凑 key 展成 4 个小端 u32 字（测试用）。
pub fn compact_key_words(k: &InstanceCullData) -> [u32; 4] {
    [
        k.position.x.to_bits(),
        k.position.y.to_bits(),
        k.position.z.to_bits(),
        k.radius.to_bits() as u32 | ((k.mesh_index as u32) << 16),
    ]
}

/// `max(|sx|,|sy|,|sz|)`（O：`andps abs; maxss` 链，NaN 时取另一操作数）。
pub fn max_abs_scale(scale: Vec3) -> f32 {
    scale.z.abs().max(scale.y.abs()).max(scale.x.abs())
}

// ===========================================================================
// `warn_once`（O：maintain 里 0x14206fc4e..0x14206ff32 的内联宏展开）
// ===========================================================================

/// 去重键（O：`"module_path! ()" + "$crate :: Level :: Warn" + 消息`，见 `nani::warn_once_in`）。
pub use crate::render::nani::warn_once_key;

/// 首次出现返回 true 并 `log::warn!`（O：`BTreeMap::insert` 返回 None → 打日志，
/// 目标 `country_core::render::nani::systems`；否则静默）。本展开点自带去重表。
pub fn warn_once(message: String) -> bool {
    nani_warn_once!("country_core::render::nani::systems", message)
}

// ===========================================================================
// `nani_maintain_instances`（0x14206f300）
// ===========================================================================

/// `nani_maintain_instances` 的 `Local` 状态（O：`HashMap<Entity, NaniActiveInstance, RandomState>`，
/// 值为 12 字节记录 `{u32,u32,kind u8,[u8;3]}`——与 `NaniActiveInstance` 同形；
/// `hash_one<RandomState,&Entity>` / `RawTable::remove_entry` / `HashMap::insert` 三处调用）。
#[derive(Default)]
pub struct NaniActiveInstanceCollection {
    pub entity_to_handle: HashMap<Entity, NaniActiveInstance, std::hash::RandomState>,
}

/// 新实例的紧凑 key（O，0x14206fb1c..0x14206fba1）：
/// `position = transform.translation`；`sort_key = f16::from_f32(max(|sx|,|sy|,|sz|))`；`mesh_part = part as u16`。
pub fn compact_key_for(transform: &Transform, mesh_part: u32) -> NaniCompactKey {
    InstanceCullData {
        position: transform.translation,
        radius: half::f16::from_f32(max_abs_scale(transform.scale)),
        mesh_index: mesh_part as u16,
    }
}

/// 解析 `NaniInstanceDesc` 的 atlas part 序号（O，0x14206fad0..0x14206fb1a / 0x14207061d）：
/// - `AtlasPart(p)` 直接取 `p`；
/// - `Named(hash)`：`MeshAtlasLibrary::get_by_name::<NameHash>(hash)` 返回 `&[PrefabMeshAtlasPart]`，
///   线性查找 `part.subset == desc.subset_kind`（元素步长 8，kind 在 +4）；找不到 → `panic_fmt`
///   （源行 63:21）。
pub fn resolve_mesh_part(atlas: &MeshAtlasLibrary, desc: &NaniInstanceDesc) -> u32 {
    match desc.mesh {
        NaniInstanceMeshDesc::AtlasPart(p) => p,
        NaniInstanceMeshDesc::Named(hash) => {
            let name = NameHash(hash);
            let parts = atlas.get_by_name(name);
            match parts.iter().find(|p| p.subset as u8 == desc.subset_kind) {
                Some(part) => part.part_index,
                None => panic!(
                    "NaniInstanceDesc requested mesh {:?} and shader {:?},\nbut this mesh is not registered for that shader in nani_meshes.ron",
                    name,
                    kind_from_u8(desc.subset_kind),
                ),
            }
        }
    }
}

/// `Changed<NaniInstanceDesc>` 行的 atlas part（O，0x142070474..0x1420704c9）：
/// `AtlasPart(p)` 直接用；`Named`：`get_by_name` 必须恰好 1 个 part（`assert_eq!(len, 1)`，源行 106:17），
/// 且 `part.subset == desc.subset_kind`（`assert_eq!`，源行 108:17）。
pub fn resolve_changed_mesh_part(atlas: &MeshAtlasLibrary, desc: &NaniInstanceDesc) -> u32 {
    match desc.mesh {
        NaniInstanceMeshDesc::AtlasPart(p) => p,
        NaniInstanceMeshDesc::Named(hash) => {
            let parts = atlas.get_by_name(NameHash(hash));
            assert_eq!(parts.len(), 1);
            let part = parts[0];
            assert_eq!(part.subset as u8, desc.subset_kind);
            part.part_index
        }
    }
}

/// 维护系统（O，PDB 签名，7 个参数）。
///
/// 1. puffin 作用域。
/// 2. **删除**（先于新建，O：0x14206f690 循环在 0x14206f79e 之前）：`RemovedComponents<NaniActiveInstance>`
///    每个实体（generation 为 0 的无效实体被跳过，`shr rcx,0x20; je`）→
///    `local.entity_to_handle.remove_entry(&entity)`；缺失 → `Option::unwrap` panic（源行 51:41）→
///    `gpu_world.remove_instance(&record)`。
/// 3. **新建**（`Without<NaniActiveInstance>`）：见 `resolve_mesh_part` / `compact_key_for`；
///    `owned_subset_mut(desc.subset_kind).write()` 内 `extend_no_data(kind, [key], cb)`，回调把 12 字节记录存入
///    局部 `Option`；解锁后 `gpu_world.total_instance_count += 1`（**无条件**，O：0x14206fc31）；
///    记录存在 → `local.entity_to_handle.insert(entity, record)` + `Commands` 压入 `Insert<NaniActiveInstance>`
///    （0x28B 命令）；记录缺失（实例上限）→ `warn_once("Instance limit reached while trying to spawn {desc:?}")`。
/// 4. **变更**（`Changed<NaniInstanceDesc>`）：`resolve_changed_mesh_part` →
///    `gpu_world.update_instance_mesh(&active, part)`。
pub fn nani_maintain_instances(
    new: Query<(Entity, &NaniInstanceDesc, &Transform), Without<NaniActiveInstance>>,
    changed: Query<(&NaniInstanceDesc, &NaniActiveInstance), Changed<NaniInstanceDesc>>,
    mut removed: RemovedComponents<NaniActiveInstance>,
    mesh_atlas: Res<MeshAtlasLibrary>,
    mut gpu_world: ResMut<GpuWorld>,
    mut local: Local<NaniActiveInstanceCollection>,
    mut commands: Commands,
) {
    let _scope = crate::puffin_contract::scope("nani_maintain_instances");

    // 2. 删除。
    for entity in removed.read() {
        let (_, mut record) = local
            .entity_to_handle
            .remove_entry(&entity)
            .expect("called `Option::unwrap()` on a `None` value");
        gpu_world.remove_instance(&mut record);
    }

    // 3. 新建。
    for (entity, desc, transform) in new.iter() {
        let part = resolve_mesh_part(&mesh_atlas, desc);
        let key = compact_key_for(transform, part);
        let kind = kind_from_u8(desc.subset_kind);

        let mut created: Option<NaniActiveInstance> = None;
        {
            // `owned_subset_mut`：External → panic "Subset {kind:?} is not owned"，空槽 → unwrap panic（O）。
            let mut subset = gpu_world.owned_subset_mut(kind);
            // closure$1（0x141565950）：把回调收到的 12 字节记录存入 `created`。
            subset.extend_no_data(kind, core::slice::from_ref(&key), |record| {
                created = Some(record)
            });
        }
        // 解锁后无条件 `+= 1`（O：0x14206fc31）。
        gpu_world.total_instance_count += 1;

        match created {
            Some(record) => {
                local.entity_to_handle.insert(entity, record);
                commands.entity(entity).insert(record);
            }
            None => {
                warn_once(format!(
                    "Instance limit reached while trying to spawn {desc:?}"
                ));
            }
        }
    }

    // 4. 变更。
    for (desc, active) in changed.iter() {
        let part = resolve_changed_mesh_part(&mesh_atlas, desc);
        let mut key: InstanceKey = *active;
        gpu_world.update_instance_mesh(&mut key, part as u16);
    }
}

// ===========================================================================
// `nani_update_plant`（0x142074740，kind 10，数据 0x30）
// ===========================================================================

/// 植物（O）：12 字 = `Mat3::from_quat(rot)` 各列按 `scale.{x,y,z}` 缩放（列主序）+ 平移；
/// `position = translation`，`sort_key = max(|s|)`。
pub fn build_plant(translation: Vec3, rotation: Quat, scale: Vec3) -> InstanceUpdate<12> {
    let (words, sort_key) = trs_words12(translation, rotation, scale);
    InstanceUpdate {
        position: translation,
        sort_key,
        words,
    }
}

/// 所有 `nani_update_*` 共用的 12 字仿射打包（O，plant 0x142074740 起每个系统体内同一序列）。
pub fn trs_words12(translation: Vec3, rotation: Quat, scale: Vec3) -> ([u32; 12], f32) {
    let m = Mat3::from_quat(rotation);
    let c0 = m.x_axis * scale.x;
    let c1 = m.y_axis * scale.y;
    let c2 = m.z_axis * scale.z;
    let w = [
        c0.x,
        c0.y,
        c0.z,
        c1.x,
        c1.y,
        c1.z,
        c2.x,
        c2.y,
        c2.z,
        translation.x,
        translation.y,
        translation.z,
    ];
    let mut out = [0u32; 12];
    for (o, v) in out.iter_mut().zip(w) {
        *o = f32w(v);
    }
    (out, max_abs_scale(scale))
}

/// `nani_update_plant` 0x142074740（kind 10；查询 `(&NaniActiveInstance, &Transform)`）。
pub fn nani_update_plant(
    q: Query<(&NaniActiveInstance, &Transform), With<InstanceDataMarker<PlantInstanceData>>>,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_plant");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(10));
    {
        for (active, t) in q.iter() {
            let u = build_plant(t.translation, t.rotation, t.scale);
            push_update::<PlantInstanceData, 12>(&mut subset, active, u);
        }
    }
}

// ===========================================================================
// `nani_flush_instance_updates`（0x142080640）
// ===========================================================================

/// 冲刷系统（O）：puffin 作用域 → 把 ResMut 的 changed tick 置为 this_run → 对 27 个 subset 槽
/// （stride 0xc0，`0x1440/0xc0 == 27`）里 **Owned** 的（槽判别值 `>= 0x8000000000000002`）：
/// `ShadowedBuffer<GpuInstance>::gpu_flush(device)` + 数据缓冲 trait 对象 vtable `+0x30`（`gpu_flush`）；
/// 最后 `total_instance_count`（+0x1440）→ `+0x1448`，+0x1440 清零。
/// 参数类型：`ResMut<GpuWorld>`（I：`in_RDX[2]` 为其 changed-tick 槽）+ `Res<GraphicsDeviceResource>`
/// （I：`*(*in_RCX)+0x10` 即 `Arc` 内值）。参数顺序 U。
pub fn nani_flush_instance_updates(
    mut gpu_world: ResMut<GpuWorld>,
    device: Res<crate::render::GraphicsDeviceResource>,
) {
    let _scope = crate::puffin_contract::scope("nani_flush_instance_updates");
    let dev: &crate::rhapsody_contract::GraphicsDevice = &**device;
    gpu_world.flush_instance_buffers(dev);
    gpu_world.roll_instance_counts();
}

// ===========================================================================
// FRAGMENT gA — nani_update_{solid_vertex_color, solid_vertex_color_window,
// solid_albedo_map, solid_albedo_map_emissive, solid_vertex_color_window_glass,
// solid_vertex_color_bricks}.   (merge into systems.rs; uses common.rs helpers)
//
// Extra imports needed by this fragment:
//   use crate::components::{SemiTransparent, FlatLighting, BrickColor};
//   use crate::components::drawable::{NaniActiveInstance, NaniSolidTexture, AlbedoMap};
//   use crate::resources::color_id::ColorId;
//   use crate::resources::window_lights::{WindowLight, GlobalWindowLightsControl};
//   use crate::asset_libraries::texture_library::AssetTextureLibrary;
//   use crate::name_hash::NameHash;
//   half = "2" (original lock: half 2.4.1) for `half::f16` (O: utils::pack_2x16f_uint).
// ===========================================================================

/// `utils::pack_2x16f_uint` (O 0x140c90df0): `f16(a) | f16(b) << 16`.
pub fn pack_2x16f_uint(a: f32, b: f32) -> u32 {
    let lo = half::f16::from_f32(a).to_bits() as u32;
    let hi = half::f16::from_f32(b).to_bits() as u32;
    (hi << 16) | lo
}

/// 12-word packed affine shared by every `nani_update_*` (O; same code in
/// plant 0x142074740 and all of group A):
/// `Mat3::from_quat(rot)` columns scaled per-column by `scale.{x,y,z}` (words 0..9, column
/// major), then `translation` (words 9..12).  Second result = `sort_key` passed in xmm3 =
/// `max(|sx|, |sy|, |sz|)` (maxss chain with NaN-skipping select, equals `f32::max`).
pub fn trs_words(translation: Vec3, rotation: Quat, scale: Vec3) -> ([u32; 12], f32) {
    let m = Mat3::from_quat(rotation);
    let c0 = m.x_axis * scale.x;
    let c1 = m.y_axis * scale.y;
    let c2 = m.z_axis * scale.z;
    let w = [
        c0.x,
        c0.y,
        c0.z,
        c1.x,
        c1.y,
        c1.z,
        c2.x,
        c2.y,
        c2.z,
        translation.x,
        translation.y,
        translation.z,
    ];
    let mut out = [0u32; 12];
    for (o, v) in out.iter_mut().zip(w) {
        *o = f32w(v);
    }
    let sort_key = scale.z.abs().max(scale.y.abs()).max(scale.x.abs());
    (out, sort_key)
}

fn with_tail<const N: usize>(head: [u32; 12], tail: &[u32]) -> [u32; N] {
    let mut w = [0u32; N];
    w[..12].copy_from_slice(&head);
    w[12..12 + tail.len()].copy_from_slice(tail);
    w
}

// ---------------------------------------------------------------------------
// nani_update_solid_vertex_color  0x142070ac0  (kind 3, data 0x40)
// ---------------------------------------------------------------------------
/// O: words 12 = `Option<&SemiTransparent>.is_some()` (ptr!=0 & presence flag),
/// 13 = `Option<&FlatLighting>.is_some()`, 14/15 = 0.
pub fn build_solid_vertex_color(
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
    semi_transparent: bool,
    flat_lighting: bool,
) -> InstanceUpdate<16> {
    let (head, sort_key) = trs_words(translation, rotation, scale);
    InstanceUpdate {
        position: translation,
        sort_key,
        words: with_tail(head, &[semi_transparent as u32, flat_lighting as u32, 0, 0]),
    }
}

/// `nani_update_solid_vertex_color` (O, PDB signature; `owned_subset_mut(kind 3)`).
pub fn nani_update_solid_vertex_color(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            Option<&SemiTransparent>,
            Option<&FlatLighting>,
        ),
        With<InstanceDataMarker<SolidVertexColorInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_solid_vertex_color");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(3));
    {
        for (active, t, semi, flat) in q.iter() {
            let u = build_solid_vertex_color(
                t.translation,
                t.rotation,
                t.scale,
                semi.is_some(),
                flat.is_some(),
            );
            push_update::<SolidVertexColorInstanceData, 16>(&mut subset, active, u);
        }
    }
}

// ---------------------------------------------------------------------------
// nani_update_solid_vertex_color_window  0x142071590  (kind 4, data 0x40)
// ---------------------------------------------------------------------------
/// O: word 12 = `Option<&SemiTransparent>.is_some()`, word 13 = `ColorId.0` (raw, no range
/// check in this system), 14/15 = 0.
pub fn build_solid_vertex_color_window(
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
    color_id: u32,
    semi_transparent: bool,
) -> InstanceUpdate<16> {
    let (head, sort_key) = trs_words(translation, rotation, scale);
    InstanceUpdate {
        position: translation,
        sort_key,
        words: with_tail(head, &[semi_transparent as u32, color_id, 0, 0]),
    }
}

/// `nani_update_solid_vertex_color_window` (O; kind 4).
pub fn nani_update_solid_vertex_color_window(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            &ColorId,
            Option<&SemiTransparent>,
        ),
        With<InstanceDataMarker<SolidVertexColorWindowInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_solid_vertex_color_window");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(4));
    {
        for (active, t, color, semi) in q.iter() {
            let u = build_solid_vertex_color_window(
                t.translation,
                t.rotation,
                t.scale,
                color.0,
                semi.is_some(),
            );
            push_update::<SolidVertexColorWindowInstanceData, 16>(&mut subset, active, u);
        }
    }
}

// ---------------------------------------------------------------------------
// nani_update_solid_albedo_map 0x142072010 (kind 0xf) / _emissive 0x142072b50 (kind 0x10)
// ---------------------------------------------------------------------------
/// O: `NaniSolidTexture` is a 1-byte enum (`movzx eax, byte [row]`); it indexes a u32 table
/// at 0x142fd38c8.  The table has 9 entries (I: = number of enum variants; the bytes after
/// entry 8 belong to unrelated data).  Out-of-range index is UB in the binary; here it panics.
pub const NANI_SOLID_TEXTURE_TABLE: [u32; 9] = [0, 1, 8, 2, 3, 4, 5, 6, 7];

/// O: words 12 = `TABLE[texture as u8]`, 13..15 = 0 (identical for emissive variant).
pub fn build_solid_albedo_map(
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
    texture: u8,
) -> InstanceUpdate<16> {
    let (head, sort_key) = trs_words(translation, rotation, scale);
    InstanceUpdate {
        position: translation,
        sort_key,
        words: with_tail(head, &[NANI_SOLID_TEXTURE_TABLE[texture as usize], 0, 0, 0]),
    }
}

/// `nani_update_solid_albedo_map` (O; kind 0xf).
pub fn nani_update_solid_albedo_map(
    q: Query<
        (&NaniActiveInstance, &Transform, &NaniSolidTexture),
        With<InstanceDataMarker<SolidAlbedoMapInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_solid_albedo_map");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(0xf));
    {
        for (active, t, tex) in q.iter() {
            // U: `NaniSolidTexture` is `Gap` in components/drawable.rs; asm reads 1 byte at +0.
            let tex = gap_words::<_, 1>(tex)[0] as u8;
            let u = build_solid_albedo_map(t.translation, t.rotation, t.scale, tex);
            push_update::<SolidAlbedoMapInstanceData, 16>(&mut subset, active, u);
        }
    }
}

/// `nani_update_solid_albedo_map_emissive` (O; kind 0x10; body identical to albedo_map).
pub fn nani_update_solid_albedo_map_emissive(
    q: Query<
        (&NaniActiveInstance, &Transform, &NaniSolidTexture),
        With<InstanceDataMarker<SolidAlbedoMapEmissiveInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_solid_albedo_map_emissive");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(0x10));
    {
        for (active, t, tex) in q.iter() {
            let tex = gap_words::<_, 1>(tex)[0] as u8; // U: see albedo_map
            let u = build_solid_albedo_map(t.translation, t.rotation, t.scale, tex);
            push_update::<SolidAlbedoMapEmissiveInstanceData, 16>(&mut subset, active, u);
        }
    }
}

// ---------------------------------------------------------------------------
// nani_update_solid_vertex_color_window_glass 0x142075070 (kind 5, data 0x40)
// ---------------------------------------------------------------------------
/// Panic text of `WindowGlassFlags::with_color_id_checked(..).unwrap()` (O: string at
/// 0x142fd3700, len 0x37; the `: OutOfBounds` suffix is the modular_bitfield error Debug, I).
pub const WINDOW_GLASS_COLOR_ID_OOB: &str =
    "value out of bounds for field WindowGlassFlags.color_id";

/// O: word 12 = `color_id | (semi_transparent << 31)` (color_id must be < 2^31 else
/// `Result::unwrap_failed(WINDOW_GLASS_COLOR_ID_OOB)`), word 13 = `window_light_value` f32
/// bits, word 14 = `albedo_map.is_some()`, word 15 = bindless index of the albedo texture (0 if none).
pub fn build_window_glass(
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
    color_id: u32,
    semi_transparent: bool,
    window_light_value: f32,
    albedo_bindless: Option<u32>,
) -> InstanceUpdate<16> {
    if color_id & 0x8000_0000 != 0 {
        panic!("{WINDOW_GLASS_COLOR_ID_OOB}: OutOfBounds");
    }
    let (head, sort_key) = trs_words(translation, rotation, scale);
    InstanceUpdate {
        position: translation,
        sort_key,
        words: with_tail(
            head,
            &[
                color_id | ((semi_transparent as u32) << 31),
                f32w(window_light_value),
                albedo_bindless.is_some() as u32,
                albedo_bindless.unwrap_or(0),
            ],
        ),
    }
}

/// U-typed gap: `GlobalWindowLightsControl` (resource is `Gap` in resources/window_lights.rs).
/// O semantics: an internal hash map (stride-0x18 entries: key u64 @+0, f32 @+8; the map's
/// item count is at +0x18 of the resource) keyed by the first 8 bytes of `WindowLight`;
/// empty map or missing key -> 0.0.  Resource owner must supply the real lookup.
pub fn window_light_value(_ctl: &GlobalWindowLightsControl, _key: u64) -> f32 {
    0.0 // U: map contents not recovered; 0.0 is the O default for "absent"
}

/// `nani_update_solid_vertex_color_window_glass` (O; kind 5).
pub fn nani_update_solid_vertex_color_window_glass(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            &WindowLight,
            &ColorId,
            Option<&SemiTransparent>,
            Option<&AlbedoMap>,
        ),
        With<InstanceDataMarker<SolidVertexColorWindowGlassInstanceData>>,
    >,
    window_lights: Res<GlobalWindowLightsControl>,
    textures: Res<AssetTextureLibrary>,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_solid_vertex_color_window_glass");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(5));
    {
        for (active, t, light, color, semi, albedo) in q.iter() {
            // U: WindowLight layout (16 B): first u64 = map key (O: `[row+0]`).
            let key = {
                let w = gap_words::<_, 4>(light);
                (w[0] as u64) | ((w[1] as u64) << 32)
            };
            let light_value = window_light_value(&window_lights, key);
            // O: AlbedoMap row = 8 B NameHash; texture lookup by name hash, None -> Option::unwrap
            // panic; then `handle_to_bindless_index(handle)`.
            let albedo_bindless = albedo.map(|a| {
                let w = gap_words::<_, 2>(a); // U: AlbedoMap is `Gap`; asm: u64 NameHash at +0
                let hash = NameHash((w[0] as u64) | ((w[1] as u64) << 32));
                let (handle, _) = textures
                    .get_handle_and_texture_by_name(hash)
                    .expect("called `Option::unwrap()` on a `None` value");
                textures.handle_to_bindless_index(handle).0
            });
            let u = build_window_glass(
                t.translation,
                t.rotation,
                t.scale,
                color.0,
                semi.is_some(),
                light_value,
                albedo_bindless,
            );
            push_update::<SolidVertexColorWindowGlassInstanceData, 16>(&mut subset, active, u);
        }
    }
}

// ---------------------------------------------------------------------------
// nani_update_solid_vertex_color_bricks 0x142076230 (kind 6, data 0x40)
// ---------------------------------------------------------------------------
/// `BrickColor` raw words (O: row stride 0x18): w0 u32, w1 u32, w2 f32, w3 f32, w4 f32,
/// w5 = dword whose bit 0 is a bool (`shl 31`).  Field names U.
/// Output (O): w12 = `pack_2x16f_uint(f32(w2), f32(w3))`, w13 = w0, w14 = `(w5 << 31) | w1`,
/// w15 = w4 (raw bits).
pub fn build_solid_vertex_color_bricks(
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
    brick: [u32; 6],
) -> InstanceUpdate<16> {
    let (head, sort_key) = trs_words(translation, rotation, scale);
    InstanceUpdate {
        position: translation,
        sort_key,
        words: with_tail(
            head,
            &[
                pack_2x16f_uint(f32::from_bits(brick[2]), f32::from_bits(brick[3])),
                brick[0],
                (brick[5] << 31) | brick[1],
                brick[4],
            ],
        ),
    }
}

/// `nani_update_solid_vertex_color_bricks` (O; kind 6).
pub fn nani_update_solid_vertex_color_bricks(
    q: Query<
        (&NaniActiveInstance, &Transform, &BrickColor),
        With<InstanceDataMarker<SolidVertexColorBricksInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_solid_vertex_color_bricks");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(6));
    {
        for (active, t, brick) in q.iter() {
            let u = build_solid_vertex_color_bricks(
                t.translation,
                t.rotation,
                t.scale,
                gap_words::<_, 6>(brick), // U: BrickColor is `Gap`; asm layout documented above
            );
            push_update::<SolidVertexColorBricksInstanceData, 16>(&mut subset, active, u);
        }
    }
}

#[cfg(test)]
mod group_a_tests {
    use super::*;

    const ID: (Vec3, Quat, Vec3) = (Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
    const IDENT_WORDS: [f32; 12] = [1., 0., 0., 0., 1., 0., 0., 0., 1., 0., 0., 0.];

    fn f(u: &[u32]) -> Vec<f32> {
        u.iter().map(|w| f32::from_bits(*w)).collect()
    }

    #[test]
    fn trs_identity_and_scale_translation() {
        let (w, k) = trs_words(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
        assert_eq!(f(&w), IDENT_WORDS);
        assert_eq!(k, 1.0);
        let (w, k) = trs_words(
            Vec3::new(1., 2., 3.),
            Quat::IDENTITY,
            Vec3::new(2., -5., 3.),
        );
        assert_eq!(f(&w), [2., 0., 0., 0., -5., 0., 0., 0., 3., 1., 2., 3.]);
        assert_eq!(k, 5.0); // max(|sx|,|sy|,|sz|)
    }

    #[test]
    fn trs_rotation_90deg_z_scales_columns() {
        let q = Quat::from_rotation_z(core::f32::consts::FRAC_PI_2);
        let (w, _) = trs_words(Vec3::ZERO, q, Vec3::new(2., 3., 4.));
        let v = f(&w);
        assert!((v[0]).abs() < 1e-6 && (v[1] - 2.0).abs() < 1e-6); // col0 = (0,1,0)*2
        assert!((v[3] + 3.0).abs() < 1e-6 && v[4].abs() < 1e-6); // col1 = (-1,0,0)*3
        assert!((v[8] - 4.0).abs() < 1e-6);
    }

    #[test]
    fn solid_vertex_color_flags() {
        let u = build_solid_vertex_color(ID.0, ID.1, ID.2, true, false);
        assert_eq!(&u.words[12..], &[1, 0, 0, 0]);
        let u = build_solid_vertex_color(ID.0, ID.1, ID.2, false, true);
        assert_eq!(&u.words[12..], &[0, 1, 0, 0]);
        assert_eq!(u.position, Vec3::ZERO);
    }

    #[test]
    fn window_color_id_word13() {
        let u = build_solid_vertex_color_window(Vec3::X, ID.1, ID.2, 0xdead_beef, true);
        assert_eq!(&u.words[12..], &[1, 0xdead_beef, 0, 0]);
        assert_eq!(u.position, Vec3::X);
    }

    #[test]
    fn albedo_table_matches_binary() {
        assert_eq!(NANI_SOLID_TEXTURE_TABLE, [0, 1, 8, 2, 3, 4, 5, 6, 7]);
        let u = build_solid_albedo_map(ID.0, ID.1, ID.2, 2);
        assert_eq!(&u.words[12..], &[8, 0, 0, 0]);
        let u = build_solid_albedo_map(ID.0, ID.1, ID.2, 8);
        assert_eq!(u.words[12], 7);
    }

    #[test]
    #[should_panic]
    fn albedo_texture_out_of_range_panics() {
        let _ = build_solid_albedo_map(ID.0, ID.1, ID.2, 9);
    }

    #[test]
    fn glass_packs_color_semi_bit_and_albedo() {
        let u = build_window_glass(ID.0, ID.1, ID.2, 0x1234, true, 0.5, Some(77));
        assert_eq!(u.words[12], 0x1234 | 0x8000_0000);
        assert_eq!(f32::from_bits(u.words[13]), 0.5);
        assert_eq!((u.words[14], u.words[15]), (1, 77));
        let u = build_window_glass(ID.0, ID.1, ID.2, 7, false, 0.0, None);
        assert_eq!(&u.words[12..], &[7, 0, 0, 0]);
    }

    #[test]
    #[should_panic(expected = "value out of bounds for field WindowGlassFlags.color_id")]
    fn glass_color_id_bit31_panics() {
        let _ = build_window_glass(ID.0, ID.1, ID.2, 0x8000_0000, false, 0.0, None);
    }

    #[test]
    fn bricks_packing() {
        let brick = [
            0x11u32,
            0x22,
            1.0f32.to_bits(),
            2.0f32.to_bits(),
            0.25f32.to_bits(),
            1,
        ];
        let u = build_solid_vertex_color_bricks(ID.0, ID.1, ID.2, brick);
        assert_eq!(u.words[12], 0x4000_3c00); // f16(1.0)=0x3c00 low, f16(2.0)=0x4000 high
        assert_eq!(u.words[13], 0x11);
        assert_eq!(u.words[14], 0x8000_0022);
        assert_eq!(u.words[15], 0.25f32.to_bits());
        let brick0 = [0, 5, 0, 0, 0, 2]; // only bit0 of the dword at +0x14 matters? (shl 31 keeps bit0)
        assert_eq!(
            build_solid_vertex_color_bricks(ID.0, ID.1, ID.2, brick0).words[14],
            5
        );
    }

    #[test]
    fn pack_2x16f_order() {
        assert_eq!(pack_2x16f_uint(1.0, 0.0), 0x0000_3c00);
        assert_eq!(pack_2x16f_uint(0.0, 1.0), 0x3c00_0000);
    }
}

// ===========================================================================
// FRAGMENT gB — nani_update_{preplaced_flower, gothic_window_bricks, curved_halftimber,
// stone_floor, wren}.  To be merged into systems.rs (uses the shared helpers of common.rs:
// InstanceUpdate, f32w, pack_instance, push_update, gap_words, kind_from_u8).
//
// Common O facts (see /tmp/n_plant.txt, update_instance_data 0x14156fff0):
//   update_instance_data<T>(subset, key:&mut {chunk u32, index u32}, position:&Vec3,
//                           sort_key:f32, data:&T)
//   Rust-ABI: rcx=subset rdx=key r8=&position xmm3=sort_key stack=&data.
//   Binary Transform row (stride 0x30): rotation Quat @+0x00, translation Vec3 @+0x10,
//   scale Vec3 @+0x1c.  "affine words" (48 B) = Mat3(rot)*diag(scale) columns (9 f32)
//   followed by translation (3 f32).
// ===========================================================================

// ---- shared small helpers of this group (candidates for promotion into the common block) ----

/// 48-byte packed affine used by Plant/Flower/Wren/Gothic payloads (O):
/// `[c0.x c0.y c0.z c1.x c1.y c1.z c2.x c2.y c2.z t.x t.y t.z]`, `c_i = R.col_i * scale_i`.
/// Quat->Mat3 follows the glam `from_quat` formula (verified against the SSE sequence).
pub fn gb_affine_words(rotation: Quat, translation: Vec3, scale: Vec3) -> [u32; 12] {
    let m = Mat3::from_quat(rotation);
    let c0 = m.x_axis * scale.x;
    let c1 = m.y_axis * scale.y;
    let c2 = m.z_axis * scale.z;
    [
        f32w(c0.x),
        f32w(c0.y),
        f32w(c0.z),
        f32w(c1.x),
        f32w(c1.y),
        f32w(c1.z),
        f32w(c2.x),
        f32w(c2.y),
        f32w(c2.z),
        f32w(translation.x),
        f32w(translation.y),
        f32w(translation.z),
    ]
}

/// `max(|sx|, max(|sy|, |sz|))` with maxss+cmpunord NaN blend == `f32::max` semantics (O).
/// This is the `sort_key` (xmm3) passed to update_instance_data by Flower/Wren/Gothic.
pub fn gb_max_abs_scale(scale: Vec3) -> f32 {
    scale.x.abs().max(scale.y.abs().max(scale.z.abs()))
}

/// `utils::pack_2x16f_uint` (O, 0x140c90df0): `f16(a) | f16(b) << 16` (half::f16::from_f32).
pub fn gb_pack_2x16f(a: f32, b: f32) -> u32 {
    let lo = half::f16::from_f32(a).to_bits() as u32;
    let hi = half::f16::from_f32(b).to_bits() as u32;
    (hi << 16) | lo
}

/// Reinterpret 10 raw words of a binary `Transform`-shaped row (stride 0x30: quat, translation
/// @+0x10, scale @+0x1c) as `(rotation, translation, scale)`.  Used for `PrevTransform` whose
/// Rust layout is a typed gap in components/drawable.rs (O: read with the same offsets as Transform).
pub fn gb_transform_from_binary_words(w: [u32; 10]) -> (Quat, Vec3, Vec3) {
    let f = |i: usize| f32::from_bits(w[i]);
    (
        Quat::from_xyzw(f(0), f(1), f(2), f(3)),
        Vec3::new(f(4), f(5), f(6)),
        Vec3::new(f(7), f(8), f(9)),
    )
}

// ===========================================================================
// 1. nani_update_wren  0x1420797b0  (kind 0x11, update_instance_data<Wren> 0x141571080,
//    WrenInstanceData = 0x70 B = 28 words)
// ===========================================================================

/// Wren payload (O, 112 B):
/// `[0..12)` affine(current Transform), `[12..24)` affine(PrevTransform), `[24..26)` Wren+0x44 (8 B),
/// `[26]` Wren+0x4c, `[27]` = 0.  position = current translation, sort_key = max|scale| of current.
/// `wren_44` = words at Wren+0x44/+0x48, `wren_4c` = word at Wren+0x4c (field names U).
pub fn build_wren(
    cur: (Quat, Vec3, Vec3),
    prev: (Quat, Vec3, Vec3),
    wren_44: [u32; 2],
    wren_4c: u32,
) -> InstanceUpdate<28> {
    let mut words = [0u32; 28];
    words[0..12].copy_from_slice(&gb_affine_words(cur.0, cur.1, cur.2));
    words[12..24].copy_from_slice(&gb_affine_words(prev.0, prev.1, prev.2));
    words[24] = wren_44[0];
    words[25] = wren_44[1];
    words[26] = wren_4c;
    words[27] = 0;
    InstanceUpdate {
        position: cur.1,
        sort_key: gb_max_abs_scale(cur.2),
        words,
    }
}

/// `nani_update_wren` 0x1420797b0 (O).  Query = (NaniActiveInstance, Transform, PrevTransform,
/// resources::critters::Wren), With<InstanceDataMarker<WrenInstanceData>>.
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
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(0x11));
    {
        for (active, transform, prev, wren) in q.iter() {
            // PrevTransform: binary layout == Transform (U in Rust: gap component).
            let prev_t = gb_transform_from_binary_words(gap_words::<_, 10>(prev));
            // Wren: 0x50-byte row; only +0x44 (8 B) and +0x4c (4 B) are read (O).
            let w = gap_words::<_, 20>(wren);
            let u = build_wren(
                (transform.rotation, transform.translation, transform.scale),
                prev_t,
                [w[17], w[18]],
                w[19],
            );
            push_update::<WrenInstanceData, 28>(&mut subset, active, u);
        }
    }
}

// ===========================================================================
// 2. nani_update_gothic_window_bricks  0x142076e40 (kind 7, update fn 0x14156ebd0,
//    GothicWindowBricksInstanceData = 0x70 B = 28 words)
// ===========================================================================

/// Gothic payload (O, 112 B):
/// `[0..12)` affine(Transform);
/// `[12..14)` BrickColor+0x00 (8 B) , `[14]` pack_2x16f(BrickColor+0x08, +0x0c), `[15]` (BrickColor+0x14) as u8->u32;
/// `[16..19)` Deformation+0x18 (12 B), `[19]` pack_2x16f(Def+0x00,+0x04);
/// `[20..23)` Deformation+0x24 (12 B), `[23]` pack_2x16f(Def+0x08,+0x0c);
/// `[24..27)` Deformation+0x30 (12 B), `[27]` pack_2x16f(Def+0x10,+0x14).
/// `brick` = BrickColor row (0x18 B = 6 words), `def` = GothicWindowBrickDeformation row (0x3c B = 15 words).
pub fn build_gothic_window_bricks(
    rotation: Quat,
    translation: Vec3,
    scale: Vec3,
    brick: [u32; 6],
    def: [u32; 15],
) -> InstanceUpdate<28> {
    let f = |i: usize| f32::from_bits(def[i]);
    let mut w = [0u32; 28];
    w[0..12].copy_from_slice(&gb_affine_words(rotation, translation, scale));
    w[12] = brick[0];
    w[13] = brick[1];
    w[14] = gb_pack_2x16f(f32::from_bits(brick[2]), f32::from_bits(brick[3]));
    w[15] = brick[5] & 0xff; // movzx byte [BrickColor+0x14]
    w[16] = def[6];
    w[17] = def[7];
    w[18] = def[8];
    w[19] = gb_pack_2x16f(f(0), f(1));
    w[20] = def[9];
    w[21] = def[10];
    w[22] = def[11];
    w[23] = gb_pack_2x16f(f(2), f(3));
    w[24] = def[12];
    w[25] = def[13];
    w[26] = def[14];
    w[27] = gb_pack_2x16f(f(4), f(5));
    InstanceUpdate {
        position: translation,
        sort_key: gb_max_abs_scale(scale),
        words: w,
    }
}

/// `nani_update_gothic_window_bricks` 0x142076e40 (O).
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
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(7));
    {
        for (active, t, brick, def) in q.iter() {
            // BrickColor 0x18 B, GothicWindowBrickDeformation 0x3c B: Rust layouts are typed gaps (U).
            let u = build_gothic_window_bricks(
                t.rotation,
                t.translation,
                t.scale,
                gap_words::<_, 6>(brick),
                gap_words::<_, 15>(def),
            );
            push_update::<GothicWindowBricksInstanceData, 28>(&mut subset, active, u);
        }
    }
}

// ===========================================================================
// 3. nani_update_preplaced_flower  0x142073690 (kind 0xe, update fn 0x141571de0,
//    PreplacedFlowerInstanceData = 0x30 B = 12 words)   [NOT 0x40 as in instance_data.rs]
// ===========================================================================

/// Name asserted (`assert_eq!`, memcmp 0x3f bytes) on the BufferState fetched from AssetSsboLibrary (O).
pub const TERRAIN_ROCK_MASK_SSBO_NAME: &str =
    "country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo";
/// World extent used for the mask UV and passed as 3rd float (`0x43020000`) to
/// `bilinear_sample_at_world_xz` (O).
pub const FLOWER_WORLD_SIZE: f32 = 130.0;
/// Raster side (cells) of the rock mask (O: 512.0, clamp 0x1ff).
pub const FLOWER_MASK_SIDE: f32 = 512.0;
/// Hide when water sample is strictly greater than this (O: 0x3c23d70a).
pub const FLOWER_WATER_MAX: f32 = 0.01;
/// Hide when clamp01(sum of 4 path rasters) strictly greater than this (O: 0x3dcccccd).
pub const FLOWER_PATH_MAX: f32 = 0.1;
/// Hidden flowers are placed at this translation inside the payload (O: f64 bits
/// 0xc2c8000080000000 = (-0.0, -100.0) + 0x80000000 = -0.0), identity rotation, unit scale.
pub const FLOWER_HIDDEN_TRANSLATION: Vec3 = Vec3::new(-0.0, -100.0, -0.0);

/// The five rasters sampled in order (O): water first, then the four path rasters
/// (PathRastersResources fields at +0x00/+0x20/+0x40/+0x60).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowerRaster {
    Water,
    Path0,
    Path1,
    Path2,
    Path3,
}

/// `(word_index, bit)` of the rock-mask cell for world (x, z) (O):
/// `u = clamp01(x/130 + 0.5) * 512` (saturating cvttss2si, min 0x1ff), same for v with z;
/// `word = ((v << 9) | u) >> 5`, `bit = u & 31` (`bt ecx, eax` uses eax = u, mod 32).
pub fn flower_mask_cell(x: f32, z: f32) -> (usize, u32) {
    let cell = |c: f32| -> u32 {
        let t = (c / FLOWER_WORLD_SIZE + 0.5).max(0.0).min(1.0) * FLOWER_MASK_SIDE;
        (t as u32).min(0x1ff) // `as` saturates like cmov/cvttss2si sequence; NaN -> 0
    };
    let (u, v) = (cell(x), cell(z));
    ((((v << 9) | u) >> 5) as usize, u & 31)
}

/// Visibility verdict (O). Order and short-circuiting follow the machine code:
/// 1. rock-mask bit set -> hide (indexing the mask is bounds-checked: panic_bounds_check);
/// 2. water sample `> 0.01` -> hide;
/// 3. `s = clamp(0,1)(((p0 + p1) + p2) + p3)`, `s > 0.1` -> hide; otherwise show.
pub fn flower_visible(
    x: f32,
    z: f32,
    rock_mask: &[u32],
    mut sample: impl FnMut(FlowerRaster, f32, f32) -> f32,
) -> bool {
    let (word, bit) = flower_mask_cell(x, z);
    if (rock_mask[word] >> bit) & 1 != 0 {
        return false;
    }
    if sample(FlowerRaster::Water, x, z) > FLOWER_WATER_MAX {
        return false;
    }
    let p0 = sample(FlowerRaster::Path0, x, z);
    let p1 = sample(FlowerRaster::Path1, x, z);
    let p2 = sample(FlowerRaster::Path2, x, z);
    let p3 = sample(FlowerRaster::Path3, x, z);
    let s = (((p0 + p1) + p2) + p3).max(0.0).min(1.0);
    !(s > FLOWER_PATH_MAX)
}

/// Payload builder (O). Visible: affine(Transform), sort_key = max|scale|.
/// Hidden: affine(identity rot, scale 1, translation (-0,-100,-0)), sort_key = 0.0.
/// `position` passed to update_instance_data is always the real translation.
pub fn build_preplaced_flower(
    rotation: Quat,
    translation: Vec3,
    scale: Vec3,
    visible: bool,
) -> InstanceUpdate<12> {
    if visible {
        InstanceUpdate {
            position: translation,
            sort_key: gb_max_abs_scale(scale),
            words: gb_affine_words(rotation, translation, scale),
        }
    } else {
        InstanceUpdate {
            position: translation,
            sort_key: 0.0,
            words: gb_affine_words(Quat::IDENTITY, FLOWER_HIDDEN_TRANSLATION, Vec3::ONE),
        }
    }
}

/// Asserts the BufferState name (O: `assert_eq!(name, "...TerrainRockMaskSsbo")`).
pub fn check_rock_mask_name(name: &str) {
    assert_eq!(name, TERRAIN_ROCK_MASK_SSBO_NAME);
}

/// `AssetSsboLibrary::get(SsboId(TerrainRockMaskSsbo))` -> BufferState -> name assert -> vtable+0x18
/// -> CPU shadow `&[u32]` (O). The accessor lives on the library
/// (`AssetSsboLibrary::get_cpu_shadow_words`); `None` when the SSBO is not registered
/// (original: `Option::unwrap_failed`).
pub fn terrain_rock_mask_words(
    lib: &crate::asset_libraries::ssbo_library::AssetSsboLibrary,
) -> Option<&[u32]> {
    lib.get_cpu_shadow_words::<crate::resources::terrain_rock_mask::TerrainRockMaskSsbo>()
}

/// 机器码里 `PathRastersResources` 是四个 `Res<…Raster>` 的裸 SystemParam（B_n_sys §7.10：crate 的
/// `systems::paths::draw_path::PathRastersResources` 是普通结构，不能作系统参数），这里按真实的四个
/// 光栅资源（`resources::world_raster_defs::{PathRaster, Cobblestone1/2/3Raster}`）声明同形 SystemParam。
#[derive(bevy_ecs::system::SystemParam)]
pub struct NaniPathRasters<'w> {
    pub path: Res<'w, crate::resources::world_raster_defs::PathRaster>,
    pub cobblestone1: Res<'w, crate::resources::world_raster_defs::Cobblestone1Raster>,
    pub cobblestone2: Res<'w, crate::resources::world_raster_defs::Cobblestone2Raster>,
    pub cobblestone3: Res<'w, crate::resources::world_raster_defs::Cobblestone3Raster>,
}

/// `nani_update_preplaced_flower` 0x142073690 (O).
/// Samplers: the asm calls `bilinear_sample_at_world_xz<RasterBufferRef<512>>(grid, x, z, 130.0)`;
/// the recovered `bilinear_sample_at_world_xz` takes the (x extent, z extent) pair = (130.0, 130.0).
pub fn nani_update_preplaced_flower(
    q: Query<
        (&NaniActiveInstance, &Transform),
        With<InstanceDataMarker<PreplacedFlowerInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
    ssbo: Res<crate::asset_libraries::ssbo_library::AssetSsboLibrary>,
    path: NaniPathRasters,
    water: Res<crate::resources::world_raster_defs::WaterRaster>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_preplaced_flower");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(0xe));
    {
        // fetched once before the loop (O)
        let mask =
            terrain_rock_mask_words(&ssbo).expect("called `Option::unwrap()` on a `None` value");
        for (active, t) in q.iter() {
            let visible =
                flower_visible(t.translation.x, t.translation.z, mask, |r, x, z| match r {
                    FlowerRaster::Water => crate::utils::float_grid::bilinear_sample_at_world_xz(
                        &water.0,
                        x,
                        z,
                        FLOWER_WORLD_SIZE,
                        FLOWER_WORLD_SIZE,
                    ),
                    FlowerRaster::Path0 => crate::utils::float_grid::bilinear_sample_at_world_xz(
                        &path.path.0,
                        x,
                        z,
                        FLOWER_WORLD_SIZE,
                        FLOWER_WORLD_SIZE,
                    ),
                    FlowerRaster::Path1 => crate::utils::float_grid::bilinear_sample_at_world_xz(
                        &path.cobblestone1.0,
                        x,
                        z,
                        FLOWER_WORLD_SIZE,
                        FLOWER_WORLD_SIZE,
                    ),
                    FlowerRaster::Path2 => crate::utils::float_grid::bilinear_sample_at_world_xz(
                        &path.cobblestone2.0,
                        x,
                        z,
                        FLOWER_WORLD_SIZE,
                        FLOWER_WORLD_SIZE,
                    ),
                    FlowerRaster::Path3 => crate::utils::float_grid::bilinear_sample_at_world_xz(
                        &path.cobblestone3.0,
                        x,
                        z,
                        FLOWER_WORLD_SIZE,
                        FLOWER_WORLD_SIZE,
                    ),
                });
            let u = build_preplaced_flower(t.rotation, t.translation, t.scale, visible);
            push_update::<PreplacedFlowerInstanceData, 12>(&mut subset, active, u);
        }
    }
}

// ===========================================================================
// 4. nani_update_curved_halftimber  0x142077d20 (kind 0x18, update fn 0x1415713f0,
//    CurvedHalfTimberInstanceData = 0x40 B = 16 words, mutated in place)
// ===========================================================================

/// `CurvedHalfTimber` row (16 B, O): `key: u64 @+0`, `direct: u8 @+8`.
/// Colour selection (O):
///  - `direct != 0`           -> `ids[key]` (bounds checked);
///  - `direct == 0`           -> if PublicWalls (non-empty) maps `key` to a live roof entity whose
///    `Roof` has discriminant byte `@+0x40 == 2`, colour = `roof.u32 @+0x44`; otherwise `ids[key]`.
/// `roof_curved_colour` is that lookup already resolved by the caller.
/// Panics like the original (`panic_bounds_check`) when `ids` has no entry for `key`.
pub fn curved_halftimber_colour(
    key: u64,
    direct: bool,
    roof_curved_colour: Option<u32>,
    ids: impl Fn(u64) -> Option<u32>,
) -> u32 {
    if !direct {
        if let Some(c) = roof_curved_colour {
            return c;
        }
    }
    ids(key).unwrap_or_else(|| panic!("index out of bounds: HalftimberColorIds[{key}]"))
}

/// Roof discriminant check (O): `byte @+0x40 == 2` -> colour `u32 @+0x44` (words 16 / 17).
pub fn roof_curved_colour_from_words(roof: &[u32; 22]) -> Option<u32> {
    if (roof[16] & 0xff) == 2 {
        Some(roof[17])
    } else {
        None
    }
}

/// In-place write + upload payload (O): `data[3] = colour` (`mov [rbx+0xc], eax`), change tick set,
/// `position = data[0..3]` (as f32), `sort_key = 1.0` (xmm6), payload = whole 64 B.
pub fn build_curved_halftimber(mut data_words: [u32; 16], colour: u32) -> InstanceUpdate<16> {
    data_words[3] = colour;
    let pos = Vec3::new(
        f32::from_bits(data_words[0]),
        f32::from_bits(data_words[1]),
        f32::from_bits(data_words[2]),
    );
    InstanceUpdate {
        position: pos,
        sort_key: 1.0,
        words: data_words,
    }
}

/// Explicit field -> words view of `CurvedHalfTimberInstanceData` (offsets from instance_data.rs doc;
/// independent of Rust struct padding).
pub fn curved_halftimber_words(d: &CurvedHalfTimberInstanceData) -> [u32; 16] {
    [
        f32w(d.position.x),
        f32w(d.position.y),
        f32w(d.position.z),
        d.pad_0c,
        f32w(d.x0),
        f32w(d.x1),
        f32w(d.y0),
        f32w(d.y1),
        d.w20,
        d.w24,
        f32w(d.w28),
        d.w2c,
        d.w30,
        d.w34,
        d.w38,
        d.w3c,
    ]
}

/// `nani_update_curved_halftimber` 0x142077d20 (O).  The only update system that writes its
/// component in place (`&mut`) — `Mut` DerefMut sets the change tick exactly like the asm store
/// of this_run into the ticks array.
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
    colour_ids: Res<crate::resources::color_id::HalftimberColorIds>,
    roofs: Query<&crate::resources::roofs::roof_shape::Roof>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_curved_halftimber");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(0x18));
    {
        for (active, mut data, chunk) in q.iter_mut() {
            // CurvedHalfTimber row = {u64 key, u8 direct} (O) — Rust type is a typed gap.
            let cw = gap_words::<_, 4>(chunk);
            let key = (cw[0] as u64) | ((cw[1] as u64) << 32);
            let direct = (cw[2] & 0xff) != 0;
            // PublicWalls HashMap (key -> Entity) lookup, then Roof row (0x58 B) via Query<&Roof>.
            let roof_colour = if direct || public_walls.roofs.is_empty() {
                None
            } else {
                public_walls
                    .roofs
                    .get(&crate::resources::walls::WallId(key))
                    .and_then(|e| roofs.get(*e).ok())
                    .and_then(|roof| roof_curved_colour_from_words(&gap_words::<_, 22>(roof)))
            };
            let colour = curved_halftimber_colour(key, direct, roof_colour, |k| {
                colour_ids.0.get(&k).map(|c| c.0)
            });
            data.pad_0c = colour; // DerefMut -> change detection tick (O)
            let u = build_curved_halftimber(curved_halftimber_words(&data), colour);
            // O：被复制的 `T` 就是 ECS 组件本体（`&*data`）；`u.words` 仅用于断言/测试。
            let mut instance_key: InstanceKey = *active;
            subset.update_instance_data::<CurvedHalfTimberInstanceData>(
                &mut instance_key,
                u.position,
                u.sort_key,
                *data,
            );
        }
    }
}

// ===========================================================================
// 5. nani_update_stone_floor  0x142078780 (kind 9, update fn 0x141570690,
//    StoneFloorInstanceData = 0x40 B = 16 words)
// ===========================================================================

/// Roof row words (0x58 B = 22 words); Floor row words (0x28 B = 10 words).
/// Field names are U; offsets are O (see build_stone_floor doc).
pub type RoofWords = [u32; 22];
pub type FloorWords = [u32; 10];

const SF_TALL_THRESHOLD: f32 = 0.1; // 0x3dcccccd
const SF_RADIUS_SCALE: f32 = 0.5; // 0x3f000000
const SF_RADIUS_BIAS: f32 = 0.63; // 0x3f2147ae
const SF_LOW_Y_BIAS: f32 = -0.69; // 0x bf30a3d7

/// `Circle2d::get_tangent_at_u(&circle, 0.0)` result (x, y); `circle` = row words starting at +0xc.
/// Provided by the caller (crate `Circle2d` is a stub with a different signature).
pub type TangentAtU0<'a> = &'a dyn Fn(&[u32]) -> Vec2;

/// Stone floor payload (O).  Inputs: `translation` (Transform), optional Roof / RoofAnimation / Floor rows.
/// Neither Roof nor Floor -> `Option::unwrap_failed` panic (O @0x142079435).
///
/// Roof present (r14), `h = f32(roof[+0x30])`, `e = f32(roof[+0x2c])`, `tag = roof[+8]`:
///  * `x14 = (h <= 0.1) ? 0 : e*0.5 + 0.63`
///  * tag&1 (straight): A = f32x2(roof+0xc); T = (h<=0.1 || f32(+0x34)!=1.0) ? (1,1)
///    : (roof[+0x3c]&1 ? (1,0) : (0,1)); B = x14*T + f32x2(roof+0x1c); straight=1.
///  * else (curved): A = tangent(circle@+0xc, u=0); B = splat(x14 + 2*f32(roof+0x14)); straight=0.
///  * is_floor=0; position = (f32[roof+0xc+8*tag], f32(roof+0x4c) (+(-0.69) if h<=0.1), f32[roof+0x10+8*tag]);
///    tall(h>0.1): B = 0, flag=1 else flag=0.
///  * discriminant `d = byte roof[+0x40]`: d==2 -> w12 = byte[+0x41]|2, w13 = u32[+0x44], w14 = u32[+0x48];
///    d==1 -> (0,0,0); d==0 -> (byte[+0x54],0,0); d>2 -> (byte[+0x54], 0, u32[+0x48]).
/// Roof absent, Floor present: x14 = 0, is_floor=1, same straight/curved formulas using floor rows
/// (A=f32x2(floor+0xc), straight: B = f32x2(floor+0x1c); curved: tangent, B = splat(2*f32(floor+0x14))),
/// position = Transform.translation, flag = 0, w12..14 = 0.
/// Output words: `[x y z, src_u64_lo, A.x A.y B.x B.y, anim, straight, is_floor, flag, w12, w13, w14, 0]`.
/// sort_key = max(|B.x|, |B.y|).
pub fn build_stone_floor(
    translation: Vec3,
    roof: Option<&RoofWords>,
    anim: Option<f32>,
    floor: Option<&FloorWords>,
    tangent: TangentAtU0<'_>,
) -> InstanceUpdate<16> {
    let f = |w: &[u32], i: usize| f32::from_bits(w[i]);
    let anim = anim.unwrap_or(1.0);

    // (src_lo, A, B, straight, is_floor)
    let (src_lo, a, mut b, straight, is_floor, tall_info): (
        u32,
        Vec2,
        Vec2,
        u32,
        u32,
        Option<&RoofWords>,
    );
    if let Some(r) = roof {
        let h = f(r, 12);
        let e = f(r, 11);
        let x14 = if h <= SF_TALL_THRESHOLD {
            0.0
        } else {
            e * SF_RADIUS_SCALE + SF_RADIUS_BIAS
        };
        src_lo = r[0];
        if r[2] & 1 != 0 {
            a = Vec2::new(f(r, 3), f(r, 4));
            let t = if h <= SF_TALL_THRESHOLD || f(r, 13) != 1.0 {
                Vec2::new(1.0, 1.0)
            } else if r[15] & 1 != 0 {
                Vec2::new(1.0, 0.0)
            } else {
                Vec2::new(0.0, 1.0)
            };
            b = Vec2::new(x14 * t.x + f(r, 7), x14 * t.y + f(r, 8));
            straight = 1;
        } else {
            a = tangent(&r[3..]);
            let v = x14 + f(r, 5) * 2.0;
            b = Vec2::new(v, v);
            straight = 0;
        }
        is_floor = 0;
        tall_info = Some(r);
    } else {
        let fl = floor.expect("called `Option::unwrap()` on a `None` value");
        src_lo = fl[0];
        if fl[2] & 1 != 0 {
            a = Vec2::new(f(fl, 3), f(fl, 4));
            b = Vec2::new(0.0 * 1.0 + f(fl, 7), 0.0 * 1.0 + f(fl, 8));
            straight = 1;
        } else {
            a = tangent(&fl[3..]);
            let v = 0.0 + f(fl, 5) * 2.0;
            b = Vec2::new(v, v);
            straight = 0;
        }
        is_floor = 1;
        tall_info = None;
    }

    // position / tall flag / discriminant words
    let (pos, flag, w12, w13, w14);
    match tall_info {
        Some(r) => {
            let h = f(r, 12);
            let tag = r[2] as usize;
            let mut y = f(r, 19);
            if h > SF_TALL_THRESHOLD {
                b = Vec2::ZERO;
                flag = 1u32;
            } else {
                y += SF_LOW_Y_BIAS;
                flag = 0u32;
            }
            // x/z come from the (tag-selected) payload slot; indexing is bounds-checked here
            // (asm reads unchecked memory; tag is 0/1 in practice).
            pos = Vec3::new(f(r, 3 + 2 * tag), y, f(r, 4 + 2 * tag));
            let d = r[16] & 0xff;
            let (a12, a13, a14) = match d {
                2 => (((r[16] >> 8) & 0xff) | 2, r[17], r[18]),
                1 => (0, 0, 0),
                0 => (r[21] & 0xff, 0, 0),
                _ => (r[21] & 0xff, 0, r[18]),
            };
            w12 = a12;
            w13 = a13;
            w14 = a14;
        }
        None => {
            pos = translation;
            flag = 0;
            w12 = 0;
            w13 = 0;
            w14 = 0;
        }
    }

    let words = [
        f32w(pos.x),
        f32w(pos.y),
        f32w(pos.z),
        src_lo,
        f32w(a.x),
        f32w(a.y),
        f32w(b.x),
        f32w(b.y),
        f32w(anim),
        straight,
        is_floor,
        flag,
        w12,
        w13,
        w14,
        0,
    ];
    InstanceUpdate {
        position: pos,
        sort_key: b.x.abs().max(b.y.abs()),
        words,
    }
}

/// `nani_update_stone_floor` 0x142078780 (O).
/// The Circle2d tangent is delegated to the crate's `Circle2d` once its layout is recovered;
/// meanwhile `circle_tangent_u0` is the documented dependency point (typed gap, U).
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
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(9));
    {
        for (active, t, roof, anim, floor) in q.iter() {
            let roof_w = roof.map(|r| gap_words::<_, 22>(r));
            let floor_w = floor.map(|r| gap_words::<_, 10>(r));
            // RoofAnimation row = 8 B, f32 @+0 read (O).
            let anim_v = anim.map(|a| f32::from_bits(gap_words::<_, 2>(a)[0]));
            let u = build_stone_floor(
                t.translation,
                roof_w.as_ref(),
                anim_v,
                floor_w.as_ref(),
                &circle_tangent_u0,
            );
            push_update::<StoneFloorInstanceData, 16>(&mut subset, active, u);
        }
    }
}

/// TYPED GAP (U): `utils::geometry::circle::Circle2d::get_tangent_at_u(&circle, 0.0)` (0x140c97660) on the
/// circle laid out at row+0xc.  Machine code: `ang = fmodf(circle[+0xc], 2pi)` (+2pi if <0) `+ (1-u)*2pi`;
/// returns `(-cos?/sin ...)` as two f32 (xmm0, xmm1) — circle layout not recovered in the crate.
pub fn circle_tangent_u0(_circle_words: &[u32]) -> Vec2 {
    Vec2::ZERO
}

// ===========================================================================
// tests (pure functions only)
// ===========================================================================
#[cfg(test)]
mod gb_tests {
    use super::*;

    fn f(w: u32) -> f32 {
        f32::from_bits(w)
    }

    #[test]
    fn affine_identity_and_scale() {
        let w = gb_affine_words(
            Quat::IDENTITY,
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(2.0, 3.0, 4.0),
        );
        let v: Vec<f32> = w.iter().map(|x| f(*x)).collect();
        assert_eq!(
            v,
            vec![2.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 4.0, 1.0, 2.0, 3.0]
        );
    }

    #[test]
    fn max_abs_scale_nan_and_neg() {
        assert_eq!(gb_max_abs_scale(Vec3::new(-3.0, 2.0, 1.0)), 3.0);
        assert_eq!(gb_max_abs_scale(Vec3::new(f32::NAN, 2.0, -5.0)), 5.0);
    }

    #[test]
    fn pack_2x16f_matches_half() {
        assert_eq!(gb_pack_2x16f(1.0, 2.0), 0x4000_3c00);
    }

    #[test]
    fn wren_layout() {
        let cur = (Quat::IDENTITY, Vec3::new(1.0, 2.0, 3.0), Vec3::ONE * -2.0);
        let prev = (Quat::IDENTITY, Vec3::new(9.0, 8.0, 7.0), Vec3::ONE);
        let u = build_wren(cur, prev, [0xaa, 0xbb], 0xcc);
        assert_eq!(u.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(u.sort_key, 2.0);
        assert_eq!(f(u.words[0]), -2.0);
        assert_eq!(f(u.words[9]), 1.0);
        assert_eq!(f(u.words[12]), 1.0); // prev c0.x
        assert_eq!(f(u.words[21]), 9.0); // prev translation
        assert_eq!(&u.words[24..], &[0xaa, 0xbb, 0xcc, 0]);
    }

    #[test]
    fn gothic_layout() {
        let brick = [1, 2, f32w(1.0), f32w(2.0), 0, 0x1234_5607];
        let mut def = [0u32; 15];
        for (i, d) in def.iter_mut().enumerate() {
            *d = 100 + i as u32;
        }
        def[0] = f32w(1.0);
        def[1] = f32w(2.0);
        def[2] = f32w(0.5);
        def[3] = f32w(-1.0);
        def[4] = f32w(4.0);
        def[5] = f32w(8.0);
        let u =
            build_gothic_window_bricks(Quat::IDENTITY, Vec3::ZERO, Vec3::splat(-3.0), brick, def);
        assert_eq!(u.sort_key, 3.0);
        assert_eq!(&u.words[12..14], &[1, 2]);
        assert_eq!(u.words[14], 0x4000_3c00);
        assert_eq!(u.words[15], 7); // byte only
        assert_eq!(&u.words[16..19], &[106, 107, 108]);
        assert_eq!(u.words[19], gb_pack_2x16f(1.0, 2.0));
        assert_eq!(&u.words[20..23], &[109, 110, 111]);
        assert_eq!(u.words[23], gb_pack_2x16f(0.5, -1.0));
        assert_eq!(&u.words[24..27], &[112, 113, 114]);
        assert_eq!(u.words[27], gb_pack_2x16f(4.0, 8.0));
    }

    #[test]
    fn flower_cell_math() {
        assert_eq!(flower_mask_cell(0.0, 0.0), ((256 << 9 | 256) >> 5, 0)); // centre
        assert_eq!(flower_mask_cell(-1000.0, -1000.0), (0, 0)); // clamp low
        let (w, b) = flower_mask_cell(1000.0, 1000.0); // clamp high -> 0x1ff
        assert_eq!(w, (0x1ff << 9 | 0x1ff) >> 5);
        assert_eq!(b, 31);
        assert_eq!(flower_mask_cell(f32::NAN, 0.0).1, 0);
    }

    #[test]
    fn flower_visibility_branches() {
        let mut mask = vec![0u32; 512 * 512 / 32];
        let sampled = std::cell::Cell::new(0);
        let zero = |_r: FlowerRaster, _x: f32, _z: f32| {
            sampled.set(sampled.get() + 1);
            0.0
        };
        assert!(flower_visible(0.0, 0.0, &mask, zero));
        assert_eq!(sampled.get(), 5);
        // water > 0.01 hides after exactly one sample
        sampled.set(0);
        let water = |r: FlowerRaster, _x: f32, _z: f32| {
            sampled.set(sampled.get() + 1);
            if r == FlowerRaster::Water { 0.02 } else { 0.0 }
        };
        assert!(!flower_visible(0.0, 0.0, &mask, water));
        assert_eq!(sampled.get(), 1);
        // water == 0.01 does not hide (strict >)
        assert!(flower_visible(0.0, 0.0, &mask, |r, _, _| {
            if r == FlowerRaster::Water { 0.01 } else { 0.0 }
        }));
        // path sum clamp: 0.04*4 = 0.16 > 0.1 hides; 0.025*4 = 0.1 shows
        assert!(!flower_visible(0.0, 0.0, &mask, |r, _, _| {
            if r == FlowerRaster::Water { 0.0 } else { 0.04 }
        }));
        assert!(flower_visible(0.0, 0.0, &mask, |r, _, _| {
            if r == FlowerRaster::Water { 0.0 } else { 0.025 }
        }));
        // rock mask bit hides without sampling
        let (w, b) = flower_mask_cell(0.0, 0.0);
        mask[w] |= 1 << b;
        let boom = |_r: FlowerRaster, _x: f32, _z: f32| -> f32 { panic!("must not sample") };
        assert!(!flower_visible(0.0, 0.0, &mask, boom));
    }

    #[test]
    #[should_panic]
    fn flower_mask_bounds_checked() {
        let mask = vec![0u32; 4];
        flower_visible(0.0, 0.0, &mask, |_, _, _| 0.0);
    }

    #[test]
    fn flower_payloads() {
        let vis = build_preplaced_flower(
            Quat::IDENTITY,
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::splat(2.0),
            true,
        );
        assert_eq!(vis.sort_key, 2.0);
        assert_eq!(f(vis.words[9]), 1.0);
        let hid = build_preplaced_flower(
            Quat::IDENTITY,
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::splat(2.0),
            false,
        );
        assert_eq!(hid.sort_key, 0.0);
        assert_eq!(hid.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(f(hid.words[0]), 1.0);
        assert_eq!(f(hid.words[10]), -100.0);
        assert_eq!(hid.words[9], 0x8000_0000);
        assert_eq!(hid.words[11], 0x8000_0000);
        assert_eq!(TERRAIN_ROCK_MASK_SSBO_NAME.len(), 0x3f);
    }

    #[test]
    #[should_panic]
    fn rock_mask_name_assert() {
        check_rock_mask_name("nope");
    }

    #[test]
    fn halftimber_colour_selection() {
        let ids = |k: u64| if k < 3 { Some(10 + k as u32) } else { None };
        assert_eq!(curved_halftimber_colour(1, true, Some(99), ids), 11);
        assert_eq!(curved_halftimber_colour(1, false, Some(99), ids), 99);
        assert_eq!(curved_halftimber_colour(2, false, None, ids), 12);
        let mut roof = [0u32; 22];
        roof[16] = 2;
        roof[17] = 7;
        assert_eq!(roof_curved_colour_from_words(&roof), Some(7));
        roof[16] = 1;
        assert_eq!(roof_curved_colour_from_words(&roof), None);
    }

    #[test]
    #[should_panic]
    fn halftimber_missing_id_panics() {
        curved_halftimber_colour(5, true, None, |_| None);
    }

    #[test]
    fn halftimber_payload() {
        let mut w = [0u32; 16];
        w[0] = f32w(1.0);
        w[1] = f32w(2.0);
        w[2] = f32w(3.0);
        w[3] = 0xdead;
        w[8] = 77;
        let u = build_curved_halftimber(w, 5);
        assert_eq!(u.words[3], 5);
        assert_eq!(u.words[8], 77);
        assert_eq!(u.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(u.sort_key, 1.0);
    }

    fn roof_straight(h: f32) -> RoofWords {
        let mut r = [0u32; 22];
        r[0] = 0x1234;
        r[2] = 1;
        r[3] = f32w(5.0);
        r[4] = f32w(6.0);
        r[5] = f32w(7.0);
        r[6] = f32w(8.0);
        r[7] = f32w(0.5);
        r[8] = f32w(0.25);
        r[11] = f32w(2.0);
        r[12] = f32w(h);
        r[13] = f32w(1.0);
        r[15] = 1;
        r[19] = f32w(4.0);
        r[16] = 2 | (0x10 << 8);
        r[17] = 111;
        r[18] = 222;
        r
    }
    const NOTAN: &dyn Fn(&[u32]) -> Vec2 = &|_| Vec2::new(0.5, -0.5);

    #[test]
    fn stone_floor_roof_straight_tall() {
        let r = roof_straight(1.0);
        let u = build_stone_floor(Vec3::ZERO, Some(&r), None, None, NOTAN);
        // tall: B zeroed, flag 1, y unchanged, x/z from slot 5/6? tag=1 -> words 5,6
        assert_eq!(u.position, Vec3::new(7.0, 4.0, 8.0));
        assert_eq!(u.words[3], 0x1234);
        assert_eq!((f(u.words[4]), f(u.words[5])), (5.0, 6.0));
        assert_eq!((f(u.words[6]), f(u.words[7])), (0.0, 0.0));
        assert_eq!(f(u.words[8]), 1.0); // default anim
        assert_eq!(&u.words[9..12], &[1, 0, 1]);
        assert_eq!(&u.words[12..15], &[0x10 | 2, 111, 222]);
        assert_eq!(u.sort_key, 0.0);
    }

    #[test]
    fn stone_floor_roof_straight_low() {
        let r = roof_straight(0.05); // h <= 0.1
        let u = build_stone_floor(Vec3::ZERO, Some(&r), Some(0.5), None, NOTAN);
        assert_eq!(f(u.words[8]), 0.5);
        assert_eq!(u.words[11], 0); // not tall
        assert!((u.position.y - (4.0 - 0.69)).abs() < 1e-6);
        // x14 = 0 (h<=0.1); T=(1,1): B = (0+0.5, 0+0.25)
        assert_eq!((f(u.words[6]), f(u.words[7])), (0.5, 0.25));
        assert_eq!(u.sort_key, 0.5);
    }

    #[test]
    fn stone_floor_roof_straight_axis_select() {
        // h>0.1 but still B zeroed by tall; check T via a roof whose tag path differs: use h<=0.1 to force T=(1,1)
        let mut r = roof_straight(1.0);
        r[13] = f32w(2.0); // != 1.0 -> T=(1,1), x14 = 2*0.5+0.63
        let u = build_stone_floor(Vec3::ZERO, Some(&r), None, None, NOTAN);
        assert_eq!(f(u.words[6]), 0.0); // zeroed by tall anyway
        assert_eq!(u.words[11], 1);
    }

    #[test]
    fn stone_floor_discriminants() {
        let mut r = roof_straight(1.0);
        r[21] = 0x1ab;
        r[16] = 0;
        let u = build_stone_floor(Vec3::ZERO, Some(&r), None, None, NOTAN);
        assert_eq!(&u.words[12..15], &[0xab, 0, 0]);
        r[16] = 1;
        let u = build_stone_floor(Vec3::ZERO, Some(&r), None, None, NOTAN);
        assert_eq!(&u.words[12..15], &[0, 0, 0]);
        r[16] = 3;
        let u = build_stone_floor(Vec3::ZERO, Some(&r), None, None, NOTAN);
        assert_eq!(&u.words[12..15], &[0xab, 0, 222]);
    }

    #[test]
    fn stone_floor_roof_curved() {
        let mut r = roof_straight(1.0);
        r[2] = 0; // curved
        r[5] = f32w(1.5);
        let u = build_stone_floor(Vec3::ZERO, Some(&r), None, None, NOTAN);
        assert_eq!((f(u.words[4]), f(u.words[5])), (0.5, -0.5)); // tangent
        assert_eq!(u.words[9], 0); // straight flag
        assert_eq!(u.words[11], 1); // tall -> B zeroed
        assert_eq!(u.position, Vec3::new(5.0, 4.0, 6.0)); // tag 0 slot
    }

    #[test]
    fn stone_floor_floor_only() {
        let mut fl = [0u32; 10];
        fl[0] = 0x77;
        fl[2] = 1;
        fl[3] = f32w(1.0);
        fl[4] = f32w(2.0);
        fl[7] = f32w(-4.0);
        fl[8] = f32w(3.0);
        let u = build_stone_floor(Vec3::new(9.0, 8.0, 7.0), None, None, Some(&fl), NOTAN);
        assert_eq!(u.position, Vec3::new(9.0, 8.0, 7.0));
        assert_eq!(u.words[3], 0x77);
        assert_eq!((f(u.words[6]), f(u.words[7])), (-4.0, 3.0));
        assert_eq!(&u.words[9..12], &[1, 1, 0]);
        assert_eq!(&u.words[12..16], &[0, 0, 0, 0]);
        assert_eq!(u.sort_key, 4.0);
        // curved floor: B = splat(0 + 2*f32(+0x14))
        fl[2] = 0;
        fl[5] = f32w(1.25);
        let u = build_stone_floor(Vec3::ZERO, None, None, Some(&fl), NOTAN);
        assert_eq!((f(u.words[6]), f(u.words[7])), (2.5, 2.5));
        assert_eq!(u.words[9], 0);
    }

    #[test]
    #[should_panic(expected = "None")]
    fn stone_floor_requires_roof_or_floor() {
        build_stone_floor(Vec3::ZERO, None, None, None, NOTAN);
    }
}

// =====================================================================================
// Fragment gC — nani_update_{lantern_emissive, plaster, cloth, smoke, balcony_door}
// (to be merged into render/nani/systems.rs by n_sys; uses shared helpers from common.rs:
//  InstanceUpdate, f32w, push_update, gap_words, kind_from_u8)
//
// Extra imports needed at top of systems.rs:
//   use bevy_ecs::query::{Changed, Or, With};
//   use glam::{Mat3, Quat, Vec3};
//   use crate::asset_libraries::texture_library::AssetTextureLibrary;
//   use crate::components::{ClothlineCloth, LanternLightParams, NormalMap, Smoke, SemiTransparent};
//   use crate::name_hash::NameHash;
//   use crate::resources::color_id::ColorId;
//   use crate::resources::walls::plaster::{AnimatedPlasterPeel, Plaster};
//   use crate::resources::walls::public_walls::PublicWalls;
//   use crate::resources::walls::WallId;
// Cargo (shared file, append only): half = "=2.7.1"   (already in the offline cache)
// =====================================================================================

// ---------------------------------------------------------------------------
// Shared per-row math used by all 5 systems (O: byte-identical sequence in every
// nani_update_* body; first seen in nani_update_plant 0x142074740).
// ---------------------------------------------------------------------------

/// `x86 maxss dst, src` semantics: `dst > src ? dst : src` (returns `src` if either is NaN).
#[inline]
fn maxss(dst: f32, src: f32) -> f32 {
    if dst > src { dst } else { src }
}

/// Per-row Transform → ( packed 3x3(R*S) column-major + translation = 12 words, translation,
/// sort_key = max(|sx|,|sy|,|sz|) ) (O).
///
/// * words 0..9  = columns of `Mat3::from_quat(rotation) * diag(scale)` (col_i * scale_i),
///   stored contiguous (c0.xyz, c1.xyz, c2.xyz) — O: shuffles at 0x14207d3f6..0x14207d40a;
/// * words 9..12 = translation (O: `movaps xmm0,[row+0x10]` lanes 0..2);
/// * position passed to `update_instance_data` = translation (O: r8 = copy of row+0x10 12 B);
/// * sort_key (xmm3) = NaN-propagating max chain of |scale| (O: andps mask 0x7fffffff,
///   cmpunordss/maxss sequence order y=[+0x20], z=[+0x24], then x=[+0x1c]).
/// Binary Transform layout is `{rotation: Quat @+0, translation: Vec3 @+0x10, scale: Vec3 @+0x1c}`
/// (stride 0x30) — NOTE: crate `Transform` in utils/components/transform.rs declares
/// translation @+0, rotation @+0xC; field *names* are used here so layout is irrelevant.
pub fn trs_packed(rotation: Quat, translation: Vec3, scale: Vec3) -> ([u32; 12], Vec3, f32) {
    let r = Mat3::from_quat(rotation);
    let c0 = r.x_axis * scale.x;
    let c1 = r.y_axis * scale.y;
    let c2 = r.z_axis * scale.z;
    let w = [
        c0.x,
        c0.y,
        c0.z,
        c1.x,
        c1.y,
        c1.z,
        c2.x,
        c2.y,
        c2.z,
        translation.x,
        translation.y,
        translation.z,
    ];
    let mut words = [0u32; 12];
    for (o, v) in words.iter_mut().zip(w) {
        *o = v.to_bits();
    }
    // sort key: y = |s.y|, z = |s.z|; m1 = y.is_nan() ? z : maxss(z, y); m2 = x.is_nan() ? m1 : maxss(m1, x)
    let (ax, ay, az) = (scale.x.abs(), scale.y.abs(), scale.z.abs());
    let m1 = if ay.is_nan() { az } else { maxss(az, ay) };
    let sort_key = if ax.is_nan() { m1 } else { maxss(m1, ax) };
    (words, translation, sort_key)
}

// ---------------------------------------------------------------------------
// nani_update_lantern_emissive  0x14207a4f0 (3474 B)  kind=18 (`mov dl,0x12`)  data 0x40
// ---------------------------------------------------------------------------

/// `LanternLightParams` fields read by the system (O; component is a typed gap in
/// `components/mod.rs`, row stride 36 B = 9 words).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LanternParamsView {
    /// +0x0c f32 (packed into word12 low half).
    pub p0: f32,
    /// +0x10 f32 (word12 high half).
    pub p1: f32,
    /// +0x14 f32 (word13 low half).
    pub p2: f32,
    /// +0x18 f32 (word13 high half).
    pub p3: f32,
    /// +0x20 u8 (shifted into bit 30 of word14; `shl edi,0x1e`). O: bytes 0x00..0x0b / 0x1c..0x1f unread.
    pub flag: u8,
}

/// O: `data = [mat3(9) , translation(3), pack2f16(p0,p1), pack2f16(p2,p3),
///            color_id | semi<<31 | flag<<30, 0]` (0x40 B `LanternEmissiveInstanceData`).
/// `color_id` None ⇒ 0 (O: `xor r12d,r12d` when Option<&ColorId> is None).
pub fn build_lantern_emissive(
    rotation: Quat,
    translation: Vec3,
    scale: Vec3,
    params: LanternParamsView,
    semi_transparent: bool,
    color_id: Option<u32>,
) -> InstanceUpdate<16> {
    let (t, position, sort_key) = trs_packed(rotation, translation, scale);
    let mut words = [0u32; 16];
    words[..12].copy_from_slice(&t);
    words[12] = pack_2x16f_uint(params.p0, params.p1);
    words[13] = pack_2x16f_uint(params.p2, params.p3);
    words[14] =
        color_id.unwrap_or(0) | ((semi_transparent as u32) << 31) | ((params.flag as u32) << 30);
    words[15] = 0;
    InstanceUpdate {
        position,
        sort_key,
        words,
    }
}

/// `nani_update_lantern_emissive` (O: PDB signature; kind 18).
pub fn nani_update_lantern_emissive(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            &LanternLightParams,
            Option<&SemiTransparent>,
            Option<&ColorId>,
        ),
        With<InstanceDataMarker<LanternEmissiveInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_lantern_emissive");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(18));
    {
        for (active, transform, params, semi, color) in q.iter() {
            // U (component owner): LanternLightParams layout = 9 words, see LanternParamsView.
            let w = gap_words::<_, 9>(params);
            let view = LanternParamsView {
                p0: f32::from_bits(w[3]),
                p1: f32::from_bits(w[4]),
                p2: f32::from_bits(w[5]),
                p3: f32::from_bits(w[6]),
                flag: (w[8] & 0xff) as u8,
            };
            let u = build_lantern_emissive(
                transform.rotation,
                transform.translation,
                transform.scale,
                view,
                semi.is_some(),
                color.map(|c| c.0),
            );
            push_update::<LanternEmissiveInstanceData, 16>(&mut subset, active, u);
        }
    }
}

// ---------------------------------------------------------------------------
// nani_update_plaster  0x14207b290 (3588 B)  kind=19 (`mov dl,0x13`)  data 0x30
// ---------------------------------------------------------------------------

/// Fields of the `PublicWallState` value (hash bucket = `WallId u64` key + 0x2a0 value, stride 0x2a8)
/// read by the system (O). Offsets are relative to the *value* (bucket + 8).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PlasterWallInputs {
    /// value+0x210 u64: `simplified_curve_2d` point count (O: `cmp qword,2; jae`).
    pub simplified_point_count: u64,
    /// value+0x230 f32: `simplified_curve_2d` length (O: `ucomiss >0`).
    pub simplified_length: f32,
    /// value+0x258 u32: shape discriminant (0 Circle, 1 Rectangle, 2 Freehand); `!= 2` → word6.
    pub shape_tag: u32,
    /// value+0x290 f32: fallback peel value when `AnimatedPlasterPeel` has no entry for the wall.
    pub default_peel: f32,
}

impl PlasterWallInputs {
    /// Reads the observed offsets from the 0x2a0-byte `PublicWallState` record (O offsets).
    pub fn from_state(state: &crate::resources::walls::public_wall_state::PublicWallState) -> Self {
        let b = state.raw_bytes();
        let u64_at = |o: usize| u64::from_le_bytes(b[o..o + 8].try_into().unwrap());
        let u32_at = |o: usize| u32::from_le_bytes(b[o..o + 4].try_into().unwrap());
        Self {
            simplified_point_count: u64_at(0x210),
            simplified_length: f32::from_bits(u32_at(0x230)),
            shape_tag: u32_at(0x258),
            default_peel: f32::from_bits(u32_at(0x290)),
        }
    }

    /// `w.simplified_curve_2d.is_valid()` (O: inlined; `count >= 2 && length > 0.0`,
    /// NaN length ⇒ invalid since `ucomiss; jbe`).
    pub fn simplified_curve_is_valid(&self) -> bool {
        self.simplified_point_count >= 2 && self.simplified_length > 0.0
    }
}

/// `Plaster` row (stride 0x28 = 10 words, O; component is a typed gap).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PlasterView {
    /// +0x00..0x10: 16 B copied verbatim (O: 0x140dc0a90 is the ICF'd 16-byte memcpy; words 0..4).
    pub head: [u32; 4],
    /// +0x10 u64 (data words 8..10).
    pub qword_10: u64,
    /// +0x18 u32 (data word 7).
    pub dword_18: u32,
    /// +0x1c u32 (data word 10).
    pub dword_1c: u32,
    /// +0x20 u64: `WallId` — key into `PublicWalls` / `AnimatedPlasterPeel` (O).
    pub wall_id: u64,
}

impl PlasterView {
    pub fn from_words(w: [u32; 10]) -> Self {
        Self {
            head: [w[0], w[1], w[2], w[3]],
            qword_10: (w[4] as u64) | ((w[5] as u64) << 32),
            dword_18: w[6],
            dword_1c: w[7],
            wall_id: (w[8] as u64) | ((w[9] as u64) << 32),
        }
    }
}

/// O: `data = [head(4), wall_id as u32, simplified_length, (shape_tag != 2) as u32, p.dword_18,
///             p.qword_10 (2), p.dword_1c, peel]`; position = (0,0,0), sort_key = 1.0
/// (O: `[rbp+0x38]=0`, `movaps xmm3,xmm6(=1.0)`).
/// `peel` = `AnimatedPlasterPeel[wall_id]` if present else `wall.default_peel`.
/// Panics (O): "assertion failed: w.simplified_curve_2d.is_valid()".
pub fn build_plaster(
    p: PlasterView,
    wall: PlasterWallInputs,
    animated_peel: Option<f32>,
) -> InstanceUpdate<12> {
    assert!(
        wall.simplified_curve_is_valid(),
        "assertion failed: w.simplified_curve_2d.is_valid()"
    );
    let peel = animated_peel.unwrap_or(wall.default_peel);
    let words = [
        p.head[0],
        p.head[1],
        p.head[2],
        p.head[3],
        p.wall_id as u32,
        wall.simplified_length.to_bits(),
        (wall.shape_tag != 2) as u32,
        p.dword_18,
        p.qword_10 as u32,
        (p.qword_10 >> 32) as u32,
        p.dword_1c,
        peel.to_bits(),
    ];
    InstanceUpdate {
        position: Vec3::ZERO,
        sort_key: 1.0,
        words,
    }
}

/// `AnimatedPlasterPeel` lookup (O: hashbrown `HashMap<WallId(u64), f32>` at resource +0, bucket 16 B =
/// key u64 + f32 @+8; skipped when the map is empty). Resource is a typed gap in
/// `resources/walls/plaster.rs` ⇒ U: always `None` until its owner recovers the map.
pub fn animated_peel_lookup(_peel: &AnimatedPlasterPeel, _wall_id: u64) -> Option<f32> {
    None
}

/// `nani_update_plaster` (O: PDB signature; kind 19). Rows are visited only when
/// `Changed<Plaster> || Changed<NaniActiveInstance>` (O: tick comparisons in the iterator).
pub fn nani_update_plaster(
    q: Query<
        (&NaniActiveInstance, &Plaster),
        (
            With<InstanceDataMarker<PlasterInstanceData>>,
            Or<(Changed<Plaster>, Changed<NaniActiveInstance>)>,
        ),
    >,
    public_walls: Res<PublicWalls>,
    gpu_world: Res<GpuWorld>,
    peel: Res<AnimatedPlasterPeel>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_plaster");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(19));
    {
        for (active, plaster) in q.iter() {
            let view = PlasterView::from_words(gap_words::<_, 10>(plaster));
            // O: missing wall ⇒ `Option::unwrap` on None (core::option::unwrap_failed).
            let wall = public_walls.walls.get(&WallId(view.wall_id)).unwrap();
            let u = build_plaster(
                view,
                PlasterWallInputs::from_state(wall),
                animated_peel_lookup(&peel, view.wall_id),
            );
            push_update::<PlasterInstanceData, 12>(&mut subset, active, u);
        }
    }
}

// ---------------------------------------------------------------------------
// nani_update_cloth  0x14207c0a0 (3253 B)  kind=20 (`mov dl,0x14`)  data 0x40
// ---------------------------------------------------------------------------

/// O: `data = [mat3(9), translation(3), cloth[0], cloth[1], cloth[2], bindless_index]`.
/// `cloth` = the 12-byte `ClothlineCloth` row (O stride 12: u64 @+0 copied via movsd, u32 @+8).
/// `normal_map_bindless_index` = `AssetTextureLibrary::handle_to_bindless_index(handle of NormalMap name)`.
pub fn build_cloth(
    rotation: Quat,
    translation: Vec3,
    scale: Vec3,
    cloth: [u32; 3],
    normal_map_bindless_index: u32,
) -> InstanceUpdate<16> {
    let (t, position, sort_key) = trs_packed(rotation, translation, scale);
    let mut words = [0u32; 16];
    words[..12].copy_from_slice(&t);
    words[12] = cloth[0];
    words[13] = cloth[1];
    words[14] = cloth[2];
    words[15] = normal_map_bindless_index;
    InstanceUpdate {
        position,
        sort_key,
        words,
    }
}

/// `nani_update_cloth` (O: PDB signature; kind 20).
pub fn nani_update_cloth(
    q: Query<
        (&NaniActiveInstance, &Transform, &ClothlineCloth, &NormalMap),
        With<InstanceDataMarker<ClothInstanceData>>,
    >,
    textures: Res<AssetTextureLibrary>,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_cloth");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(20));
    {
        for (active, transform, cloth, normal_map) in q.iter() {
            // U (component owners): ClothlineCloth = 12 B row; NormalMap = 8 B row holding a NameHash (u64).
            let cloth_w = gap_words::<_, 3>(cloth);
            let nm = gap_words::<_, 2>(normal_map);
            let name = NameHash((nm[0] as u64) | ((nm[1] as u64) << 32));
            // O: `by_name` lookup (utils::name_hash::eq probing; miss ⇒ Option::unwrap on None),
            // then `handle_to_bindless_index(handle)` (0x14093d310). The original skips the
            // `textures.get(idx)` bounds check done by `get_handle_and_texture_by_name`.
            let handle = textures
                .get_handle_and_texture_by_name(name)
                .map(|(h, _)| h)
                .unwrap();
            let index = textures.handle_to_bindless_index(handle).0;
            let u = build_cloth(
                transform.rotation,
                transform.translation,
                transform.scale,
                cloth_w,
                index,
            );
            push_update::<ClothInstanceData, 16>(&mut subset, active, u);
        }
    }
}

// ---------------------------------------------------------------------------
// nani_update_smoke  0x14207cd60 (2562 B)  kind=21 (`mov dl,0x15`)  data 0x40
// ---------------------------------------------------------------------------

/// `Smoke` row fields (O: stride 16 B; component is a typed gap).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SmokeView {
    /// +0x00 u32 → data word 13.
    pub dword_0: u32,
    /// +0x08 f32 (bit pattern copied) → data word 14.
    pub float_8: f32,
    /// +0x0c u8 (zero-extended, `movzx r8d,byte`) → data word 12.
    pub byte_c: u8,
}

/// O: `data = [mat3(9), translation(3), byte_c as u32, dword_0, float_8, 0]`.
pub fn build_smoke(
    rotation: Quat,
    translation: Vec3,
    scale: Vec3,
    smoke: SmokeView,
) -> InstanceUpdate<16> {
    let (t, position, sort_key) = trs_packed(rotation, translation, scale);
    let mut words = [0u32; 16];
    words[..12].copy_from_slice(&t);
    words[12] = smoke.byte_c as u32;
    words[13] = smoke.dword_0;
    words[14] = smoke.float_8.to_bits();
    words[15] = 0;
    InstanceUpdate {
        position,
        sort_key,
        words,
    }
}

/// `nani_update_smoke` (O: PDB signature; kind 21).
pub fn nani_update_smoke(
    q: Query<
        (&NaniActiveInstance, &Transform, &Smoke),
        With<InstanceDataMarker<SmokeInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_smoke");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(21));
    {
        for (active, transform, smoke) in q.iter() {
            // U (component owner): Smoke = 16 B row {u32 @0, ? @4, f32 @8, u8 @0xc}.
            let w = gap_words::<_, 4>(smoke);
            let view = SmokeView {
                dword_0: w[0],
                float_8: f32::from_bits(w[2]),
                byte_c: (w[3] & 0xff) as u8,
            };
            let u = build_smoke(
                transform.rotation,
                transform.translation,
                transform.scale,
                view,
            );
            push_update::<SmokeInstanceData, 16>(&mut subset, active, u);
        }
    }
}

// ---------------------------------------------------------------------------
// nani_update_balcony_door  0x14207d770 (2673 B)  kind=22 (`mov dl,0x16`)  data 0x40
// ---------------------------------------------------------------------------

/// O: `data = [mat3(9), translation(3), semi_transparent as u32 (`and cl,1`), color_id, 0, 0]`.
pub fn build_balcony_door(
    rotation: Quat,
    translation: Vec3,
    scale: Vec3,
    color_id: u32,
    semi_transparent: bool,
) -> InstanceUpdate<16> {
    let (t, position, sort_key) = trs_packed(rotation, translation, scale);
    let mut words = [0u32; 16];
    words[..12].copy_from_slice(&t);
    words[12] = semi_transparent as u32;
    words[13] = color_id;
    InstanceUpdate {
        position,
        sort_key,
        words,
    }
}

/// `nani_update_balcony_door` (O: PDB signature; kind 22).
pub fn nani_update_balcony_door(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            &ColorId,
            Option<&SemiTransparent>,
        ),
        With<InstanceDataMarker<BalconyDoorInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_balcony_door");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(22));
    {
        for (active, transform, color, semi) in q.iter() {
            let u = build_balcony_door(
                transform.rotation,
                transform.translation,
                transform.scale,
                color.0,
                semi.is_some(),
            );
            push_update::<BalconyDoorInstanceData, 16>(&mut subset, active, u);
        }
    }
}

// ---------------------------------------------------------------------------
// tests (pure functions only)
// ---------------------------------------------------------------------------
#[cfg(test)]
mod group_c_tests {
    use super::*;

    fn w(x: f32) -> u32 {
        x.to_bits()
    }

    #[test]
    fn pack_2x16f_low_is_first_arg() {
        // f16(1.0)=0x3c00, f16(2.0)=0x4000, f16(-0.5)=0xb800
        assert_eq!(pack_2x16f_uint(1.0, 2.0), 0x4000_3c00);
        assert_eq!(pack_2x16f_uint(-0.5, 0.0), 0x0000_b800);
    }

    #[test]
    fn trs_identity_layout() {
        let (words, pos, key) = trs_packed(
            Quat::IDENTITY,
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(2.0, 3.0, 4.0),
        );
        // columns: (2,0,0)(0,3,0)(0,0,4) then translation
        let expect = [2.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 4.0, 1.0, 2.0, 3.0];
        for i in 0..12 {
            assert_eq!(words[i], w(expect[i]), "word {i}");
        }
        assert_eq!(pos, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(key, 4.0);
    }

    #[test]
    fn trs_sort_key_is_max_abs_scale() {
        let (_, _, k) = trs_packed(Quat::IDENTITY, Vec3::ZERO, Vec3::new(-5.0, 2.0, 3.0));
        assert_eq!(k, 5.0);
        let (_, _, k) = trs_packed(Quat::IDENTITY, Vec3::ZERO, Vec3::new(1.0, -7.0, 3.0));
        assert_eq!(k, 7.0);
    }

    #[test]
    fn trs_rotation_90_about_z() {
        let q = Quat::from_rotation_z(core::f32::consts::FRAC_PI_2);
        let (words, _, _) = trs_packed(q, Vec3::ZERO, Vec3::ONE);
        // x axis -> +y, y axis -> -x
        assert!((f32::from_bits(words[0]) - 0.0).abs() < 1e-6);
        assert!((f32::from_bits(words[1]) - 1.0).abs() < 1e-6);
        assert!((f32::from_bits(words[3]) + 1.0).abs() < 1e-6);
    }

    #[test]
    fn lantern_flags_and_packing() {
        let p = LanternParamsView {
            p0: 1.0,
            p1: 2.0,
            p2: 0.5,
            p3: 4.0,
            flag: 1,
        };
        let u = build_lantern_emissive(Quat::IDENTITY, Vec3::ZERO, Vec3::ONE, p, true, Some(7));
        assert_eq!(u.words[12], 0x4000_3c00);
        assert_eq!(u.words[13], pack_2x16f_uint(0.5, 4.0));
        assert_eq!(u.words[14], 7 | (1 << 31) | (1 << 30));
        assert_eq!(u.words[15], 0);
        // no color id and not semi-transparent: only flag bit
        let u = build_lantern_emissive(Quat::IDENTITY, Vec3::ZERO, Vec3::ONE, p, false, None);
        assert_eq!(u.words[14], 1 << 30);
        let p0 = LanternParamsView { flag: 0, ..p };
        let u = build_lantern_emissive(Quat::IDENTITY, Vec3::ZERO, Vec3::ONE, p0, false, Some(3));
        assert_eq!(u.words[14], 3);
    }

    fn wall(count: u64, len: f32, tag: u32, def: f32) -> PlasterWallInputs {
        PlasterWallInputs {
            simplified_point_count: count,
            simplified_length: len,
            shape_tag: tag,
            default_peel: def,
        }
    }

    #[test]
    fn plaster_layout_and_constants() {
        let view = PlasterView {
            head: [1, 2, 3, 4],
            qword_10: 0x1111_2222_3333_4444,
            dword_18: 0x18,
            dword_1c: 0x1c,
            wall_id: 0xAAAA_BBBB_CCCC_DDDD,
        };
        let u = build_plaster(view, wall(2, 3.5, 1, 0.25), None);
        assert_eq!(u.position, Vec3::ZERO);
        assert_eq!(u.sort_key, 1.0);
        assert_eq!(
            u.words,
            [
                1,
                2,
                3,
                4,
                0xCCCC_DDDD,
                w(3.5),
                1,
                0x18,
                0x3333_4444,
                0x1111_2222,
                0x1c,
                w(0.25)
            ]
        );
    }

    #[test]
    fn plaster_peel_override_and_freehand() {
        let view = PlasterView::default();
        let u = build_plaster(view, wall(5, 1.0, 2, 0.25), Some(0.75));
        assert_eq!(u.words[6], 0); // shape_tag == 2 (Freehand) -> 0
        assert_eq!(u.words[11], w(0.75));
        let u = build_plaster(view, wall(5, 1.0, 0, 0.25), None);
        assert_eq!(u.words[6], 1);
        assert_eq!(u.words[11], w(0.25));
    }

    #[test]
    #[should_panic(expected = "assertion failed: w.simplified_curve_2d.is_valid()")]
    fn plaster_invalid_curve_few_points() {
        build_plaster(PlasterView::default(), wall(1, 1.0, 0, 0.0), None);
    }

    #[test]
    #[should_panic(expected = "assertion failed: w.simplified_curve_2d.is_valid()")]
    fn plaster_invalid_curve_zero_length() {
        build_plaster(PlasterView::default(), wall(4, 0.0, 0, 0.0), None);
    }

    #[test]
    #[should_panic(expected = "is_valid")]
    fn plaster_invalid_curve_nan_length() {
        build_plaster(PlasterView::default(), wall(4, f32::NAN, 0, 0.0), None);
    }

    #[test]
    fn plaster_view_from_words() {
        let v = PlasterView::from_words([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(v.head, [1, 2, 3, 4]);
        assert_eq!(v.qword_10, 5 | (6u64 << 32));
        assert_eq!((v.dword_18, v.dword_1c), (7, 8));
        assert_eq!(v.wall_id, 9 | (10u64 << 32));
    }

    #[test]
    fn cloth_layout() {
        let u = build_cloth(
            Quat::IDENTITY,
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::ONE,
            [10, 11, 12],
            99,
        );
        assert_eq!(&u.words[12..], &[10, 11, 12, 99]);
        assert_eq!(u.sort_key, 1.0);
    }

    #[test]
    fn smoke_layout() {
        let s = SmokeView {
            dword_0: 0xdead,
            float_8: 1.5,
            byte_c: 0xff,
        };
        let u = build_smoke(Quat::IDENTITY, Vec3::ZERO, Vec3::ONE, s);
        assert_eq!(&u.words[12..], &[0xff, 0xdead, w(1.5), 0]);
    }

    #[test]
    fn balcony_layout() {
        let u = build_balcony_door(Quat::IDENTITY, Vec3::ZERO, Vec3::ONE, 42, true);
        assert_eq!(&u.words[12..], &[1, 42, 0, 0]);
        let u = build_balcony_door(Quat::IDENTITY, Vec3::ZERO, Vec3::ONE, 42, false);
        assert_eq!(&u.words[12..], &[0, 42, 0, 0]);
    }
}

// ===========================================================================
// Group D fragment: nani_update_uber / nani_update_velocity_only / nani_update_stairs_floor
// Uses common.rs helpers (InstanceUpdate, f32w, push_update, gap_words, kind_from_u8).
// Shared helpers introduced here (other groups may need identical ones; merge by name):
//   pack_affine3x4, max_abs_scale
// ===========================================================================

/// 48B packed `Mat3 (col-major, rot*scale) ++ translation` as 12 f32 words (O: identical tail in
/// plant/uber/velocity_only). Layout: [c0.xyz, c1.xyz, c2.xyz, t.xyz]; c_i = rot_axis_i * scale_i.
pub fn pack_affine3x4(t: &Transform) -> [u32; 12] {
    let m = Mat3::from_quat(t.rotation);
    let c0 = m.x_axis * t.scale.x;
    let c1 = m.y_axis * t.scale.y;
    let c2 = m.z_axis * t.scale.z;
    let tr = t.translation;
    [
        f32w(c0.x),
        f32w(c0.y),
        f32w(c0.z),
        f32w(c1.x),
        f32w(c1.y),
        f32w(c1.z),
        f32w(c2.x),
        f32w(c2.y),
        f32w(c2.z),
        f32w(tr.x),
        f32w(tr.y),
        f32w(tr.z),
    ]
}

// ---------------------------------------------------------------------------
// nani_update_uber  0x14207e1f0  (subset kind 0x17 = 23; data = UberInstanceData 0x40 B, N = 16)
// ---------------------------------------------------------------------------

/// Uber flags word (O): `bit0 = has AlbedoMap`, `bit1 = SemiTransparent present`, `bit2 = always set`
/// (asm: `lea ecx,[rcx + 2*r8 + 4]` with ecx = 1 when albedo present else 0, r8 = (semi != null)).
pub const UBER_FLAG_ALBEDO: u32 = 1;
pub const UBER_FLAG_SEMI_TRANSPARENT: u32 = 2;
pub const UBER_FLAG_BASE: u32 = 4;

/// O: words[0..12] = affine3x4(transform); words[12] = flags; words[13] = bindless index of albedo
/// texture (0 if no AlbedoMap); words[14..16] = 0. position = translation, sort_key = max |scale| (O).
pub fn build_uber(
    transform: &Transform,
    semi_transparent: bool,
    albedo_bindless_index: Option<u32>,
) -> InstanceUpdate<16> {
    let mut w = [0u32; 16];
    w[..12].copy_from_slice(&pack_affine3x4(transform));
    let has_albedo = albedo_bindless_index.is_some() as u32;
    w[12] = has_albedo * UBER_FLAG_ALBEDO
        + (semi_transparent as u32) * UBER_FLAG_SEMI_TRANSPARENT
        + UBER_FLAG_BASE;
    w[13] = albedo_bindless_index.unwrap_or(0);
    InstanceUpdate {
        position: transform.translation,
        sort_key: max_abs_scale(transform.scale),
        words: w,
    }
}

/// `AlbedoMap` payload is a `NameHash` (u64) (O: row stride 8, `*row` hashed with the library's by_name map).
/// The crate's `AlbedoMap` is a unit `Gap` (components/drawable.rs, not ours) -> U: `gap_words` until recovered.
fn albedo_name_hash(c: &AlbedoMap) -> crate::name_hash::NameHash {
    let w: [u32; 2] = gap_words(c);
    crate::name_hash::NameHash(w[0] as u64 | ((w[1] as u64) << 32))
}

/// O: `AssetTextureLibrary` by_name lookup (miss -> `Option::unwrap` failure, asm 0x14207ed8b) then
/// `handle_to_bindless_index(handle)` (0x14093d310). I: crate API `get_handle_and_texture_by_name` also bounds-checks
/// `textures[handle]` (extra vs asm, which reads the NamedTexture.handle directly; by_name is private).
fn albedo_bindless_index(lib: &AssetTextureLibrary, a: &AlbedoMap) -> u32 {
    let (handle, _tex) = lib
        .get_handle_and_texture_by_name(albedo_name_hash(a))
        .unwrap();
    lib.handle_to_bindless_index(handle).0
}

pub fn nani_update_uber(
    q: Query<
        (
            &NaniActiveInstance,
            &Transform,
            Option<&crate::components::SemiTransparent>,
            Option<&AlbedoMap>,
        ),
        With<InstanceDataMarker<UberInstanceData>>,
    >,
    textures: Res<AssetTextureLibrary>,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_uber");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(0x17)); // O: mov dl,0x17
    {
        for (active, transform, semi, albedo) in q.iter() {
            let idx = albedo.map(|a| albedo_bindless_index(&textures, a));
            let u = build_uber(transform, semi.is_some(), idx);
            push_update::<UberInstanceData, 16>(&mut subset, active, u);
        }
    }
}

// ---------------------------------------------------------------------------
// nani_update_velocity_only  0x14207f060  (kind 0x19 = 25; VelocityOnlyInstanceData 0x60 B, N = 24)
// ---------------------------------------------------------------------------

/// O: words[0..12] = affine3x4(current); words[12..24] = affine3x4(prev). position = current translation,
/// sort_key = max |current scale| (the prev transform's scale is not used for the key).
pub fn build_velocity_only(current: &Transform, prev: &Transform) -> InstanceUpdate<24> {
    let mut w = [0u32; 24];
    w[..12].copy_from_slice(&pack_affine3x4(current));
    w[12..].copy_from_slice(&pack_affine3x4(prev));
    InstanceUpdate {
        position: current.translation,
        sort_key: max_abs_scale(current.scale),
        words: w,
    }
}

/// O: `PrevTransform` row has the same layout as `Transform` (quat +0, translation +0x10, scale +0x1c, stride 0x30).
/// The crate's `PrevTransform` is a unit `Gap` (not ours) -> U: identity until components/drawable.rs recovers it.
fn prev_transform_of(_c: &PrevTransform) -> Transform {
    Transform {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    }
}

pub fn nani_update_velocity_only(
    q: Query<
        (&NaniActiveInstance, &Transform, &PrevTransform),
        With<InstanceDataMarker<VelocityOnlyInstanceData>>,
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_velocity_only");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(0x19)); // O: mov dl,0x19
    {
        for (active, transform, prev) in q.iter() {
            let u = build_velocity_only(transform, &prev_transform_of(prev));
            push_update::<VelocityOnlyInstanceData, 24>(&mut subset, active, u);
        }
    }
}

// ---------------------------------------------------------------------------
// nani_update_stairs_floor  0x14207fcc0  (kind 0x1a = 26; StairsFloorInstanceData 0x30 B, N = 12)
// ---------------------------------------------------------------------------

/// O: data words[0..9] = 36 bytes copied from `StairsFloorMeshChunk + 8` (movups +8, movups +0x18, mov dword +0x28),
/// words[9..12] = 0; position = Vec3::ZERO; sort_key = 1.0 (xmm6 = 1.0f). `chunk_words` = those 9 words.
pub fn build_stairs_floor(chunk_words: [u32; 9]) -> InstanceUpdate<12> {
    let mut w = [0u32; 12];
    w[..9].copy_from_slice(&chunk_words);
    InstanceUpdate {
        position: Vec3::ZERO,
        sort_key: 1.0,
        words: w,
    }
}

/// Query filter is `Or<(Changed<StairsFloorMeshChunk>, Changed<NaniActiveInstance>)>` (O: two
/// `Table::get_changed_ticks_slice_for` calls; row processed if either `this_run - tick < threshold`;
/// threshold = 0xc233b000 when `this_run < last_run`... i.e. bevy `Tick::is_newer_than`).
/// `StairsFloorMeshChunk` is a unit `Gap` in resources/stairs/visual.rs (not ours) -> U: payload words via `gap_words`.
pub fn nani_update_stairs_floor(
    q: Query<
        (&NaniActiveInstance, &StairsFloorMeshChunk),
        (
            With<InstanceDataMarker<StairsFloorInstanceData>>,
            Or<(Changed<StairsFloorMeshChunk>, Changed<NaniActiveInstance>)>,
        ),
    >,
    gpu_world: Res<GpuWorld>,
) {
    let _scope = crate::puffin_contract::scope("nani_update_stairs_floor");
    let mut subset = gpu_world.owned_subset_mut(kind_from_u8(0x1a)); // O: mov dl,0x1a
    {
        for (active, chunk) in q.iter() {
            let u = build_stairs_floor(gap_words::<_, 9>(chunk));
            push_update::<StairsFloorInstanceData, 12>(&mut subset, active, u);
        }
    }
}

#[cfg(test)]
mod group_d_tests {
    use super::*;

    fn tf(t: Vec3, r: Quat, s: Vec3) -> Transform {
        Transform {
            translation: t,
            rotation: r,
            scale: s,
        }
    }
    fn f(w: u32) -> f32 {
        f32::from_bits(w)
    }

    #[test]
    fn affine_identity_layout() {
        let w = pack_affine3x4(&tf(
            Vec3::new(1.0, 2.0, 3.0),
            Quat::IDENTITY,
            Vec3::new(2.0, 3.0, 4.0),
        ));
        let v: Vec<f32> = w.iter().map(|x| f(*x)).collect();
        assert_eq!(
            v,
            vec![2.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 4.0, 1.0, 2.0, 3.0]
        );
    }

    #[test]
    fn affine_rot_z_90() {
        let w = pack_affine3x4(&tf(
            Vec3::ZERO,
            Quat::from_rotation_z(core::f32::consts::FRAC_PI_2),
            Vec3::ONE,
        ));
        assert!((f(w[0])).abs() < 1e-6 && (f(w[1]) - 1.0).abs() < 1e-6); // x axis -> +y
        assert!((f(w[3]) + 1.0).abs() < 1e-6); // y axis -> -x
    }

    #[test]
    fn max_abs_uses_abs_and_ignores_nan() {
        assert_eq!(max_abs_scale(Vec3::new(-5.0, 2.0, 3.0)), 5.0);
        assert_eq!(max_abs_scale(Vec3::new(1.0, f32::NAN, -2.0)), 2.0);
    }

    #[test]
    fn uber_flags_and_index() {
        let t = tf(Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY, Vec3::splat(2.0));
        let a = build_uber(&t, false, None);
        assert_eq!(
            (a.words[12], a.words[13], a.words[14], a.words[15]),
            (4, 0, 0, 0)
        );
        let b = build_uber(&t, true, None);
        assert_eq!(b.words[12], 6);
        let c = build_uber(&t, false, Some(77));
        assert_eq!((c.words[12], c.words[13]), (5, 77));
        let d = build_uber(&t, true, Some(9));
        assert_eq!((d.words[12], d.words[13]), (7, 9));
        assert_eq!(d.sort_key, 2.0);
        assert_eq!(d.position, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn velocity_only_two_affines_key_from_current() {
        let cur = tf(Vec3::new(1.0, 0.0, 0.0), Quat::IDENTITY, Vec3::splat(3.0));
        let prev = tf(Vec3::new(9.0, 8.0, 7.0), Quat::IDENTITY, Vec3::splat(100.0));
        let u = build_velocity_only(&cur, &prev);
        assert_eq!(f(u.words[9]), 1.0);
        assert_eq!(f(u.words[12]), 100.0);
        assert_eq!(f(u.words[21]), 9.0);
        assert_eq!(u.sort_key, 3.0);
        assert_eq!(u.position, cur.translation);
    }

    #[test]
    fn stairs_floor_zero_tail_unit_key() {
        let u = build_stairs_floor([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(&u.words[..9], &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(&u.words[9..], &[0, 0, 0]);
        assert_eq!(u.position, Vec3::ZERO);
        assert_eq!(u.sort_key, 1.0);
    }
}

// ===========================================================================
// 测试（纯函数；ECS/GPU 部分依赖 gpu_world.rs 的重写，见报告）
// ===========================================================================
#[cfg(test)]
mod core_tests {
    use super::*;

    fn size_words<T>() -> usize {
        core::mem::size_of::<T>() / 4
    }

    /// `pack_instance` 要求 `size_of::<T>() == 4*N`；N 取自各 `update_instance_data<T>`（O：元素尺寸）。
    #[test]
    fn instance_data_sizes_match_machine_code() {
        assert_eq!(size_words::<PlantInstanceData>(), 12);
        assert_eq!(size_words::<PlasterInstanceData>(), 12);
        assert_eq!(size_words::<StairsFloorInstanceData>(), 12);
        assert_eq!(size_words::<VelocityOnlyInstanceData>(), 24);
        assert_eq!(size_words::<WrenInstanceData>(), 28);
        assert_eq!(size_words::<GothicWindowBricksInstanceData>(), 28);
        for w in [
            size_words::<UberInstanceData>(),
            size_words::<SolidVertexColorInstanceData>(),
            size_words::<SolidVertexColorBricksInstanceData>(),
            size_words::<SolidVertexColorWindowInstanceData>(),
            size_words::<SolidVertexColorWindowGlassInstanceData>(),
            size_words::<SolidAlbedoMapInstanceData>(),
            size_words::<SolidAlbedoMapEmissiveInstanceData>(),
            size_words::<LanternEmissiveInstanceData>(),
            size_words::<BalconyDoorInstanceData>(),
            size_words::<SmokeInstanceData>(),
            size_words::<StoneFloorInstanceData>(),
            size_words::<ClothInstanceData>(),
        ] {
            assert_eq!(w, 16);
        }
    }

    /// 机器码里 `update_instance_data<PreplacedFlowerInstanceData>` 复制 0x30 字节（12 字）。
    /// instance_data.rs（n_world 已修为 12 字）。
    #[test]
    fn preplaced_flower_is_12_words() {
        assert_eq!(size_words::<PreplacedFlowerInstanceData>(), 12);
    }

    /// `CurvedHalfTimberInstanceData` 机器码为 0x40 B（16 字）；n_world 重写后无填充。
    #[test]
    fn curved_halftimber_is_16_words() {
        assert_eq!(size_words::<CurvedHalfTimberInstanceData>(), 16);
    }

    #[test]
    fn kind_from_u8_covers_27_and_rejects_more() {
        for v in 0u8..27 {
            assert_eq!(kind_from_u8(v) as u8, v);
        }
        assert!(std::panic::catch_unwind(|| kind_from_u8(27)).is_err());
    }

    #[test]
    fn plant_matches_affine_and_sort_key() {
        let u = build_plant(
            Vec3::new(1.0, 2.0, 3.0),
            Quat::IDENTITY,
            Vec3::new(2.0, -5.0, 3.0),
        );
        let f: Vec<f32> = u.words.iter().map(|w| f32::from_bits(*w)).collect();
        assert_eq!(
            f,
            vec![2.0, 0.0, 0.0, 0.0, -5.0, 0.0, 0.0, 0.0, 3.0, 1.0, 2.0, 3.0]
        );
        assert_eq!(u.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(u.sort_key, 5.0);
    }

    #[test]
    fn compact_key_layout() {
        let t = Transform {
            translation: Vec3::new(1.0, 2.0, 3.0),
            scale: Vec3::new(0.5, -4.0, 2.0),
            ..Default::default()
        };
        let k = compact_key_for(&t, 0x1_0007);
        assert_eq!(k.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(k.radius, half::f16::from_f32(4.0));
        assert_eq!(k.mesh_index, 7); // `mov word ptr [rbp+0x6e], bx`：截断为 u16
        let w = compact_key_words(&k);
        assert_eq!(w[3], half::f16::from_f32(4.0).to_bits() as u32 | (7 << 16));
    }

    #[test]
    fn max_abs_scale_ignores_sign_and_nan() {
        assert_eq!(max_abs_scale(Vec3::new(-3.0, 2.0, 1.0)), 3.0);
        assert_eq!(max_abs_scale(Vec3::new(f32::NAN, 2.0, -5.0)), 5.0);
    }

    #[test]
    fn warn_once_dedupes_by_message() {
        let m = format!(
            "Instance limit reached while trying to spawn test-{}",
            line!()
        );
        assert!(warn_once(m.clone()));
        assert!(!warn_once(m.clone()));
        assert!(warn_once(format!("{m} other")));
        assert!(warn_once_key("x").starts_with("module_path! ()$crate :: Level :: Warn"));
    }
}

// ===========================================================================
// ECS 级测试：真实 `GpuWorld`（n_world 重写后的签名）+ `MockDevice`
// ===========================================================================
#[cfg(test)]
mod ecs_tests {
    use super::*;
    use crate::render::ssbo::tests::MockDevice;
    use bevy_ecs::prelude::{Schedule, World};
    use bevy_ecs::system::RunSystemOnce;

    fn world_with(kinds: &[GpuWorldSubsetKind]) -> World {
        let dev = MockDevice::default();
        let mut gw = GpuWorld::new(&dev);
        for &k in kinds {
            let subset = match k {
                GpuWorldSubsetKind::Plant => OwnedGpuWorldSubset::new::<PlantInstanceData>(&dev, k),
                GpuWorldSubsetKind::PreplacedFlower => {
                    OwnedGpuWorldSubset::new::<PreplacedFlowerInstanceData>(&dev, k)
                }
                other => panic!("test world has no element type for {other:?}"),
            };
            gw.assign_subset(k, GpuWorldSubset::Owned(subset));
        }
        let mut w = World::new();
        w.insert_resource(gw);
        w.insert_resource(MeshAtlasLibrary::default());
        w
    }

    fn atlas_desc(part: u32, kind: GpuWorldSubsetKind) -> NaniInstanceDesc {
        NaniInstanceDesc {
            mesh: NaniInstanceMeshDesc::AtlasPart(part),
            subset_kind: kind as u8,
        }
    }

    fn tf(t: Vec3, s: Vec3) -> Transform {
        Transform {
            translation: t,
            scale: s,
            ..Default::default()
        }
    }

    #[test]
    fn maintain_creates_updates_mesh_then_removes() {
        let mut w = world_with(&[GpuWorldSubsetKind::Plant]);
        let mut sched = Schedule::default();
        sched.add_systems(nani_maintain_instances);
        let e = w
            .spawn((
                atlas_desc(77, GpuWorldSubsetKind::Plant),
                tf(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, -3.0, 2.0)),
            ))
            .id();
        sched.run(&mut w);

        // 12B 记录 = InstanceKey，kind 来自 `extend_no_data(kind, ..)`。
        let key = *w
            .get::<NaniActiveInstance>(e)
            .expect("Insert<NaniActiveInstance>");
        assert_eq!(key.kind, GpuWorldSubsetKind::Plant);
        assert_eq!(core::mem::size_of::<NaniActiveInstance>(), 12);
        assert_eq!(
            w.resource::<GpuWorld>().total_instance_count,
            1,
            "解锁后无条件 += 1"
        );
        {
            let gw = w.resource::<GpuWorld>();
            let sub = gw.owned_subset_mut(GpuWorldSubsetKind::Plant);
            assert_eq!(sub.instance_count(), 1);
            let c = sub.positions()[key.index as usize];
            assert_eq!(c.position, Vec3::new(1.0, 2.0, 3.0));
            assert_eq!(c.radius, half::f16::from_f32(3.0), "max(|scale|)");
            assert_eq!(c.mesh_index, 77);
        }

        // Changed<NaniInstanceDesc> → update_instance_mesh。
        w.get_mut::<NaniInstanceDesc>(e).unwrap().mesh = NaniInstanceMeshDesc::AtlasPart(5);
        sched.run(&mut w);
        {
            let gw = w.resource::<GpuWorld>();
            let sub = gw.owned_subset_mut(GpuWorldSubsetKind::Plant);
            assert_eq!(sub.positions()[key.index as usize].mesh_index, 5);
        }

        // 删除路径：RemovedComponents<NaniActiveInstance> → remove_instance。
        w.despawn(e);
        sched.run(&mut w);
        let gw = w.resource::<GpuWorld>();
        assert_eq!(
            gw.owned_subset_mut(GpuWorldSubsetKind::Plant)
                .instance_count(),
            0
        );
    }

    #[test]
    fn maintain_at_capacity_warns_and_skips_insert() {
        let mut w = world_with(&[GpuWorldSubsetKind::PreplacedFlower]);
        let cap = GpuWorldSubsetKind::PreplacedFlower.max_instances();
        let n = cap + 2;
        for i in 0..n {
            w.spawn((
                atlas_desc(1, GpuWorldSubsetKind::PreplacedFlower),
                tf(Vec3::new(i as f32, 0.0, 0.0), Vec3::ONE),
            ));
        }
        w.run_system_once(nani_maintain_instances).unwrap();
        let active = w.query::<&NaniActiveInstance>().iter(&w).count();
        assert_eq!(active, cap, "容量用尽：不 panic，不插入组件");
        assert_eq!(
            w.resource::<GpuWorld>().total_instance_count,
            n as u64,
            "O：`total_instance_count += 1` 无条件（含失败者）"
        );
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn maintain_with_unassigned_subset_panics_like_owned_subset_mut() {
        let mut w = world_with(&[]);
        w.spawn((
            atlas_desc(1, GpuWorldSubsetKind::Plant),
            Transform::default(),
        ));
        w.run_system_once(nani_maintain_instances).unwrap();
    }

    #[test]
    #[should_panic(expected = "Subset Plant is not owned")]
    fn external_subset_panics_in_update_system() {
        let mut w = world_with(&[]);
        w.resource_mut::<GpuWorld>().assign_subset(
            GpuWorldSubsetKind::Plant,
            GpuWorldSubset::External(Default::default()),
        );
        w.run_system_once(nani_update_plant).unwrap();
    }

    #[test]
    fn update_plant_writes_cull_position_radius_and_data() {
        let mut w = world_with(&[GpuWorldSubsetKind::Plant]);
        let e = w
            .spawn((
                atlas_desc(1, GpuWorldSubsetKind::Plant),
                tf(Vec3::new(4.0, 5.0, 6.0), Vec3::splat(2.0)),
            ))
            .id();
        w.run_system_once(nani_maintain_instances).unwrap();
        w.entity_mut(e)
            .insert(InstanceDataMarker::<PlantInstanceData>::default());
        // 位置在 maintain 里已按创建时的 Transform 写入；移动后 update 系统必须覆写。
        *w.get_mut::<Transform>(e).unwrap() =
            tf(Vec3::new(7.0, 8.0, 9.0), Vec3::new(1.0, 4.0, 2.0));
        w.run_system_once(nani_update_plant).unwrap();
        let key = *w.get::<NaniActiveInstance>(e).unwrap();
        let gw = w.resource::<GpuWorld>();
        let sub = gw.owned_subset_mut(GpuWorldSubsetKind::Plant);
        let c = sub.positions()[key.index as usize];
        assert_eq!(c.position, Vec3::new(7.0, 8.0, 9.0));
        assert_eq!(c.radius, half::f16::from_f32(4.0));
        assert_eq!(c.mesh_index, 1, "update_instance_data 不改 mesh_index");
    }

    #[test]
    fn flush_rolls_instance_counts() {
        let mut w = world_with(&[GpuWorldSubsetKind::Plant]);
        w.insert_resource(crate::render::GraphicsDeviceResource(std::sync::Arc::new(
            crate::rhapsody_contract::GraphicsDevice::default(),
        )));
        w.spawn((
            atlas_desc(1, GpuWorldSubsetKind::Plant),
            Transform::default(),
        ));
        w.run_system_once(nani_maintain_instances).unwrap();
        assert_eq!(w.resource::<GpuWorld>().total_instance_count, 1);
        w.run_system_once(nani_flush_instance_updates).unwrap();
        let gw = w.resource::<GpuWorld>();
        assert_eq!(gw.total_instance_count, 0, "+0x1440 清零");
        assert_eq!(gw.previous_instance_count(), 1, "+0x1448 = 旧 +0x1440");
    }
}
