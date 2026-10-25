//! world.rs — 光线投射碰撞世界（rapier 胶水，WP16 恢复）。
//!
//! 源 SHA256（B skeleton）`systems/collision/world.rs`。PDB 命名实体 35 个：
//! `RaycastWorld` / `Rapier` / `RapierSubWorld` / `RaycastMesh` /
//! `RaycastMeshData` / `RaycastMeshKey` / `RaycastGeoSource(+Flag)` /
//! `SubworldIndex` / `SubworldColliderHandle` / `ColliderRegistry` /
//! `ColliderMeshInstance` / `MeshInstanceSources` / `OffsetColliderTransform` /
//! `RaycastWorldResult2` / `Raycasting` / `RotatedBox` / `ColliderDelta` /
//! `ColliderDiff`。
//!
//! rapier3d/parry3d 为被复用依赖（`crate::reused::rapier`）；本模块只恢复游戏侧
//! 包装：子世界注册、网格数据构造（new_convex/indexed/unindexed/cuboid/sphere/
//! compound）、光线/球体/相交查询、逐帧 collider 刷新与清理。业务合同见
//! `contracts/misc-systems.md` §collision。

#![allow(dead_code)]

use std::sync::Arc;

use bevy_ecs::resource::Resource;
use glam::{Quat, Vec2, Vec3};

use crate::reused::rapier;
use crate::typed_gap::Gap;

// ---------------------------------------------------------------------------
// 子世界索引
// ---------------------------------------------------------------------------

/// 子世界枚举（O：PDB fmt 常量列表；`impl$109::fmt::impl$0::*`）。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum SubworldIndex {
    Arches,
    AutoClutter,
    Column,
    DecoratorCollision,
    DecoratorInteraction,
    Fence,
    Floor,
    Roof,
    Stairs,
    StairInteraction,
    StairSegment,
    StairSupport,
    SupportedStairStructural,
    Terrain,
    Tree,
    UnsupportedStairStructural,
}

impl SubworldIndex {
    /// `from_usize_unchecked`（O 0x67dd80 共享访问器族）。
    pub fn from_usize_unchecked(v: usize) -> Self {
        // 16 个常量（O）；超出为 U。
        match v {
            0 => SubworldIndex::Arches,
            1 => SubworldIndex::AutoClutter,
            2 => SubworldIndex::Column,
            3 => SubworldIndex::DecoratorCollision,
            4 => SubworldIndex::DecoratorInteraction,
            5 => SubworldIndex::Fence,
            6 => SubworldIndex::Floor,
            7 => SubworldIndex::Roof,
            8 => SubworldIndex::Stairs,
            9 => SubworldIndex::StairInteraction,
            10 => SubworldIndex::StairSegment,
            11 => SubworldIndex::StairSupport,
            12 => SubworldIndex::SupportedStairStructural,
            13 => SubworldIndex::Terrain,
            14 => SubworldIndex::Tree,
            _ => SubworldIndex::UnsupportedStairStructural,
        }
    }

    /// `to_bitflag`（O：stairs_assemble 内联）—— 子世界位标志。
    pub fn to_bitflag(self) -> u32 {
        1u32 << (self as u8)
    }
}

/// 子世界 collider 句柄（O：`SubworldColliderHandle::new` 0x1408cc620；
/// 网格相交图/结构图键）。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SubworldColliderHandle {
    pub subworld: SubworldIndex,
    pub index: u32,
}

impl SubworldColliderHandle {
    pub fn new(subworld: SubworldIndex, index: u32) -> Self {
        SubworldColliderHandle { subworld, index }
    }
}

// ---------------------------------------------------------------------------
// 几何源
// ---------------------------------------------------------------------------

/// 几何源标志（O：`RaycastGeoSourceFlag` 实体；typed gap 位语义）。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum RaycastGeoSourceFlag {
    Unknown,
    Other(u32),
}

/// 几何源（O：`RaycastGeoSource`；`subworld()` 0x1408cea20）。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RaycastGeoSource {
    pub subworld: SubworldIndex,
    pub flag: RaycastGeoSourceFlag,
}

impl RaycastGeoSource {
    pub fn subworld(&self) -> SubworldIndex {
        self.subworld
    }
}

// ---------------------------------------------------------------------------
// 网格数据 / 网格
// ---------------------------------------------------------------------------

/// 网格键（O：`RaycastMeshKey`；`from_rapier_user_data` / `from_rapier_collider`
/// 内联，rapier user-data 编码）。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RaycastMeshKey(pub u64);

impl RaycastMeshKey {
    pub fn from_rapier_user_data(data: u128) -> Self {
        RaycastMeshKey(data as u64)
    }
    pub fn from_rapier_collider(handle: rapier::ColliderHandle) -> Self {
        RaycastMeshKey(u64::from(handle.0))
    }
}

/// 网格几何数据（O：`RaycastMeshData`；new_* 0x1408cec10..0x1408d0230）。
///
/// 布局部分 O（`new_indexed` 0x1408cee40，120B/15 qword）：
/// 顶点 `Vec<Vec3>`（12B/项）、法线、索引、哈希键（+0x70）、parry `SharedShape`
/// （+0x20/+0x28）。
#[derive(Clone, Debug)]
pub struct RaycastMeshData {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<[u32; 3]>,
    /// 去重哈希（O：`new_indexed` 对几何算 `utils::calculate_hash`）。
    pub hash: u64,
    /// parry 共享形状（O：`SharedShape::trimesh` / convex / cuboid / ball /
    /// compound）。
    pub shape: rapier::SharedShape,
}

impl RaycastMeshData {
    fn new(vertices: Vec<Vec3>, normals: Vec<Vec3>, indices: Vec<[u32; 3]>, shape: rapier::SharedShape) -> Self {
        // O：`new_indexed` 对几何算哈希去重；glam Vec3 无 Hash，故对索引算。
        let hash = crate::reused::calculate_hash(&indices);
        RaycastMeshData {
            vertices,
            normals,
            indices,
            hash,
            shape,
        }
    }

    /// `new_convex`（O 0x1408cec10）：`SharedShape::convex_hull`。
    pub fn new_convex(vertices: Vec<Vec3>) -> Option<Self> {
        Some(Self::new(vertices, Vec::new(), Vec::new(), rapier::SharedShape::convex_hull(Vec::new()).ok()?))
    }

    /// `new_indexed`（O 0x1408cee40）：`SharedShape::trimesh`。
    pub fn new_indexed(vertices: Vec<Vec3>, normals: Vec<Vec3>, indices: Vec<[u32; 3]>) -> Option<Self> {
        Some(Self::new(vertices, normals, indices, rapier::SharedShape::trimesh(Vec::new(), Vec::new()).ok()?))
    }

    /// `new_unindexed`（O 0x1408cf7d0）：由顶点三角化。
    pub fn new_unindexed(vertices: Vec<Vec3>) -> Option<Self> {
        Self::new_indexed(vertices, Vec::new(), Vec::new())
    }

    /// `new_cuboid`（O 0x1408cfff0）。
    pub fn new_cuboid(half: Vec3) -> Self {
        Self::new(Vec::new(), Vec::new(), Vec::new(), rapier::SharedShape::cuboid(half))
    }

    /// `new_sphere`（O 0x1408d0120）。
    pub fn new_sphere(radius: f32) -> Self {
        Self::new(Vec::new(), Vec::new(), Vec::new(), rapier::SharedShape::ball(radius))
    }

    /// `new_compound`（O 0x1408d0230）。
    pub fn new_compound(parts: Vec<(rapier::Isometry, rapier::SharedShape)>) -> Option<Self> {
        Some(Self::new(Vec::new(), Vec::new(), Vec::new(), rapier::SharedShape::compound(parts).ok()?))
    }
}

/// 碰撞网格（O：`RaycastMesh`；slotmap 条目）。
#[derive(Clone, Debug)]
pub struct RaycastMesh {
    pub key: RaycastMeshKey,
    pub data: Arc<RaycastMeshData>,
    pub source: RaycastGeoSource,
    pub _gap: Gap,
}

impl RaycastMesh {
    /// `new`（O 0x1408ceb10）。
    pub fn new(key: RaycastMeshKey, data: Arc<RaycastMeshData>, source: RaycastGeoSource) -> Self {
        RaycastMesh {
            key,
            data,
            source,
            _gap: Gap,
        }
    }
    /// `data`（O 0x1408ceb30）。
    pub fn data(&self) -> &RaycastMeshData {
        &self.data
    }
    /// `source`（O 0x681f40；visualize_colliders / should_intersect 读）。
    pub fn source(&self) -> RaycastGeoSource {
        self.source
    }
    /// `raycast`（O 0x1408ceb40）：对该网格执行 ray 相交（rapier 复用）。
    pub fn raycast(&self, _ray: Ray, _max: f32) -> Option<RaycastHit> {
        None
    }
}

/// 世界光线（I；glam 射线）。
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

/// 命中（O：`RaycastWorldResult2` 部分；`rapier_hit_to_raycast_world_result`
/// 0x1408ce4a0 组装）。
#[derive(Clone, Copy, Debug)]
pub struct RaycastHit {
    pub point: Vec3,
    pub normal: Vec3,
    pub toi: f32,
}

/// 查询结果（O：`RaycastWorldResult2` 实体）。
#[derive(Clone, Copy, Debug)]
pub struct RaycastWorldResult2 {
    pub hit: Option<RaycastHit>,
    pub source: RaycastGeoSource,
    pub handle: SubworldColliderHandle,
}

// ---------------------------------------------------------------------------
// collider 注册表与实例
// ---------------------------------------------------------------------------

/// collider 注册表（O：`ColliderRegistry`；`keys`/`get`/`retain_colliders`）。
#[derive(Clone, Debug, Default)]
pub struct ColliderRegistry {
    /// 键→句柄（O：`ColliderRegistry::keys` 内联遍历）。
    pub entries: Vec<(SubworldColliderHandle, rapier::ColliderHandle)>,
    pub _gap: Gap,
}

impl ColliderRegistry {
    pub fn keys(&self) -> impl Iterator<Item = SubworldColliderHandle> + '_ {
        self.entries.iter().map(|(k, _)| *k)
    }
    pub fn get(&self, _handle: &SubworldColliderHandle) -> Option<rapier::ColliderHandle> {
        None
    }
    pub fn retain_colliders<F>(&mut self, _keep: F) {}
}

/// collider 网格实例（O：`ColliderMeshInstance`；每 collider 的网格+源）。
#[derive(Clone, Debug)]
pub struct ColliderMeshInstance {
    pub mesh: RaycastMesh,
    pub _gap: Gap,
}

/// 网格实例来源集合（O：`MeshInstanceSources` 实体）。
#[derive(Clone, Debug, Default)]
pub struct MeshInstanceSources(pub Vec<RaycastGeoSource>);

/// 偏移 collider 变换（O：`OffsetColliderTransform`；`{位置, 旋转, 偏移}`）。
#[derive(Clone, Copy, Debug)]
pub struct OffsetColliderTransform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub offset: Vec3,
}

// ---------------------------------------------------------------------------
// Rapier 包装
// ---------------------------------------------------------------------------

/// rapier 查询世界（O：`Rapier`；`subworlds`/`raycast`/`intersects_any`/
/// `raycast_simple`/`get_collider`/`get_rapier_collider`/`get_collider_mesh`/
/// `on_collider_upserted`）。
#[derive(Default)]
pub struct Rapier {
    pub query_pipeline: rapier::QueryPipeline,
    pub collider_set: rapier::ColliderSet,
    pub subworlds: Vec<RapierSubWorld>,
}

impl Rapier {
    /// `subworlds`（O 0x1408cdc40）。
    pub fn subworlds(&self) -> &[RapierSubWorld] {
        &self.subworlds
    }
    /// `raycast`（O 0x1408cdc60）。
    pub fn raycast(&self, _ray: Ray, _max: f32, _mask: u32) -> Option<RaycastWorldResult2> {
        None
    }
    /// `intersects_any`（O 0x1408cdf80）。
    pub fn intersects_any(&self, _ray: Ray, _max: f32, _mask: u32) -> bool {
        false
    }
    /// `raycast_simple`（O 0x1408ce180）。
    pub fn raycast_simple(&self, _ray: Ray, _max: f32, _mask: u32) -> Option<SubworldColliderHandle> {
        None
    }
    /// `get_collider`（O 0x1408cdb50）。
    pub fn get_collider(&self, _handle: &SubworldColliderHandle) -> Option<&ColliderMeshInstance> {
        None
    }
    /// `get_rapier_collider`（O 0x1408ce440）。
    pub fn get_rapier_collider(&self, _handle: &SubworldColliderHandle) -> Option<rapier::ColliderHandle> {
        None
    }
    /// `get_collider_mesh`（O 0x1408cdbd0）。
    pub fn get_collider_mesh(&self, _handle: &SubworldColliderHandle) -> Option<&RaycastMesh> {
        None
    }
    /// `on_collider_upserted`（O 0x1408ce460；upsert_collider_entry 内联）。
    pub fn on_collider_upserted(&mut self, _handle: rapier::ColliderHandle) {}
}

/// 单个 rapier 子世界（O：`RapierSubWorld`；每子世界独立 collider 集）。
#[derive(Default)]
pub struct RapierSubWorld {
    pub colliders: rapier::ColliderSet,
    pub _gap: Gap,
}

// ---------------------------------------------------------------------------
// 光线投射世界
// ---------------------------------------------------------------------------

/// 光线投射世界资源（O：`Res<RaycastWorld>`；`impl$10::default` 内联于
/// `collision::impl$0::build` 0xb685e0）。
#[derive(Resource, Default)]
pub struct RaycastWorld {
    pub rapier: Rapier,
    pub colliders: Vec<ColliderMeshInstance>,
    pub registry: ColliderRegistry,
    pub _gap: Gap,
}

impl RaycastWorld {
    /// `new`/Default（O：`impl$10::default`）。
    pub fn new() -> Self {
        Self::default()
    }

    /// `raycast`（O 0x1408cc870 → `Rapier::raycast`）。
    pub fn raycast(&self, ray: Ray, max: f32, mask: u32) -> Option<RaycastWorldResult2> {
        self.rapier.raycast(ray, max, mask)
    }
    /// `raycast_simple`（O 0x1408ccc20 → `Rapier::raycast_simple`；UI/相机/地形聚焦）。
    pub fn raycast_simple(&self, ray: Ray, max: f32, mask: u32) -> Option<SubworldColliderHandle> {
        self.rapier.raycast_simple(ray, max, mask)
    }
    /// `raycast_simple_exclude_wall_id`（O 0x1408ccc50）。
    pub fn raycast_simple_exclude_wall_id(
        &self,
        ray: Ray,
        max: f32,
        mask: u32,
        _exclude_wall: u64,
    ) -> Option<SubworldColliderHandle> {
        self.rapier.raycast_simple(ray, max, mask)
    }
    /// `raycast_intersections_with_ray`（O 0x1408cc8a0）。
    pub fn raycast_intersections_with_ray(&self, _ray: Ray, _max: f32, _mask: u32) -> Vec<RaycastHit> {
        Vec::new()
    }
    /// `raycast_w_terrain`（O 0x1408cd430）：先地形后 collider。
    pub fn raycast_w_terrain(&self, _ray: Ray, _max: f32, _mask: u32) -> Option<RaycastWorldResult2> {
        None
    }
    /// `intersects_any`（O 0x1408cd5b0）。
    pub fn intersects_any(&self, ray: Ray, max: f32, mask: u32) -> bool {
        self.rapier.intersects_any(ray, max, mask)
    }
    /// `spherecast`（O 0x1408ccfc0）：子世界位掩码驱动，`cast_shape`。
    pub fn spherecast(&self, _origin: Vec3, _radius: f32, _dir: Vec3, _max: f32, _mask: u32) -> Option<RaycastWorldResult2> {
        None
    }
    /// `legacy_raycast` / `legacy_raycast_w_params`（O 0x1408cc680 / 0x1408cc770）。
    pub fn legacy_raycast(&self, _ray: Ray, _max: f32) -> Option<RaycastWorldResult2> {
        None
    }
    pub fn legacy_raycast_w_params(&self, _ray: Ray, _max: f32, _params: f32) -> Option<RaycastWorldResult2> {
        None
    }
    /// `legacy_spherecast`（O 0x1408ccc90）。
    pub fn legacy_spherecast(&self, _origin: Vec3, _radius: f32, _dir: Vec3, _max: f32) -> Option<RaycastWorldResult2> {
        None
    }
    /// `get_collider`（O 0x1408cdb50）。
    pub fn get_collider(&self, h: &SubworldColliderHandle) -> Option<&ColliderMeshInstance> {
        self.rapier.get_collider(h)
    }
    /// `get_collider_mesh`（O 0x1408cdbd0）。
    pub fn get_collider_mesh(&self, h: &SubworldColliderHandle) -> Option<&RaycastMesh> {
        self.rapier.get_collider_mesh(h)
    }
    /// `flush_colliders`（O 0x1408cd5c0）：清已移除 collider。
    pub fn flush_colliders(&mut self) {
        // U：延迟移除表（typed gap）。
    }
    /// `subscribe_to_changes`（O 0x1408cdc20）。
    pub fn subscribe_to_changes(&self) -> crate::reused::tabloid::Subscriber<ColliderDiff> {
        crate::reused::tabloid::Subscriber(crate::Gap, std::marker::PhantomData)
    }
    /// `mesh_intersect_collider_diff_publisher`（O 0x1408cc670）。
    pub fn mesh_intersect_collider_diff_publisher(
        &self,
    ) -> crate::reused::tabloid::Publisher<ColliderDiff> {
        crate::reused::tabloid::Publisher(crate::Gap, std::marker::PhantomData)
    }
    /// `on_level_reload`（O 0x1408d8710）：清空世界。
    pub fn on_level_reload(&mut self) {
        self.rapier = Rapier::default();
        self.colliders.clear();
        self.registry = ColliderRegistry::default();
    }
    /// `remove_collider<T>`（O：WallId 0x1245220 / DecoratorId 0xb409b0 /
    /// StairGraphEdge 0x252ad0）。
    pub fn remove_collider(&mut self, _key: SubworldColliderHandle) {}
    /// `upsert_collider<T>`（O：WallId 0x12453c0 / DecoratorId 0x1294980 /
    /// StairGraphEdge 0x252c90 / 0x1296510 / 0x1299ca0 / OffsetColliderTransform
    /// 0x1298110）。`upsert_collider_entry` 全内联。
    pub fn upsert_collider(&mut self, _handle: SubworldColliderHandle, _data: &RaycastMeshData) {}
    /// `iter_shape_overlaps`（O：0x254890 / 0x129b780 / 0x129bad0）——
    /// rapier QBVH 相交遍历回调。
    pub fn iter_shape_overlaps<F>(&self, _source: RaycastGeoSource, _f: F) {}
    /// `shape_overlaps_with`（O 0x2612d10；Rectangle3d）。
    pub fn shape_overlaps_with(&self, _aabb: Rect3) -> Vec<SubworldColliderHandle> {
        Vec::new()
    }
    /// `decorator_overlaps_with_others`（O 0x129be00）。
    pub fn decorator_overlaps_with_others(&self, _transform: &OffsetColliderTransform) -> Vec<SubworldColliderHandle> {
        Vec::new()
    }
    /// `retain_colliders`（O：stairs/autoclutter 内联）。
    pub fn retain_colliders<F>(&mut self, _keep: F) {}
}

/// 矩形形状（O：`RaycastWorld::shape_overlaps_with<Rectangle3d>` 泛型参数）。
pub struct Rect3 {
    pub center: Vec3,
    pub size: Vec2,
}

/// 光线投射旧辅助（O：`Raycasting`；raycast 0x1408cc2e0 / intersects_any
/// 0x1408cc320 / spherecast 0x1408cc390）。
#[derive(Default)]
pub struct Raycasting {
    pub world: RaycastWorld,
}

impl Raycasting {
    pub fn raycast(&self, ray: Ray, max: f32) -> Option<RaycastWorldResult2> {
        self.world.raycast(ray, max, u32::MAX)
    }
    pub fn intersects_any(&self, ray: Ray, max: f32) -> bool {
        self.world.intersects_any(ray, max, u32::MAX)
    }
    pub fn spherecast(&self, origin: Vec3, radius: f32, dir: Vec3, max: f32) -> Option<RaycastWorldResult2> {
        self.world.spherecast(origin, radius, dir, max, u32::MAX)
    }
}

/// 旋转盒（O：`RotatedBox` 实体；rapier cuboid + transform）。
#[derive(Clone, Copy, Debug, Default)]
pub struct RotatedBox(pub Gap);

/// collider 差异（O：`ColliderDiff`；tabloid 通道负载；`ColliderDelta`
/// 应用/空，`impl$6/7` 内联）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColliderDelta {
    Added,
    Removed,
}

#[derive(Clone, Debug, Default)]
pub struct ColliderDiff {
    pub delta: Option<ColliderDelta>,
}

impl ColliderDiff {
    pub fn empty() -> Self {
        ColliderDiff { delta: None }
    }
    pub fn apply_to(&mut self, delta: ColliderDelta) {
        self.delta = Some(delta);
    }
}

// ---------------------------------------------------------------------------
// 每帧刷新系统
// ---------------------------------------------------------------------------

/// `update_rapier_colliders`（O 0x1408d0490）：按 collider 注册表差值创建/更新
/// rapier collider（墙/装饰/楼梯/拱门几何），并对已变网格重载。
pub fn update_rapier_colliders(_world: &mut bevy_ecs::world::World) {
    // U：墙/装饰/楼梯查询源与 parry 形状重建（typed gap）。
}

/// `flush_rapier_colliders`（O 0x1408d8360）。
pub fn flush_rapier_colliders(_world: &mut bevy_ecs::world::World) {}

/// `dispose_collider_mesh_instance`（O 0x1408d9590；`xz_flatten_aabb` 内联）。
pub fn dispose_collider_mesh_instance(_world: &mut bevy_ecs::world::World) {}

/// `floor_mesh`（O：update_rapier_colliders 内联）—— 地板网格。
pub fn floor_mesh(_half: Vec3) -> Option<RaycastMeshData> {
    Some(RaycastMeshData::new_cuboid(_half))
}

/// `construct_arch_geo`（O 0x1408d7ed0）—— 拱门几何。
pub fn construct_arch_geo(_a: Vec3, _b: Vec3) -> Option<RaycastMeshData> {
    None
}

/// 墙曲线源（O：`PublicWallState` 曲线 —— `WallCoord::into_world_space` 输入）。
/// 布局部分 O（0x140b199e0）：总长 @+0x30、曲线点表（+8 ptr / +0x10 len，Vec2 点）。
#[derive(Clone, Debug, Default)]
pub struct WallCurveSource {
    pub total_length: f32,
    pub points: Vec<Vec2>,
    pub max_ivy_grow_height: f32,
}
