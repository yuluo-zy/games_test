//! gpu_world.rs — Nani GPU-driven 实例世界的 CPU 侧管理（WP3 主件）。
//!
//! 机器码锚点（B patched EXE cf048e27...）：
//! - `GpuWorld::new` 0x1409917d0；`owned_subset_mut` 0x140991c50；`update_instance_mesh` 0x140991d60。
//! - `GpuWorld::remove_instance` 0x140991f60；`import_to_rg`/`export_from_rg` 0x1409921c0/0x140992220。
//! - `OwnedGpuWorldSubset::extend_no_data` 0x140992280；`GpuWorld::create_instances` 0x14156d0d0。
//! - `OwnedGpuWorldSubset::update_instance_data<T>` 0x14156db40 系（×28）；`impl$14::copy` 0x1415724f0 系。
//! - `impl$36::register_required_components` 0x141572d80 系；`impl$44::fmt`（GpuWorldSubsetKind Debug）0x140a241d0。
//!
//! 证据 bundle：`skeleton/B/crates/country-core/src/render/nani/gpu_world.rs.{pdb.json,structures.json.gz,enriched.json.gz,dependencies.json}`
//! 源 SHA256 `26c9dbcabf908aabb9393e4f00cc02e97cf43dba093a2adc3414083868809051`；158 procedure_instances。
//! 完整业务契约见 contracts/gpu_world.md（7 个子功能 A–G）。

#![allow(dead_code, unused_imports)]

use alloc::sync::Arc;
use core::any::Any;
use core::fmt;
use core::sync::atomic::{AtomicU64, Ordering};

use bevy_ecs::component::Component;
use bevy_ecs::reflect::Reflect;
use parking_lot::RwLock;

use crate::render::nani::buffers::{nani_calculate_draw_list_params, DrawListParams};
use crate::render::nani::instance_data::NaniInstanceData;
use crate::render::nani::mesh_atlas::{MeshAtlasPart, NaniMeshAtlasPartInfo};
use crate::render::sparse_instance_buffer::ChunkId;
use crate::render::ssbo::TypedStorageBuffer;
use crate::utils::index_condenser::IndexCondenser;

/// 每实例数据类型的 ECS marker 组件（`register_required_components` 注册；O）。
/// PDB 证据：`bevy_ecs::component::ComponentsRegistrator::register_component<InstanceDataMarker<D>>`。
pub struct InstanceDataMarker<D> {
    _marker: core::marker::PhantomData<D>,
}

/// 实例句柄（O）：kind 字节 + 实例索引。
/// `create_instances` 的 pending `Vec<(u32, u32, u8)>` 与 `remove_instance` 的 `*(key+8)` 编码。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InstanceHandle {
    pub kind: GpuWorldSubsetKind,
    pub index: u32,
}

/// 实例键（16B，O）：`{ chunk_id: u64, instance_index: u32, sort_key: f32 }`。
/// `update_instance_data` 把 sort_key 以 f16 写入紧凑 key 数组（`half::f16::from_f32`）。
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstanceKey {
    pub chunk_id: u64,
    pub instance_index: u32,
    /// 排序键（GPU draw-list 排序用；GPU 侧以 f16 存储）。
    pub sort_key: f32,
}

/// subset 种类（u8 判别；**27 个变体**，O——`import_to_rg` 27 槽 / `default_value` switch 上限 0x1b）。
/// 前 9 个变体名与 cull_and_bucket shader 家族对应；其余 18 个为 typed gap。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GpuWorldSubsetKind {
    Main = 0,
    Bush = 1,
    CurvedHalftimber = 2,
    Flowers = 3,
    Plaster = 4,
    RoofTile = 5,
    Smoke = 6,
    StairsFloor = 7,
    WoodenPlank = 8,
    // 18 个未命名变体（typed gap）：变体 9..=26 语义 U（PDB 仅给方法证据，无变体名）。
    Unknown9 = 9,
    Unknown10 = 10,
    Unknown11 = 11,
    Unknown12 = 12,
    Unknown13 = 13,
    Unknown14 = 14,
    Unknown15 = 15,
    Unknown16 = 16,
    Unknown17 = 17,
    Unknown18 = 18,
    Unknown19 = 19,
    Unknown20 = 20,
    Unknown21 = 21,
    Unknown22 = 22,
    Unknown23 = 23,
    Unknown24 = 24,
    Unknown25 = 25,
    Unknown26 = 26,
}

impl GpuWorldSubsetKind {
    /// shader 名（对应 `_nani_cull_and_bucket{...}` 变体；I）。
    pub fn shader(self) -> &'static str {
        match self {
            Self::Main => "main",
            Self::Bush => "bush",
            Self::CurvedHalftimber => "curved_halftimber",
            Self::Flowers => "flowers",
            Self::Plaster => "plaster",
            Self::RoofTile => "roof_tile",
            Self::Smoke => "smoke",
            Self::StairsFloor => "stairs_floor",
            Self::WoodenPlank => "wooden_plank",
            _ => "unknown", // typed gap
        }
    }

    /// RON 字符串 → kind（`_::impl$0::deserialize::visit_str<ron::Error>` 0x141572870）。
    pub fn try_from_shader(_s: &str) -> Result<Self, ron::Error> {
        // 反编译（1290B）显示匹配 → 变体；不匹配 → Error::Message。
        todo!("从 shader 名匹配 27 变体；未恢复")
    }

    /// 每 kind 最大实例数（`DAT_142aba790`/`DAT_142c77010[kind]` 常量表；数值表未读内存）。
    pub fn max_instances(self) -> usize {
        todo!("读 per-kind 常量表；见 contracts/buffers.md §5")
    }

    /// 每 kind 每 draw-list 最大 draw 数（`DAT_142aa14c0[kind]`；见 contracts/buffers.md §2）。
    pub fn max_draws_per_list(self) -> u32 {
        todo!("读 per-kind 常量表")
    }

    /// 是否使用细粒度排序（O）：kind < 2 或 `NANI_FORCE_FINE_GRAINED_SORTING` 强制时 true。
    pub fn use_fine_grained_sorting(self) -> bool {
        if crate::render::nani::NANI_FORCE_FINE_GRAINED_SORTING {
            return true;
        }
        (self as u8) < 2 && !crate::render::nani::NANI_FORCE_NO_FINE_GRAINED_SORTING
    }

    /// draw-list 索引（内联；draw_list_index 4 处 inline 证据）。
    pub fn draw_list_index(self) -> usize {
        self as usize
    }

    /// subset 描述（`gpu_world_subset_desc::{owned, external, owned_custom}`；行内联）。
    pub fn gpu_world_subset_desc(self) -> GpuWorldSubsetDesc {
        // owned 为带 key HashMap 的完全 subset；external 无 HashMap（`[6]==0` 跳过查找）。
        todo!("owned/external/owned_custom 描述；未恢复具体布局")
    }
}

/// subset 描述（尺寸/布局；typed gap——字段 U）。
pub struct GpuWorldSubsetDesc {
    pub owned: bool,
}

/// subset 枚举：owned（稀疏实例仓库）或 external（外部缓冲）。
/// trait 对象 {data_ptr, vtable}；vtable 方法：+0x18 type-id、+0x28 lock/数据、+0x40 import_to_rg、+0x50 remove。
pub enum GpuWorldSubset {
    Owned(OwnedGpuWorldSubset),
    External(ExternalGpuWorldSubset),
}

/// Owned subset 布局（u64 单位索引，O；见 contracts/gpu_world.md §0）：
/// - [0]/[1]/[2] `Vec<ChunkId>`（IndexCondenser 后端）
/// - [3]/[4]/[6] `HashMap<InstanceKey, u32>`
/// - [9]/[10] `AtomicU64` min/max
/// - [0xb]/[0xc] `Vec<InstanceKey>`（紧凑 key，sort_key 存 f16）
/// - [0x12]/[0x13] 类型化实例数据缓冲 trait 对象 {ptr, vtable}
/// - [0x14] instance_count；[0x16] next_instance_index
pub struct OwnedGpuWorldSubset {
    /// IndexCondenser<InstanceKey>：chunk_id 向量 + 索引压缩（O）。
    condenser: IndexCondenser<InstanceKey>,
    /// 键 → 实例索引（owned subset 专属；external 无）。
    key_to_index: hashbrown::HashMap<InstanceKey, u32>,
    /// 已分配实例索引的 min/max（swap_remove 维护；O）。
    min_index: AtomicU64,
    max_index: AtomicU64,
    /// 紧凑实例键数组（16B/元素；sort_key f16）。
    instance_keys: Vec<InstanceKey>,
    /// 类型化实例数据缓冲（`T: NaniInstanceData`，dyn Any 降级）。
    data: Box<dyn Any + Send + Sync>,
    /// 实例计数与下一索引。
    instance_count: u64,
    next_instance_index: u32,
}

impl OwnedGpuWorldSubset {
    /// 新 owned subset（inline 21 处）。
    pub fn new() -> Self {
        todo!("construct OwnedGpuWorldSubset")
    }

    /// 实例数（`instance_count`，inline 2 处）。
    pub fn instance_count(&self) -> u64 {
        self.instance_count
    }

    /// 无数据扩展：为 keys 分配实例槽（0x140992280，O）。
    /// `callback` 每实例调用一次，收到 `(instance_index, kind)` 打包值。
    /// 上限 `instance_count >= max_instances[kind]` → panic "Max instance count reached for {kind}"。
    pub fn extend_no_data(
        &self,
        keys: &[InstanceKey],
        kind: GpuWorldSubsetKind,
        callback: impl FnMut(u64),
    ) {
        // O 路径：每 key → i = next_index++；
        //   HashMap insert key→i；Vec<ChunkId> push i；pending Vec push (i, key_low, kind)；
        //   Vec<InstanceKey> 写 key；CAS min/max；数据缓冲 type-id 断言 + 元素写；
        //   instance_count++。
        todo!("recover extend_no_data body; see contracts/gpu_world.md §2")
    }

    /// 更新单个实例数据（`update_instance_data<T>` 0x14156db40 系，O）。
    /// fast-path：`key.instance_index < keys.len() && chunk_id_vec[key.instance_index] == key.chunk_id`；
    /// 否则 HashMap 查找，缺失 → `unwrap_failed`。
    /// 写紧凑 key（f16 sort_key）+ 锁类型化缓冲 type-id 断言 + 复制 T + CAS min/max。
    pub fn update_instance_data<T: NaniInstanceData>(&self, key: InstanceKey, data: T) {
        todo!("recover update_instance_data<T>; element size = size_of::<T>()")
    }

    /// 移除实例（swap_remove，O）：`IndexCondenser::remove` → 命中则末元素补位 + 收缩 min/max + 缓冲 remove。
    pub fn remove_instance(&self, key: InstanceKey) {
        todo!("recover remove_instance; see contracts/gpu_world.md §4")
    }

    /// GPU flush（inline 1 处）：标记数据脏，供 gpu_generate_draw_lists 上传。
    pub fn gpu_flush(&self) {
        todo!()
    }

    /// 清空（inline 1 处）。
    pub fn clear(&self) {
        todo!()
    }
}

/// External subset（无 key HashMap；`[6]==0` 跳过查找——O）。
pub struct ExternalGpuWorldSubset {
    // typed gap：外部缓冲句柄/引用字段 U。
}

impl ExternalGpuWorldSubset {
    pub fn new() -> Self {
        todo!()
    }
}

/// GPU 世界：27 subset 槽 + 两个 TypedStorageBuffer + 总计数。
pub struct GpuWorld {
    /// 27 × 0xc0B subset 槽（trait 对象在每槽 +0x98/+0xa0；O）。
    subsets: [RwLock<Option<GpuWorldSubset>>; 27],
    /// 总实例计数（`create_instances` 尾部累加；+0x1440）。
    total_instance_count: u64,
    /// 第二个计数/保留字段（+0x1448；typed gap）。
    _reserved: u64,
    /// 7 个"主" subset 锁数组（+0x1450；`array::default<RwLock<Option<GpuWorldSubset>>>`）。
    main_subsets: Box<[RwLock<Option<GpuWorldSubset>>; 7]>,
    /// 每 atlas part 的 mesh 信息缓冲（+0x1458）。
    mesh_atlas_info: TypedStorageBuffer<NaniMeshAtlasPartInfo>,
    /// draw-list 信息缓冲（+0x1480）。
    draw_list_info: TypedStorageBuffer<NaniDrawListInfo>,
    /// `nani_calculate_draw_list_params` 结果数据（+0x14a0）。
    draw_list_params: DrawListParams,
}

impl GpuWorld {
    /// 构造（0x1409917d0，O）：params → 27 槽布局 → 两个 TypedStorageBuffer → 计数清零。
    pub fn new() -> Self {
        let params = nani_calculate_draw_list_params();
        let main_subsets = Box::new([RwLock::new(None), RwLock::new(None), RwLock::new(None),
                                     RwLock::new(None), RwLock::new(None), RwLock::new(None),
                                     RwLock::new(None)]);
        Self {
            subsets: [(); 27].map(|_| RwLock::new(None)),
            total_instance_count: 0,
            _reserved: 0,
            main_subsets,
            mesh_atlas_info: TypedStorageBuffer::new(),
            draw_list_info: TypedStorageBuffer::new(),
            draw_list_params: params,
        }
    }

    /// 取 subset 写锁（0x140991c50，O；Option 空 → `unwrap_failed`）。
    pub fn owned_subset_mut(&self, kind: GpuWorldSubsetKind) -> RwLock<Option<GpuWorldSubset>> {
        todo!("lock subsets[kind]; unwrap_failed if None")
    }

    /// 共享取 subset（get_subset，inline）。
    pub fn get_subset(&self, _kind: GpuWorldSubsetKind) -> Option<&GpuWorldSubset> {
        todo!()
    }

    /// 尝试取 subset 写锁（try_get_subset_mut，inline 2 处）。
    pub fn try_get_subset_mut(&self, _kind: GpuWorldSubsetKind) -> Option<GpuWorldSubset> {
        todo!()
    }

    /// 独占写（owned_subset_exclusive_mut，inline 1 处）。
    pub fn owned_subset_exclusive_mut(&self, _kind: GpuWorldSubsetKind) {
        todo!()
    }

    /// 分配 subset（assign_subset，inline 1 处）。
    pub fn assign_subset(&mut self, _kind: GpuWorldSubsetKind, _subset: GpuWorldSubset) {
        todo!()
    }

    /// 创建实例（0x14156d0d0，O，monomorphized `<PlacementDataSsbo, create_prefab_gpu_instances::closure_env$0>`）：
    /// `count = min(keys.len(), cap)`；每 key 分配索引、登记 HashMap/紧凑 key/pending、
    /// 数据缓冲写 `create(key)`、min/max、计数；见 contracts/gpu_world.md §2。
    pub fn create_instances<T: NaniInstanceData, F: FnMut(InstanceKey) -> T>(
        &self,
        kind: GpuWorldSubsetKind,
        keys: &[InstanceKey],
        cap: usize,
        create: F,
        pending: &mut Vec<(u32, u32, u8)>,
    ) {
        todo!("recover create_instances; see contracts/gpu_world.md §2")
    }

    /// 更新实例 mesh（0x140991d60，O；nani_maintain_instances 的 Changed<NaniInstanceDesc> 分支）。
    pub fn update_instance_mesh(&self, _entity: bevy_ecs::entity::Entity) {
        todo!("recover update_instance_mesh; called from nani_maintain_instances")
    }

    /// 移除实例（0x140991f60，O；见 contracts/gpu_world.md §4）。
    pub fn remove_instance(&self, key: InstanceKey) {
        todo!("recover remove_instance")
    }

    /// 导入 RG（0x1409921c0，O）：遍历 27 槽，非空子集调 vtable+0x40。
    pub fn import_to_rg(&self, _rg: &mut rhapsody_rg::render_graph::RenderGraph) {
        todo!("for each subset slot: subset.import_to_rg(rg)")
    }

    /// 导出 RG（0x140992220，O，85B 对称）。
    pub fn export_from_rg(&self, _rg: &mut rhapsody_rg::render_graph::RenderGraph) {
        todo!()
    }

    /// list 索引 → cmd 偏移（`list_index_to_cmd_offset`，inline 1 处；params 表查）。
    pub fn list_index_to_cmd_offset(&self, _list: usize) -> u32 {
        todo!()
    }

    /// mesh atlas 信息缓冲句柄（`mesh_atlas_info_buf`，inline 2 处）。
    pub fn mesh_atlas_info_buf(&self) -> &TypedStorageBuffer<NaniMeshAtlasPartInfo> {
        &self.mesh_atlas_info
    }

    /// draw-list 信息缓冲句柄（`draw_list_info_buf`，inline 1 处）。
    pub fn draw_list_info_buf(&self) -> &TypedStorageBuffer<NaniDrawListInfo> {
        &self.draw_list_info
    }
}

/// Nani draw-list 信息（TypedStorageBuffer 元素；字段 U——GPU `draw_list_info` SSBO 布局）。
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct NaniDrawListInfo {
    /// typed gap：{draw_count, first_index, ...} 字段 U。
    pub _gap: [u32; 4],
}

/// NaniInstanceData trait（impl$14，O）：每实例类型的方法族。
/// 尺寸证据见 contracts/instance_data.md §1。
pub trait NaniInstanceData: Any + Send + Sync + 'static {
    fn type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn gpu_flush(&self);
    fn rg_handle(&self) -> u64;
    fn import_to_rg(&self, _rg: &mut rhapsody_rg::render_graph::RenderGraph);
    fn export_from_rg(&self, _rg: &mut rhapsody_rg::render_graph::RenderGraph);
    fn copy(&self, src: u32, dst: u32);
}

impl fmt::Debug for GpuWorld {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // impl$17::fmt（0x140992a60 等 5 处 monomorphization，39B）。
        Ok(())
    }
}
