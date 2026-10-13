//! rt/mod.rs — 软件 BVH 光线追踪世界（WP16 逻辑重述；拆分骨架子路径）。
//!
//! O：`RtWorld`（HashMap<RtMeshHandle, DirtyCow<RtMesh>> + 移除/脏计数）、`RtMesh`
//! （ArcInner 0x60B；albedo 打包 + 顶点 Vec）、`DirtyCow<T>`（{arc,dirty}）、
//! `RtBvhBuilderGlueResource`（Arc<RwLock<Glue>>）。函数：get_mut<RtMeshHandle>
//! 0x141562aa0/0x1400cce50、get 0x141562cf0、update_rt_world 0x1408bc650、
//! upload_rt_world 0x1408bcff0、is_idle 0x1408bc530。

pub mod bvh;
pub mod bvh_builder;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use bevy_ecs::event::Event;
use bevy_ecs::resource::Resource;
use glam::{Vec3, Vec4};

use crate::boundary::WallId;
use crate::typed_gap::Gap;

/// `RtMeshHandle`（O：24B 枚举；tag@+0x0 + payload@+0x10）。`impl$2::from` tag=1、
/// `impl$5::from` 空。墙/楼梯/装饰共享一表。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum RtMeshHandle {
    Empty,
    Wall(WallId),
    Decorator(u64),
}

impl RtMeshHandle {
    /// `From<WallId>`（O：`impl$2::from` tag=1 + payload）。
    pub fn from_wall(wall_id: WallId) -> Self {
        RtMeshHandle::Wall(wall_id)
    }
    pub fn from_decorator(id: u64) -> Self {
        RtMeshHandle::Decorator(id)
    }
}

/// `RtMesh`（O：ArcInner 0x60B：{strong,weak} + RtMesh 0x50B）。默认值 O：
/// 矩阵单位、`+0x38 Vec2{1,1}`、vertices Vec cap=4。`albedo` 打包 u32
/// （`sqrt(c)*255+0.5` 各通道 clamp，`g<<8|r|b<<16`）。
#[derive(Clone, Debug)]
pub struct RtMesh {
    /// 打包 albedo（O：`impl$1::from` 0x1408bc460 线性 RGB→sRGB 打包）。
    pub albedo: u32,
    /// 顶点（O：Vec<Vec3>，+0x58 起 ptr/cap/len）。
    pub vertices: Vec<Vec3>,
    /// 变换/材质/标志（U；0x08..0x58，typed gap）。
    pub _gap: Gap,
}

impl Default for RtMesh {
    fn default() -> Self {
        Self {
            albedo: 0,
            vertices: Vec::new(),
            _gap: Gap::default(),
        }
    }
}

impl RtMesh {
    /// `set_albedo`（O：墙/楼梯构造内联）。
    pub fn set_albedo(&mut self, albedo: u32) {
        self.albedo = albedo;
    }
}

/// `DirtyCow<T>`（O：{ Arc<T> @+0x0, dirty: bool @+0x8 }；get_mut make_mut + 置脏）。
#[derive(Clone, Debug)]
pub struct DirtyCow<T> {
    pub arc: Arc<T>,
    pub dirty: bool,
}

impl<T: Clone> DirtyCow<T> {
    /// `get_mut`（O 0x15626f0：Arc::make_mut 语义 + 置脏）。
    pub fn get_mut(&mut self, make: impl FnOnce(&mut T)) {
        self.dirty = true;
        let arc = Arc::make_mut(&mut self.arc);
        make(arc);
    }
    /// `get`（O 名，内联）。
    pub fn get(&self) -> &T {
        &self.arc
    }
    /// `consume`（O 名，内联）。
    pub fn consume(self) -> T {
        Arc::try_unwrap(self.arc).unwrap_or_else(|a| (*a).clone())
    }
}

/// `RtWorld`（O：HashMap<RtMeshHandle, DirtyCow<RtMesh>> + 移除/脏计数簿记）。
#[derive(Resource, Clone, Debug, Default)]
pub struct RtWorld {
    pub meshes: HashMap<RtMeshHandle, DirtyCow<RtMesh>>,
    /// 移除/脏计数簿记（U；+0x10..+0x40）。
    pub counters: Gap,
}

impl RtWorld {
    /// `get_mut`（O 0x141562aa0/0x1400cce50）：查表；缺省建默认 RtMesh 并置脏。
    pub fn get_mut(&mut self, handle: RtMeshHandle) -> &mut RtMesh {
        let entry = self.meshes.entry(handle).or_insert_with(|| {
            let arc = Arc::new(RtMesh::default());
            // O：缺省即脏（置 dirty=true）。
            DirtyCow { arc, dirty: true }
        });
        entry.dirty = true;
        Arc::make_mut(&mut entry.arc)
    }
    /// `get`（O 0x141562cf0）。
    pub fn get(&self, handle: &RtMeshHandle) -> Option<&RtMesh> {
        self.meshes.get(handle).map(|c| c.get())
    }
    /// `remove`（O 名，3 内联）。
    pub fn remove(&mut self, handle: &RtMeshHandle) {
        self.meshes.remove(handle);
    }
    /// `is_dirty`（O 名，3 内联）。
    pub fn is_dirty(&self, handle: &RtMeshHandle) -> bool {
        self.meshes.get(handle).map(|c| c.dirty).unwrap_or(false)
    }
    /// `consume`（O 名，1 内联）。
    pub fn consume(&mut self, handle: &RtMeshHandle) -> Option<DirtyCow<RtMesh>> {
        self.meshes.remove(handle)
    }
    /// `reset`（O 名，1 内联）。
    pub fn reset(&mut self) {
        self.meshes.clear();
    }
    /// `generate_handle`（O 名，1 内联）。
    pub fn generate_handle(&mut self) -> RtMeshHandle {
        RtMeshHandle::Empty
    }
}

/// `GpuRtTriangle`（O：`new` 1 内联；上传紧凑三角形 24B 布局，U）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpuRtTriangle(pub Gap);

/// RT 三角形 SSBO（O：count×0x18）。
#[derive(Resource, Clone, Debug, Default)]
pub struct RtTrianglesSsbo(pub Gap);
/// RT 三角形元 SSBO（O：count×4）。
#[derive(Resource, Clone, Debug, Default)]
pub struct RtTriangleMetaSsbo(pub Gap);
/// RT BVH 节点 SSBO（O：count×0x50=80B）。
#[derive(Resource, Clone, Debug, Default)]
pub struct RtBvhSsbo(pub Gap);

/// `UpdateRtWorldEvent`（O：事件）。
#[derive(Event, Clone, Copy, Debug, Default, PartialEq)]
pub struct UpdateRtWorldEvent;

/// `RtBvhBuilderGlueResource`（O：Arc<RwLock<RtBvhBuilderGlue>>；is_idle 0x1408bc530）。
#[derive(Resource, Clone, Debug, Default)]
pub struct RtBvhBuilderGlueResource(pub Arc<RwLock<bvh_builder::RtBvhBuilderGlue>>);

impl RtBvhBuilderGlueResource {
    /// `new`（O 0x67dd80 折叠 stub；Glue::default）。
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(bvh_builder::RtBvhBuilderGlue::default())))
    }
    /// `is_idle`（O 0x1408bc530）。
    pub fn is_idle(&self) -> bool {
        self.0.read().map(|g| g.is_idle()).unwrap_or(true)
    }
}

/// `update_rt_world`（O 0x1408bc650）：puffin scope 内清已移除位图、把脏网格汇入工作
/// 队列并写脏标志。
pub fn update_rt_world(_world: &mut RtWorld, _glue: &RtBvhBuilderGlueResource) {
    // 业务逻辑 O（wp16）；系统级接线 U。
}

/// `upload_rt_world`（O 0x1408bcff0）：锁 glue，把三个输出缓冲上传
/// （RtTrianglesSsbo count×0x18 / RtTriangleMetaSsbo count×4 / RtBvhSsbo count×0x50）。
pub fn upload_rt_world(_world: &mut RtWorld, _glue: &RtBvhBuilderGlueResource) {
    // 超限日志 "Out of pre-allocated buffer space for ray tracing-data"（O）。
}
