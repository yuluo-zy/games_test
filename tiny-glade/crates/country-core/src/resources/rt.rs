//! `resources/rt` — 光线追踪世界与 GPU BVH（B 原版证据，验证边界见模块合同）。
//!
//! 证据：ghidra shared fresh（b1-rt-findmesh.result.json / b2-hardpoint-disasm /
//! b4-upload-disasm / b5-read-strings，2026-10-17）：
//! `update_rt_world`（0x1408bc650）/`upload_rt_world`（0x1408bcff0）/
//! `build_gpu_bvh`（0x140b0da50）/`bvh_builder_worker`（0x140b24d50）/
//! `RtWorld::get_mut`（0x1415626f0 DirtyCow / 0x141562aa0 包装 / 0x1400cce50 内联）/
//! `RtBvhBuilderGlueResource::is_idle`（0x1408bc530）/`impl$1::from`（0x1408bc460）/
//! `impl$2::from`（0x1408bc510）/`impl$5::from`（0x1408bc520）。
//!
//! ## 本轮 fresh 锚定（ui-restorer-seams，2026-10-17）
//!
//! - **`RtMesh` = 0x60 B**（订正此前 0x50）：
//!   `DirtyCow<RtMesh>::get_mut`（0x1415626f0）按 value layout **0x60** 分配 ArcInner，
//!   并逐 12 qword 克隆/复制。字段：
//!   * `+0x00..0x40` `matrix: [f32;16]` —— `create_prefab_rt` 0x140841550 写
//!     `[RDI+0x00..0x3c]`；`update_prefabs` 0x140841d60 读 `+0x10..0x4c` 做变更检测后写全 16 项。
//!   * `+0x40..0x58` `positions: Vec<Vec3>` —— `create_prefab_rt` 用 cube mesh 的
//!     `positions()` 索引展开后 extend（0x1415634c0 = `Copied<Iter<u32>>` → `Vec3`）。
//!   * `+0x58` `color: u32` —— `create_prefab_rt` 的 `MOV [RDI+0x58],ECX`（`sqrt(c)*255+0.5` 打包）。
//!   * 默认值（`RtWorld::get_mut` 0x141562aa0 写入）：单位矩阵（`m[0]=m[5]=m[10]=m[11]=1`；
//!     与 create 的 `[10]/[11]` 重复 lane 一致）、空 `positions`（cap 0 / dangling 4 / len 0）、
//!     color 0。
//!   `+0x30` 是 `RtWorld` 内的新句柄计数器（`create_prefab_rt`：`RCX=*ctx`、`[RCX+0x30]+=1`）。
//! - **`RtMeshHandle` 变体 2**：`create_prefab_rt` 以 `MOV EDX,0x2` 构造
//!   `RtMeshHandle { tag: 2, payload: counter }`；`impl$2::from` 0x1408bc510 的 tag 1 是另一变体。
//! - `RtBvhSsbo` 元素大小 0x50（`upload_rt_world` 断言），**不是** `RtMesh`（0x60）；本轮拆成
//!   [`RtBvhRecord`] 是原锁定 obvhs 的 CwBvhNode（0x50 / align4），保留真实量化节点载荷。
//!
//! ## 分类（协议 §2026-10-17）
//!
//! | VA | 原函数 | 处置 | 依赖 |
//! |---|---|---|---|
//! | 0x1408bc530 | `RtBvhBuilderGlueResource::is_idle` | **实现** | 同一 RwLock 的 request/completed/busy 三态 |
//! | 0x1408bc460 | `impl$1::from`（Vec3→u32 颜色） | **实现** | `SQRT*255+0.5` 打包 |
//! | 0x1408bc510 | `impl$2::from` | 原 albedo 引用构造 | sret tag u32=1、payload@8；不能据此认定为 RtMeshHandle |
//! | 0x1408bc520 | `impl$5::from` | **实现** | `{0, WallId}`，保留原传入的 wall identity |
//! | 0x1415626f0/0x141562aa0/0x1400cce50 | `RtWorld::get_mut`/`DirtyCow::get_mut` | **实现** | HashMap 键查 + COW |
//! | 0x1408bc650 | `update_rt_world` | **实现** | 删除事件、显式更新事件、COW 真实脏状态和快照消费 |
//! | 0x1408bcff0 | `upload_rt_world` | **实现**（`upload_rt_world(rt, library, device)`） | canonical InstanceBuffer计数已连接 |
//! | 0x140b0da50 | `build_gpu_bvh` | **实现** | 原 commit 的 obvhs CWBVH、原参数、geometry/metadata 同步重排 |
//! | 0x140b24d50 | `bvh_builder_worker` | **实现** | 真实快照、仿射变换、有限坐标过滤、单请求 worker 与结果所有权 |
//!
//! `build_gpu_bvh`/`bvh_builder_worker` 在子模块中复用原锁定依赖；原完整 RNE 与独立核验在
//! task-index/work/TG-aca2b4fd535d278cb4。v19 有界 CPU 验证不能代替完整场景 GPU 差分。
//! `upload_rt_world` 已按完整 RNE 恢复 Option 待上传状态、三段上传顺序、严格容量边界和
//! 各段完成后立即发布的 InstanceBuffer 计数；compute/draw 使用该计数，不再读取 buffer ID。
//! 这只关闭已有上传数据到 GPU 库的消费者，不能代替真实 BVH 数据生产与完整世界接线。

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use bevy_ecs::resource::Resource;
use glam::Vec3;

use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType};
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{InstanceBuffer, StorageElement, TypedStorageBuffer};

pub mod bvh;
pub mod bvh_builder;

/// Original 24-byte GPU triangle; metadata is a separate 4-byte buffer.
pub type GpuRtTriangle = RtTrianglesSsbo;

/// `RtMesh`（O：CPU 网格记录，**0x60 B**；fresh `DirtyCow<RtMesh>::get_mut` 0x1415626f0
/// 的 0x60 value layout + `create_prefab_rt`/`update_prefabs` 写面锚定）。
///
/// `#[repr(C)]` 使字段偏移与机器一致。默认值为 `RtWorld::get_mut` 0x141562aa0 的写序列。
#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct RtMesh {
    /// Four padded XYZ columns of an affine transform. Padding lanes are not
    /// a perspective row; the BVH worker never divides by w.
    pub matrix: [f32; 16],
    /// +0x40：索引展开后的网格顶点（`create_prefab_rt` 的 cube mesh `positions()`）。
    pub positions: Vec<Vec3>,
    /// +0x58：打包颜色（`r | g<<8 | b<<16`，见 [`color_to_u32`]）。
    pub color: u32,
}

impl RtMesh {
    /// 默认值（O：0x141562aa0 的写序列，12 qword）：
    /// `matrix = [1,0,0,0, 0,1,0,0, 0,0,1,1, 0,0,0,0]`（[10]/[11] 重复 lane 与
    /// `create_prefab_rt` 一致）、空 `positions`、color 0。
    pub fn empty() -> Self {
        let mut matrix = [0.0f32; 16];
        matrix[0] = 1.0;
        matrix[5] = 1.0;
        matrix[10] = 1.0;
        matrix[11] = 1.0;
        Self {
            matrix,
            positions: Vec::new(),
            color: 0,
        }
    }

    /// `create_prefab_rt` 的矩阵写面（0x140841550 `[RDI+0x00..0x3c]`）。
    pub fn set_matrix(&mut self, matrix: [f32; 16]) {
        self.matrix = matrix;
    }

    /// Original construct_walls @140759626..9c6: albedo refers to the
    /// wall colour table by its numeric identity, tagged with bit24. This
    /// original temporary enum holds the WallId value, then reads it through
    /// a stack wrapper. Encode that value directly; it is not a raw pointer.
    pub fn set_wall_albedo(&mut self, wall: crate::resources::walls::WallId) {
        self.color = (wall.get_inner() as u32) | 0x0100_0000;
    }

    /// `update_prefabs` 0x140841d60 的变更检测：只比较 `matrix[4..16]`
    /// （反汇编比较 `+0x10..0x4c`，**不比较 `[0..4]`**）。
    pub fn matrix_tail_differs(&self, matrix: &[f32; 16]) -> bool {
        self.matrix[4..16]
            .iter()
            .zip(&matrix[4..16])
            .any(|(a, b)| a.to_bits() != b.to_bits())
    }
}

impl Default for RtMesh {
    /// O：`RtWorld::get_mut` 插入新条目时的默认构造（[`RtMesh::empty`]）。
    fn default() -> Self {
        Self::empty()
    }
}

/// `RtMeshHandle`（O：`enum2$<RtMeshHandle>`，**16 B 键**；`hash_one<ref<
/// enum2_<RtMeshHandle>>>` 哈希整键、键比较 2×u64）。
///
/// Rust 侧以 `#[repr(C)] { tag: u64, payload: u64 }` 表示；原已锚定 tag0 为 WallId、
/// tag2 为 prefab 分配值。tag1 的转换保留传入值，不推断额外身份。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RtMeshHandle {
    /// 判别：wall=0，impl$2=1，prefab=2。
    pub tag: u64,
    /// 8 字节原身份载荷。
    pub payload: u64,
}

impl RtMeshHandle {
    /// Existing source-build tag1 constructor. The native @1408bc510 belongs
    /// to albedo conversion, not proof of this handle constructor's identity.
    pub fn from_handle(payload: u64) -> Self {
        Self { tag: 1, payload }
    }

    /// `create_prefab_rt`（0x140841550，O）：`MOV EDX,0x2` → `{2, payload}`。
    /// payload 来自 `RtWorld +0x30` 计数器。
    pub fn from_prefab(payload: u64) -> Self {
        Self { tag: 2, payload }
    }

    /// Original impl$5::from@1408bc520: xor eax,eax; mov rdx,rcx; ret.
    pub fn from_wall_id(id: crate::resources::walls::WallId) -> Self {
        Self {
            tag: 0,
            payload: id.0,
        }
    }

    /// Compatibility sentinel retained for existing callers; original wall
    /// conversion is `from_wall_id` and must retain the real WallId payload.
    pub fn empty() -> Self {
        Self { tag: 0, payload: 0 }
    }
}

#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default)]
pub struct UpdateRtWorldEvent;

/// Original four canonical ECS inputs. Snapshot consumption clears dirty bits
/// only after detecting a real mutation; unchanged frames do not rebuild.
pub fn update_rt_world(
    mut world: bevy_ecs::prelude::ResMut<RtWorld>,
    glue: bevy_ecs::prelude::Res<RtBvhBuilderGlueResource>,
    mut deleted: bevy_ecs::event::EventReader<crate::resources::walls::public_walls::OnWallDeleted>,
    mut updates: bevy_ecs::event::EventReader<UpdateRtWorldEvent>,
) {
    for event in deleted.read() {
        world.remove_mesh(&RtMeshHandle::from_wall_id(event.0));
    }
    let external_update = updates.read().count() != 0;
    bvh_builder::request_rebuild(&mut world, &glue, external_update);
}

/// `DirtyCow<T>`（O：脏标记 + `Arc<T>` 的 copy-on-write 容器；`HashMap<RtMeshHandle,
/// DirtyCow<RtMesh>>` 值；布局 `{inner: Arc<T> @+0, dirty: AtomicBool @+8}`——
/// `DirtyCow::get_mut` 0x1415626f0 写 +8 脏字节、读 +0 Arc 指针）。
#[derive(Debug)]
pub struct DirtyCow<T> {
    /// 共享值（O：+0x00 Arc 指针）。
    pub inner: Arc<T>,
    /// 脏标记（O：+0x08 字节）。
    pub dirty: AtomicBool,
}

impl<T: Default> Default for DirtyCow<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Clone for DirtyCow<T> {
    /// O：clone 共享 Arc + 复制脏标记。
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            dirty: AtomicBool::new(self.dirty.load(Ordering::Relaxed)),
        }
    }
}

impl<T> DirtyCow<T> {
    /// 新建（O：`impl$2::from` `{1, 值}`；脏=0）。
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(value),
            dirty: AtomicBool::new(false),
        }
    }

    /// `DirtyCow::get_mut`（O：0x1415626f0 —— 置脏字节 + 引用计数 >1 时克隆值）。
    pub fn get_mut(&mut self) -> &mut T
    where
        T: Clone,
    {
        self.dirty.store(true, Ordering::Relaxed);
        Arc::make_mut(&mut self.inner)
    }

    /// 脏标记（O：+0x08）。
    pub fn is_dirty(&self) -> bool {
        self.dirty.load(Ordering::Relaxed)
    }
}

/// `RtWorld`（O：`resources/rt` 的 GPU 世界；`update_rt_world`/`upload_rt_world` 经
/// `RwLock<RtWorldInner>` 访问；`RtWorld::get_mut` 直接操作 `HashMap<RtMeshHandle,
/// DirtyCow<RtMesh>>`）。
///
/// inner 布局（fresh upload_rt_world 0x1408bcff0 读 +0x20/+0x30/+0x48 Vec + +0x60 计数）：
/// 上传三缓冲 `RtTriangleSsbo`(0x18/元素)/`RtTriangleMetaSsbo`(4/元素)/`RtBvhSsbo`
/// (0x50/元素，[`RtBvhRecord`])。Rust 侧保留权威 meshes map 与待处理 Option 上传包；源宿主不冒充原 RwLock ABI。
///
/// **`Resource`**：`hot_reload_prefabs` 的系统签名（mangled `FunctionSystem<… ResMut<RtWorld> …>`）
/// 证明 `RtWorld: Resource`；`create_prefab_rt` 生成的句柄计数器位于 inner **+0x30**。
#[derive(Clone, Debug, Default, Resource)]
pub struct RtWorld {
    /// 网格表（O：`HashMap<RtMeshHandle, DirtyCow<RtMesh>>`；`get_mut` 键查表）。
    pub meshes: std::collections::HashMap<RtMeshHandle, DirtyCow<RtMesh>>,
    /// Original pending Option tuple. None preserves the existing GPU data;
    /// Some(empty) explicitly publishes zero counts. The BVH worker producer
    /// must publish its real result here; no empty upload is manufactured.
    pub pending_upload: Option<RtUploadData>,
    /// 预制作网格句柄计数器（O：`create_prefab_rt` 读写 `RtWorld inner +0x30`；
    /// 每次返回旧值后 +1，句柄 tag = 2）。
    pub next_prefab_handle: u64,
    /// Native structural dirty bit; distinct from each mesh's COW dirty flag.
    topology_dirty: bool,
}

impl RtWorld {
    /// B uninit_clearing@140a7a65d..665 clears the actual mesh table and
    /// marks topology dirty. Pending upload and prefab handle counter survive.
    pub fn clear_meshes(&mut self) {
        self.meshes.clear();
        self.topology_dirty = true;
    }

    /// Native MetaCommander WaitAsync observes both structural changes and
    /// dirty mesh COW rows, before asking the BVH worker whether it is idle.
    /// Pending GPU upload is a later state and cannot replace this predicate.
    pub fn has_pending_changes(&self) -> bool {
        self.topology_dirty || self.meshes.values().any(DirtyCow::is_dirty)
    }

    /// `RtWorld::get_mut`（O：0x141562aa0 / 0x1400cce50）：`HashMap<RtMeshHandle,
    /// DirtyCow<RtMesh>>` 键查表（hashbrown SIMD 探测 + 未命中 `reserve_rehash` +
    /// 插入默认 `RtMesh`）→ `DirtyCow::get_mut`。
    ///
    /// fresh 反编译：`hash_one<RtMeshHandle>` → RawTable 探测键（2×u64）→ 命中直接
    /// `DirtyCow::get_mut`；未命中在容量满时 `reserve_rehash`，分配新 ArcInner 写默认
    /// `RtMesh::empty()`，插入后 `DirtyCow::get_mut`。Rust 侧以 `HashMap::entry` +
    /// `or_insert_with(RtMesh::empty)` + `DirtyCow::get_mut` 等价实现（哈希表内部
    /// 为库行为，登记）。
    pub fn get_mut(&mut self, handle: RtMeshHandle) -> &mut RtMesh {
        self.meshes
            .entry(handle)
            .or_insert_with(|| DirtyCow::new(RtMesh::default()))
            .get_mut()
    }

    /// `RtWorld::get`：只读取网格表（O：`get_mut` 的只读面；`bvh_builder_worker` 经
    /// `Values<HashMap>` 消费）。
    pub fn get(&self, handle: RtMeshHandle) -> Option<&RtMesh> {
        self.meshes.get(&handle).map(|cow| &*cow.inner)
    }

    /// Existing source-build tag1 handle convenience; wall and prefab owners
    /// use their separately evidenced constructors.
    pub fn handle(payload: u64) -> RtMeshHandle {
        RtMeshHandle::from_handle(payload)
    }

    /// `create_prefab_rt` 的句柄分配（O：0x140841550 在调用 `get_mut` 前
    /// `payload = *[RtWorld+0x30]; *[RtWorld+0x30] = payload + 1`，键 tag = 2）。
    pub fn alloc_prefab_handle(&mut self) -> RtMeshHandle {
        let payload = self.next_prefab_handle;
        self.next_prefab_handle = self.next_prefab_handle.wrapping_add(1);
        RtMeshHandle::from_prefab(payload)
    }

    /// 写入一条预制作网格更新（矩阵/颜色/顶点），等价于
    /// `get_mut(handle)` 后按 `create_prefab_rt` 的写序写 `+0x00..0x40`、`+0x40` Vec、`+0x58`。
    pub fn write_prefab_mesh(
        &mut self,
        handle: RtMeshHandle,
        matrix: [f32; 16],
        positions: Vec<Vec3>,
        color: u32,
    ) {
        let mesh = self.get_mut(handle);
        mesh.matrix = matrix;
        mesh.positions = positions;
        mesh.color = color;
    }

    /// `update_rt_world`（0x1408bc650）的移除 seam：逐句柄 `RawTable::remove_entry`
    /// + `Arc::drop_slow<RtMesh>`（fresh：命中时 LOCK 递减并 drop）。
    ///
    /// 原版两段 `RtMeshHandle` 变更切片（0x10 B/元素）逐条 remove_entry，然后
    /// `from_iter<RtMeshHandle, DirtyCow<RtMesh>, Map<IterMut, consume>>` 重建哈希表
    /// （语义 = 把剩余条目搬进新表；Rust `HashMap` 无此步，等价为空操作）。
    /// 原版对 inner +0x68..+0xa7 的 log 记录与 +0x38 脏字节写未建模（登记）。
    pub fn remove_mesh(&mut self, handle: &RtMeshHandle) -> Option<DirtyCow<RtMesh>> {
        self.topology_dirty = true;
        self.meshes.remove(handle)
    }

    /// 原invalidate140b37a1a..a57：仅命中时先drop真实DirtyCow/Arc，再标记拓扑变化。
    /// 单次查表；miss不制造脏帧，不改其它入口原有dirty合同。
    pub fn remove_mesh_if_present(&mut self, handle: &RtMeshHandle) -> bool {
        let Some(removed) = self.meshes.remove(handle) else { return false; };
        drop(removed);
        self.topology_dirty = true;
        true
    }

    /// 批量移除（`update_rt_world` 的两段变更切片）。
    pub fn remove_meshes<'a>(&mut self, handles: impl IntoIterator<Item = &'a RtMeshHandle>) {
        for handle in handles {
            let _ = self.remove_mesh(handle);
        }
    }

    /// Native @1408bd2db takes the pending Option, using the Vec capacity
    /// niche for None; @1408bd319/320 skips the absent tuple.
    pub fn take_upload_data(&mut self) -> Option<RtUploadData> {
        self.pending_upload.take()
    }
}

// ---------------------------------------------------------------------------
// RT GPU upload elements. Fresh B evidence and independent review are in
// TG-29b7e480e17d8a9977/main-chain-frame-20261018/RT-CONTRACT.md.
// Storage is count + actual fixed-capacity typed buffer, not an empty carrier.
// ---------------------------------------------------------------------------

/// `RtTriangleSsbo`（O：0x18 B/元素；`upload_rt_world` 的 `RtTrianglesSsbo` 上传面，
/// 类型名断言长度 0x2c = `country_core::resources::rt::RtTrianglesSsbo`）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::NoUninit)]
pub struct RtTrianglesSsbo {
    pub base_vertex: [f32; 3],
    /// Low half = e2; high half = e1. This differs from obvhs's helper type.
    pub edge_pairs: [u32; 3],
}

impl RtTrianglesSsbo {
    pub fn from_vertices(a: Vec3, b: Vec3, c: Vec3) -> Self {
        let e1 = (b - a).to_array();
        let e2 = (c - a).to_array();
        Self {
            base_vertex: a.to_array(),
            edge_pairs: std::array::from_fn(|axis| {
                u32::from(half::f16::from_f32(e2[axis]).to_bits())
                    | (u32::from(half::f16::from_f32(e1[axis]).to_bits()) << 16)
            }),
        }
    }
}

impl StorageElement for RtTrianglesSsbo {
    const NAME: &'static str = "RtTriangles";
    const BUFFER_LEN: usize = 0x40000;
}

impl SsboType for RtTrianglesSsbo {
    type Storage = InstanceBuffer<Self>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        rt_storage::<Self>(device, initial)
    }
}

/// Compatibility spelling for earlier source consumers. The canonical type
/// and library type_name are the original plural RtTrianglesSsbo.
pub type RtTriangleSsbo = RtTrianglesSsbo;

/// Per-triangle packed color/metadata, reordered by original primitive index.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::NoUninit)]
pub struct RtTriangleMetaSsbo {
    pub packed_color: u32,
}

impl StorageElement for RtTriangleMetaSsbo {
    const NAME: &'static str = "RtTriangleMeta";
    const BUFFER_LEN: usize = 0x40000;
}

impl SsboType for RtTriangleMetaSsbo {
    type Storage = InstanceBuffer<Self>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        rt_storage::<Self>(device, initial)
    }
}

/// Exact original dependency node encoding (80 bytes, alignment 4).
pub type RtBvhRecord = obvhs::cwbvh::node::CwBvhNode;

/// `RtBvhSsbo`（O：0x50 B/元素 = [`RtBvhRecord`]；类型名断言长度 0x26）。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::NoUninit)]
pub struct RtBvhSsbo {
    /// BVH 记录（0x50 B）。
    pub record: RtBvhRecord,
}
const _: () = assert!(core::mem::size_of::<RtBvhSsbo>() == 80);
const _: () = assert!(core::mem::align_of::<RtBvhSsbo>() == 4);

impl StorageElement for RtBvhSsbo {
    const NAME: &'static str = "RtBvh";
    const BUFFER_LEN: usize = 0x10000;
}

impl SsboType for RtBvhSsbo {
    type Storage = InstanceBuffer<Self>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        rt_storage::<Self>(device, initial)
    }
}

/// Original add_new instances allocate 48 bytes: count0 followed by Buffer40.
/// The three typed constructors use fixed capacities, usage0x22, memory1 and
/// alignment4. None supplies no bytes (no host-generated zero array).
fn rt_storage<T: StorageElement>(
    device: &dyn GraphicsDevice,
    initial: Option<&[u8]>,
) -> InstanceBuffer<T> {
    let buffer = TypedStorageBuffer::<T>::new_storage_buffer(device, initial);
    InstanceBuffer {
        instance_count: initial.map_or(0, |bytes| bytes.len() / core::mem::size_of::<T>()),
        buffer,
    }
}

#[derive(Clone, Debug, Default)]
pub struct RtUploadData {
    pub triangles: Vec<RtTrianglesSsbo>,
    pub metadata: Vec<RtTriangleMetaSsbo>,
    pub bvh: Vec<RtBvhSsbo>,
}

impl crate::render::pass_builder::pass_resource_access::SsboType for RtTrianglesSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
            .0
    }
}
impl crate::render::pass_builder::pass_resource_access::SsboType for RtTriangleMetaSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
            .0
    }
}
impl crate::render::pass_builder::pass_resource_access::SsboType for RtBvhSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
            .0
    }
}

/// `upload_rt_world`（0x1408bcff0）设备面落地：取走三份缓冲并按原顺序上传。
///
/// fresh 反编译：
/// 1. `RwLock` 写锁；读 inner `+0x20/+0x30/+0x48` 三 Vec 与 `+0x60` 计数；
///    `mem::take` 后解锁；
/// 2. 越界门：`(meta_len | tri_len) < 0x40000 && record_len < 0x10000`，否则打印
///    `"Error OOB RT data: len tris …"` + `"Out of pre-allocated buffer space for
///    ray tracing-data"` 错误日志并走失败路径；
/// 3. 依次 `FrozenMap::get<SsboId>`（名字断言）→ 容量检查 → `update_buffer_bytes`
///    （size = len·0x18 / len·4 / len·0x50）→ `*storage_count = len`。
///
/// Missing storage, capacity and backend failures retain original unwrap/panic.
/// The outer input-size guard logs and returns. None preserves current GPU
/// state; Some(empty) publishes zero counts without uploading bytes.
pub fn upload_rt_world(
    rt: &mut RtWorld,
    library: &mut AssetSsboLibrary,
    device: &dyn GraphicsDevice,
) {
    let Some(data) = rt.take_upload_data() else {
        return;
    };
    upload_rt_data(data, library, device);
}

/// Production consumer of the actual asynchronous result owner. The source
/// library requires a mutable Rust borrow; native FrozenMap mutability is not
/// reproduced with an unsafe cast.
pub fn upload_rt_world_system(
    device: bevy_ecs::prelude::Res<crate::render::device::GraphicsDeviceResource>,
    mut library: bevy_ecs::prelude::ResMut<AssetSsboLibrary>,
    glue: bevy_ecs::prelude::Res<RtBvhBuilderGlueResource>,
) {
    if let Some(data) = glue.take_completed() {
        upload_rt_data(data, &mut library, device.device());
    }
}

fn upload_rt_data(data: RtUploadData, library: &mut AssetSsboLibrary, device: &dyn GraphicsDevice) {
    if (data.triangles.len() | data.metadata.len()) < 0x40000 && data.bvh.len() < 0x10000 {
        upload_buffer::<RtTrianglesSsbo>(library, device, &data.triangles);
        upload_buffer::<RtTriangleMetaSsbo>(library, device, &data.metadata);
        upload_buffer::<RtBvhSsbo>(library, device, &data.bvh);
    } else {
        log::error!("Out of pre-allocated buffer space for ray tracing-data");
        eprintln!(
            "Error OOB RT data: len tris {} meta {} bvh {}",
            data.triangles.len(),
            data.metadata.len(),
            data.bvh.len()
        );
    }
}

/// 单缓冲上传（O：`FrozenMap::get` → 名字断言 → 容量检查 → `update_buffer_bytes`）。
fn upload_buffer<T: SsboType<Storage = InstanceBuffer<T>>>(
    library: &mut AssetSsboLibrary,
    device: &dyn GraphicsDevice,
    data: &[T],
) {
    let storage = library.get_mut::<T>().unwrap_or_else(|| {
        panic!(
            "upload_rt_world: Ssbo not registered: {}",
            core::any::type_name::<T>()
        )
    });
    let buffer = &storage.buffer.buffer;
    let bytes = crate::render::ssbo::rhapsody_boundary::slice_as_bytes(data);
    // O：`buffer.capacity[4] < len * element_size` → unwrap_failed。
    if (buffer.byte_size as usize) < bytes.len() {
        eprintln!(
            "Error OOB RT data: buffer {} needs {} bytes, has {}",
            core::any::type_name::<T>(),
            bytes.len(),
            buffer.byte_size
        );
        panic!("Out of pre-allocated buffer space for ray tracing-data");
    }
    if !bytes.is_empty() {
        device
            .update_buffer_bytes(buffer, bytes, &[0..bytes.len()], 0)
            .expect("called `Result::unwrap()` on an `Err` value: update_buffer_bytes");
    }
    // Empty Some publishes zero too; None was handled before looking up SSBOs.
    storage.instance_count = data.len();
}

/// `InputTriangle`（O：0x28 B；`build_gpu_bvh` 0x140b0da50 的输入三角，
/// `{ a: Vec3 @+0x00, b: Vec3 @+0x0c, c: Vec3 @+0x18, user_data: u32 @+0x24 }`）。
///
/// fresh：函数把每个输入三角逐分量复制到 0x30 B 的 `[Vec3;3]`（第三分量重复为第 4 分量）
/// 供 `obvhs::cwbvh::builder::build_cwbvh_from_tris`；`user_data`（+0x24）作为
/// `GpuRtTriangle` 的载荷（`from_iter<GpuRtTriangle, Map<Iter<u32>, closure_env$1>>`）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InputTriangle {
    /// +0x00。
    pub a: glam::Vec3,
    /// +0x0c。
    pub b: glam::Vec3,
    /// +0x18。
    pub c: glam::Vec3,
    /// +0x24。
    pub user_data: u32,
}

/// Real single-slot request/result owners. Request tag 2 means None; it is
/// unrelated to thread shutdown. Completed empty data is still Some(data).
#[derive(Debug, Default)]
pub struct RtBvhBuilderGlue {
    request: Option<bvh_builder::RtBvhBuildRequest>,
    completed: Option<RtUploadData>,
    pub busy: bool,
}

#[derive(Clone, Default, Resource)]
pub struct RtBvhBuilderGlueResource {
    inner: Arc<std::sync::RwLock<RtBvhBuilderGlue>>,
}

impl RtBvhBuilderGlueResource {
    /// 1408bc594..5a5: no request, no output, and not busy. A finished result
    /// remains non-idle until the real upload consumer takes it.
    pub fn is_idle(&self) -> bool {
        let inner = self
            .inner
            .read()
            .unwrap_or_else(|_| panic!("RtBvhBuilderGlueResource::is_idle: lock poisoned"));
        inner.request.is_none() && inner.completed.is_none() && !inner.busy
    }

    pub fn take_completed(&self) -> Option<RtUploadData> {
        self.inner
            .write()
            .expect("RT BVH lock poisoned")
            .completed
            .take()
    }
}

/// `impl$1::from`（0x1408bc460，O）：`Vec3` 颜色 → u32（`SQRT(c)*255+0.5` 打包，
/// `r | g<<8 | b<<16`，首位 0）。
pub fn color_to_u32(color: glam::Vec3) -> u32 {
    let f = |v: f32| {
        let x = v.sqrt() * 255.0 + 0.5;
        if x < 0.0 {
            0
        } else if x > 4294967295.0 {
            u32::MAX
        } else {
            x as u32
        }
    };
    let r = f(color.x);
    let g = f(color.y);
    let b = f(color.z);
    (g << 8) | r | (b << 16)
}

/// Existing source-build tag1 handle convenience.
pub fn from_handle(value: u64) -> RtMeshHandle {
    RtMeshHandle::from_handle(value)
}

/// `impl$5::from` was previously misclassified as an empty function. It creates
/// the actual tag0 wall identity, retaining the passed WallId payload.
pub fn from_wall_id(id: crate::resources::walls::WallId) -> RtMeshHandle {
    RtMeshHandle::from_wall_id(id)
}

// Complete native ranges and algorithm/worker contracts are retained under
// task-index/work/TG-aca2b4fd535d278cb4/main-chain-frame-20261018/.
// Actual scene mesh producers and production world rendering remain separate
// integration obligations; successful BVH construction does not fabricate them.

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering as AOrdering;

    #[test]
    fn color_conversion_packs_sqrt_scaled_channels() {
        let c = color_to_u32(glam::Vec3::new(1.0, 0.0, 1.0));
        assert_eq!(c, 0x00ff00ff);
    }

    #[test]
    fn dirty_cow_get_mut_marks_dirty_and_clones_on_share() {
        let mut cow = DirtyCow::new(RtMesh::default());
        cow.get_mut().color = 9;
        assert!(cow.is_dirty());
        assert_eq!(cow.inner.color, 9);
    }

    #[test]
    fn dirty_cow_layout_matches_machine() {
        // O：`get_mut` 0x1415626f0 写 +8 脏字节、读 +0 Arc 指针 → inner@+0、dirty@+8。
        assert_eq!(core::mem::offset_of!(DirtyCow<RtMesh>, inner), 0x00);
        assert_eq!(core::mem::offset_of!(DirtyCow<RtMesh>, dirty), 0x08);
        assert_eq!(core::mem::size_of::<DirtyCow<RtMesh>>(), 16);
    }

    #[test]
    fn rt_mesh_default_matches_anchored_write_sequence() {
        // O：0x141562aa0 默认写序列 + 0x1415626f0 的 0x60 value layout：
        // matrix[0]=1、[5]=1、[10]=1、[11]=1；positions 空；color 0。
        let m = RtMesh::default();
        assert_eq!(core::mem::size_of::<RtMesh>(), 0x60);
        assert_eq!(m.matrix[0], 1.0);
        assert_eq!(m.matrix[5], 1.0);
        assert_eq!(m.matrix[10], 1.0);
        assert_eq!(m.matrix[11], 1.0);
        assert_eq!(m.matrix[1], 0.0);
        assert!(m.positions.is_empty());
        assert_eq!(m.color, 0);
        assert_eq!(core::mem::offset_of!(RtMesh, matrix), 0x00);
        assert_eq!(core::mem::offset_of!(RtMesh, positions), 0x40);
        assert_eq!(core::mem::offset_of!(RtMesh, color), 0x58);
    }

    #[test]
    fn rt_mesh_handle_is_sixteen_bytes_with_tag_payload() {
        // O：键 2×u64（hash_one + 键比较）。
        assert_eq!(core::mem::size_of::<RtMeshHandle>(), 16);
        let h = RtMeshHandle::from_handle(7);
        assert_eq!(h.tag, 1);
        assert_eq!(h.payload, 7);
        assert_eq!(RtMeshHandle::empty().tag, 0);
    }

    #[test]
    fn rt_world_get_mut_inserts_default_mesh_and_returns_mut() {
        let mut world = RtWorld::default();
        let h = RtWorld::handle(3);
        {
            let m = world.get_mut(h);
            assert_eq!(m.matrix[0], 1.0);
            assert_eq!(m.color, 0);
            m.color = 8;
        }
        // 同一句柄复用既有条目。
        assert_eq!(world.get(h).unwrap().color, 8);
        assert_eq!(world.meshes.len(), 1);
    }

    #[test]
    fn alloc_prefab_handle_uses_tag_two_and_counter() {
        // O：0x140841550 的 `MOV EDX,0x2` + `[RtWorld+0x30]` 计数器。
        let mut world = RtWorld::default();
        let a = world.alloc_prefab_handle();
        let b = world.alloc_prefab_handle();
        assert_eq!(a, RtMeshHandle { tag: 2, payload: 0 });
        assert_eq!(b, RtMeshHandle { tag: 2, payload: 1 });
        assert_eq!(world.next_prefab_handle, 2);
    }

    #[test]
    fn is_idle_reflects_pending_work() {
        let res = RtBvhBuilderGlueResource::default();
        assert!(res.is_idle());
        res.inner.write().unwrap().completed = Some(RtUploadData::default());
        assert!(!res.is_idle());
        assert!(res.take_completed().is_some());
        assert!(res.is_idle());
        res.inner.write().unwrap().busy = true;
        assert!(!res.is_idle());
    }

    #[test]
    fn dirty_cow_default_uses_rt_mesh_empty() {
        let cow = DirtyCow::<RtMesh>::default();
        assert_eq!(cow.inner.matrix[0], 1.0);
        assert!(!cow.dirty.load(AOrdering::Relaxed));
    }

    /// O：`upload_rt_world` 0x1408bcff0 的元素大小 0x18 / 4 / 0x50；
    /// `InputTriangle` 0x28（`user_data` @+0x24）。
    #[test]
    fn rt_upload_element_layouts_are_anchored() {
        // O：`upload_rt_world` 0x1408bcff0 的元素大小 0x18 / 4 / 0x50；
        // `InputTriangle` 0x28（`user_data` @+0x24）。
        assert_eq!(core::mem::size_of::<RtTriangleSsbo>(), 0x18);
        assert_eq!(core::mem::size_of::<RtTriangleMetaSsbo>(), 4);
        assert_eq!(core::mem::size_of::<RtBvhSsbo>(), 0x50);
        assert_eq!(core::mem::size_of::<RtBvhRecord>(), 0x50);
        assert_eq!(core::mem::size_of::<InputTriangle>(), 0x28);
        assert_eq!(core::mem::offset_of!(InputTriangle, a), 0x00);
        assert_eq!(core::mem::offset_of!(InputTriangle, b), 0x0c);
        assert_eq!(core::mem::offset_of!(InputTriangle, c), 0x18);
        assert_eq!(core::mem::offset_of!(InputTriangle, user_data), 0x24);
        // O：RtMesh 是 0x60（cube 记录），RtBvhSsbo 元素是独立的 0x50 记录。
        assert_eq!(core::mem::size_of::<RtMesh>(), 0x60);
    }

    #[test]
    fn upload_rt_world_publishes_counts_and_distinguishes_none_from_empty() {
        use crate::render::test_device::TestDevice;
        let device = TestDevice(0);
        let mut library = AssetSsboLibrary::new();
        library.add_new::<RtTrianglesSsbo>(&device);
        library.add_new::<RtTriangleMetaSsbo>(&device);
        library.add_new::<RtBvhSsbo>(&device);
        assert_eq!(library.get::<RtTrianglesSsbo>().unwrap().instance_count, 0);
        assert_eq!(
            library.get_buffer::<RtTrianglesSsbo>().unwrap().byte_size,
            0x600000
        );
        assert_eq!(
            library
                .get_buffer::<RtTriangleMetaSsbo>()
                .unwrap()
                .byte_size,
            0x100000
        );
        assert_eq!(
            library.get_buffer::<RtBvhSsbo>().unwrap().byte_size,
            0x500000
        );
        let mut rt = RtWorld::default();
        rt.pending_upload = Some(RtUploadData {
            triangles: vec![
                RtTrianglesSsbo {
                    base_vertex: [1., 2., 3.],
                    edge_pairs: [4, 5, 6],
                };
                2
            ],
            metadata: vec![RtTriangleMetaSsbo { packed_color: 7 }; 2],
            bvh: vec![RtBvhSsbo::default(); 3],
        });
        upload_rt_world(&mut rt, &mut library, &device);
        assert!(rt.pending_upload.is_none());
        assert_eq!(library.get::<RtTrianglesSsbo>().unwrap().instance_count, 2);
        assert_eq!(
            library.get::<RtTriangleMetaSsbo>().unwrap().instance_count,
            2
        );
        assert_eq!(library.get::<RtBvhSsbo>().unwrap().instance_count, 3);
        upload_rt_world(&mut rt, &mut library, &device);
        assert_eq!(library.get::<RtTrianglesSsbo>().unwrap().instance_count, 2);
        assert_eq!(library.get::<RtBvhSsbo>().unwrap().instance_count, 3);
        rt.pending_upload = Some(RtUploadData::default());
        upload_rt_world(&mut rt, &mut library, &device);
        assert_eq!(library.get::<RtTrianglesSsbo>().unwrap().instance_count, 0);
        assert_eq!(
            library.get::<RtTriangleMetaSsbo>().unwrap().instance_count,
            0
        );
        assert_eq!(library.get::<RtBvhSsbo>().unwrap().instance_count, 0);
    }

    #[test]
    #[should_panic(expected = "Out of pre-allocated buffer space for ray tracing-data")]
    fn upload_rt_world_panics_when_actual_buffer_capacity_is_too_small() {
        use crate::render::{ssbo::rhapsody_boundary::Buffer, test_device::TestDevice};
        let device = TestDevice(0);
        let mut library = AssetSsboLibrary::new();
        // A bounded CPU failure fixture: a real typed owner with insufficient
        // byte capacity. The normal native constructor always allocates 6MiB.
        library.register::<RtTrianglesSsbo>(InstanceBuffer {
            instance_count: 0,
            buffer: TypedStorageBuffer::from_created_buffer(Buffer {
                byte_size: 24,
                ..Default::default()
            }),
        });
        let mut rt = RtWorld::default();
        rt.pending_upload = Some(RtUploadData {
            triangles: vec![RtTrianglesSsbo::default(); 2],
            ..Default::default()
        });
        upload_rt_world(&mut rt, &mut library, &device);
    }

    #[test]
    fn original_whole_upload_limit_logs_and_returns_before_library_lookup() {
        use crate::render::test_device::TestDevice;
        let mut library = AssetSsboLibrary::new();
        let mut rt = RtWorld::default();
        rt.pending_upload = Some(RtUploadData {
            metadata: vec![RtTriangleMetaSsbo::default(); 0x40000],
            ..Default::default()
        });
        // No registered buffer: an incorrect <= guard or panic fails this test.
        upload_rt_world(&mut rt, &mut library, &TestDevice(0));
        assert!(rt.pending_upload.is_none());
    }

    /// O：`update_rt_world` 0x1408bc650 的移除面 —— `remove_entry` + Arc drop。
    #[test]
    fn remove_mesh_drops_the_entry() {
        let mut rt = RtWorld::default();
        let h = RtWorld::handle(9);
        rt.get_mut(h).color = 8;
        assert!(rt.meshes.contains_key(&h));
        let removed = rt.remove_mesh(&h).expect("entry existed");
        assert_eq!(removed.inner.color, 8);
        assert!(rt.meshes.is_empty());
        rt.remove_meshes([&h]);
        assert!(rt.meshes.is_empty());
    }
}
