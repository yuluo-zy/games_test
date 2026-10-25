//! prefab.rs — 预制体实例/生成/更新/回收（B `systems/prefab`，fresh 反编译 2026-10-17）。
//!
//! 证据：ghidra shared `get_functions fields=signature,decompiled_code,callees` +
//! `disassemble_function`（`reports/interaction/ui_mode/evidence/` 与
//! `agent-team/work/ui-restorer-remainder/` 下的原始结果）：
//!
//! | VA | 函数 | 处置 |
//! |---|---|---|
//! | 0x140840610 | `spawn_prefabs` | 库内算法与生产 runtime::spawn_prefab_commands 接线 |
//! | 0x140841550 | `create_prefab_rt` | 矩阵/颜色/半径；生产 runtime 插入 BirdHaven |
//! | inline | `create_prefab_audio_trigger` | 实现（并入 [`build_prefab_contents`]：`Aabb3::scaled`+`translated`→`add_trigger`） |
//! | 0x140841b00 | `create_prefab_gpu_instances` | 实现（`MeshAtlasLibrary::get_by_name` + `GpuWorld::create_instances`） |
//! | 0x140841d60 | `update_prefabs` | 核心实现（GPU/音频直写；RT 矩阵变更检测以输出 seam 返回） |
//! | 0x140842d60 | `despawn_prefabs` | 实现（事件→实体→contents→despawn） |
//! | 0x140843390 | `despawn_prefab_contents` | 实现（三 Vec 回收） |
//! | 0x140843740 | `hot_reload_prefabs` | 核心实现（名称/子集变更集合→重建） |
//! | 0x140840430 | `PrefabPlugin::build` | 胶水（Bevy 事件注册/系统链/资源插入） |
//! | 0x1408446b0 / 0x1408e5d10 | `register_required_components` | 胶水-legitimate（Bevy derive 注册体） |
//! | 0x140002370 | `impl$3::clone_behavior` | 胶水（3B `return`；与 `WireframePass::clone_behavior` ICF 同址） |
//! | inline | `create_prefab_gpu_instances::closure$0` / `hot_reload_prefabs::closure$0` | 胶水 |
//!
//! # 原生布局锚点（fresh）
//! * `PrefabInstance` = **0x58 B**：`+0x08` `Vec<GpuInstanceRecord>`（元素 0xc，ptr/len/cap）、
//!   `+0x20` `Vec<RtMeshHandle>`（元素 0x10）、`+0x38` `Vec<AudioTriggerRecord>`（元素 0x28）、
//!   `+0x48/+0x50` `PrefabName`（Arc ptr/len）。`despawn_prefab_contents` 0x140843390 逐 Vec
//!   `ptr@+8/len@+0x10`、`ptr@+0x20/len@+0x28`、`ptr@+0x38/len@+0x40` 读取；
//!   `hot_reload_prefabs` 0x140843740 读 `+0x48/+0x50` 作 `PrefabLibrary::get` 的名称。
//! * 实例变换是**独立 0x30 B 组件**：`+0x00 Vec3 平移`、`+0x0c f32 缩放`、`+0x10 f32 旋转(弧度)`；
//!   `create_prefab_rt` 0x140841550 与 `update_prefabs` 0x140841d60 的 `sinf/cosf` 矩阵都从它读。
//!   Rust 侧 [`PrefabTransform`] 表出（组件注册归属 asset/forest 域，登记）。
//! * GPU 实例记录 0xc：`+0x00 u32 id`、`+0x04 u32 index`、`+0x08 u8 kind`；kind `0x1b`（27，
//!   越过 `GpuWorldSubsetKind` 27 变体）是终止哨兵（`despawn_prefab_contents` 遇之 `break`）。
//! * 音频记录 0x28：`+0x00 Aabb3`；`+0x18` Option 判别（原生 `2` = None → 停止，`0` = Some 继续，
//!   其它 → bounds panic）；`+0x20` slotmap 键。
//!
//! # seam / 登记（解锁条件）
//! 1. Production ECS wiring is in [`runtime`]. Original dedupe is a
//!    `HashSet<Entity>` populated from same-frame DespawnPrefabCmd, not a
//!    GlobalId table. The older slice adapter retains its u64 field name for
//!    callers; these bits represent the target Entity. Complete cmd+18..48
//!    placement and cull position/scale now come from the same record.
//! 2. **RT 写回已落地**（2026-10-17 ui-restorer-seams）：`resources::rt::RtMesh` 订正为 0x60 B
//!    `{matrix [f32;16] @+0, positions Vec<Vec3> @+0x40, color u32 @+0x58}`；`RtWorld` 派生
//!    `Resource`，+0x30 计数器经 [`RtWorld::alloc_prefab_handle`]（tag 2）。`create_prefab_rt`
//!    经 `get_mut` 写矩阵/顶点/颜色；`update_prefabs` 比较 `matrix[4..16]` 后写回。
//!    0x48 B 延迟命令已定名为 insert<BirdHaven>：runtime 消费真实
//!    `(handle, radius)` 返回值，按 part local center/radius 插入同一实体。
//!    `{2, handle}` 为 PrefabInstance.rt_meshes 内容，不是待处理事件。
//! 3. `hot_reload_prefabs` 的 `PrefabWatcher`（hotwatch + 100 ms）与两个事件集合来源已在
//!    `startup::load_prefabs` 落地（见该模块）；本函数接收已收集的 `HashSet<PrefabName>` /
//!    `HashSet<GpuWorldSubsetKind>`。
//! 4. 组件注册：`components::trees::PrefabInstanceData` re-exports the canonical48B placement。
//! 5. `GpuWorld` 构造需 `GraphicsDevice`（`GpuWorld::new`），spawn/hot_reload 的 host 侧负责。
//! 6. Shared BirdHaven 的构造字段已接；critters 旧算法对该组件的完整消费仍另行审查。

use std::collections::HashSet;

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::event::{Event, EventReader};
use bevy_ecs::prelude::{Commands, Query};
use glam::Vec3;

use crate::asset_libraries::audio_trigger_library::{
    AudioTriggerHandle, AudioTriggerKind, AudioTriggerLibrary,
};
use crate::geometry::cube::{CubeBox, cube_mesh};
use crate::render::mesh::Mesh;
use crate::render::nani::gpu_world::{GpuWorld, GpuWorldSubsetKind, InstanceCullData, InstanceKey};
use crate::resources::placement_data::PlacementDataSsbo;
use crate::resources::prefab::{Prefab, PrefabLibrary};
use crate::resources::render::mesh_atlas_library::{MeshAtlasLibrary, PrefabName, PrefabNameRef};
use crate::resources::rt::{RtMeshHandle, RtWorld, color_to_u32};
use crate::utils_contract::Aabb3;

pub mod runtime;

/// 日志 target（O：`spawn_prefabs`/`create_prefab_gpu_instances` 的 `&str` 长 0x1d = 29）。
pub const LOG_TARGET: &str = "country_core::systems::prefab";

/// 预制体实例引用（权威定义在 `resources::prefab`）。
pub use crate::resources::prefab::PrefabInstance;

/// Existing transform projection of PlacementDataSsbo's first five floats.
/// GPU consumers retain the full placement record, including its id, custom
/// values, animation channels and bend vector.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PrefabTransform {
    /// `+0x00` 世界平移（O：`create_prefab_rt` 矩阵 `+0x30..0x3c` 的源）。
    pub translation: Vec3,
    /// `+0x0c` 均匀缩放（O：矩阵每元素乘数）。
    pub scale: f32,
    /// `+0x10` 绕 Z 旋转（弧度；O：`sinf/cosf(rot*0.5)`）。
    pub rotation: f32,
}

/// GPU 剔除输入（O：`create_prefab_gpu_instances` 0x140841b00 的 `base_cull`，16 B）：
/// 前 12 B 原样复制到输出 `InstanceCullData` 的 `position`，`+0xc` 的 **f32** 经
/// `half::binary16::f16::from_f32` 写入输出 `radius`（f16 @+0xc），随后
/// `+0xe` 被 `part.part_index`（`PrefabMeshAtlasPart+0` 的低 16 位）覆盖为 `mesh_index`。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CullInstanceSource {
    /// 输出 `InstanceCullData.position`（O：0..0xc 逐字节复制）。
    pub position: Vec3,
    /// 输出 `InstanceCullData.radius` 的 f32 源（O：`+0xc`，`f16::from_f32`）。
    pub radius: f32,
}

/// GPU 实例记录（O：`PrefabInstance +0x08` Vec 的元素，stride 0xc）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct GpuInstanceRecord {
    /// `+0x00`（O）。
    pub id: u32,
    /// `+0x04`（O）。
    pub index: u32,
    /// `+0x08`（O；`0x1b` = 终止哨兵）。
    pub kind: u8,
}

/// 终止哨兵（O：`despawn_prefab_contents` 0x140843390 的 `cmp byte[+8], 0x1b`）。
pub const GPU_INSTANCE_END: u8 = 0x1b;

impl GpuInstanceRecord {
    /// 转 `GpuWorld` 键；`kind` 越界（哨兵/未知）→ `None`。
    pub fn key(self) -> Option<InstanceKey> {
        GpuWorldSubsetKind::from_u8(self.kind).map(|kind| InstanceKey {
            id: self.id,
            index: self.index,
            kind,
        })
    }
}

/// 音频触发记录（O：`PrefabInstance +0x38` Vec 的元素，stride 0x28）。
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct AudioTriggerRecord {
    /// `+0x00` 触发盒（O：Aabb3 24 B）。
    pub shape: Aabb3,
    /// `+0x18` Option 句柄（O：原生判别 `2`=None、`0`=Some；`+0x20` slotmap 键）。
    pub handle: Option<AudioTriggerHandle>,
}

impl Default for AudioTriggerRecord {
    fn default() -> Self {
        Self {
            shape: Aabb3 {
                min: Vec3::ZERO,
                max: Vec3::ZERO,
            },
            handle: None,
        }
    }
}

/// Canonical B systems::prefab::PrefabInstanceData: mutable48B placement.
/// Original register_component140b7e440 writes size0x30 at140b7e6ed.
#[repr(transparent)]
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct PrefabInstanceData(pub PlacementDataSsbo);

/// 生成预制体命令（O：`systems::prefab::SpawnPrefabCmd`；`PrefabPlugin::build`
/// 0x140840430 注册）。
///
/// 载荷 **0x48B**（事件槽 stride 0x50 = EventInstance 头 8B + 0x48；fresh
/// process_tree_cmds Insert 0x142267430 行 683-719 / asm 142267e55-142267ec8）：
///
/// | 偏移 | 字段 |
/// |---|---|
/// | +0x00/+0x08 | `PrefabName`（Arc<str> ptr/len，16B） |
/// | +0x10 | 目标实体（Entity 8B） |
/// | +0x18/+0x20 | position x/y/z |
/// | +0x24 | scale f32 |
/// | +0x28 | rotation f32 |
/// | +0x2c | variant u32 |
/// | +0x30 | 常量 1.0f（`0x3f800000`） |
/// | +0x34 | flags `[u8; 0x10]`（原生 XORPS 零值） |
/// | +0x44 | mode u32 |
#[derive(Event, Clone, Debug)]
pub struct SpawnPrefabCmd {
    /// +0x00：预制体名（`Option` 仅为 Rust 构造默认值，恒 `Some`）。
    pub prefab_name: Option<PrefabName>,
    /// +0x10：由 `Commands::spawn_empty` 生成的实体。
    pub entity: Option<Entity>,
    /// +0x18：位置。
    pub position: Vec3,
    /// +0x24：缩放。
    pub scale: f32,
    /// +0x28：旋转。
    pub rotation: f32,
    /// +0x2c：变体。
    pub variant: u32,
    /// +0x30：常量 1.0。
    pub one: f32,
    /// +0x34：16B 标志区（原生零初始化）。
    pub flags: [u8; 0x10],
    /// +0x44：模式（naked/deny/drag 合成）。
    pub mode: u32,
}

impl Default for SpawnPrefabCmd {
    fn default() -> Self {
        Self {
            prefab_name: None,
            entity: None,
            position: Vec3::ZERO,
            scale: 0.0,
            rotation: 0.0,
            variant: 0,
            one: 1.0,
            flags: [0; 0x10],
            mode: 0,
        }
    }
}

impl SpawnPrefabCmd {
    /// B spawn_prefabs copies cmd+18..48 into the 0x30-byte placement
    /// component. Keep every active field and all packed channels, including
    /// mode@44; none of these bytes come from PlacementDataSsbo::default.
    pub fn placement(&self) -> PlacementDataSsbo {
        PlacementDataSsbo {
            position: self.position,
            scale: self.scale,
            rotation_angle: self.rotation,
            id: self.variant as i32,
            custom0: self.one.to_bits(),
            custom1: u32::from_le_bytes(self.flags[0..4].try_into().unwrap()),
            anim_t_packed: u32::from_le_bytes(self.flags[4..8].try_into().unwrap()),
            bend_vector: glam::Vec2::new(
                f32::from_le_bytes(self.flags[8..12].try_into().unwrap()),
                f32::from_le_bytes(self.flags[12..16].try_into().unwrap()),
            ),
            pad0: self.mode as i32,
        }
    }
}

/// 回收预制体命令（O：`systems::prefab::DespawnPrefabCmd`；载荷 `+0x08` = 目标实体）。
#[derive(Event, Clone, Debug, Default)]
pub struct DespawnPrefabCmd {
    /// 目标实体（O：`despawn_prefabs` 0x140842d60 读事件元素 `+0x08` 作 packed Entity）。
    pub entity: Option<Entity>,
    /// typed gap。
    pub _gap: u64,
}

/// Legacy slice adapter input. Production [`runtime::spawn_prefab_commands`]
/// reads SpawnPrefabCmd directly and preserves its complete placement.
#[derive(Clone, Debug)]
pub struct PrefabSpawnRequest {
    /// 预制体名（O：`PrefabLibrary::get` 的键）。
    pub prefab: PrefabName,
    /// Legacy field name: target Entity bits, used for same-frame deletion
    /// dedupe. Original PDB instantiates HashSet<bevy_ecs::entity::Entity>.
    pub global_id: u64,
    /// 实例变换。
    pub transform: PrefabTransform,
    /// GPU 剔除数据基值（O：`create_prefab_gpu_instances` 的 16 B `base_cull`）。
    pub cull: CullInstanceSource,
    /// `PlacementDataSsbo` 数据。
    pub placement: PlacementDataSsbo,
}

/// Legacy slice adapter output. Production inserts these owned contents and
/// the full placement on the command's Entity, plus any BirdHaven component.
#[derive(Clone, Debug)]
pub struct PrefabSpawnOutput {
    /// 去重键。
    pub global_id: u64,
    /// 预制体名。
    pub prefab: PrefabName,
    /// 实例数据。
    pub data: PrefabInstance,
}

/// `create_prefab_rt` resolved mesh output. The optional radius also requests
/// BirdHaven insertion using the source part's local bounds center.
#[derive(Clone, Debug, PartialEq)]
pub struct PrefabRtMeshUpdate {
    /// 颜色（O：`+0x58` u32，`sqrt(c)*255+0.5` 饱和打包，同 [`color_to_u32`]）。
    pub color: u32,
    /// 4×4 变换（O：`+0x00..0x40`，按下标逐项判等）。
    pub matrix: [f32; 16],
    /// 索引展开后的 cube 顶点（O：`+0x40` `Vec<Vec3>`，`cube mesh positions()` 后按索引映射）。
    pub positions: Vec<Vec3>,
    /// AABB 半径（O：kind bit0 时 `0.5 * min(extent)`；`None` = 不建触发）。
    pub radius: Option<f32>,
}

/// RT 变换矩阵（O：`create_prefab_rt` 0x140841550 / `update_prefabs` 0x140841d60 同一段
/// `sinf/cosf(rot*0.5)` 代码）。逐项对应原指令写序：
/// `[0]=s*t00, [1]=s*0, [2]=s*t01, [3]=s*t01, [4]=s*0, [5]=s*1, [6]=s*0, [7]=s*0,
///  [8]=s*t10, [9]=s*0, [10]=s*t00, [11]=s*t00, [12..16]=t.xyz,z`。
pub fn prefab_rt_matrix(transform: &PrefabTransform) -> [f32; 16] {
    let s = (transform.rotation * 0.5).sin();
    let c = (transform.rotation * 0.5).cos();
    // 原版以零分量参与运算（绕 Z 旋转）；保留变量以复现浮点表达式顺序。
    let z = s * 0.0;
    let two_s = s + s;
    let two_z = z + z;
    let zz = z * two_z;
    let zc = two_z * c;
    let t00 = 1.0 - (s * two_s + zz);
    let t01 = zz - two_s * c;
    let t02 = s * two_z + zc;
    let t10 = two_s * c + zz;
    let t11 = s * two_z - zc;
    let sc = transform.scale;
    let (tx, ty, tz) = (
        transform.translation.x,
        transform.translation.y,
        transform.translation.z,
    );
    [
        sc * t00,
        sc * (z * two_s + zc),
        sc * t01,
        sc * t01,
        sc * (z * two_s - zc),
        sc * (1.0 - (zz + zz)),
        sc * t02,
        sc * t02,
        sc * t10,
        sc * t11,
        sc * t00,
        sc * t00,
        tx,
        ty,
        tz,
        tz,
    ]
}

/// 网格 AABB（O：`create_prefab_rt` 0x140841550 直接读 part **+0x48..0x60** 的 Aabb3；
/// 这里保留从网格顶点重建的辅助函数供生产者写缓存，不在 create 链中使用）。
pub fn part_bounds(part_mesh: &Mesh) -> Aabb3 {
    Aabb3::from_points(part_mesh.positions().1.iter().copied())
}

/// `create_prefab_rt` 的 cube 顶点展开（O）：`CubeBox{[min.x,max.x],...}` →
/// `geometry::cube::cube_mesh` → `Mesh::positions()` → `indices.map(|i| positions[i])`。
pub fn rt_cube_positions(bounds: &Aabb3) -> Vec<Vec3> {
    let cube = CubeBox {
        x: [bounds.min.x, bounds.max.x],
        y: [bounds.min.y, bounds.max.y],
        z: [bounds.min.z, bounds.max.z],
    };
    let mesh = cube_mesh(&cube);
    let (indices, positions) = mesh.positions();
    indices
        .iter()
        .map(|&i| positions[i as usize])
        .collect::<Vec<_>>()
}

/// `create_prefab_rt`（O：0x140841550）：kind==2 跳过；否则计算颜色、矩阵、cube 顶点与
/// （bit0）AABB 半径。
///
/// 逐条对应（fresh/disasm）：
/// - `bVar2 = part+0x84`；`== 2` → 直接返回（不建 RtMesh）。
/// - 颜色：`sqrt(c)*255+0.5`，负→0、`> u32::MAX`→u32::MAX，打包 `r | g<<8 | b<<16`（O）。
/// - 矩阵：见 [`prefab_rt_matrix`]（`sinf/cosf` 半角，绕 Z）。
/// - cube：`geometry::cube::impl$5::from`（0x140a05530）吃 part+0x48 的 Aabb3（shuffle 成
///   `[x0,x1,y0,y1,z0,z1]`）；`Mesh::positions()`（0x140a07b30）后按索引映射（0x1415634c0）。
/// - `bVar2 & 1` → `Aabb3::center()`/`extent()`（part+0x48），半径 = `0.5 * min(extent.x,y,z)`
///   （NaN：`f32::min` 的非 NaN 语义）。
/// The native 0x48B Commands record invokes insert<BirdHaven> (callback
/// 1412dbe50), with local center/radius, state0, cooldown10 and flagfalse.
/// Production runtime applies that component write. `{2, handle}` is the
/// tag2 RtMeshHandle appended to PrefabInstance.rt_meshes, not an event.
pub fn create_prefab_rt(
    part: &crate::resources::prefab::PrefabPart,
    transform: &PrefabTransform,
    season: usize,
) -> Option<PrefabRtMeshUpdate> {
    if part.rt_kind == 2 {
        return None;
    }
    let albedo = part.albedo_for_season(season);
    let radius = if part.rt_kind & 1 != 0 {
        let e = part.bounds.extent();
        Some(e.x.min(e.y).min(e.z) * 0.5)
    } else {
        None
    };
    Some(PrefabRtMeshUpdate {
        color: color_to_u32(albedo),
        matrix: prefab_rt_matrix(transform),
        positions: rt_cube_positions(&part.bounds),
        radius,
    })
}

/// `create_prefab_gpu_instances`（O：0x140841b00）：
/// `MeshAtlasLibrary::get_by_name<&PrefabNameRef>` → 每个 `PrefabMeshAtlasPart{part_index,subset}`
/// 调一次 `GpuWorld::create_instances<PlacementDataSsbo>`。逐 part 的剔除记录（O disasm）：
/// 前 12 B 复制 `base_cull.position`，`radius = f16::from_f32(base_cull.radius)`（+0xc），
/// `mesh_index = part_index`（低 16 位写 +0xe），`kind = part.subset`（`[part+4]`）。
///
/// 日志（O）：`Trace` 级 `"Creating {} parts for prefab {:?}"`（target [`LOG_TARGET`]）。
/// 返回新压入 `pending` 的 GPU 键数。
pub fn create_prefab_gpu_instances(
    gpu_world: &mut GpuWorld,
    mesh_atlases: &MeshAtlasLibrary,
    name: &PrefabNameRef,
    base_cull: CullInstanceSource,
    data: PlacementDataSsbo,
    pending: &mut Vec<(u32, u32, u8)>,
) -> usize {
    let parts = mesh_atlases.get_by_name(name);
    if log::max_level() >= log::LevelFilter::Trace {
        log::trace!(
            target: LOG_TARGET,
            "Creating {} parts for prefab {:?}",
            parts.len(),
            name
        );
    }
    for part in parts {
        let cull = InstanceCullData {
            position: base_cull.position,
            radius: half::f16::from_f32(base_cull.radius),
            mesh_index: part.part_index as u16,
        };
        gpu_world.create_instances(part.subset, &[cull], &[data], pending);
    }
    parts.len()
}

/// 构造一条实例的完整内容（spawn 0x140840610 与 hot_reload 0x140843740 共用的段）。
///
/// 顺序（O）：`create_prefab_gpu_instances` → 遍历 `prefab.parts`（stride 0x98）：
/// `Option` 音频触发（kind 门：`tag==4` 且名以 `"Tree"` 开头，否则 panic）→
/// `Aabb3::scaled(splat(scale))` → `translated(translation)` → `AudioTriggerLibrary::add_trigger`；
/// 之后每 part 调 `create_prefab_rt`（kind != 2）：从 `RtWorld` 分配 tag-2 句柄并
/// Writes the actual RT mesh and records its tag2 handle in the contents.
/// Returned `(handle, radius)` entries identify odd-rt_kind parts for the
/// production runtime's real BirdHaven component insertion; no RT work queue
/// is needed. The handle retains the part order/association for its center.
#[allow(clippy::too_many_arguments)]
fn build_prefab_contents(
    data: &mut PrefabInstance,
    prefab: &Prefab,
    transform: &PrefabTransform,
    mesh_atlases: &MeshAtlasLibrary,
    gpu_world: &mut GpuWorld,
    rt_world: &mut RtWorld,
    audio: &mut AudioTriggerLibrary,
    pending: &mut Vec<(u32, u32, u8)>,
    season: usize,
    cull: CullInstanceSource,
    placement: PlacementDataSsbo,
) -> Vec<(RtMeshHandle, f32)> {
    let start = pending.len();
    create_prefab_gpu_instances(
        gpu_world,
        mesh_atlases,
        data.prefab_name.as_name_ref(),
        cull,
        placement,
        pending,
    );
    data.gpu_instances.extend(
        pending[start..]
            .iter()
            .map(|(id, index, kind)| GpuInstanceRecord {
                id: *id,
                index: *index,
                kind: *kind,
            }),
    );

    let mut rt_commands = Vec::new();
    for part in &prefab.parts {
        if let Some(trigger) = &part.audio_trigger {
            if !trigger.is_tree() {
                panic!(
                    "Unknown audio trigger kind {} in prefab {:?}",
                    trigger.kind_name,
                    data.prefab_name.as_name_ref()
                );
            }
            let shape = trigger
                .bounds
                .scaled(Vec3::splat(transform.scale))
                .translated(transform.translation);
            let handle = audio.add_trigger(AudioTriggerKind::Primary, shape);
            data.audio_triggers.push(AudioTriggerRecord {
                // Native stores part's LOCAL bounds beside the handle;
                // update_prefabs applies the placement exactly once again.
                shape: trigger.bounds,
                handle: Some(handle),
            });
        }
        if let Some(update) = create_prefab_rt(part, transform, season) {
            let handle = rt_world.alloc_prefab_handle();
            rt_world.write_prefab_mesh(handle, update.matrix, update.positions, update.color);
            data.rt_meshes.push(handle);
            if let Some(radius) = update.radius {
                rt_commands.push((handle, radius));
            }
        }
    }
    rt_commands
}

/// Legacy slice adapter for the resolved spawn algorithm. It returns owned
/// contents to callers; the production EventReader/Commands path is
/// [`runtime::spawn_prefab_commands`], which also applies BirdHaven insertion.
///
/// 去重（O）：target Entity bits 已在 `issued` 中 → `Warn` 级
/// `"Issued creation of a prefab {:?} on the same frame as issuing its deletion!"` 并跳过。
/// `issued` contains same-frame deleted Entity bits. It is not an existing
/// instance registry or a second game-ID namespace.
#[allow(clippy::too_many_arguments)]
pub fn spawn_prefabs(
    requests: &[PrefabSpawnRequest],
    library: &PrefabLibrary,
    issued: &HashSet<u64>,
    mesh_atlases: &MeshAtlasLibrary,
    gpu_world: &mut GpuWorld,
    rt_world: &mut RtWorld,
    audio: &mut AudioTriggerLibrary,
    pending: &mut Vec<(u32, u32, u8)>,
    season: usize,
) -> Vec<PrefabSpawnOutput> {
    let mut out = Vec::new();
    for request in requests {
        if issued.contains(&request.global_id) {
            log::warn!(
                target: LOG_TARGET,
                "Issued creation of a prefab {:?} on the same frame as issuing its deletion!",
                request.global_id
            );
            continue;
        }
        let prefab = library.get(&request.prefab);
        let mut data = PrefabInstance {
            prefab_name: request.prefab.clone(),
            ..PrefabInstance::default()
        };
        let _rt_commands = build_prefab_contents(
            &mut data,
            prefab,
            &request.transform,
            mesh_atlases,
            gpu_world,
            rt_world,
            audio,
            pending,
            season,
            request.cull,
            request.placement,
        );
        out.push(PrefabSpawnOutput {
            global_id: request.global_id,
            prefab: request.prefab.clone(),
            data,
        });
    }
    out
}

/// `update_prefabs`（O：0x140841d60）单实例核心。
///
/// 逐条（fresh/disasm）：
/// 1. GPU Vec：每个 `GpuInstanceRecord` → `GpuWorld::owned_subset_mut(kind)`
///    `update_instance_data::<PlacementDataSsbo>`（位置/半径来自实例变换，写 `placement`）。
/// 2. RT Vec：对每个 `RtMeshHandle` 调 `RtWorld::get`，重算 [`prefab_rt_matrix`] 并比较
///    **`matrix[4..16]`**（反汇编只比较 `+0x10..0x4c`）；不同才 `DirtyCow::get_mut` 并写全 16 项。
///    返回本次真正写过的句柄。
/// 3. 音频 Vec：`Aabb3::scaled(splat(scale))` + `translated(translation)` →
///    `AudioTriggerLibrary::update_trigger_shape`；键失效 → `unwrap_failed` panic（O）。
pub fn update_prefabs_instance(
    data: &PrefabInstance,
    transform: &PrefabTransform,
    gpu_world: &mut GpuWorld,
    rt_world: &mut RtWorld,
    audio: &mut AudioTriggerLibrary,
    placement: PlacementDataSsbo,
) -> Vec<RtMeshHandle> {
    for record in &data.gpu_instances {
        if let Some(mut key) = record.key() {
            gpu_world.owned_subset_mut(key.kind).update_instance_data(
                &mut key,
                transform.translation,
                transform.scale,
                placement,
            );
        }
    }
    let matrix = prefab_rt_matrix(transform);
    let mut updated = Vec::new();
    for handle in &data.rt_meshes {
        let changed = rt_world
            .get(*handle)
            .is_some_and(|mesh| mesh.matrix_tail_differs(&matrix));
        if changed {
            rt_world.get_mut(*handle).set_matrix(matrix);
            updated.push(*handle);
        }
    }
    for record in &data.audio_triggers {
        let Some(handle) = record.handle else {
            continue;
        };
        let shape = record
            .shape
            .scaled(Vec3::splat(transform.scale))
            .translated(transform.translation);
        if !audio.update_trigger_shape(AudioTriggerKind::Primary, handle, shape) {
            panic!("called `Option::unwrap()` on a `None` value");
        }
    }
    updated
}

/// `despawn_prefab_contents`（O：0x140843390）：回收实例的三组内容。
///
/// 逐条（fresh）：
/// - GPU Vec：遇 `kind == 0x1b` **停止**（`break`，不是 continue），否则
///   `GpuWorld::remove_instance(&mut key)`；原生对 mismatch 情形用 `local_a8 != 3` 控制 Arc 释放
///   与 +0x38 脏位（外部资源 seam，登记）。
/// - RT Vec：`RawTable::remove_entry<RtMeshHandle, DirtyCow<RtMesh>>`（= `HashMap::remove`）并释放
///   Arc；脏位写 `+0x38`（外部资源 seam，登记）。
/// - 音频 Vec：原生槽 Option 判别（`2`=None → `return`；`0`=Some → `remove`；其它 → panic）。
#[allow(clippy::too_many_arguments)]
pub fn despawn_prefab_contents(
    data: &mut PrefabInstance,
    gpu_world: &GpuWorld,
    rt_world: &mut RtWorld,
    audio: &mut AudioTriggerLibrary,
) {
    for record in data.gpu_instances.drain(..) {
        if record.kind == GPU_INSTANCE_END {
            break;
        }
        if let Some(mut key) = record.key() {
            gpu_world.remove_instance(&mut key);
        }
    }
    for handle in data.rt_meshes.drain(..) {
        let _removed = rt_world.remove_mesh(&handle);
    }
    for record in data.audio_triggers.drain(..) {
        let Some(handle) = record.handle else {
            break;
        };
        audio.remove_trigger(AudioTriggerKind::Primary, handle);
    }
}

/// `despawn_prefabs`（O：0x140842d60）：消费 `DespawnPrefabCmd`，逐实体回收内容并 `despawn()`。
///
/// 原生在命中 `PrefabInstance` 时先写外部计数/GPU 数据再调 contents（计数资源 seam）；
/// 实体查找用 archetype/table 位图（= `Query<&mut PrefabInstance>::get_mut`）。
pub fn despawn_prefabs(
    mut commands: Commands,
    mut events: EventReader<DespawnPrefabCmd>,
    mut query: Query<&mut PrefabInstance>,
    gpu_world: &GpuWorld,
    rt_world: &mut RtWorld,
    audio: &mut AudioTriggerLibrary,
) {
    for cmd in events.read() {
        let Some(entity) = cmd.entity else {
            continue;
        };
        if let Ok(mut data) = query.get_mut(entity) {
            despawn_prefab_contents(&mut data, gpu_world, rt_world, audio);
        }
        commands.entity(entity).despawn();
    }
}

/// `hot_reload_prefabs`（O：0x140843740）核心。
///
/// 前置（O）：两个事件迭代器先收集 `HashSet<PrefabName>`（克隆名）与
/// `HashSet<GpuWorldSubsetKind>`（拷贝）。随后对每个实例：
/// 名称在集合中 **或** 任一 GPU 记录的 subset 在集合中 → `despawn_prefab_contents` →
/// `PrefabLibrary::get` → 重建内容（与 spawn 同一段）。
/// 返回被重建的实体列表。
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn hot_reload_prefabs(
    instances: &mut [(Entity, PlacementDataSsbo, PrefabInstance)],
    modified_prefabs: &HashSet<PrefabName>,
    modified_subsets: &HashSet<GpuWorldSubsetKind>,
    library: &PrefabLibrary,
    mesh_atlases: &MeshAtlasLibrary,
    gpu_world: &mut GpuWorld,
    rt_world: &mut RtWorld,
    audio: &mut AudioTriggerLibrary,
    pending: &mut Vec<(u32, u32, u8)>,
    season: usize,
) -> Vec<Entity> {
    let mut reloaded = Vec::new();
    for (entity, placement, data) in instances.iter_mut() {
        let name_hit = modified_prefabs.contains(&data.prefab_name);
        let subset_hit = data.gpu_instances.iter().any(|record| {
            GpuWorldSubsetKind::from_u8(record.kind)
                .is_some_and(|kind| modified_subsets.contains(&kind))
        });
        if !name_hit && !subset_hit {
            continue;
        }
        despawn_prefab_contents(data, gpu_world, rt_world, audio);
        // 配置已删除时只清理旧内容，保留引用实体供资源重新出现后重建。
        let Some(prefab) = library.inner.get(&data.prefab_name) else {
            reloaded.push(*entity);
            continue;
        };
        // B @140843ed5..ee4 selects the actual 48-byte component; @140844174
        // forwards that same pointer to create_prefab_gpu_instances. Its first
        // sixteen bytes also supply the cull position and scale/radius.
        let transform = PrefabTransform {
            translation: placement.position,
            scale: placement.scale,
            rotation: placement.rotation_angle,
        };
        let cull = CullInstanceSource {
            position: placement.position,
            radius: placement.scale,
        };
        let _rt_commands = build_prefab_contents(
            data,
            prefab,
            &transform,
            mesh_atlases,
            gpu_world,
            rt_world,
            audio,
            pending,
            season,
            cull,
            *placement,
        );
        reloaded.push(*entity);
    }
    reloaded
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::nani::gpu_world::{GpuWorldSubset, OwnedGpuWorldSubset};
    use crate::render::test_device::TestDevice;
    use crate::resources::prefab::{PrefabAudioTrigger, PrefabPart};

    fn test_placement() -> PlacementDataSsbo {
        PlacementDataSsbo {
            position: Vec3::new(1.0, 2.0, 3.0),
            scale: 2.0,
            rotation_angle: 0.75,
            id: -7,
            custom0: 0x0102_0304,
            custom1: 0x0506_0708,
            anim_t_packed: 0x3c00_3800,
            bend_vector: glam::Vec2::new(0.5, -0.25),
            pad0: -6,
        }
    }

    fn approx(a: [f32; 16], b: [f32; 16]) {
        for (x, y) in a.iter().zip(b.iter()) {
            assert!((x - y).abs() < 1e-6, "{a:?} != {b:?}");
        }
    }

    fn test_world(kind: GpuWorldSubsetKind, mesh_part_count: usize) -> GpuWorld {
        let dev = TestDevice(1);
        let mut world = GpuWorld::new(&dev);
        world.assign_subset(
            kind,
            GpuWorldSubset::Owned(OwnedGpuWorldSubset::new::<PlacementDataSsbo>(&dev, kind, mesh_part_count)),
        );
        world
    }

    fn test_atlas(name: &str, kind: GpuWorldSubsetKind) -> MeshAtlasLibrary {
        let mut atlases = MeshAtlasLibrary::default();
        let mut mesh = Mesh::new("test");
        mesh.set_indices(vec![0, 1, 2]);
        atlases.add(kind, PrefabName::new(name), &mesh);
        atlases
    }

    #[test]
    fn rt_matrix_is_z_rotation_scaled_with_duplicated_lane() {
        let t = PrefabTransform {
            translation: Vec3::new(1.0, 2.0, 3.0),
            scale: 2.0,
            rotation: std::f32::consts::FRAC_PI_2,
        };
        let s = (t.rotation * 0.5).sin();
        let c = (t.rotation * 0.5).cos();
        let t00 = 1.0 - 2.0 * s * s;
        let t01 = -2.0 * s * c;
        let t10 = 2.0 * s * c;
        approx(
            prefab_rt_matrix(&t),
            [
                2.0 * t00,
                0.0,
                2.0 * t01,
                2.0 * t01,
                0.0,
                2.0,
                0.0,
                0.0,
                2.0 * t10,
                0.0,
                2.0 * t00,
                2.0 * t00,
                1.0,
                2.0,
                3.0,
                3.0,
            ],
        );
    }

    #[test]
    fn create_prefab_rt_skips_kind_two_and_packs_color() {
        let mut part = PrefabPart::default();
        part.rt_kind = 2;
        assert!(create_prefab_rt(&part, &PrefabTransform::default(), 0).is_none());

        part.rt_kind = 0;
        part.albedos = [Vec3::new(1.0, 0.0, 1.0), Vec3::ZERO, Vec3::ZERO];
        let out = create_prefab_rt(&part, &PrefabTransform::default(), 0).unwrap();
        assert_eq!(out.color, 0x00ff00ff);
        assert!(out.radius.is_none());
    }

    #[test]
    fn create_prefab_rt_aabb_radius_is_half_min_extent() {
        let part = PrefabPart {
            rt_kind: 1,
            bounds: Aabb3 {
                min: Vec3::ZERO,
                max: Vec3::new(2.0, 4.0, 8.0),
            },
            ..PrefabPart::default()
        };
        let out = create_prefab_rt(&part, &PrefabTransform::default(), 0).unwrap();
        assert_eq!(out.radius, Some(1.0)); // min extent = 2 → 半径 1
        // O：cube mesh 24 顶点、36 索引进 RT 顶点表。
        assert_eq!(out.positions.len(), 36);
    }

    #[test]
    fn rt_cube_positions_expand_the_box_by_cube_indices() {
        let bounds = Aabb3 {
            min: Vec3::new(-1.0, -2.0, -3.0),
            max: Vec3::new(1.0, 2.0, 3.0),
        };
        let positions = rt_cube_positions(&bounds);
        assert_eq!(positions.len(), 36);
        assert!(positions.iter().all(|p| {
            p.x >= -1.0 && p.x <= 1.0 && p.y >= -2.0 && p.y <= 2.0 && p.z >= -3.0 && p.z <= 3.0
        }));
    }

    #[test]
    fn audio_trigger_kind_gate_requires_exact_tree_string() {
        let bounds = Aabb3 {
            min: Vec3::ZERO,
            max: Vec3::ONE,
        };
        for (kind, accepted) in [
            ("Tree", true),
            ("TreeX", false),
            ("Tre", false),
            ("Bush", false),
        ] {
            let trigger = PrefabAudioTrigger {
                kind_name: kind.into(),
                bounds,
            };
            assert_eq!(trigger.is_tree(), accepted);
        }
    }

    #[test]
    fn despawn_prefab_contents_stops_at_gpu_end_sentinel() {
        let gpu_world = test_world(GpuWorldSubsetKind::Canopy, 0);
        let mut rt_world = RtWorld::default();
        let mut audio = AudioTriggerLibrary::default();
        let mut data = PrefabInstance {
            gpu_instances: vec![
                GpuInstanceRecord {
                    id: 0,
                    index: 0,
                    kind: GpuWorldSubsetKind::Canopy as u8,
                },
                GpuInstanceRecord {
                    id: 1,
                    index: 0,
                    kind: GPU_INSTANCE_END,
                },
                GpuInstanceRecord {
                    id: 2,
                    index: 0,
                    kind: GpuWorldSubsetKind::Plant as u8,
                },
            ],
            ..PrefabInstance::default()
        };
        despawn_prefab_contents(&mut data, &gpu_world, &mut rt_world, &mut audio);
        // 哨兵之后（id=2）不被访问；Vec 整体 drain。
        assert!(data.gpu_instances.is_empty());
    }

    #[test]
    fn spawn_dedupes_issued_global_ids_and_returns_contents() {
        let mut library = PrefabLibrary::default();
        library.add(
            PrefabName::new("tree/a"),
            Prefab {
                parts: vec![PrefabPart::default()],
            },
        );
        let kind = GpuWorldSubsetKind::Canopy;
        let mesh_atlases = test_atlas("tree/a", kind);
        let mut gpu_world = test_world(kind, mesh_atlases.get_by_world_subset(kind).atlas.layouts.len());
        let mut rt_world = RtWorld::default();
        let mut audio = AudioTriggerLibrary::default();
        let mut pending = Vec::new();
        let requests = vec![
            PrefabSpawnRequest {
                prefab: PrefabName::new("tree/a"),
                global_id: 7,
                transform: PrefabTransform::default(),
                cull: CullInstanceSource::default(),
                placement: PlacementDataSsbo {
                    position: Vec3::ZERO,
                    scale: 1.0,
                    ..Default::default()
                },
            },
            PrefabSpawnRequest {
                prefab: PrefabName::new("tree/a"),
                global_id: 7,
                transform: PrefabTransform::default(),
                cull: CullInstanceSource::default(),
                placement: PlacementDataSsbo {
                    position: Vec3::ZERO,
                    scale: 1.0,
                    ..Default::default()
                },
            },
        ];
        let issued = HashSet::new();
        let out = spawn_prefabs(
            &requests,
            &library,
            &issued,
            &mesh_atlases,
            &mut gpu_world,
            &mut rt_world,
            &mut audio,
            &mut pending,
            0,
        );
        // Empty same-frame deletion set permits both creation commands.
        assert_eq!(out.len(), 2);
        assert_eq!(&*out[0].prefab.0, "tree/a");
        assert_eq!(&*out[0].data.prefab_name.0, "tree/a");
        // atlas 给了一条 part → 一次 GPU 实例；默认 part rt_kind=0 → 建 RT 句柄。
        assert_eq!(out[0].data.gpu_instances.len(), 1);
        assert_eq!(out[0].data.rt_meshes.len(), 1);
        assert_eq!(rt_world.next_prefab_handle, 2);
        assert_eq!(out[0].data.rt_meshes[0].tag, 2);
        assert!(rt_world.get(out[0].data.rt_meshes[0]).is_some());

        // Entity bits in the same-frame deletion set → warn and skip.
        let mut issued = HashSet::new();
        issued.insert(7u64);
        let out = spawn_prefabs(
            &requests[..1],
            &library,
            &issued,
            &mesh_atlases,
            &mut gpu_world,
            &mut rt_world,
            &mut audio,
            &mut pending,
            0,
        );
        assert!(out.is_empty());
    }

    #[test]
    fn hot_reload_rebuilds_only_affected_instances() {
        let mut library = PrefabLibrary::default();
        library.add(
            PrefabName::new("tree/a"),
            Prefab {
                parts: vec![PrefabPart::default()],
            },
        );
        library.add(
            PrefabName::new("tree/b"),
            Prefab {
                parts: vec![PrefabPart::default()],
            },
        );
        let kind = GpuWorldSubsetKind::Canopy;
        let mut mesh_atlases = test_atlas("tree/a", kind);
        let mut mesh_b = Mesh::new("test");
        mesh_b.set_indices(vec![0, 1, 2]);
        mesh_atlases.add(kind, PrefabName::new("tree/b"), &mesh_b);
        let mut gpu_world = test_world(kind, mesh_atlases.get_by_world_subset(kind).atlas.layouts.len());
        let mut rt_world = RtWorld::default();
        let mut audio = AudioTriggerLibrary::default();
        let mut pending = Vec::new();
        let mut modified = HashSet::new();
        modified.insert(PrefabName::new("tree/a"));
        let mut instances = vec![
            (
                Entity::from_raw(1),
                test_placement(),
                PrefabInstance {
                    prefab_name: PrefabName::new("tree/a"),
                    ..PrefabInstance::default()
                },
            ),
            (
                Entity::from_raw(2),
                test_placement(),
                PrefabInstance {
                    prefab_name: PrefabName::new("tree/b"),
                    ..PrefabInstance::default()
                },
            ),
        ];
        let reloaded = hot_reload_prefabs(
            &mut instances,
            &modified,
            &HashSet::new(),
            &library,
            &mesh_atlases,
            &mut gpu_world,
            &mut rt_world,
            &mut audio,
            &mut pending,
            0,
        );
        assert_eq!(reloaded, vec![Entity::from_raw(1)]);
        assert_eq!(instances[0].2.rt_meshes.len(), 1);
        assert!(instances[1].2.rt_meshes.is_empty());
        let subset = gpu_world.owned_subset_mut(kind);
        let data = subset
            .data()
            .as_any()
            .downcast_ref::<crate::render::nani::gpu_world::InstanceDataBuffer<PlacementDataSsbo>>()
            .unwrap();
        assert_eq!(
            bytemuck::bytes_of(&data.shadow.data[0]),
            bytemuck::bytes_of(&instances[0].1),
            "reload must preserve all 48 placement bytes, including animation/custom fields"
        );
        assert_eq!(subset.positions()[0].position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(subset.positions()[0].radius.to_f32(), 2.0);
        let mesh = rt_world.get(instances[0].2.rt_meshes[0]).unwrap();
        assert_eq!(&mesh.matrix[12..16], &[1.0, 2.0, 3.0, 3.0]);
    }

    #[test]
    fn update_prefabs_updates_audio_shape_and_reports_rt_matrix() {
        let mut audio = AudioTriggerLibrary::default();
        let audio_handle = audio.add_trigger(
            AudioTriggerKind::Primary,
            Aabb3 {
                min: Vec3::ZERO,
                max: Vec3::ONE,
            },
        );
        let dev = TestDevice(1);
        let mut gpu_world = GpuWorld::new(&dev);
        let rt_handle = RtMeshHandle::from_handle(3);
        let mut rt_world = RtWorld::default();
        let _ = rt_world.get_mut(rt_handle); // 默认单位矩阵
        let data = PrefabInstance {
            audio_triggers: vec![AudioTriggerRecord {
                shape: Aabb3 {
                    min: Vec3::ZERO,
                    max: Vec3::ONE,
                },
                handle: Some(audio_handle),
            }],
            rt_meshes: vec![rt_handle],
            ..PrefabInstance::default()
        };
        let t = PrefabTransform {
            translation: Vec3::new(1.0, 1.0, 1.0),
            scale: 2.0,
            rotation: 0.0,
        };
        let updated = update_prefabs_instance(
            &data,
            &t,
            &mut gpu_world,
            &mut rt_world,
            &mut audio,
            PlacementDataSsbo {
                position: Vec3::ZERO,
                scale: 1.0,
                ..Default::default()
            },
        );
        assert_eq!(updated, vec![rt_handle]);
        assert_eq!(rt_world.get(rt_handle).unwrap().matrix[0], 2.0);
        assert_eq!(rt_world.get(rt_handle).unwrap().matrix[12], 1.0);
        // 第二次同变换：矩阵不变量 `[4..16]` 已一致 → 不重复写。
        let updated = update_prefabs_instance(
            &data,
            &t,
            &mut gpu_world,
            &mut rt_world,
            &mut audio,
            PlacementDataSsbo {
                position: Vec3::ZERO,
                scale: 1.0,
                ..Default::default()
            },
        );
        assert!(updated.is_empty());
        let shape = audio
            .get_triggers(AudioTriggerKind::Primary)
            .triggers
            .get(audio_handle)
            .copied()
            .unwrap();
        assert_eq!(shape.min, Vec3::splat(1.0));
        assert_eq!(shape.max, Vec3::splat(3.0));
    }

    #[test]
    fn despawn_prefabs_command_payload_roundtrip() {
        let cmd = DespawnPrefabCmd {
            entity: Some(Entity::from_raw(9)),
            _gap: 0,
        };
        assert_eq!(cmd.entity, Some(Entity::from_raw(9)));
    }

    #[test]
    fn prefab_rt_update_uses_vec4_lanes() {
        let t = PrefabTransform {
            scale: 1.0,
            ..PrefabTransform::default()
        };
        let m = prefab_rt_matrix(&t);
        // 单位变换：对角线 1，末尾平移 0。
        approx(
            m,
            [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            ],
        );
    }

    #[test]
    fn gpu_instance_end_sentinel_matches_kind_count() {
        assert_eq!(GPU_INSTANCE_END as usize, GpuWorldSubsetKind::ALL.len());
        assert!(GpuWorldSubsetKind::from_u8(GPU_INSTANCE_END).is_none());
    }
}
