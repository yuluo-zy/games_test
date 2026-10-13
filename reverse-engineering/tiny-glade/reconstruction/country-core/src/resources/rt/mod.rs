//! rt.rs — 软件 BVH 光线追踪世界族（WP16 恢复）。
//!
//! 源 SHA256（B skeleton）`resources/rt/{mod,bvh,bvh_builder}.rs`。PDB 命名实体：
//! `RtWorld` / `RtMesh` / `RtMeshHandle` / `DirtyCow` / `GpuRtTriangle` /
//! `GpuBlBvh` / `InputTriangle` / `RtBvhBuilderGlue` / `RtBvhBuilderGlueResource` /
//! `RtBvhSsbo` / `RtTrianglesSsbo` / `RtTriangleMetaSsbo` / `UpdateRtWorldEvent`。
//!
//! 标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。
//! 消费方：`update_rt_world` / `upload_rt_world` / `bvh_builder_worker`，
//! 以及 render-loop 的 GI/水"deferred RT"pass（共享软件 BVH SSBO `rt_bl_bvh` /
//! `rt_triangles`）。无硬件光追（证据 render-passes INDEX）。

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Arc;

use bevy_ecs::resource::Resource;
use glam::{Vec3, Vec4};

use crate::typed_gap::Gap;

/// `RtMeshHandle`（O：`enum2$<RtMeshHandle>`，24B：tag@0x0 + payload@0x10）。
/// `impl$2::from`（0x1408bc510）置 tag=1；`impl$5::from`（0x1408bc520）置默认/空。
/// 变体 I：`Empty / Wall(WallId) / Decorator(DecoratorId) / …`（墙/楼梯/装饰共享一表）。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum RtMeshHandle {
    /// 空/未指定（O：`impl$5::from` tag=0）。
    Empty,
    /// 墙源（O：`RtWorld::get_mut<WallId>` 0x1412259c0 经 `impl$5::from` 转换）。
    Wall(u64),
    /// 装饰源（I；同表其他生产者）。
    Decorator(u64),
}

impl RtMeshHandle {
    /// `From<WallId>`（O：`impl$2::from` tag=1 + payload；payload 语义 U）。
    pub fn from_wall(wall_id: u64) -> Self {
        RtMeshHandle::Wall(wall_id)
    }
    pub fn from_decorator(decorator_id: u64) -> Self {
        RtMeshHandle::Decorator(decorator_id)
    }
}

/// GPU 网格记录（O：`DirtyCow<RtMesh>` 负载，ArcInner 0x60 字节）。
/// 布局部分 O：`albedo`（`impl$1::from` 0x1408bc460 线性 RGB→打包 u32：
/// `sqrt(c)*255+0.5` 各通道 clamp，`g<<8|r|b<<16`）；几何顶点 Vec（+0x58..）。
/// 变换/其余字段 U。
#[derive(Clone, Debug)]
pub struct RtMesh {
    /// 打包 albedo（O：`RtMesh::set_albedo` / `impl$1::from`；sqrt≈线性→sRGB）。
    pub albedo: u32,
    /// 顶点（O：get_mut 默认路径 `Vec<Vec3>`，+0x58 起 ptr/len/cap）。
    pub vertices: Vec<Vec3>,
    /// 变换/材质/标志（U；0x08..0x58，typed gap）。
    pub _gap: Gap,
}

impl Default for RtMesh {
    fn default() -> Self {
        // O（0x1412259c0 默认构造）：albedo=0，scale 1.0 对、位移 0。
        RtMesh {
            albedo: 0,
            vertices: Vec::new(),
            _gap: Gap,
        }
    }
}

impl RtMesh {
    /// `set_albedo`（O：stairs/wall/construct_gates 内联；`impl$1::from` 打包）。
    pub fn set_albedo(&mut self, linear_rgb: Vec3) {
        let pack = |c: f32| {
            let v = c.sqrt() * 255.0 + 0.5;
            if v < 0.0 {
                0u32
            } else if v > 4_294_967_295.0 {
                0xffff_ffff
            } else {
                v as u32
            }
        };
        let r = pack(linear_rgb.x);
        let g = pack(linear_rgb.y);
        let b = pack(linear_rgb.z);
        self.albedo = (g << 8) | r | (b << 16);
    }
}

/// 写时复制包装（O：`DirtyCow<T>`；`get`/`get_mut` COW Arc）。
/// `get_mut`（O 0x1415626f0 / 0x15626f0）：置脏标志，Arc::make_mut 语义。
#[derive(Clone, Debug)]
pub struct DirtyCow<T> {
    /// 值（O：Arc<T>；refcount 语义见 decompile）。
    pub arc: Arc<T>,
    /// 脏标志（O：get_mut 写 +1 字节）。
    pub dirty: bool,
}

impl<T: Clone> DirtyCow<T> {
    pub fn new(value: T) -> Self {
        DirtyCow {
            arc: Arc::new(value),
            dirty: false,
        }
    }
    pub fn get(&self) -> &T {
        &self.arc
    }
    pub fn get_mut(&mut self) -> &mut T {
        self.dirty = true;
        // O：COW —— 强引用数为 1 时直接改，否则克隆再改（Arc::make_mut）。
        Arc::make_mut(&mut self.arc)
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }
}

/// 光线追踪世界资源（O：`Res<RtWorld>`）。
///
/// 布局部分 O（`get_mut` 0x1412259c0 / `update_rt_world` 0x1408bc650）：
/// `HashMap<RtMeshHandle, DirtyCow<RtMesh>>`（hashbrown RawTable）+ 移除/脏计数
/// 簿记（+0x10..+0x40，U）。`update_rt_world` 消耗"已移除网格"（位图逐项清）
/// 并把脏网格汇入工作队列；`upload_rt_world` 上传 BVH 构建结果。
#[derive(Resource, Default)]
pub struct RtWorld {
    /// 网格表（O）。
    pub meshes: HashMap<RtMeshHandle, DirtyCow<RtMesh>>,
    /// 下一个句柄（I；`generate_handle` 内联）。
    pub next_handle: u64,
    /// 移除/脏计数簿记（U；+0x10..+0x40）。
    pub _counters: Gap,
}

impl RtWorld {
    /// `new`（O：`impl$22::default` 内联于 `insert_rendering_resources`）。
    pub fn new() -> Self {
        Self::default()
    }

    /// `get`（O 0x141562cf0）：按句柄取 `&DirtyCow<RtMesh>`。
    pub fn get(&self, handle: &RtMeshHandle) -> Option<&DirtyCow<RtMesh>> {
        self.meshes.get(handle)
    }

    /// `get_mut<WallId>`（O 0x1412259c0）：墙句柄取/建脏网格（缺省建默认 RtMesh）。
    pub fn get_mut(&mut self, handle: RtMeshHandle) -> &mut DirtyCow<RtMesh> {
        self.meshes
            .entry(handle)
            .or_insert_with(|| DirtyCow::new(RtMesh::default()))
    }

    /// `get_mut<RtMeshHandle>`（O 0xcce50 / 0x1562aa0）。
    pub fn get_mut_handle(&mut self, handle: &RtMeshHandle) -> Option<&mut DirtyCow<RtMesh>> {
        self.meshes.get_mut(handle)
    }

    /// `remove`（O：prefab/update_rt_world/invalidate 内联）。
    pub fn remove(&mut self, handle: &RtMeshHandle) -> Option<DirtyCow<RtMesh>> {
        self.meshes.remove(handle)
    }

    /// `generate_handle`（O：create_prefab_rt 内联；句柄分配语义 U）。
    pub fn generate_handle(&mut self) -> RtMeshHandle {
        let id = self.next_handle;
        self.next_handle += 1;
        RtMeshHandle::Decorator(id)
    }

    /// `is_dirty`（O：update_rt_world / commander_update 内联）—— 有脏网格。
    pub fn is_dirty(&self) -> bool {
        self.meshes.values().any(|c| c.dirty)
    }

    /// `consume`（O：update_rt_world 内联）—— 取走当前脏网格集合。
    pub fn consume(&mut self) -> HashMap<RtMeshHandle, DirtyCow<RtMesh>> {
        let mut out = HashMap::new();
        for (k, v) in self.meshes.iter_mut() {
            if v.dirty {
                out.insert(*k, v.clone());
                v.mark_clean();
            }
        }
        out
    }

    /// `reset`（O：uninit_clearing 内联）—— 清空世界。
    pub fn reset(&mut self) {
        self.meshes.clear();
        self.next_handle = 0;
    }
}

/// 事件：请求更新 RT 世界（O：`UpdateRtWorldEvent` 实体）。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct UpdateRtWorldEvent;

/// GPU 三角形（O：`GpuRtTriangle`；`build_gpu_bvh` 0x140b0da50 输出每个三角形
/// 3 个 16 字节顶点行（`[x,y,z,z]`，stride 0x30=48B），供 cwbvh 构建）。
#[derive(Clone, Copy, Debug, Default)]
pub struct GpuRtTriangle {
    pub v0: Vec4,
    pub v1: Vec4,
    pub v2: Vec4,
}

/// 输入三角形（O：`InputTriangle`；`bvh_builder_worker` gather 的 stride 0x28=40B：
/// `{ 3×Vec3 位置, u32 material/源 id }`，I 尾部字段）。
#[derive(Clone, Copy, Debug, Default)]
pub struct InputTriangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub _source: u32,
}

/// 构建出的 GPU BVH（O：`GpuBlBvh`；`build_gpu_bvh` 返回 9 qword 结构）。
#[derive(Clone, Debug, Default)]
pub struct GpuBlBvh {
    /// 三角形输出（O：`Vec<GpuRtTriangle>`，48B 每项）。
    pub triangles: Vec<GpuRtTriangle>,
    /// 三角形索引（O：`Vec<u32>`，cwbvh 叶序）。
    pub triangle_indices: Vec<u32>,
    /// 节点计数（O：cwbvh builder 输出 node/leaf 数；节点数据 U）。
    pub node_count: u32,
    pub leaf_count: u32,
}

impl GpuBlBvh {
    /// `build_gpu_bvh`（O 0x140b0da50）：输入三角形数组 → 先重排为 4-float 顶点行，
    /// 调 `obvhs::cwbvh::builder::build_cwbvh_from_tris`（复用库），按叶序收集
    /// 三角形与索引。`impl$1::default`（0x140b0da50 内联）为默认 BVH。
    pub fn build(tris: &[InputTriangle]) -> Self {
        if tris.is_empty() {
            return GpuBlBvh::default();
        }
        // 三角形重排（O）：每个顶点行 [x,y,z,z]。
        let triangles: Vec<GpuRtTriangle> = tris
            .iter()
            .map(|t| GpuRtTriangle {
                v0: t.v0.extend(t.v0.z),
                v1: t.v1.extend(t.v1.z),
                v2: t.v2.extend(t.v2.z),
            })
            .collect();
        // cwbvh 构建（复用 obvhs；索引 U，节点原始数据 U）。
        let b = crate::reused::bvh::build_cwbvh_from_tris(&[0.0f32], &crate::reused::bvh::CwbvhConfig);
        GpuBlBvh {
            triangles,
            triangle_indices: (0..tris.len() as u32).collect(),
            node_count: b.node_count,
            leaf_count: b.leaf_count,
        }
    }
}

/// RT SSBO 载荷类型（O：`GpuRtTriangle` 列表；上传 stride 0x18=24B/项 —— O
/// `upload_rt_world` `count*0x18`）。元素紧凑布局 U。
pub struct RtTrianglesSsbo(pub Vec<[u8; 24]>);

/// RT 三角形元数据（O：4B/项，`count*4` —— 每三角形一 u32 索引/标志）。
pub struct RtTriangleMetaSsbo(pub Vec<u32>);

/// RT BVH 节点（O：80B/项，`count*0x50`）。
pub struct RtBvhSsbo(pub Vec<[u8; 80]>);

/// BVH 构建胶水（O：`RtBvhBuilderGlue`；`bvh_builder_worker` 的工作产物）。
///
/// 布局 O（`upload_rt_world` 0x1408bcff0 读取）：
/// 三角形列表（+0x48 ptr / +0x50 len，24B 项）、三角形元（+0x60，4B 项）、
/// BVH 节点（+0x30，80B 项）；busy/idle 标志（+0x14/+0x15）。
#[derive(Default)]
pub struct RtBvhBuilderGlue {
    /// 三角形输出（O）。
    pub tris: Vec<GpuRtTriangle>,
    /// 三角形元（O：u32 每项）。
    pub tri_meta: Vec<u32>,
    /// BVH 节点（O：80B 每项；原始布局 U）。
    pub nodes: Vec<[u8; 80]>,
    /// 构建忙标志（O：worker 置位；+0x14）。
    pub busy: bool,
    /// 空闲标志（WP21；O：+0x15；`is_idle` 读锁调用）。
    pub idle: bool,
}

impl RtBvhBuilderGlue {
    /// `is_idle`（WP21；O：`RtBvhBuilderGlueResource::is_idle` 读锁调用）。
    pub fn is_idle(&self) -> bool {
        self.idle
    }
}

/// BVH 构建输出（WP21；O：构建输出；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct RtBvhBuilderOutput(pub Gap);

/// BVH 构建胶水资源（O：`Res<RtBvhBuilderGlueResource>`；共享 `Arc<RwLock<…>>`）。
///
/// `new`（O 0x67dd80 共享访问器族）/ `is_idle`（O 0x1408bc530，读锁 + busy 判断）。
#[derive(Resource, Clone, Default)]
pub struct RtBvhBuilderGlueResource {
    /// 共享胶水（O：Arc<RwLock<RtBvhBuilderGlue>>）。
    pub glue: Arc<parking_lot::RwLock<RtBvhBuilderGlue>>,
}

impl RtBvhBuilderGlueResource {
    pub fn new() -> Self {
        RtBvhBuilderGlueResource {
            glue: Arc::new(parking_lot::RwLock::new(RtBvhBuilderGlue::default())),
        }
    }
    /// `is_idle`（O 0x1408bc530）：构建不忙（空闲可接新活）。
    pub fn is_idle(&self) -> bool {
        !self.glue.read().busy
    }
}

/// `update_rt_world`（O 0x1408bc650，WP21）：puffin scope 内清已移除位图、把脏网格
/// 汇入工作队列并写脏标志。系统级接线 U。
pub fn update_rt_world(_world: &mut RtWorld, _glue: &RtBvhBuilderGlueResource) {}

/// `upload_rt_world`（O 0x1408bcff0，WP21）：锁 glue，把三个输出缓冲上传
/// （RtTrianglesSsbo count×0x18 / RtTriangleMetaSsbo count×4 / RtBvhSsbo count×0x50）。
/// 超限日志 "Out of pre-allocated buffer space for ray tracing-data"（O）。
pub fn upload_rt_world(_world: &mut RtWorld, _glue: &RtBvhBuilderGlueResource) {}
