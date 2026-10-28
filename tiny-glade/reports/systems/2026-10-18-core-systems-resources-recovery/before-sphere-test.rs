//! `systems/collision/world.rs` — rapier3d 碰撞世界（B 170 函数，核心查询族 O）。
//!
//! 证据：`logic-collision` fresh 反编译（ghidra shared 只读，2026-10-17）+ `collision-eco/ALGORITHMS.md`：
//! - `RaycastWorld`：17 子世界槽**内联**@+0x20（每槽 0x390B，含 RigidBodySet@+0x0 /
//!   ColliderSet@+0x30 / QueryPipeline / 脏标志@+0x388），`Publisher<ColliderDiff>`@+0x3cb0
//!   （`mesh_intersect_collider_diff_publisher` 0x1408cc670 机器码 `lea rax,[rcx+0x3cb0]; ret`）；
//! - `RaycastWorld::raycast`（0x1408cc870）→ `Rapier::raycast`（0x1408cdc60）：17 槽 **u32** mask
//!   遍历，每槽 `QueryPipeline::cast_ray_and_get_normal`，**逐槽收紧 max_toi，保留最近命中；等距后槽覆盖**，
//!   返回 `RaycastWorldResult`（~0x40B：handle/地形标记@+0x00、position@+0x18、normal@+0x24、
//!   toi@+0x30、判别@+0x34【0=命中/2=未命中】、额外@+0x38；地形命中首字节 0xb）；
//! - `Raycasting::raycast`（0x1408cc2e0）→ `RaycastWorld::raycast_w_terrain`（0x1408cd430，含
//!   地形 0x20 位分支）；`Raycasting::intersects_any`（0x1408cc320）＝地形 0x20 短路 +
//!   `Rapier::intersects_any`（0x1408cdf80，Qbvh `traverse_depth_first_node` +
//!   `RayIntersectionsVisitor`，命中即返回）；`Raycasting::spherecast`（0x1408cc390）＝
//!   `RaycastWorld::spherecast`（0x1408ccfc0，`cast_shape` 17 槽）+ 地形合并（地形 toi =
//!   `(dist - radius) / max(dir_len, 4e-8)`，NaN 语义同机器码）；
//! - `RaycastMeshData::new_convex`（0x1408cec10）：`calculate_hash<&[UVec3]>` 存 hash@+0x70、
//!   `try_convex_hull` + `ConvexPolyhedron::from_convex_mesh` → `Box<dyn Shape>`@+0x20/+0x28、
//!   tag=4@+0、kind=4@+0x10、+0x68=0x3f80000000000000；
//! - `SubworldColliderHandle::new`（0x1408cc620）：{u32@+0 index, u32@+4 gen, u8@+8 slot}，
//!   24 B 枚举。
//!
//! rapier 查询在 `native-physics` 特征下实现；非该特征用类型壳（查询不落地）。
//! 本轮（impl-collision-deep，2026-10-17）新增：`RaycastMeshData` 布局锚定 + 构造器
//! 修正（new_unindexed/new_cuboid/new_sphere/new_compound）、`RaycastGeoSource`/
//! `RaycastMesh` 类型族、`rapier_hit_to_raycast_world_result`（barycentric 插值）、
//! dirty 瓦片生命周期（`flush_colliders`/`flush_rapier_colliders`/`subscribe_to_changes`）、
//! collider 生命周期泛型（`upsert_collider`/`remove_collider`/`dispose_collider_mesh_instance`）。
//! 仍登记（REBUILD-DEEP.md / REMANDER2）：`update_rapier_colliders`、`on_level_reload`、
//! `construct_arch_geo`、`to_offset_collider_builder`/`into_transformed_shape` 族、
//! `iter_shape_overlaps`/`shape_overlaps_with`/`decorator_overlaps_with_others`、
//! `upsert_collider_entry` 的注册表插入接线、`Rapier::spherecast` 命中法线方向修正、
//! `Rapier::raycast` 的 `rapier_hit_to_raycast_world_result` barycentric 链接等。
#[cfg(feature = "native-physics")]
use rapier3d::prelude::*;

use bevy_ecs::prelude::Resource;
use std::sync::Arc;
use tabloid::Publisher;
#[cfg(feature = "native-physics")]
use utils::geometry::Ray as TerrainRay;

use crate::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
use crate::systems::collision::EMPTY_COLLIDER_HANDLE_TAG;
#[cfg(feature = "native-physics")]
use crate::systems::collision::SUBWORLD_SLOT_COUNT;
use utils::geometry::aabb::{Aabb2, Aabb3};

/// `xz_flatten_aabb`（0x1408d98a0）：把 3D AABB 压到 XZ 平面（丢弃 y），
/// 返回 `Aabb2 { min: (min.x, min.z), max: (max.x, max.z) }`。
///
/// 机器码逐条对应（`disassemble_function` 2026-10-17）：`MOVSS XMM0,[RDX]`（min.x）、
/// `MOVSD XMM0,[RDX+8]`（min.z,max.x）、`MOVSS XMM1,[RDX+0x14]`（max.z）→
/// 输出 {+0:min.x, +4:min.z, +8:max.x, +0xc:max.z}。
pub fn xz_flatten_aabb(aabb: &Aabb3) -> Aabb2 {
    Aabb2 {
        min: glam::Vec2::new(aabb.min.x, aabb.min.z),
        max: glam::Vec2::new(aabb.max.x, aabb.max.z),
    }
}

/// `RaycastWorldResult` 判别：命中。
pub const RAYCAST_WORLD_RESULT_HIT: u32 = 0;
/// `RaycastWorldResult` 判别：未命中（O：+0x34）。
pub const RAYCAST_WORLD_RESULT_MISS: u32 = 2;
/// `RaycastWorldResult` 地形命中标记（O：+0x00 首字节）。
pub const RAYCAST_WORLD_RESULT_TERRAIN: u8 = 0xb;

/// `SubworldColliderHandle`（O：0x1408cc620 写 {u32@+0 index, u32@+4 gen, u8@+8 slot}，
/// 24 B 枚举；空判别 0x11 = 槽数）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SubworldColliderHandle {
    pub index: u32,
    pub generation: u32,
    pub slot: u8,
}

/// `impl$3::partial_cmp`（0x1408cc5d0，O）：**只按 +8 槽字节比较**（Less/Equal/Greater），
/// 忽略 index/generation。fresh 机器码：`cVar1 = *(in_RCX+8) != *(in_RDX+8);
/// if (*(in_RCX+8) < *(in_RDX+8)) cVar1 = -1; return cVar1;` —— 即先判 `<`，不等则返回
/// `!=` 的 bool（槽大 → 1 / 槽小 → -1）。`derive(PartialOrd)` 按字段序（index→generation→slot）
/// 与机器语义不符，故手写为仅槽字节比较。
impl PartialOrd for SubworldColliderHandle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.slot.cmp(&other.slot))
    }
}

impl Ord for SubworldColliderHandle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.slot.cmp(&other.slot)
    }
}

impl Default for SubworldColliderHandle {
    fn default() -> Self {
        Self {
            index: u32::MAX,
            generation: u32::MAX,
            slot: EMPTY_COLLIDER_HANDLE_TAG,
        }
    }
}

impl SubworldColliderHandle {
    /// 0x1408cc620：`SubworldColliderHandle::new`（判等，参数位置与原 ABI 不同但字段映射一致）。
    pub fn new(index: u32, generation: u32, slot: u8) -> Self {
        Self {
            index,
            generation,
            slot,
        }
    }
}

/// `RaycastWorldResult`（O：~0x40B）。判别语义：+0x34 = 0 命中 / 2 未命中；地形命中时 +0x00
/// 首字节 = 0xb（此时 handle 区无 rapier 句柄）。`#[repr(C)]` 使字段偏移与机器码一致：
/// handle 24B@+0x00、position@+0x18、normal@+0x24、toi@+0x30、判别@+0x34、额外@+0x38。
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct RaycastWorldResult {
    /// +0x00..+0x18：命中 = **注册表条目前 24 字节**（O：`rapier_hit_to_raycast_world_result`
    /// `*in_RCX = *in_stack_00000028` 三 qword 拷贝）；地形命中时首字节 0xb。
    pub handle: [u64; 3],
    /// +0x18。
    pub position: glam::Vec3,
    /// +0x24（地形命中 = (0, 1, 0)）。
    pub normal: glam::Vec3,
    /// +0x30。
    pub toi: f32,
    /// +0x34：0 = 命中，2 = 未命中。
    pub discriminant: u32,
    /// +0x38。
    pub extra: [f32; 2],
}

impl RaycastWorldResult {
    /// 未命中结果（O：判别 2；其余字段原版未写，Rust 侧零值）。
    pub fn miss() -> Self {
        Self {
            handle: [0; 3],
            position: glam::Vec3::ZERO,
            normal: glam::Vec3::ZERO,
            toi: 0.0,
            discriminant: RAYCAST_WORLD_RESULT_MISS,
            extra: [0.0; 2],
        }
    }

    pub fn is_hit(&self) -> bool {
        self.discriminant == RAYCAST_WORLD_RESULT_HIT
    }

    pub fn is_miss(&self) -> bool {
        self.discriminant == RAYCAST_WORLD_RESULT_MISS
    }

    /// 地形命中（+0x00 首字节 0xb）。
    pub fn is_terrain(&self) -> bool {
        self.handle[0] as u8 == RAYCAST_WORLD_RESULT_TERRAIN
    }

    /// 命中槽号（+0x08 低字节）。
    pub fn slot(&self) -> u8 {
        self.handle[1] as u8
    }

    /// 旧版把句柄区解成 rapier 句柄的辅助。刷新后句柄区是**注册表条目前 24 B**
    /// （见 `handle` 字段），此方法不再对应真实句柄，语义登记；调用方应读原始字节。
    pub fn collider_handle(&self) -> SubworldColliderHandle {
        SubworldColliderHandle {
            index: self.handle[0] as u32,
            generation: (self.handle[0] >> 32) as u32,
            slot: self.handle[1] as u8,
        }
    }
}

/// `RaycastSimpleResult`（O：`Rapier::raycast_simple` 0x1408ce180 输出）。
/// 布局：+0x00..+0x18 句柄区、+0x18 position、+0x24 toi；**未命中只写 +0x00 首字节 0x11**
/// （= 槽数，O：`*(undefined1 *)in_RCX = 0x11`）。
///
/// 命中时句柄区 = **注册表条目的前 24 字节**（`in_RCX[0..2] = puVar1[0..2]`，条目
/// stride 0x28），不是 rapier 句柄打包。Rust 侧以 [`registry_handle_region`] 拷贝。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RaycastSimpleResult {
    /// +0x00..+0x18：命中 = 注册表条目前 24 B；未命中首字节判别 `EMPTY_COLLIDER_HANDLE_TAG`（0x11）。
    pub handle: [u64; 3],
    /// +0x18。
    pub position: glam::Vec3,
    /// +0x24。
    pub toi: f32,
}

impl RaycastSimpleResult {
    /// 未命中结果（O：+0x00 首字节 0x11；其余字段原版未写，Rust 侧零值）。
    pub fn miss() -> Self {
        Self {
            handle: [u64::from(EMPTY_COLLIDER_HANDLE_TAG), 0, 0],
            position: glam::Vec3::ZERO,
            toi: 0.0,
        }
    }

    /// 未命中判别（O：首字节 == 0x11）。
    pub fn is_miss(&self) -> bool {
        self.handle[0] as u8 == EMPTY_COLLIDER_HANDLE_TAG
    }

    /// 命中槽号（O 注：句柄区来自注册表条目，+0x08 低字节为条目 meta 区字节；
    /// 名称沿用调用方约定，语义登记）。
    pub fn slot(&self) -> u8 {
        self.handle[1] as u8
    }
}

/// 把 rapier `ColliderHandle` 写成结果句柄区（O：{index@+0, gen@+4, slot@+8}）。
#[cfg(feature = "native-physics")]
fn handle_region(handle: ColliderHandle, slot_index: u32) -> [u64; 3] {
    let (index, generation) = handle.into_raw_parts();
    [
        u64::from(index) | (u64::from(generation) << 32),
        u64::from(slot_index as u8),
        0,
    ]
}

/// `Rapier::raycast` / `Rapier::raycast_simple` 命中的**结果句柄区**。
///
/// fresh（0x1408cdc60 / 0x1408ce180）：命中后先 `ColliderSet::get_mut_internal` 取句柄，
/// 再以 `(index, generation)` 校验注册表条目（`entry.generation | 1 == generation | 1`，
/// 失败 `begin_panic`），随后把**注册表条目的前 24 字节**整块拷进结果 +0x00..+0x18
/// （`in_RCX[0..2] = puVar1[0..2]`，条目 stride 0x28）。因此句柄区不是 rapier 句柄打包，
/// 而是 `ColliderRegistryEntry` 的前 24 字节（kind/meta 区）。
///
/// 解注册表时的 `generation` 失配按 fresh panic；**条目缺失**（注册表尚未接线）返回
/// `None`，调用方回退到旧句柄打包——登记项：`upsert_collider_entry` 的注册接线未落地
/// （`upsert_collider` 0x140b40b50 的 registry 插入面未锚定）。
#[cfg(feature = "native-physics")]
fn registry_handle_region(world: &RaycastWorld, handle: ColliderHandle) -> Option<[u64; 3]> {
    let (index, generation) = handle.into_raw_parts();
    let entry = world.colliders.get(index as usize)?;
    if entry.generation | 1 != generation | 1 {
        panic!("get_collider: collider handle invalid");
    }
    let mut region = [0u64; 3];
    // SAFETY: `ColliderRegistryEntry` 是 `#[repr(C)]` 0x28 B；前 24 字节按字节拷出，
    // 避免条目对齐（4）与 `[u64;3]`（8）不一致。
    unsafe {
        core::ptr::copy_nonoverlapping(
            entry as *const ColliderRegistryEntry as *const u8,
            region.as_mut_ptr() as *mut u8,
            24,
        );
    }
    Some(region)
}

/// 地形命中结果（O：+0x00 首字节 0xb、position@+0x18 = origin + dir*toi、normal=(0,1,0)@+0x24、
/// toi@+0x30、判别 0@+0x34）。
#[cfg(feature = "native-physics")]
fn terrain_result(toi: f32, origin: glam::Vec3, dir: glam::Vec3) -> RaycastWorldResult {
    RaycastWorldResult {
        handle: [u64::from(RAYCAST_WORLD_RESULT_TERRAIN), 0, 0],
        position: origin + dir * toi,
        normal: glam::Vec3::Y,
        toi,
        discriminant: RAYCAST_WORLD_RESULT_HIT,
        extra: [0.0, 0.0],
    }
}

/// `ColliderDiff`（O：`on_collider_upserted` 0x1408ce460 / `dispose_collider_mesh_instance`
/// 0x1408d9590 / `upsert_collider_entry` 0x140b40b50 发布 `publish<ColliderDiff, ColliderDelta>`
/// 的 tabloid State）。
///
/// 载荷锚定（fresh `find_mesh_mesh_intersections` 0x140a97b70，2026-10-17）：
/// - `Subscriber::take<ColliderDiff>()` 后逐 **removed** 句柄 `GraphMap::remove_node`（图剪枝）；
/// - 再逐句柄（kind<2 门控）`ColliderSet::get_mut_internal` + `compute_aabb` + `SimdAabb::splat`
///   + 候选槽 `Qbvh::traverse_depth_first_node<…, BoundingVolumeIntersectionsVisitor<…,
///   intersect_newly_added_colliders::closure_env_0>>`（新增 collider 的 AABB 相交装配）。
/// 故载荷 = {removed: 移除句柄列表, added: 新增/变更句柄列表}（add/remove 双列表，登记）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ColliderDiff {
    /// 被移除的 collider 句柄（O：find_mesh 图剪枝消费）。
    pub removed: Vec<SubworldColliderHandle>,
    /// 新增/变更的 collider 句柄（O：find_mesh 逐句柄 AABB + Qbvh 遍历消费）。
    pub added: Vec<SubworldColliderHandle>,
}

impl tabloid::State for ColliderDiff {
    fn empty() -> Self {
        Self::default()
    }
}

/// `ColliderDelta`（O：`ColliderDiff` 的增量）。`dispose_collider_mesh_instance` 发布
/// removed=true 的句柄；`upsert_collider_entry` 发布 removed=false 的句柄。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColliderDelta {
    /// 变更句柄；None 为无载荷发布（等价空增量）。
    pub handle: Option<SubworldColliderHandle>,
    /// true = 移除（dispose），false = 新增/变更（upsert）。
    pub removed: bool,
}

impl tabloid::StateDelta<ColliderDiff> for ColliderDelta {
    fn apply_to(&self, state: &mut ColliderDiff) {
        if let Some(h) = self.handle {
            if self.removed {
                state.removed.push(h);
            } else {
                state.added.push(h);
            }
        }
    }
}

/// `RaycastMeshData`（O：0x78B；布局按 fresh `new_indexed` 0x1408cee40 /
/// `new_unindexed` 0x1408cf7d0 的输出 store 序列重新锚定，2026-10-17）：
///
/// | +0x00 | +0x08 | +0x10 | +0x18 | +0x20/+0x28 | +0x30 | +0x38 | +0x40 | +0x48 | +0x50..+0x68 | +0x70 |
/// |---|---|---|---|---|---|---|---|---|---|---|---|
/// | 顶点 ptr | 顶点 len | 索引 ptr | 索引 len（计数） | shape Box<dyn Shape> | 法线 ptr | 法线 len | uv ptr | uv len | 常量区（0/0/0/1.0） | hash |
///
/// 关键修正（fresh）：
/// - 缓冲区是 **ptr+len**（无 cap 槽）；原输入 Vec 在 store 前被 shrink 到 cap==len；
/// - `+0x10` 是**索引缓冲 ptr**（旧实现误作顶点 cap）；`+0x18` = 索引数；
/// - `+0x40/+0x48` 是 uv ptr/len（旧实现把 uv 放在 +0x48/+0x50）；
/// - `+0x50/+0x58/+0x60 = 0`、`+0x68 = 0x3f80000000000000`（常量 0x142aa3ec0/0x142aa3ed0
///   已 read_memory 核对）。
///
/// 形状变体（`new_convex`/`new_cuboid`/`new_sphere`/`new_compound`）复用同布局：
/// +0x00=tag（4）、+0x10=kind（4）、+0x50..+0x68 存形状参数，hash@+0x70。
#[cfg(feature = "native-physics")]
#[repr(C)]
pub struct RaycastMeshData {
    /// +0x00 顶点缓冲 ptr（形状变体 = tag 4）。
    pub vertices_ptr: u64,
    /// +0x08 顶点缓冲 len（形状变体 = 0）。
    pub vertices_len: u64,
    /// +0x10 索引缓冲 ptr（形状变体 = kind 4）。
    pub indices_ptr: u64,
    /// +0x18 索引 len / 计数（形状变体 = 0）。
    pub count: u64,
    /// +0x20/+0x28：`Box<dyn Shape>`（数据 ptr + vtable）。
    pub shape: Option<Box<dyn Shape>>,
    /// +0x30 法线（逐角点）缓冲 ptr。
    pub normals_ptr: u64,
    /// +0x38 法线缓冲 len。
    pub normals_len: u64,
    /// +0x40 uv 缓冲 ptr。
    pub uvs_ptr: u64,
    /// +0x48 uv 缓冲 len。
    pub uvs_len: u64,
    /// +0x50 常量/形状参数（网格变体写 0）。
    pub _pad0: u64,
    /// +0x58 常量/形状参数（网格变体写 0）。
    pub _pad1: u64,
    /// +0x60 常量/形状参数（网格变体写 0）。
    pub _pad2: u64,
    /// +0x68 scale（网格变体 = 0x3f80000000000000，即 1.0）。
    pub scale: u64,
    /// +0x70 几何哈希。
    pub hash: u64,
}

/// `RaycastMeshData`（非 native-physics 壳：无形状句柄，仅布局信息）。
#[cfg(not(feature = "native-physics"))]
#[repr(C)]
pub struct RaycastMeshData {
    pub vertices_ptr: u64,
    pub vertices_len: u64,
    pub indices_ptr: u64,
    pub count: u64,
    pub _shape_pad: [u64; 2],
    pub normals_ptr: u64,
    pub normals_len: u64,
    pub uvs_ptr: u64,
    pub uvs_len: u64,
    pub _pad0: u64,
    pub _pad1: u64,
    pub _pad2: u64,
    pub scale: u64,
    pub hash: u64,
}

impl Default for RaycastMeshData {
    fn default() -> Self {
        Self {
            vertices_ptr: 0,
            vertices_len: 0,
            indices_ptr: 0,
            count: 0,
            #[cfg(feature = "native-physics")]
            shape: None,
            #[cfg(not(feature = "native-physics"))]
            _shape_pad: [0; 2],
            normals_ptr: 0,
            normals_len: 0,
            uvs_ptr: 0,
            uvs_len: 0,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
            scale: 0,
            hash: 0,
        }
    }
}

impl RaycastMeshData {
    /// 顶点缓冲切片（O：+0x00 ptr / +0x08 len，元素为 `Vec3` 位型；`UVec3` 仅为哈希载体）。
    pub fn vertices(&self) -> &[glam::Vec3] {
        if self.vertices_ptr <= 4 {
            &[]
        } else {
            let ptr = self.vertices_ptr as *const glam::Vec3;
            unsafe { std::slice::from_raw_parts(ptr, self.vertices_len as usize) }
        }
    }

    /// 索引缓冲切片（O：+0x10 ptr / +0x18 len，元素 `[u32;3]`，stride 0xc）。
    pub fn indices(&self) -> &[[u32; 3]] {
        if self.indices_ptr <= 4 {
            &[]
        } else {
            let ptr = self.indices_ptr as *const [u32; 3];
            unsafe { std::slice::from_raw_parts(ptr, self.count as usize) }
        }
    }

    /// 法线（逐角点）缓冲切片（O：+0x30 ptr / +0x38 len；元素 `Vec3` 位型）。
    pub fn normals(&self) -> &[glam::Vec3] {
        if self.normals_ptr <= 4 {
            &[]
        } else {
            let ptr = self.normals_ptr as *const glam::Vec3;
            unsafe { std::slice::from_raw_parts(ptr, self.normals_len as usize) }
        }
    }

    /// uv 缓冲切片（O：+0x40 ptr / +0x48 len；元素 `Vec2` 位型）。
    pub fn uvs(&self) -> &[glam::Vec2] {
        if self.uvs_ptr <= 4 {
            &[]
        } else {
            let ptr = self.uvs_ptr as *const glam::Vec2;
            unsafe { std::slice::from_raw_parts(ptr, self.uvs_len as usize) }
        }
    }

    /// 形状指针（O：+0x20/+0x28 Box<dyn Shape>）。
    #[cfg(feature = "native-physics")]
    pub fn shape_ref(&self) -> Option<&dyn Shape> {
        self.shape.as_deref()
    }

    /// `RaycastMeshData::new_convex`（0x1408cec10，O）：`calculate_hash<&[UVec3]>` 存 hash、
    /// `try_convex_hull` + `ConvexPolyhedron::from_convex_mesh` → `Box<dyn Shape>`；
    /// None 写判别 0（O：`*out = 0`）。输出：tag=4@+0x00、kind=4@+0x10、shape@+0x20/+0x28、
    /// scale@+0x68=1.0、hash@+0x70（其余写 0，disasm 0x1408cec10 核对）。
    #[cfg(feature = "native-physics")]
    pub fn new_convex(points: &[glam::UVec3]) -> Option<Self> {
        // O：`utils::calculate_hash<ref$<slice$<UVec3>>>` → 存 @+0x70。
        let hash = crate::render::utils::calculate_hash(&points);
        let points_f: Vec<Point<f32>> = points
            .iter()
            .map(|p| Point::new(p.x as f32, p.y as f32, p.z as f32))
            .collect();
        // O：`parry3d::transformation::convex_hull3::convex_hull::try_convex_hull`。
        let (hull_points, indices) =
            rapier3d::parry::transformation::try_convex_hull(&points_f).ok()?;
        // O：`ConvexPolyhedron::from_convex_mesh`。
        let convex = ConvexPolyhedron::from_convex_mesh(hull_points, &indices)?;
        let shape: Box<dyn Shape> = Box::new(convex);
        Some(Self {
            vertices_ptr: 4, // tag（O：+0x00 = 4）
            vertices_len: 0,
            indices_ptr: 4, // kind（O：+0x10 = 4）
            count: 0,
            shape: Some(shape),
            normals_ptr: 0,
            normals_len: 0,
            uvs_ptr: 0,
            uvs_len: 0,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
            scale: 0x3f80000000000000,
            hash,
        })
    }

    /// `RaycastMeshData::new_cuboid`（0x1408cfff0，O）：`SharedShape::cuboid` +
    /// `IsoTransform::from_translation`；tag=4@+0x00、kind=4@+0x10、shape@+0x20/+0x28、
    /// +0x50..+0x6f = 参数区、hash@+0x70。哈希输入 `tuple<array<u32,3>, UVec3>`
    /// （half_extents 位 + hash_input）。
    ///
    /// fresh 反编译（b6-layout-tail）：cuboid 半边长 = 入参 × `0x3f000000`（0.5，
    /// 0x142925870，`MULSS` ×3）；+0x50..+0x5f = {hash_input.x, hash_input.y,
    /// hash_input.z, half_extents.x}、+0x60..+0x6f = {half_extents.y, half_extents.z,
    /// 未初始化→0, 0}（形状变体复用 uvs/scale 槽为参数区）。
    #[cfg(feature = "native-physics")]
    pub fn new_cuboid(half_extents: glam::Vec3, hash_input: glam::UVec3) -> Self {
        let hash = crate::render::utils::calculate_hash(&(
            [
                half_extents.x.to_bits(),
                half_extents.y.to_bits(),
                half_extents.z.to_bits(),
            ],
            hash_input,
        ));
        // O：入参 × 0.5（0x142925870 = 0x3f000000）才是 Cuboid 半边长。
        let half = half_extents * 0.5;
        let shape: Box<dyn Shape> = Box::new(Cuboid::new(Vector::new(half.x, half.y, half.z)));
        Self {
            vertices_ptr: 4,
            vertices_len: 0,
            indices_ptr: 4,
            count: 0,
            shape: Some(shape),
            normals_ptr: 0,
            normals_len: 0,
            uvs_ptr: 0,
            uvs_len: 0,
            // O：+0x50..+0x6f 参数区（形状变体复用常量槽）。
            _pad0: u64::from(hash_input.x) | u64::from(hash_input.y) << 32,
            _pad1: u64::from(hash_input.z) | u64::from(half_extents.x.to_bits()) << 32,
            _pad2: u64::from(half_extents.y.to_bits()) | u64::from(half_extents.z.to_bits()) << 32,
            scale: 0,
            hash,
        }
    }

    /// `RaycastMeshData::new_sphere`（0x1408d0120，O）：`SharedShape::ball` +
    /// `IsoTransform::from_translation`；tag=4@+0x00、kind=4@+0x10、shape@+0x20/+0x28、
    /// +0x50..+0x6f = 参数区、hash@+0x70。哈希输入 `tuple<u32, UVec3>`（半径位 + hash_input）。
    ///
    /// fresh 反编译：+0x50..+0x5f = {radius, hash_input.x, hash_input.y, hash_input.z}、
    /// +0x60..+0x6f = 未初始化→0。半径不经缩放直接进 `Ball::new`。
    #[cfg(feature = "native-physics")]
    pub fn new_sphere(radius: f32, hash_input: glam::UVec3) -> Self {
        let hash = crate::render::utils::calculate_hash(&(radius.to_bits(), hash_input));
        let shape: Box<dyn Shape> = Box::new(Ball::new(radius));
        Self {
            vertices_ptr: 4,
            vertices_len: 0,
            indices_ptr: 4,
            count: 0,
            shape: Some(shape),
            normals_ptr: 0,
            normals_len: 0,
            uvs_ptr: 0,
            uvs_len: 0,
            _pad0: u64::from(radius.to_bits()) | u64::from(hash_input.x) << 32,
            _pad1: u64::from(hash_input.y) | u64::from(hash_input.z) << 32,
            _pad2: 0,
            scale: 0,
            hash,
        }
    }

    /// `RaycastMeshData::new_compound`（0x1408d0230，O）：`from_iter<Isometry, SharedShape>`
    /// → `SharedShape::compound`；tag=4@+0x00、kind=4@+0x10、shape@+0x20/+0x28、
    /// +0x50..+0x6f = hash_input（`array<u32,8>` 的 4 字，O：`in_RCX[10..0xd]=in_R8[0..3]`）、
    /// hash@+0x70。
    ///
    /// fresh 反编译：入参 `shapes: Vec<RaycastMeshData>`（子形状，每 0x80B），哈希 =
    /// `calculate_hash<array<u32,8>>(hash_input)` 后逐子形状折叠 `calculate_hash<tuple<u64,u64>>`
    /// （hash, 子形状.hash@+0x70）；compound 由各子形状 `(Isometry, SharedShape)` 装配。
    #[cfg(feature = "native-physics")]
    pub fn new_compound(shapes: Vec<RaycastMeshData>, hash_input: [u32; 8]) -> Self {
        let mut hash = crate::render::utils::calculate_hash(&hash_input);
        for child in &shapes {
            hash = crate::render::utils::calculate_hash(&(hash, child.hash));
        }
        let parts: Vec<(Isometry<f32>, SharedShape)> = shapes
            .into_iter()
            .filter_map(|mut child| {
                let shape = child.shape.take()?;
                // O：子形状（Box<dyn Shape>）→ SharedShape（Arc<dyn Shape>，pub 字段直构）。
                Some((Isometry::identity(), SharedShape(Arc::from(shape))))
            })
            .collect();
        let shape: Box<dyn Shape> = Box::new(Compound::new(parts));
        Self {
            vertices_ptr: 4,
            vertices_len: 0,
            indices_ptr: 4,
            count: 0,
            shape: Some(shape),
            normals_ptr: 0,
            normals_len: 0,
            uvs_ptr: 0,
            uvs_len: 0,
            // O：+0x50..+0x68 四 qword = hash_input 8 字（`in_RCX[10..0xd]=in_R8[0..3]`）。
            _pad0: u64::from(hash_input[0]) | u64::from(hash_input[1]) << 32,
            _pad1: u64::from(hash_input[2]) | u64::from(hash_input[3]) << 32,
            _pad2: u64::from(hash_input[4]) | u64::from(hash_input[5]) << 32,
            scale: u64::from(hash_input[6]) | u64::from(hash_input[7]) << 32,
            hash,
        }
    }

    /// `RaycastMeshData::new_indexed`（0x1408cee40，fresh 逐语句，2026-10-17）：
    /// - hash = `hash((indices, vertices))`，随后 **`Some` 时**依次折叠 normals、uvs
    ///   （fresh `NEG/JO` 门控 = Option 判别；`Some(empty)` 与 `None` 指纹不同）；
    /// - `trimesh(vertices → Vec<Point>, indices)` 装配 shape；
    /// - 四条缓冲各 shrink 到 len 后以 ptr+len 存入（无 cap 槽）。
    ///
    /// 哈希表示登记：原 `Triangle`/`Vec3` 的 Hash 实现不可见，以 f32 位型（UVec3/UVec2）
    /// 喂 `calculate_hash`。
    #[cfg(feature = "native-physics")]
    pub fn new_indexed(
        indices: &[[u32; 3]],
        vertices: &[glam::Vec3],
        normals: Option<&[glam::Vec3]>,
        uvs: Option<&[glam::Vec2]>,
    ) -> Self {
        let vertex_bits: Vec<glam::UVec3> = vertices
            .iter()
            .map(|p| glam::UVec3::new(p.x.to_bits(), p.y.to_bits(), p.z.to_bits()))
            .collect();
        let mut hash = crate::render::utils::calculate_hash(&(indices, &vertex_bits));
        if let Some(normals) = normals {
            let normal_bits: Vec<glam::UVec3> = normals
                .iter()
                .map(|n| glam::UVec3::new(n.x.to_bits(), n.y.to_bits(), n.z.to_bits()))
                .collect();
            hash = crate::render::utils::calculate_hash(&(hash, &normal_bits));
        }
        if let Some(uvs) = uvs {
            let uv_bits: Vec<glam::UVec2> = uvs
                .iter()
                .map(|u| glam::UVec2::new(u.x.to_bits(), u.y.to_bits()))
                .collect();
            hash = crate::render::utils::calculate_hash(&(hash, &uv_bits));
        }
        let points: Vec<Point<f32>> = vertices.iter().map(|p| Point::new(p.x, p.y, p.z)).collect();
        let shape: Box<dyn Shape> = Box::new(TriMesh::new(points, indices.to_vec()));
        let (vertices_ptr, vertices_len) = leak_slice(vertices);
        let (indices_ptr, count) = leak_slice(indices);
        let (normals_ptr, normals_len) = match normals {
            Some(normals) => leak_slice(normals),
            None => (0, 0),
        };
        let (uvs_ptr, uvs_len) = match uvs {
            Some(uvs) => leak_slice(uvs),
            None => (0, 0),
        };
        Self {
            vertices_ptr,
            vertices_len,
            indices_ptr,
            count,
            shape: Some(shape),
            normals_ptr,
            normals_len,
            uvs_ptr,
            uvs_len,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
            scale: 0x3f80000000000000,
            hash,
        }
    }

    /// `RaycastMeshData::new_unindexed`（0x1408cf7d0，fresh 逐语句，2026-10-17）：
    /// - 入参：三角面片 + **逐三角 3 个角点**（哈希类型 `[UVec3;3]`，语义为逐角点法线）
    ///   + **可选逐三角 3 个 uv**（`Option<[UVec2;3]>`）；
    /// - 顺序索引 `0,1,2,…`（每三角 3 个），`trimesh` 装配 shape；
    /// - hash = `hash((triangles, 角点数组))`，随后 `Some` 时折叠 uv 数组；
    /// - +0x30 = 逐角点法线（flat 3/三角），不是每三角法线。
    #[cfg(feature = "native-physics")]
    pub fn new_unindexed(
        triangles: &[utils::geometry::intersect::Triangle],
        normals_per_triangle: &[[glam::Vec3; 3]],
        triangle_uvs: Option<&[[glam::Vec2; 3]]>,
    ) -> Self {
        // O：顺序索引 0,1,2,3,…（每三角 3 个 u32）。
        let indices: Vec<[u32; 3]> = (0..triangles.len())
            .map(|i| {
                let base = (i * 3) as u32;
                [base, base + 1, base + 2]
            })
            .collect();
        // O：哈希表示 —— 三角点位位型 + 逐角点位型数组。
        let triangle_bits: Vec<[glam::UVec3; 3]> = triangles
            .iter()
            .map(|t| {
                t.points
                    .map(|p| glam::UVec3::new(p.x.to_bits(), p.y.to_bits(), p.z.to_bits()))
            })
            .collect();
        let normal_bits: Vec<[glam::UVec3; 3]> = normals_per_triangle
            .iter()
            .map(|n| n.map(|p| glam::UVec3::new(p.x.to_bits(), p.y.to_bits(), p.z.to_bits())))
            .collect();
        let mut hash = crate::render::utils::calculate_hash(&(&triangle_bits, &normal_bits));
        if let Some(triangle_uvs) = triangle_uvs {
            let uv_bits: Vec<[glam::UVec2; 3]> = triangle_uvs
                .iter()
                .map(|u| u.map(|p| glam::UVec2::new(p.x.to_bits(), p.y.to_bits())))
                .collect();
            hash = crate::render::utils::calculate_hash(&(hash, &uv_bits));
        }
        let points: Vec<Point<f32>> = triangles
            .iter()
            .flat_map(|t| t.points.iter())
            .map(|p| Point::new(p.x, p.y, p.z))
            .collect();
        let shape: Box<dyn Shape> = Box::new(TriMesh::new(points, indices.clone()));
        let vertices: Vec<glam::Vec3> = triangles
            .iter()
            .flat_map(|t| t.points.iter().copied())
            .collect();
        let normals: Vec<glam::Vec3> = normals_per_triangle
            .iter()
            .flat_map(|n| n.iter().copied())
            .collect();
        let uvs: Option<Vec<glam::Vec2>> =
            triangle_uvs.map(|u| u.iter().flat_map(|v| v.iter().copied()).collect());
        let (vertices_ptr, vertices_len) = leak_slice(&vertices);
        let (indices_ptr, count) = leak_slice(&indices);
        let (normals_ptr, normals_len) = leak_slice(&normals);
        let (uvs_ptr, uvs_len) = match &uvs {
            Some(uvs) => leak_slice(uvs.as_slice()),
            None => (0, 0),
        };
        Self {
            vertices_ptr,
            vertices_len,
            indices_ptr,
            count,
            shape: Some(shape),
            normals_ptr,
            normals_len,
            uvs_ptr,
            uvs_len,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
            scale: 0x3f80000000000000,
            hash,
        }
    }
}

/// 把切片复制进独立堆缓冲并以 `(ptr, len)` 返回（O：Vec `shrink_to_fit` 后取 ptr+len；
/// 空切片使用对齐悬垂哨兵 4，与 fresh `_Dst = &DAT_00000004` 一致）。
#[cfg(feature = "native-physics")]
fn leak_slice<T: Copy>(values: &[T]) -> (u64, u64) {
    if values.is_empty() {
        return (4, 0);
    }
    let boxed: Box<[T]> = values.to_vec().into_boxed_slice();
    let len = boxed.len() as u64;
    (Box::into_raw(boxed) as *mut T as u64, len)
}

/// `RaycastMeshData` 的缓冲释放（O：shrink 后 ptr+len 拥有堆；空/形状变体哨兵 4 不释放）。
#[cfg(feature = "native-physics")]
impl Drop for RaycastMeshData {
    fn drop(&mut self) {
        unsafe {
            free_leaked::<glam::Vec3>(self.vertices_ptr, self.vertices_len);
            free_leaked::<[u32; 3]>(self.indices_ptr, self.count);
            free_leaked::<glam::Vec3>(self.normals_ptr, self.normals_len);
            free_leaked::<glam::Vec2>(self.uvs_ptr, self.uvs_len);
        }
    }
}

#[cfg(feature = "native-physics")]
unsafe fn free_leaked<T>(ptr: u64, len: u64) {
    if ptr > 4 && len > 0 {
        unsafe {
            let raw = std::ptr::slice_from_raw_parts_mut(ptr as *mut T, len as usize);
            drop(Box::from_raw(raw));
        }
    }
}

/// `RaycastGeoSource`（O：`RaycastGeoSource::subworld` 0x1408cea20 / `impl$18::eq`
/// 0x1408cea30；`impl$86::fmt` 0x1409816a0 证实 17 变体 switch，判别 0..0x10）。
///
/// **source→slot 映射 fresh 锚定**（`upsert_collider_entry` 0x140b40b50 的
/// `bVar14 = (&DAT_142aef03c)[*in_R9]`；该表已 read_memory 核对：
/// `[0,9,1,2,8,14,3,7,6,10,4,5,11,12,13,15,16]`）。**不是恒等映射**：例如判别 5 → 槽 14、
/// 判别 11 → 槽 5。旧实现按判别恒等并硬编码 Terrain=5 为槽 5 是分歧，已按表修正；
/// 变体名仍属未锚定（U，判别 5 的旧名 `Terrain` 保留仅为兼容）。
/// `impl$18::eq` 对部分变体比较 payload 字段（case 2/3/4/5/13/14/15/16），
/// Rust 枚举无 payload → 判别相等即 eq（payload 比较登记）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RaycastGeoSource {
    Slot0 = 0,
    Slot1 = 1,
    Slot2 = 2,
    Slot3 = 3,
    Slot4 = 4,
    /// Terrain（O：判别 5；按 fresh 映射表落子世界槽 **14**）。
    Terrain = 5,
    Slot6 = 6,
    /// PlatformStair（O：判别 7；映射表 7 → 槽 7）。
    PlatformStair = 7,
    Slot8 = 8,
    Slot9 = 9,
    Slot10 = 10,
    Slot11 = 11,
    Slot12 = 12,
    Slot13 = 13,
    Slot14 = 14,
    Slot15 = 15,
    Slot16 = 16,
}

impl RaycastGeoSource {
    /// fresh 锚定的 source→subworld 槽映射（0x142aef03c，17 B；read_memory 核对）。
    pub const SOURCE_TO_SLOT: [u8; 17] = [0, 9, 1, 2, 8, 14, 3, 7, 6, 10, 4, 5, 11, 12, 13, 15, 16];

    /// 来源对应的子世界槽号（O：`subworld` 0x1408cea20，查表 0x142aef03c）。
    pub fn subworld(self) -> u8 {
        Self::SOURCE_TO_SLOT[self as usize]
    }

    /// 判别（+0x00）。
    pub fn tag(self) -> u8 {
        self as u8
    }

    /// `impl$18::eq`（0x1408cea30）：判别 + 变体 payload 相等。Rust 无 payload → 判别相等。
    /// 原对 case 0/1/6/7/8/9/12 仅比判别；对其余变体比 payload 字段（登记）。
    pub fn eq(self, other: Self) -> bool {
        self == other
    }
}

/// `RaycastMesh`（O：`RaycastMesh::new` 0x1408ceb10 / `data` 0x1408ceb30 /
/// `raycast` 0x1408ceb40）。
///
/// `new`：把数据 3 字拷贝进 +0x00..+0x17、source@+0x18（`in_RCX[3]=in_RDX;
/// {*in_RCX,in_RCX[1],in_RCX[2]} = in_R8[0..2]`）。`raycast`：经 source 的 vtable
/// （[source+0x38]+0x38）分派到子世界形状直查；Rust 侧以 source 槽号 + 数据 shape 直查等价
/// （vtable 分派登记）。
#[derive(Clone)]
pub struct RaycastMesh {
    /// 网格数据（O：`new` 拷贝 3 字；此处 = `Arc<RaycastMeshData>` 引用计数指针）。
    pub data: Arc<RaycastMeshData>,
    /// 来源（O：@+0x18）。
    pub source: RaycastGeoSource,
}

impl std::fmt::Debug for RaycastMesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RaycastMesh")
            .field("source", &self.source)
            .field("data_hash", &self.data.hash)
            .finish()
    }
}

impl RaycastMesh {
    /// `RaycastMesh::new`（0x1408ceb10）。
    pub fn new(data: Arc<RaycastMeshData>, source: RaycastGeoSource) -> Self {
        Self { data, source }
    }

    /// `RaycastMesh::data`（0x1408ceb30，访问器）。
    pub fn data(&self) -> &Arc<RaycastMeshData> {
        &self.data
    }

    /// `RaycastMesh::raycast`（0x1408ceb40）：以数据 shape 直查（solid=1），命中经
    /// `rapier_hit_to_raycast_world_result` 做 barycentric 法线/uv 插值；shape 缺或未中 =
    /// 判别 2。
    #[cfg(feature = "native-physics")]
    pub fn raycast(&self, origin: glam::Vec3, dir: glam::Vec3) -> RaycastWorldResult {
        let ray = Ray::new(
            Point::new(origin.x, origin.y, origin.z),
            Vector::new(dir.x, dir.y, dir.z),
        );
        let Some(shape) = self.data.shape_ref() else {
            return RaycastWorldResult::miss();
        };
        let Some(hit) = shape.cast_ray_and_get_normal(&Isometry::identity(), &ray, f32::MAX, true)
        else {
            return RaycastWorldResult::miss();
        };
        let tri_index = match hit.feature {
            rapier3d::parry::shape::FeatureId::Face(id) => Some(id),
            _ => None,
        };
        rapier_hit_to_raycast_world_result(
            &self.data,
            origin,
            dir,
            hit.time_of_impact,
            glam::Vec3::new(hit.normal.x, hit.normal.y, hit.normal.z),
            tri_index,
            SubworldColliderHandle::default(),
            glam::Quat::IDENTITY,
        )
    }

    /// `RaycastMesh::raycast`（非 native-physics 壳）。
    #[cfg(not(feature = "native-physics"))]
    pub fn raycast(&self, _origin: glam::Vec3, _dir: glam::Vec3) -> RaycastWorldResult {
        RaycastWorldResult::miss()
    }
}

/// `rapier_hit_to_raycast_world_result`（0x1408ce4a0，fresh 逐语句，2026-10-17）：把命中转换成
/// `RaycastWorldResult`。
///
/// 输出布局（fresh）：handle 24B @+0x00（注册表条目前 24 B）、position=origin+dir*toi @+0x18、
/// normal @+0x24、toi @+0x30、判别 @+0x34、uv/extra @+0x38。
///
/// 三角网格路径：
/// - `mesh.indices`（O：`*(mesh+0x28)`）为 0 → 非网格命中（判别 0）；
/// - 否则若命中判别（O：`hit[0]`）!= 2 → 判别 = 2（miss）；
/// - 索引 = `tri - (tri>=len ? len : 0)`；再越界 panic（O：两次 `panic_bounds_check`）；
/// - barycentric 权重经 `barycentric(pos, v1, v2, v0)` 施加到逐角点法线/uv；
/// - **法线经旋转四元数（O：in_R9）旋转** `n' = q * n`；uv 旋转不变；
/// - 法线缓冲为空 → 用命中法线（O：`in_R8+0x0c` 的 fallback Vec3，已是世界空间，不旋转）；
/// - 有 uv → 判别 1，extra = uv；无 uv → 判别 0，extra = {bc.y, bc.y}。
#[cfg(feature = "native-physics")]
pub fn rapier_hit_to_raycast_world_result(
    mesh: &RaycastMeshData,
    origin: glam::Vec3,
    dir: glam::Vec3,
    toi: f32,
    hit_normal: glam::Vec3,
    tri_index: Option<u32>,
    handle: SubworldColliderHandle,
    rotation: glam::Quat,
) -> RaycastWorldResult {
    let position = origin + dir * toi;
    let handle_region = [
        u64::from(handle.index) | (u64::from(handle.generation) << 32),
        u64::from(handle.slot),
        0,
    ];
    let mut result = RaycastWorldResult {
        handle: handle_region,
        position,
        normal: hit_normal,
        toi,
        discriminant: RAYCAST_WORLD_RESULT_HIT,
        extra: [0.0, 0.0],
    };
    // O：`mesh+0x28`（索引 len）为 0 → 非三角路径，返回射线上位置 + 命中法线。
    let indices = mesh.indices();
    if indices.is_empty() {
        return result;
    }
    // O：`*in_R8 != 2` → +0x34 = 2（网格命中判别不符 = miss）。
    let Some(tri) = tri_index else {
        result.discriminant = RAYCAST_WORLD_RESULT_MISS;
        return result;
    };
    let len = indices.len();
    let mut tri = tri as usize;
    if tri >= len {
        tri -= len;
    }
    if tri >= len {
        panic!("rapier_hit_to_raycast_world_result: triangle index out of bounds");
    }
    let vertices = mesh.vertices();
    let [va, vb, vc] = indices[tri];
    // O：越界 panic（panic_bounds_check），不做静默跳过。
    let pa = vertices[va as usize];
    let pb = vertices[vb as usize];
    let pc = vertices[vc as usize];
    // O：barycentric(point, v1, v2, v0)，权重 {x→v1, y→v2, z→v0}。
    let bc = utils::geometry::barycentric::barycentric(position, pb, pc, pa);
    let normals = mesh.normals();
    if normals.is_empty() {
        // O：法线缓冲为空 → fallback 命中法线（世界空间）。
        result.normal = hit_normal;
    } else {
        let n0 = normals[va as usize];
        let n1 = normals[vb as usize];
        let n2 = normals[vc as usize];
        let x = glam::Vec3::new(
            n1.x * bc.x + n2.x * bc.y + n0.x * bc.z,
            n1.y * bc.x + n2.y * bc.y + n0.y * bc.z,
            n1.z * bc.x + n2.z * bc.y + n0.z * bc.z,
        );
        // O：四元数旋转插值法线（in_R9 = Quat）。
        result.normal = rotation * x;
    }
    let uvs = mesh.uvs();
    if uvs.is_empty() {
        result.discriminant = RAYCAST_WORLD_RESULT_HIT;
        result.extra = [bc.y, bc.y];
    } else {
        let u0 = uvs[va as usize];
        let u1 = uvs[vb as usize];
        let u2 = uvs[vc as usize];
        result.extra = [
            u1.x * bc.x + u2.x * bc.y + u0.x * bc.z,
            u1.y * bc.x + u2.y * bc.y + u0.y * bc.z,
        ];
        result.discriminant = 1;
    }
    result
}

/// `RaycastCollider`（O：0x28 B；`get_collider` 0x1408cdb50 的输出）。
///
/// 机器布局：`+0x00..+0x1c` = Isometry（translation @+0x00、rotation @+0x0c，与
/// `Collider` +0x34/+0x40 对应）、`+0x20` = `Collider::user_data` 高 32 位 `| 1`、
/// `+0x24` = user_data 低 32 位。Rust `Quat` 16 B 对齐使布局为 0x30 B（字节级偏移登记）。
#[cfg(feature = "native-physics")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RaycastCollider {
    /// +0x00。
    pub position: glam::Vec3,
    /// +0x0c。
    pub rotation: glam::Quat,
    /// +0x20：user_data 高 32 位 | 1。
    pub generation: u32,
    /// +0x24：user_data 低 32 位。
    pub generation_low: i32,
}

/// 布局（`dispose_collider_mesh_instance` 0x1408d9590 / `get_collider_mesh` 0x1408cdbd0 /
/// `should_intersect_collider_meshes` 0x140a9b3c0 fresh，2026-10-17）：
/// 条目 stride **0x28**；`+0x00` 活条目 = kind 字节、**释放条目 = free-list next u32**；
/// `+0x01..+0x1f` meta；`+0x20` generation（精确比较/释放时 +1）；`+0x24` generation 低 32。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColliderRegistryEntry {
    /// +0x00：活条目 = kind 字节；释放条目 = free-list next index（u32）。
    pub first: u32,
    /// +0x04..+0x20 meta（`mesh_ref` 取自 +0x08）。
    pub meta: [u8; 0x1c],
    /// +0x20 generation（dispose 时 `= gen + 1`）。
    pub generation: u32,
    /// +0x24 generation 低 32 位（O：`(int)uVar2`）。
    pub generation_low: i32,
}

impl ColliderRegistryEntry {
    /// 无参空条目。
    pub fn empty() -> Self {
        Self {
            first: 0,
            meta: [0; 0x1c],
            generation: 0,
            generation_low: 0,
        }
    }

    /// kind 字节（O：+0x00 低字节；0=纯地形网格、2=碰撞网格）。
    pub fn kind(&self) -> u8 {
        self.first as u8
    }

    /// 写 kind 字节（保留 +0x01..+0x03）。
    pub fn set_kind(&mut self, kind: u8) {
        self.first = (self.first & 0xffff_ff00) | u32::from(kind);
    }

    /// 释放条目的 free-list next（O：`+0x00` 整 4 字节）。
    pub fn free_next(&self) -> u32 {
        self.first
    }

    /// 写入 free-list next（覆盖 kind/meta 前 3 字节，O：`*(u32*)(entry) = head`）。
    pub fn set_free_next(&mut self, next: u32) {
        self.first = next;
    }

    /// 注册表条目 `+0x08` 的 qword（`MeshIntersectionArtifacts::mesh_ref_*` 来源；语义未锚定）。
    pub fn mesh_ref(&self) -> u64 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.meta[4..12]);
        u64::from_le_bytes(bytes)
    }
}

/// `RaycastWorld`：17 子世界槽的查询管线（O 布局：collider 注册表 Vec@+0x08/+0x10、
/// 槽内联@+0x20、Publisher@+0x3cb0）。
///
/// Resource（组 A restorer 接线）：原版 `Res<RaycastWorld>` 参数面；core 侧此前为普通结构。
#[derive(Resource)]
pub struct RaycastWorld {
    /// 游戏侧 collider 注册表（O：内联 Vec @+0x08 data / +0x10 len，条目 0x28B）。
    /// `get_collider_mesh`/`should_intersect_collider_meshes` 以 (index, generation) 校验。
    colliders: Vec<ColliderRegistryEntry>,
    /// 注册表 free-list 头（O：@+0x18；`dispose` 推入 index，空表哨兵**未实测**，
    /// Rust 以 `u32::MAX` 表示 —— 登记：分配侧弹栈语义未锚定）。
    #[cfg_attr(not(feature = "native-physics"), allow(dead_code))]
    registry_free_head: u32,
    /// 注册表活跃计数（O：@+0x1c；`dispose` 时 `-= 1`）。
    #[cfg_attr(not(feature = "native-physics"), allow(dead_code))]
    registry_live: i32,
    /// 17 槽（O：内联 @+0x20，每槽 0x390B）。
    #[cfg(feature = "native-physics")]
    slots: [RapierSubWorld; SUBWORLD_SLOT_COUNT as usize],
    /// `mesh_intersect_collider_diff_publisher` 返回此字段引用（O：@+0x3cb0）。
    publisher: Publisher<ColliderDiff>,
    /// 脏瓦片发布器（O：`subscribe_to_changes` 0x1408cdc20 的
    /// `Publisher::subscribe<DirtyTiles>` 目标；`flush_colliders` 0x1408cd5c0 逐槽
    /// take-and-clear 后 `publish<DirtyTiles,DirtyTiles>`）。`on_level_reload` 0x1408d8710
    /// 显示每槽另有内联 `Publisher<DirtyTiles>`（slot+0x150）；world 级发布器为订阅合同。
    dirty_tiles_publisher: Publisher<crate::utils::dirty_tiles::DirtyTiles>,
    /// 网格实例源（O：@+0x3dc8；`slotmap<RaycastMeshKey, RaycastMesh>`）。
    /// `on_level_reload`/`update_rapier_colliders` 消费；非 native-physics 壳不落地。
    /// `update_rapier_colliders` 实现后接入；当前 `#[allow(dead_code)]`（待消费）。
    #[cfg(feature = "native-physics")]
    #[allow(dead_code)]
    mesh_sources: MeshInstanceSources,
}

/// 单个子世界槽（O：`world::RapierSubWorld`，`impl$0::build` 0x140b685e0 用
/// `default<RapierSubWorld>` 填 17 槽；布局 @+0x0 bodies / @+0x30 colliders /
/// 查询管线 / DirtyTiles 缓冲 @+0x188 / 脏标志 @+0x388）。
#[cfg(feature = "native-physics")]
struct RapierSubWorld {
    bodies: RigidBodySet,
    colliders: ColliderSet,
    pipeline: QueryPipeline,
    /// 脏瓦片缓冲（O：slot+0x188，0x200B）。
    dirty_tiles: crate::utils::dirty_tiles::DirtyTiles,
    /// 脏标志（O：slot+0x388）。`flush_colliders` 以该字节决定是否 take-and-clear +
    /// `publish` + `QueryPipeline::update`。
    dirty: bool,
}

#[cfg(feature = "native-physics")]
impl Default for RapierSubWorld {
    fn default() -> Self {
        Self {
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            pipeline: QueryPipeline::new(),
            dirty_tiles: crate::utils::dirty_tiles::DirtyTiles::new_clear(),
            dirty: false,
        }
    }
}

impl Default for RaycastWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl RaycastWorld {
    /// 新建（O：17 槽 + Publisher 复位；`impl$0::build` 0x140b685e0 内联构造后 `insert_resource`）。
    pub fn new() -> Self {
        Self {
            colliders: Vec::new(),
            registry_free_head: u32::MAX,
            registry_live: 0,
            #[cfg(feature = "native-physics")]
            slots: std::array::from_fn(|_| RapierSubWorld::default()),
            publisher: Publisher::new(),
            dirty_tiles_publisher: Publisher::new(),
            #[cfg(feature = "native-physics")]
            mesh_sources: MeshInstanceSources::new(),
        }
    }

    /// 全槽 collider 遍历（O：`visualize_colliders` 0x141326370 的 17 槽 arena 遍历；
    /// 回调收 `(槽号, ColliderHandle, &Collider)`）。
    #[cfg(feature = "native-physics")]
    pub fn for_each_collider(
        &self,
        mut f: impl FnMut(u8, rapier3d::geometry::ColliderHandle, &rapier3d::geometry::Collider),
    ) {
        for (slot_index, slot) in self.slots.iter().enumerate() {
            for (handle, collider) in slot.colliders.iter() {
                f(slot_index as u8, handle, collider);
            }
        }
    }

    /// `RaycastWorld::flush_colliders`（0x1408cd5c0）：17 槽步进 0x390；每槽脏标志
    /// （slot+0x388）非零时：拷贝并清零脏瓦片缓冲（slot+0x188，0x200B，take-and-clear），
    /// `Publisher::publish<DirtyTiles,DirtyTiles>`，清脏标志，`QueryPipeline::update()`。
    /// puffin scope 在本实现为调试无操作（原 `are_scopes_on` 门控）。
    #[cfg(feature = "native-physics")]
    pub fn flush_colliders(&mut self) {
        for slot in self.slots.iter_mut() {
            if slot.dirty {
                let tiles = slot.dirty_tiles.take_and_clear();
                // O：`publish<DirtyTiles,DirtyTiles>`（`take_and_clear` 的快照即 delta，
                // `DirtyTiles` 的 `StateDelta::apply_to` 为逐字 OR）。
                let _ = self.dirty_tiles_publisher.publish(tiles);
                slot.dirty = false;
                slot.pipeline.update(&slot.colliders);
            }
        }
    }

    /// `RaycastWorld::flush_colliders`（非 native-physics 壳）。
    #[cfg(not(feature = "native-physics"))]
    pub fn flush_colliders(&mut self) {}

    /// `flush_rapier_colliders`（0x1408d8360）：puffin scope + 写计数器
    /// `*[self+0x10] = [self+0x1c]`（偏移未锚定，登记）+ `RaycastWorld::flush_colliders()`。
    /// Rust 侧实现取 puffin 门控的 flush_colliders 转发；`*[self+0x10]=[self+0x1c]` 的
    /// 目标字段依赖 +0x10/+0x1c 的精确语义（on_level_reload 同款写），登记。
    pub fn flush_rapier_colliders(&mut self) {
        self.flush_colliders();
    }

    /// collider 注册表切片（O：内联 Vec @+0x08/+0x10）。
    pub fn collider_registry(&self) -> &[ColliderRegistryEntry] {
        &self.colliders
    }

    /// 向注册表追加一个条目（`upsert_collider_entry` 族；O：Vec@+0x08/+0x10）。
    /// 分配侧 free-list 弹栈语义未锚定，这里按追加 + 活跃计数建模。
    pub fn upsert_collider_entry(&mut self, entry: ColliderRegistryEntry) -> usize {
        let index = self.colliders.len();
        self.colliders.push(entry);
        self.registry_live += 1;
        index
    }

    /// 以 rapier collider `(index, generation)` 对齐注册表条目（O：注册表与 collider set
    /// 同索引；`dispose` 以 `instance.{gen,index}` 校验该数组）。数组必要时补空条目。
    pub fn ensure_registry_entry(&mut self, index: u32, generation: u32, kind: u8) {
        let index = index as usize;
        while self.colliders.len() <= index {
            self.colliders.push(ColliderRegistryEntry::empty());
        }
        let entry = &mut self.colliders[index];
        entry.set_kind(kind);
        entry.generation = generation;
        self.registry_live += 1;
    }

    /// `RaycastWorld::get_collider`（0x1408cdb50，O）：从**活 rapier Collider** 复制快照。
    ///
    /// fresh 逐语句：`ColliderSet::get_mut_internal`（未命中 `expect_failed`）→
    /// 输出 `{+0x00..0x1c = Collider +0x34..0x50 的 Isometry（translation+rotation）,
    /// +0x20 = (Collider 首 8 B 高 32 位) | 1, +0x24 = 低 32 位}`。
    /// 首 8 B 为 `Collider::user_data`（调用方存 {index, generation} 打包）。
    ///
    /// 旧实现读注册表条目（`ColliderRegistryEntry`）是 callee/数据源分歧，已修。
    #[cfg(feature = "native-physics")]
    pub fn get_collider(&self, handle: SubworldColliderHandle) -> RaycastCollider {
        let slot = &self.slots[handle.slot as usize];
        let collider_handle =
            rapier3d::geometry::ColliderHandle::from_raw_parts(handle.index, handle.generation);
        let collider = slot
            .colliders
            .get(collider_handle)
            .expect("get_collider: collider handle invalid");
        let iso = collider.position();
        let raw = collider.user_data;
        RaycastCollider {
            position: glam::Vec3::new(iso.translation.x, iso.translation.y, iso.translation.z),
            rotation: glam::Quat::from_xyzw(
                iso.rotation.i,
                iso.rotation.j,
                iso.rotation.k,
                iso.rotation.w,
            ),
            generation: ((raw >> 32) as u32) | 1,
            generation_low: raw as i32,
        }
    }

    /// `RaycastWorld::get_collider_mesh`（0x1408cdbd0，O）：注册表 (index, generation) 校验；
    /// fresh 用**精确相等**（`entry.generation@+0x20 == generation`，无 `|1`），
    /// 失败 panic（`begin_panic`）。
    pub fn get_collider_mesh(&self, index: u32, generation: u32) {
        let index_usize = index as usize;
        let ok = index_usize < self.colliders.len()
            && self.colliders[index_usize].generation == generation;
        if !ok {
            panic!("get_collider_mesh: collider handle invalid");
        }
    }

    /// `RaycastWorld::subscribe_to_changes`（0x1408cdc20，O）：`Publisher::subscribe<DirtyTiles>`
    /// —— 脏瓦片变化订阅。fresh 反编译：25 B 纯转发 `tabloid::Publisher::subscribe<DirtyTiles>()`
    /// （发布器为 world 字段；`flush_colliders` 逐槽 take-and-clear 后 publish）。
    pub fn subscribe_to_changes(
        &mut self,
    ) -> tabloid::Subscriber<crate::utils::dirty_tiles::DirtyTiles> {
        self.dirty_tiles_publisher.subscribe()
    }

    /// `Rapier::get_rapier_collider`（0x1408ce440，fresh）：纯转发
    /// `ColliderSet::get_mut_internal` —— 返回 slot 内 collider 的**可变**引用。
    #[cfg(feature = "native-physics")]
    pub fn get_rapier_collider(
        &mut self,
        slot: u8,
        handle: rapier3d::geometry::ColliderHandle,
    ) -> Option<&mut rapier3d::geometry::Collider> {
        self.slots[slot as usize].colliders.get_mut(handle)
    }

    /// 子世界槽的查询管线（O：`find_mesh_mesh_intersections` 0x140a97b70 候选槽
    /// `Qbvh::traverse_depth_first_node<ColliderHandle, BoundingVolumeIntersectionsVisitor>`
    /// 经 `QueryPipeline::colliders_with_aabb_intersecting_aabb` 等价遍历；该 API 内部
    /// 使用 fork 的 `QueryPipeline.qbvh: Qbvh<ColliderHandle>`，与机器 +0x3d80 世界 Qbvh 面
    /// 等价登记）。
    #[cfg(feature = "native-physics")]
    pub(crate) fn slot_query_pipeline(&self, slot: usize) -> &rapier3d::pipeline::QueryPipeline {
        &self.slots[slot].pipeline
    }

    /// `Rapier::on_collider_upserted`（0x1408ce460，O）：`publish<ColliderDiff, ColliderDelta>`
    /// —— 把调用方构造的增量发布到 `Publisher<ColliderDiff>`（dispose/upsert 各以
    /// removed=true/false 的句柄增量驱动 `find_mesh_mesh_intersections` 的图剪枝与 AABB 遍历）。
    pub fn on_collider_upserted(&mut self, delta: ColliderDelta) {
        let _ = self.publisher.publish(delta);
    }

    /// `RaycastWorld::raycast`（0x1408cc870）：转发 `Rapier::raycast`（0x1408cdc60）。
    #[cfg(feature = "native-physics")]
    pub fn raycast(&self, origin: glam::Vec3, dir: glam::Vec3, mask: u32) -> RaycastWorldResult {
        Rapier::raycast(self, origin, dir, mask)
    }

    /// `RaycastWorld::raycast`（非 native-physics 壳：查询不落地）。
    #[cfg(not(feature = "native-physics"))]
    pub fn raycast(&self, _origin: glam::Vec3, _dir: glam::Vec3, _mask: u32) -> RaycastWorldResult {
        RaycastWorldResult::miss()
    }

    /// `RaycastWorld::legacy_raycast`（0x1408cc680）：转发 `Rapier::raycast`，
    /// fresh 指令固定 `mask = 0xFB`（`mov dword [rsp+0x30], 0xfb`；17 槽排除 bit2）
    /// 与 `max_toi = f32::MAX`（`0x7f7fffff`）。
    /// O：未命中把 **u32 2 原样** 写 +0x30（toi 槽）而非 +0x34（`Rapier::raycast`
    /// 已在 +0x34 写 2）；调用方以 `cmp dword [result+0x30], 2` 判 miss
    /// （`hint_focused_edit_hint` 0x14227e057 / `ui_place_tree` 0x1422692ba /
    /// `hint_roof_shape` 0x1422850f3），故必须写位型 `0x00000002`，不是 `2.0f32`
    /// 的位型 `0x40000000`。
    #[cfg(feature = "native-physics")]
    pub fn legacy_raycast(&self, origin: glam::Vec3, dir: glam::Vec3) -> RaycastWorldResult {
        let mut result = Rapier::raycast(self, origin, dir, 0xfb);
        if result.is_miss() {
            // 位型等价：把 u32 判别 2 写入 +0x30 的 4 字节。
            result.toi = f32::from_bits(RAYCAST_WORLD_RESULT_MISS);
        }
        result
    }

    /// `RaycastWorld::legacy_raycast`（非 native-physics 壳）。
    #[cfg(not(feature = "native-physics"))]
    pub fn legacy_raycast(&self, _origin: glam::Vec3, _dir: glam::Vec3) -> RaycastWorldResult {
        RaycastWorldResult::miss()
    }

    /// `RaycastWorld::raycast_w_terrain`（0x1408cd430）：先 `Rapier::raycast`；mask 含地形
    /// 0x20 位时 `TerrainHeightsData::intersect_ray`，以已有 Rapier 命中的 TOI 为上限；
    /// 因此只有更近或等距的地形命中能够覆盖。
    #[cfg(feature = "native-physics")]
    pub fn raycast_w_terrain(
        &self,
        terrain: &TerrainHeightsData<TerrainHeightSsbo>,
        origin: glam::Vec3,
        dir: glam::Vec3,
        mask: u32,
    ) -> RaycastWorldResult {
        let mut result = Rapier::raycast(self, origin, dir, mask);
        if mask & 0x20 != 0 {
            let ray = TerrainRay {
                origin,
                direction: dir,
            };
            // B 0x1408cd508..0x1408cd51f passes the Rapier hit's TOI to terrain.
            let maximum_toi = if result.is_hit() {
                result.toi
            } else {
                f32::MAX
            };
            if let Some(terrain_hit) = terrain.intersect_ray(ray, maximum_toi) {
                result = terrain_result(terrain_hit.t, origin, dir);
            }
        }
        result
    }

    /// Sphere sweep using the existing API's dependency-default options.
    /// Use spherecast_with_options when the consumer supplies a sweep limit or flags.
    #[cfg(feature = "native-physics")]
    pub fn spherecast(
        &self,
        origin: glam::Vec3,
        dir: glam::Vec3,
        radius: f32,
        mask: u32,
    ) -> RaycastWorldResult {
        self.spherecast_with_options(
            origin,
            dir,
            radius,
            rapier3d::parry::query::ShapeCastOptions::default(),
            mask,
        )
    }

    /// B 0x1408ccfc0: sphere sweep with explicit original dependency options.
    /// Radius is geometry; options.max_time_of_impact is the sweep limit.
    #[cfg(feature = "native-physics")]
    pub fn spherecast_with_options(
        &self,
        origin: glam::Vec3,
        dir: glam::Vec3,
        radius: f32,
        options: rapier3d::parry::query::ShapeCastOptions,
        mask: u32,
    ) -> RaycastWorldResult {
        Rapier::spherecast(self, origin, dir, radius, options, mask)
    }

    /// `mesh_intersect_collider_diff_publisher`（0x1408cc670）：返回 &Publisher（O：
    /// `lea rax,[rcx+0x3cb0]; ret`，即 &self+0x3cb0）。
    pub fn mesh_intersect_collider_diff_publisher(&self) -> &Publisher<ColliderDiff> {
        &self.publisher
    }

    /// `RaycastWorld::intersects_any`（0x1408cd5b0）：17 槽 Qbvh 遍历 + 命中即返回
    /// （机器码与 `Rapier::intersects_any` 0x1408cdf80 同体，O）。
    #[cfg(feature = "native-physics")]
    pub fn intersects_any(&self, origin: glam::Vec3, dir: glam::Vec3, mask: u32) -> bool {
        Rapier::intersects_any(self, origin, dir, mask)
    }

    /// `RaycastWorld::intersects_any`（非 native-physics 壳）。
    #[cfg(not(feature = "native-physics"))]
    pub fn intersects_any(&self, _origin: glam::Vec3, _dir: glam::Vec3, _mask: u32) -> bool {
        false
    }

    /// `RaycastWorld::raycast_simple`（0x1408ccc20）：转发 `Rapier::raycast_simple`
    /// （17 槽 `QueryPipeline::cast_ray`，逐槽收紧 TOI 上限）。
    #[cfg(feature = "native-physics")]
    pub fn raycast_simple(
        &self,
        origin: glam::Vec3,
        dir: glam::Vec3,
        mask: u32,
    ) -> RaycastSimpleResult {
        Rapier::raycast_simple(self, origin, dir, mask)
    }

    /// `RaycastWorld::raycast_simple`（非 native-physics 壳）。
    #[cfg(not(feature = "native-physics"))]
    pub fn raycast_simple(
        &self,
        _origin: glam::Vec3,
        _dir: glam::Vec3,
        _mask: u32,
    ) -> RaycastSimpleResult {
        RaycastSimpleResult::miss()
    }

    /// `RaycastWorld::raycast_simple_exclude_wall_id`（0x1408ccc50）：fresh 反编译
    /// `Rapier::raycast_simple()` 转发与 0x1408ccc20 **同体**（未见排除实参/过滤），
    /// 故 Rust 侧同样转发、不消费 `excluded`（保留入参合同）。
    #[cfg(feature = "native-physics")]
    pub fn raycast_simple_exclude_wall_id(
        &self,
        origin: glam::Vec3,
        dir: glam::Vec3,
        mask: u32,
        _excluded: SubworldColliderHandle,
    ) -> RaycastSimpleResult {
        Rapier::raycast_simple(self, origin, dir, mask)
    }

    /// `RaycastWorld::raycast_simple_exclude_wall_id`（非 native-physics 壳）。
    #[cfg(not(feature = "native-physics"))]
    pub fn raycast_simple_exclude_wall_id(
        &self,
        _origin: glam::Vec3,
        _dir: glam::Vec3,
        _mask: u32,
        _excluded: SubworldColliderHandle,
    ) -> RaycastSimpleResult {
        RaycastSimpleResult::miss()
    }

    /// `RaycastWorld::legacy_raycast_w_params`（0x1408cc770）：转发 `Rapier::raycast`；
    /// 未命中把判别 2 写 +0x30（toi 槽，legacy 特例，同 `legacy_raycast`）。
    #[cfg(feature = "native-physics")]
    pub fn legacy_raycast_w_params(
        &self,
        origin: glam::Vec3,
        dir: glam::Vec3,
        mask: u32,
    ) -> RaycastWorldResult {
        let mut result = Rapier::raycast(self, origin, dir, mask);
        if result.is_miss() {
            result.toi = f32::from_bits(RAYCAST_WORLD_RESULT_MISS);
        }
        result
    }

    /// `RaycastWorld::legacy_raycast_w_params`（非 native-physics 壳）。
    #[cfg(not(feature = "native-physics"))]
    pub fn legacy_raycast_w_params(
        &self,
        _origin: glam::Vec3,
        _dir: glam::Vec3,
        _mask: u32,
    ) -> RaycastWorldResult {
        RaycastWorldResult::miss()
    }

    /// `RaycastWorld::legacy_spherecast`（0x1408ccc90）：转发 `Rapier::legacy_spherecast`。
    /// fresh：**无 mask 参数**，固定按 10 槽表 `[0,1,3,7,6,4,0xb,2,0xc,0xd]` 顺序
    /// `QueryPipeline::cast_shape`，最后一次命中覆盖；判别写 +0x30。
    #[cfg(feature = "native-physics")]
    pub fn legacy_spherecast(
        &self,
        origin: glam::Vec3,
        dir: glam::Vec3,
        radius: f32,
    ) -> RaycastWorldResult {
        Rapier::legacy_spherecast(self, origin, dir, radius)
    }

    /// `RaycastWorld::legacy_spherecast`（非 native-physics 壳）。
    #[cfg(not(feature = "native-physics"))]
    pub fn legacy_spherecast(
        &self,
        _origin: glam::Vec3,
        _dir: glam::Vec3,
        _radius: f32,
    ) -> RaycastWorldResult {
        RaycastWorldResult::miss()
    }

    /// `RaycastWorld::raycast_intersections_with_ray`（0x1408cc8a0）：返回全部命中
    /// （地形无条件查询 + 17 槽 `QueryPipeline::intersections_with_ray` 逐槽收集），
    /// 结果 `Vec<RaycastWorldResult>` 按 **toi 升序** 排序（O：`sort_unstable_by`
    /// closure$2 按 toi 比较；<0x15 用 insertion_sort_shift_left，否则 ipnsort）。
    ///
    /// O：地形 `intersect_ray` 在槽循环之前**无条件**执行（不看 mask 的 0x20 位），
    /// 地形结果以判别 0xb 推入。
    #[cfg(feature = "native-physics")]
    pub fn raycast_intersections_with_ray(
        &self,
        terrain: &TerrainHeightsData<TerrainHeightSsbo>,
        origin: glam::Vec3,
        dir: glam::Vec3,
        mask: u32,
    ) -> Vec<RaycastWorldResult> {
        let mut results = Vec::new();
        // O：地形无条件查询（fresh 0x1408cc8a0：`TerrainHeightsData::intersect_ray`
        // 在 `local_228 == 1` 时推入判别 0xb 结果）。
        if let Some(terrain_hit) = terrain.intersect_ray(
            TerrainRay {
                origin,
                direction: dir,
            },
            f32::MAX,
        ) {
            results.push(terrain_result(terrain_hit.t, origin, dir));
        }
        let ray = Ray::new(
            Point::new(origin.x, origin.y, origin.z),
            Vector::new(dir.x, dir.y, dir.z),
        );
        for (slot_index, slot) in self.slots.iter().enumerate() {
            if (mask >> slot_index) & 1 == 0 {
                continue;
            }
            slot.pipeline.intersections_with_ray(
                &slot.bodies,
                &slot.colliders,
                &ray,
                f32::MAX,
                true,
                QueryFilter::default(),
                |handle, hit| {
                    results.push(RaycastWorldResult {
                        handle: handle_region(handle, slot_index as u32),
                        position: origin + dir * hit.time_of_impact,
                        normal: glam::Vec3::new(hit.normal.x, hit.normal.y, hit.normal.z),
                        toi: hit.time_of_impact,
                        discriminant: RAYCAST_WORLD_RESULT_HIT,
                        extra: [0.0, 0.0],
                    });
                    true
                },
            );
        }
        // O：sort_unstable_by closure$2 按 toi 升序。
        results.sort_unstable_by(|a, b| {
            a.toi
                .partial_cmp(&b.toi)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }

    /// `RaycastWorld::raycast_intersections_with_ray`（非 native-physics 壳）。
    #[cfg(not(feature = "native-physics"))]
    pub fn raycast_intersections_with_ray(
        &self,
        _terrain: &TerrainHeightsData<TerrainHeightSsbo>,
        _origin: glam::Vec3,
        _dir: glam::Vec3,
        _mask: u32,
    ) -> Vec<RaycastWorldResult> {
        Vec::new()
    }
}

/// `ColliderMeshInstance`（O：`HashMap<K, ColliderMeshInstance>` 的 value，**0x18 B**）。
///
/// 机器布局按 fresh 重新锚定（`dispose_collider_mesh_instance` 0x1408d9590 +
/// `upsert_collider_entry` 0x140b40b50 的 hashbrown entry 写入，2026-10-17）：
///
/// | +0x00 | +0x04 | +0x08 | +0x0c | +0x10 | +0x14 |
/// |---|---|---|---|---|---|
/// | registry generation（精确比较） | registry index | collider.index | collider.generation | slot（u8） | padding |
///
/// `dispose` 读 `*in_R8`（+0x00）与注册表条目 +0x20 精确比较、`in_R8[1]`（+0x04）为
/// 注册表下标；rapier handle 在 +0x08/+0x0c；slot 在 +0x10。旧结构（mesh@+0x00）为
/// 语义近似，已按机器布局修正；mesh/position/rotation 不属于该 value（由 `MeshInstanceSources`
/// 与实时 collider 承载）。
#[cfg(feature = "native-physics")]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColliderMeshInstance {
    /// +0x00 注册表 generation（dispose 精确比较；与 collider handle generation 分开存）。
    pub generation: u32,
    /// +0x04 注册表下标。
    pub registry_index: u32,
    /// +0x08 rapier collider handle（index@+0x08、generation@+0x0c）。
    pub collider: rapier3d::geometry::ColliderHandle,
    /// +0x10 子世界槽号。
    pub slot: u8,
    /// +0x14 padding（value 0x18 B）。
    pub _pad: [u8; 4],
}

#[cfg(feature = "native-physics")]
impl ColliderMeshInstance {
    pub fn new(
        registry_index: u32,
        generation: u32,
        collider: rapier3d::geometry::ColliderHandle,
        slot: u8,
    ) -> Self {
        Self {
            generation,
            registry_index,
            collider,
            slot,
            _pad: [0; 4],
        }
    }

    /// `SubworldColliderHandle` 表示（index/generation/slot 均来自注册表 +0 区）。
    pub fn handle(&self) -> SubworldColliderHandle {
        SubworldColliderHandle {
            index: self.registry_index,
            generation: self.generation,
            slot: self.slot,
        }
    }
}

/// `upsert_collider` 结果（O：返回 {index@+0, generation@+8, 判别@+0xc}；
/// 判别 0=插入 / 1=替换 / 2=保持不变）。
#[cfg(feature = "native-physics")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColliderUpsert {
    Inserted { handle: SubworldColliderHandle },
    Replaced { handle: SubworldColliderHandle },
    Kept { handle: SubworldColliderHandle },
}

/// `upsert_collider<K>`（0x140252c90 / 0x1412453c0 / 0x141294980 / 0x141296510 /
/// 0x141298110 / 0x141299ca0 等泛型 monomorphization，O）：
///
/// 1. 查 `HashMap<K, ColliderMeshInstance>`；
/// 2. 已存在且**实时 collider 的 position/rotation** 与入参逐位相等（含 NaN 拒绝）→
///    **Kept**（判别 2，直接返回现有 handle，O：比较 `Collider +0x44/+0x48/+0x4c` 与
///    `+0x34..+0x40`）；O 另做 `get_collider_mesh` + `impl$18::eq`（mesh 来源相等），
///    mesh 来源链接未锚定 → 登记；
/// 3. 否则 `dispose_collider_mesh_instance` 旧值，再装配 collider（sensor=true），
///    `ColliderSet::insert` + 注册表 `ensure_registry_entry(index, gen, kind)`、槽脏标志=1、
///    `compute_aabb` → `xz_flatten_aabb` → 栅格化，`on_collider_upserted` 发布；
///    返回 **Inserted**（0）或 **Replaced**（1）。
///
/// 登记：注册表 kind 字节写入点未锚定，此处按合法碰撞网格档 `2` 建模；mesh 存入
/// `MeshInstanceSources` 的 slotmap 接线未锚定（本函数仅消费 `Arc` 用于建 shape）。
#[cfg(feature = "native-physics")]
pub fn upsert_collider<K: std::hash::Hash + Eq>(
    world: &mut RaycastWorld,
    map: &mut std::collections::HashMap<K, ColliderMeshInstance>,
    key: K,
    source: RaycastGeoSource,
    mesh: Arc<RaycastMeshData>,
    position: glam::Vec3,
    rotation: glam::Quat,
) -> ColliderUpsert {
    let slot = source.subworld() as usize;
    // O：先取旧值；实时 collider 的 Isometry 与入参逐位比较。
    let existing = map.get(&key).copied();
    if let Some(existing) = existing {
        let pose_matches = world
            .get_rapier_collider(existing.slot, existing.collider)
            .map(|collider| {
                let iso = collider.position();
                let pos = glam::Vec3::new(iso.translation.x, iso.translation.y, iso.translation.z);
                let rot = glam::Quat::from_xyzw(
                    iso.rotation.i,
                    iso.rotation.j,
                    iso.rotation.k,
                    iso.rotation.w,
                );
                // O：ordered 比较（NaN 拒绝；signed zero 相等）。
                pos == position && rot == rotation
            })
            .unwrap_or(false);
        if pose_matches {
            return ColliderUpsert::Kept {
                handle: existing.handle(),
            };
        }
        // O：姿态变化 → dispose 旧实例（释放 collider + 注册表 free-list/generation bump +
        // 发布 removed 增量）。
        dispose_collider_mesh_instance(world, &existing);
    }
    // O：`to_offset_collider_builder` + `ColliderBuilder::build`（sensor 字节 true）。
    let shape = mesh
        .shape_ref()
        .expect("upsert_collider: mesh has no shape")
        .clone_dyn();
    let collider = ColliderBuilder::new(SharedShape(Arc::from(shape)))
        .position(Isometry::from_parts(
            nalgebra::Translation3::new(position.x, position.y, position.z),
            nalgebra::UnitQuaternion::from_quaternion(nalgebra::Quaternion::new(
                rotation.w, rotation.x, rotation.y, rotation.z,
            )),
        ))
        .sensor(true)
        .build();
    // O：槽脏标志 + compute_aabb → xz_flatten_aabb → rasterize_world_space_rect。
    world.slots[slot].dirty = true;
    let aabb = collider.compute_aabb();
    let flat = xz_flatten_aabb(&Aabb3 {
        min: glam::Vec3::new(aabb.mins.x, aabb.mins.y, aabb.mins.z),
        max: glam::Vec3::new(aabb.maxs.x, aabb.maxs.y, aabb.maxs.z),
    });
    let mut tiles = world.slots[slot].dirty_tiles.clone();
    tiles.rasterize_world_rect([flat.min.x, flat.min.y, flat.max.x, flat.max.y]);
    world.slots[slot].dirty_tiles = tiles;
    // O：`ColliderSet::insert`；注册表与 collider set 同 (index, generation)。
    let collider_handle = world.slots[slot].colliders.insert(collider);
    let (index, generation) = collider_handle.into_raw_parts();
    world.ensure_registry_entry(index, generation, 2);
    let handle = SubworldColliderHandle {
        index,
        generation,
        slot: slot as u8,
    };
    let instance = ColliderMeshInstance::new(index, generation, collider_handle, slot as u8);
    // O：`Rapier::on_collider_upserted`（发布 ColliderDiff 新增/变更增量）。
    world.on_collider_upserted(ColliderDelta {
        handle: Some(handle),
        removed: false,
    });
    let replaced = map.insert(key, instance).is_some();
    if replaced {
        ColliderUpsert::Replaced { handle }
    } else {
        ColliderUpsert::Inserted { handle }
    }
}

/// `remove_collider<K>`（0x140252ad0 / 0x140b409b0 / 0x141245220 等泛型
/// monomorphization，O）：从 `HashMap<K, ColliderMeshInstance>` 移除并
/// `dispose_collider_mesh_instance`（释放 slot collider + 槽脏 + 发布 ColliderDiff 移除增量）；
/// 未找到返回 `None`（O：`*(u8*)(out+0x20) = 0x11` = `EMPTY_COLLIDER_HANDLE_TAG`）。
///
/// **walls 接入接口**（`restore_decorators` 0x14155ba30 消费，fresh）：命中时返回
/// `Some(SubworldColliderHandle)`，调用方随后对持有的 `Arc<RaycastMeshData>` 做一次
/// refcount 递减（`LOCK; *refcount -= 1; if == 0 drop_slow<RaycastMeshData>`）；未命中返回
/// `None`，调用方保留 Arc。
#[cfg(feature = "native-physics")]
pub fn remove_collider<K: std::hash::Hash + Eq>(
    world: &mut RaycastWorld,
    map: &mut std::collections::HashMap<K, ColliderMeshInstance>,
    key: &K,
) -> Option<SubworldColliderHandle> {
    let instance = map.remove(key)?;
    Some(dispose_collider_mesh_instance(world, &instance))
}

/// `dispose_collider_mesh_instance`（0x1408d9590，O）：删除 collider 网格实例 ——
/// 释放 slot 内 collider、注册表 free-list/generation-bump、AABB 栅格化脏瓦片、
/// 标记槽脏、发布 `ColliderDiff` 移除增量、返回句柄。
///
/// fresh 反编译（2026-10-17，restorer2 + 本轮 disasm 复核）逐语句：
/// 1. `ColliderSet::remove()`（query-only，无模拟 parent wake；失败 `expect_failed`）；
/// 2. registry 校验：`index = instance+0x04 < len` 且 `registry[index].generation@+0x20 ==
///    instance.generation@+0x00`，失败 `unwrap_failed`；
/// 3. **free-list 推入**（先读 kind 字节到临时）：`entry.first4 = world+0x18`（旧 head）、
///    `world+0x18 = index`、`world+0x1c -= 1`；
/// 4. `registry[index].generation = gen + 1`（失效旧句柄）；
/// 5. kind == 0x11（空判别）→ `unwrap_failed`；否则槽脏标志=1、
///    `compute_aabb` → `xz_flatten_aabb` → `iter_world_space_rect_cells`（栅格化）、
///    `publish<ColliderDiff, ColliderDelta>`（removed 增量）+ 输出句柄。
#[cfg(feature = "native-physics")]
pub fn dispose_collider_mesh_instance(
    world: &mut RaycastWorld,
    instance: &ColliderMeshInstance,
) -> SubworldColliderHandle {
    let slot = instance.slot as usize;
    let aabb = {
        let sub = &mut world.slots[slot];
        // O：先算世界 AABB（compute_aabb，栅格化输入；remove 前取）。
        let aabb = sub
            .colliders
            .get(instance.collider)
            .map(|c| c.compute_aabb());
        // These query-only colliders have no simulated parent bodies to wake.
        let _ = sub
            .colliders
            .remove(
                instance.collider,
                &mut IslandManager::new(),
                &mut sub.bodies,
                false,
            )
            .expect("dispose_collider_mesh_instance: collider missing");
        aabb
    };
    // O：registry 校验 + free-list 推入 + generation bump（顺序与 fresh 一致）。
    let index = instance.registry_index as usize;
    let generation = instance.generation;
    let entry = world
        .colliders
        .get(index)
        .unwrap_or_else(|| panic!("dispose_collider_mesh_instance: registry index invalid"));
    if entry.generation != generation {
        panic!("dispose_collider_mesh_instance: registry generation mismatch");
    }
    let kind = entry.kind();
    let old_head = world.registry_free_head;
    world.colliders[index].set_free_next(old_head);
    world.registry_free_head = index as u32;
    world.registry_live -= 1;
    world.colliders[index].generation = generation + 1;
    // O：kind == 0x11 → unwrap_failed（在 free-list/bump 之后）。
    if kind == crate::systems::collision::EMPTY_COLLIDER_HANDLE_TAG {
        panic!("dispose_collider_mesh_instance: empty collider handle tag");
    }
    // O：槽脏 + AABB → 脏瓦片栅格化。
    {
        let sub = &mut world.slots[slot];
        sub.dirty = true;
        if let Some(aabb) = aabb {
            let flat = xz_flatten_aabb(&Aabb3 {
                min: glam::Vec3::new(aabb.mins.x, aabb.mins.y, aabb.mins.z),
                max: glam::Vec3::new(aabb.maxs.x, aabb.maxs.y, aabb.maxs.z),
            });
            let mut tiles = sub.dirty_tiles.clone();
            tiles.rasterize_world_rect([flat.min.x, flat.min.y, flat.max.x, flat.max.y]);
            sub.dirty_tiles = tiles;
        }
    }
    let handle = instance.handle();
    // O：`publish<ColliderDiff, ColliderDelta>`（removed 增量）。
    let _ = world.publisher.publish(ColliderDelta {
        handle: Some(handle),
        removed: true,
    });
    handle
}

/// 私有 rapier 查询命名空间（O：`country_core::systems::collision::world::Rapier`）。
#[cfg(feature = "native-physics")]
struct Rapier;

#[cfg(feature = "native-physics")]
impl Rapier {
    /// O：0x1408cdc60。17 槽 **u32** mask 遍历；每槽 `QueryPipeline::cast_ray_and_get_normal`
    /// （solid=1，`QueryFilter::default()`）；命中后把 TOI 作为后续槽上限。
    /// 最近命中保留，等距命中由后槽替换。
    /// 未命中写判别 2@+0x34（O：iVar11==4 → +0x34=2）。
    fn raycast(
        world: &RaycastWorld,
        origin: glam::Vec3,
        dir: glam::Vec3,
        mask: u32,
    ) -> RaycastWorldResult {
        let ray = Ray::new(
            Point::new(origin.x, origin.y, origin.z),
            Vector::new(dir.x, dir.y, dir.z),
        );
        let mut result = RaycastWorldResult::miss();
        let mut maximum_toi = f32::MAX;
        for (slot_index, slot) in world.slots.iter().enumerate() {
            if (mask >> slot_index) & 1 == 0 {
                continue;
            }
            if let Some((handle, intersection)) = slot.pipeline.cast_ray_and_get_normal(
                &slot.bodies,
                &slot.colliders,
                &ray,
                maximum_toi,
                true,
                QueryFilter::default(),
            ) {
                // B 0x1408cde07: the next slot receives this hit's TOI.
                maximum_toi = intersection.time_of_impact;
                // O：命中后校验注册表并拷贝条目前 24 B（0x1408cdc60 → 0x1408ce4a0）。
                result = RaycastWorldResult {
                    handle: registry_handle_region(world, handle)
                        .unwrap_or_else(|| handle_region(handle, slot_index as u32)),
                    position: origin + dir * intersection.time_of_impact,
                    normal: glam::Vec3::new(
                        intersection.normal.x,
                        intersection.normal.y,
                        intersection.normal.z,
                    ),
                    toi: intersection.time_of_impact,
                    discriminant: RAYCAST_WORLD_RESULT_HIT,
                    extra: [0.0, 0.0],
                };
            }
        }
        result
    }

    /// O：0x1408cdf80。17 槽 Qbvh `traverse_depth_first_node` + `RayIntersectionsVisitor`：
    /// 每槽 closure 置 found 并终止遍历，命中即返回 true。
    fn intersects_any(
        world: &RaycastWorld,
        origin: glam::Vec3,
        dir: glam::Vec3,
        mask: u32,
    ) -> bool {
        let ray = Ray::new(
            Point::new(origin.x, origin.y, origin.z),
            Vector::new(dir.x, dir.y, dir.z),
        );
        for (slot_index, slot) in world.slots.iter().enumerate() {
            if (mask >> slot_index) & 1 == 0 {
                continue;
            }
            let mut found = false;
            slot.pipeline.intersections_with_ray(
                &slot.bodies,
                &slot.colliders,
                &ray,
                f32::MAX,
                true,
                QueryFilter::default(),
                |_handle, _hit| {
                    found = true;
                    false // 命中即停（O：closure 置 found 并返回 false 终止遍历）
                },
            );
            if found {
                return true;
            }
        }
        false
    }

    /// B 0x1408ccfc0: 17 masked subworld sweeps, preserving target_distance and
    /// both caller flags. Each hit tightens max_time_of_impact for later slots.
    ///
    /// fresh 逐语句（2026-10-17）：
    /// - 输出 position = `hit.witness1`（不是 origin+dir*toi）；
    /// - normal = `hit.normal1`，并按 `(origin - witness1) · normal1 < 0` 判定**整体翻转**
    ///   （XMM14/XMM15 的符号掩码序列）；
    /// - toi @+0x30、判别 0 @+0x34。
    fn spherecast(
        world: &RaycastWorld,
        origin: glam::Vec3,
        dir: glam::Vec3,
        radius: f32,
        mut options: rapier3d::parry::query::ShapeCastOptions,
        mask: u32,
    ) -> RaycastWorldResult {
        let sphere = Ball::new(radius);
        let shape_pos = Isometry::translation(origin.x, origin.y, origin.z);
        let shape_vel = Vector::new(dir.x, dir.y, dir.z);
        let mut result = RaycastWorldResult {
            toi: options.max_time_of_impact,
            ..RaycastWorldResult::miss()
        };
        for (slot_index, slot) in world.slots.iter().enumerate() {
            if (mask >> slot_index) & 1 == 0 {
                continue;
            }
            if let Some((handle, hit)) = slot.pipeline.cast_shape(
                &slot.bodies,
                &slot.colliders,
                &shape_pos,
                &shape_vel,
                &sphere,
                options,
                QueryFilter::default(),
            ) {
                // B 0x1408cd23f replaces only the next query's maximum TOI.
                options.max_time_of_impact = hit.time_of_impact;
                let witness1 = glam::Vec3::new(hit.witness1.x, hit.witness1.y, hit.witness1.z);
                let normal1 = glam::Vec3::new(hit.normal1.x, hit.normal1.y, hit.normal1.z);
                // O：点积 < 0 → 翻转整个法线。
                let normal = if (origin - witness1).dot(normal1) < 0.0 {
                    -normal1
                } else {
                    normal1
                };
                result = RaycastWorldResult {
                    handle: registry_handle_region(world, handle)
                        .unwrap_or_else(|| handle_region(handle, slot_index as u32)),
                    position: witness1,
                    normal,
                    toi: hit.time_of_impact,
                    discriminant: RAYCAST_WORLD_RESULT_HIT,
                    extra: [0.0, 0.0],
                };
            }
        }
        result
    }

    /// O：0x1408ce180。17 槽 **u32** mask 遍历；每槽 `QueryPipeline::cast_ray`
    /// （solid=1，`QueryFilter::default()`）；命中后收紧下一槽 TOI 上限，等距后槽覆盖。
    /// 输出 `RaycastSimpleResult`：命中拷贝注册表条目前 24 B + position + toi；
    /// 全未中只写 +0x00 首字节 0x11。
    fn raycast_simple(
        world: &RaycastWorld,
        origin: glam::Vec3,
        dir: glam::Vec3,
        mask: u32,
    ) -> RaycastSimpleResult {
        let ray = Ray::new(
            Point::new(origin.x, origin.y, origin.z),
            Vector::new(dir.x, dir.y, dir.z),
        );
        let mut result = RaycastSimpleResult::miss();
        let mut maximum_toi = f32::MAX;
        for (slot_index, slot) in world.slots.iter().enumerate() {
            if (mask >> slot_index) & 1 == 0 {
                continue;
            }
            if let Some((handle, toi)) = slot.pipeline.cast_ray(
                &slot.bodies,
                &slot.colliders,
                &ray,
                maximum_toi,
                true,
                QueryFilter::default(),
            ) {
                // B 0x1408ce180: a hit shortens the next slot's maximum TOI.
                maximum_toi = toi;
                result = RaycastSimpleResult {
                    handle: registry_handle_region(world, handle)
                        .unwrap_or_else(|| handle_region(handle, slot_index as u32)),
                    position: origin + dir * toi,
                    toi,
                };
            }
        }
        result
    }

    /// O：0x1408cdc40。槽范围访问器（原输出 {start=&self+0x20, end=&self+0x3cb0, 0}，
    /// 即 17 槽迭代区间）。
    ///
    /// 消费方 `shape_overlaps_with` / `iter_shape_overlaps` / `decorator_overlaps_with_others`
    /// 依赖 `into_transformed_shape` / `RaycastGeoSource` / `OffsetColliderTransform` 类型族
    /// （登记项），落地后接上；当前无调用方，`#[allow(dead_code)]`。
    #[allow(dead_code)]
    fn subworlds(_world: &RaycastWorld) -> std::ops::Range<usize> {
        0..SUBWORLD_SLOT_COUNT as usize
    }

    /// O：0x1408ccc90 支撑实现。fresh disasm（2026-10-17）：
    /// - 无 mask；固定槽表 `[0,1,3,7,6,4,0x0b,2,0x0c,0x0d]`（10 项，常量序列
    ///   `0x020b040607030100 + word 0x0d0c` @ RSP+0xd8）逐槽 `cast_shape`；
    /// - 最后一次命中覆盖；命中 position=origin+dir*toi、normal 写 0；
    /// - **判别写 +0x30（toi 槽）**：命中 0 / 未中 2（+0x34 不写）。
    fn legacy_spherecast(
        world: &RaycastWorld,
        origin: glam::Vec3,
        dir: glam::Vec3,
        radius: f32,
    ) -> RaycastWorldResult {
        const LEGACY_SPHERE_SLOTS: [usize; 10] = [0, 1, 3, 7, 6, 4, 0x0b, 2, 0x0c, 0x0d];
        let sphere = Ball::new(radius);
        let shape_pos = Isometry::translation(origin.x, origin.y, origin.z);
        let shape_vel = Vector::new(dir.x, dir.y, dir.z);
        let mut result = RaycastWorldResult::miss();
        for slot_index in LEGACY_SPHERE_SLOTS {
            let slot = &world.slots[slot_index];
            if let Some((handle, hit)) = slot.pipeline.cast_shape(
                &slot.bodies,
                &slot.colliders,
                &shape_pos,
                &shape_vel,
                &sphere,
                rapier3d::parry::query::ShapeCastOptions::default(),
                QueryFilter::default(),
            ) {
                // O：legacy 命中写 handle + position；normal 写 0；+0x30 写 0（命中判别）。
                // 句柄区 = 注册表条目前 24 B（0x1408ccc90 的 `local_1b8 = *puVar1` 序列）。
                result = RaycastWorldResult {
                    handle: registry_handle_region(world, handle)
                        .unwrap_or_else(|| handle_region(handle, slot_index as u32)),
                    position: origin + dir * hit.time_of_impact,
                    normal: glam::Vec3::ZERO,
                    toi: RAYCAST_WORLD_RESULT_HIT as f32,
                    discriminant: RAYCAST_WORLD_RESULT_HIT,
                    extra: [0.0, 0.0],
                };
            }
        }
        // O：全未中时 +0x30 写 2（legacy 特例，同 legacy_raycast）。
        if result.is_miss() {
            result.toi = RAYCAST_WORLD_RESULT_MISS as f32;
        }
        result
    }
}

/// B 0x1408cc410..0x1408cc49c. The fallback pair loaded from 0x142aa3270
/// is (0.0, f32::from_bits(0x1e3c_e508)), and MAXPS selects the computed operand
/// on equality. Ordered comparisons preserve -0 in the numerator and use the
/// fallback for NaN. Direction is not assumed normalized.
#[cfg(feature = "native-physics")]
fn terrain_spherecast_time_of_impact(
    origin: glam::Vec3,
    direction: glam::Vec3,
    radius: f32,
    terrain_position: glam::Vec3,
) -> f32 {
    let delta = origin - terrain_position;
    let distance_squared = (delta.y * delta.y + delta.x * delta.x) + delta.z * delta.z;
    let numerator = distance_squared.sqrt() - radius;
    let direction_squared =
        direction.z * direction.z + (direction.y * direction.y + direction.x * direction.x);
    let direction_length = direction_squared.sqrt();
    let minimum_length = f32::from_bits(0x1e3c_e508);
    let numerator = if numerator >= 0.0 { numerator } else { 0.0 };
    let denominator = if direction_length >= minimum_length {
        direction_length
    } else {
        minimum_length
    };
    numerator / denominator
}

/// `Raycasting`（O：旧包装，转发 `RaycastWorld` + 地形数据）。
///
/// Resource（组 A restorer 接线）：`system_water::update_water_audio_grid` 的 BASELINE 参数面
/// 为裸 `Raycasting`（PDB 候选接口），以 `Res<Raycasting>` 访问。
#[derive(Resource)]
pub struct Raycasting {
    /// 底层碰撞世界（非 native-physics 下 stub 方法不读，仅保留类型合同）。
    #[cfg_attr(not(feature = "native-physics"), allow(dead_code))]
    world: RaycastWorld,
}

impl Raycasting {
    /// `Raycasting::raycast`（0x1408cc2e0）：转发 `raycast_w_terrain`（0x1408cd430，
    /// 含地形 0x20 位处理）。
    #[cfg(feature = "native-physics")]
    pub fn raycast(
        &self,
        terrain: &TerrainHeightsData<TerrainHeightSsbo>,
        origin: glam::Vec3,
        dir: glam::Vec3,
        mask: u32,
    ) -> RaycastWorldResult {
        self.world.raycast_w_terrain(terrain, origin, dir, mask)
    }

    /// `Raycasting::raycast`（非 native-physics 壳：查询不落地）。
    #[cfg(not(feature = "native-physics"))]
    pub fn raycast(
        &self,
        _terrain: &TerrainHeightsData<TerrainHeightSsbo>,
        _origin: glam::Vec3,
        _dir: glam::Vec3,
        _mask: u32,
    ) -> RaycastWorldResult {
        RaycastWorldResult::miss()
    }

    /// `Raycasting::intersects_any`（0x1408cc320）：mask 含地形 0x20 位时
    /// `TerrainHeightsData::intersect_ray` 命中即返回 true（地形短路）；否则 `Rapier::intersects_any`。
    #[cfg(feature = "native-physics")]
    pub fn intersects_any(
        &self,
        terrain: &TerrainHeightsData<TerrainHeightSsbo>,
        origin: glam::Vec3,
        dir: glam::Vec3,
        mask: u32,
    ) -> bool {
        if mask & 0x20 != 0 {
            let ray = TerrainRay {
                origin,
                direction: dir,
            };
            if terrain.intersect_ray(ray, f32::MAX).is_some() {
                return true;
            }
        }
        Rapier::intersects_any(&self.world, origin, dir, mask)
    }

    /// `Raycasting::intersects_any`（非 native-physics 壳）。
    #[cfg(not(feature = "native-physics"))]
    pub fn intersects_any(
        &self,
        _terrain: &TerrainHeightsData<TerrainHeightSsbo>,
        _origin: glam::Vec3,
        _dir: glam::Vec3,
        _mask: u32,
    ) -> bool {
        false
    }

    /// Existing terrain-aware sphere sweep API, using dependency-default options.
    #[cfg(feature = "native-physics")]
    pub fn spherecast(
        &self,
        terrain: &TerrainHeightsData<TerrainHeightSsbo>,
        origin: glam::Vec3,
        dir: glam::Vec3,
        radius: f32,
        mask: u32,
    ) -> RaycastWorldResult {
        self.spherecast_with_options(
            terrain,
            origin,
            dir,
            radius,
            rapier3d::parry::query::ShapeCastOptions::default(),
            mask,
        )
    }

    /// `Raycasting::spherecast`（0x1408cc390）：转发 `RaycastWorld::spherecast` + 地形合并：
    /// Terrain ray limit is options.max_time_of_impact + radius; the resulting
    /// sphere TOI must fit options.max_time_of_impact. A strict nearer collider
    /// wins; equal-distance terrain replaces it. The original denominator floor
    /// is 1e-20 (0x142aa3274), with explicit NaN fallback.
    #[cfg(feature = "native-physics")]
    pub fn spherecast_with_options(
        &self,
        terrain: &TerrainHeightsData<TerrainHeightSsbo>,
        origin: glam::Vec3,
        dir: glam::Vec3,
        radius: f32,
        options: rapier3d::parry::query::ShapeCastOptions,
        mask: u32,
    ) -> RaycastWorldResult {
        let mut result = self
            .world
            .spherecast_with_options(origin, dir, radius, options, mask);
        if mask & 0x20 != 0 {
            let ray = TerrainRay {
                origin,
                direction: dir,
            };
            let terrain_limit = options.max_time_of_impact + radius;
            if let Some(terrain_hit) = terrain.intersect_ray(ray, terrain_limit) {
                let toi =
                    terrain_spherecast_time_of_impact(origin, dir, radius, terrain_hit.position);
                if toi <= options.max_time_of_impact {
                    // O：`local_70 < fVar9`（rapier toi < 地形 toi）才保留 rapier。
                    if !(result.is_hit() && result.toi < toi) {
                        result = terrain_result(toi, origin, dir);
                    }
                }
            }
        }
        result
    }
}

// ---------------------------------------------------------------------------
// 缺类型落地（impl-collision-types，2026-10-17）：OffsetColliderTransform /
// RaycastMeshKey / MeshInstanceSources / 变换族 / 重叠查询族 / on_level_reload。
//
// 证据：ghidra shared fresh 反编译（b8-closures-visualize-rt / b7-flush-intersect /
// b1-collider-lifecycle / b2-mesh-family / b3-medium + 本轮 0x140c91b30/60/80），
// 逐语句判等见 reports/interaction/collision/REBUILD-COLLISION-TYPES.md。
// ---------------------------------------------------------------------------

// `RaycastMeshKey`（O：`slotmap::basic::SlotMap<RaycastMeshKey, RaycastMesh>` 的键；
// `update_rapier_colliders`/`iter_shape_overlaps` 消费；slotmap `new_key_type!`）。
#[cfg(feature = "native-physics")]
slotmap::new_key_type! {
    pub struct RaycastMeshKey;
}

/// `OffsetColliderTransform`（O：`{ translation: Vec3 @+0x00, rotation: Quat @+0x0c,
/// shape: Arc<dyn Shape> @+0x20 }`，0x30 B；布局证据 = `into_transformed_shape`
/// impl$42 0x1408d98c0 / impl$45 0x140b26d20 的最终 store 序列）。
///
/// Rust 布局经 repr(Rust) 对齐与机器偏移不同（Quat 16 B 对齐 → +0x10），
/// 字段语义一致；字节级偏移登记。
#[cfg(feature = "native-physics")]
#[derive(Clone)]
pub struct OffsetColliderTransform {
    /// 平移（O：+0x00）。
    pub position: glam::Vec3,
    /// 旋转（O：+0x0c）。
    pub rotation: glam::Quat,
    /// 形状 Arc（O：+0x20/+0x28）。
    pub shape: Arc<dyn Shape>,
}

/// B transformed-shape adapters copy the unit quaternion directly into the
/// dependency Isometry; do not renormalize its components at this boundary.
#[cfg(feature = "native-physics")]
fn collider_isometry(position: glam::Vec3, rotation: glam::Quat) -> Isometry<f32> {
    Isometry::from_parts(
        nalgebra::Translation3::new(position.x, position.y, position.z),
        nalgebra::UnitQuaternion::new_unchecked(nalgebra::Quaternion::new(
            rotation.w, rotation.x, rotation.y, rotation.z,
        )),
    )
}

#[cfg(feature = "native-physics")]
impl OffsetColliderTransform {
    /// `impl$47::as_collider_shape`（0x1408d9990，O）：取形状 Arc（克隆引用计数）。
    pub fn as_collider_shape(&self) -> Arc<dyn Shape> {
        self.shape.clone()
    }
}

/// `MeshInstanceSources`（O：`RaycastWorld +0x3dc8` 的 slotmap 网格实例源；
/// `RaycastMeshKey → RaycastMesh`）。on_level_reload 重建时 drop 旧值（0xe0 B
/// 默认副本）→ 新空 slotmap。布局含 slotmap 头 + slots Vec + free list（登记
/// 精确字段偏移：`Vec<Slot<RaycastMesh>> data@+0xe0/len@+0xe8`）。
#[cfg(feature = "native-physics")]
#[derive(Default)]
pub struct MeshInstanceSources {
    /// 底层 slotmap（O：`slotmap::basic::SlotMap<RaycastMeshKey, RaycastMesh>`）。
    meshes: slotmap::basic::SlotMap<RaycastMeshKey, RaycastMesh>,
}

#[cfg(feature = "native-physics")]
impl MeshInstanceSources {
    /// 新建空源（O：on_level_reload 的 0xe0 B 默认副本）。
    pub fn new() -> Self {
        Self {
            meshes: slotmap::basic::SlotMap::with_key(),
        }
    }

    /// 插入网格实例（O：`insert<RaycastMeshKey, RaycastMesh>`）。
    pub fn insert(&mut self, mesh: RaycastMesh) -> RaycastMeshKey {
        self.meshes.insert(mesh)
    }

    /// 按键取网格。
    pub fn get(&self, key: RaycastMeshKey) -> Option<&RaycastMesh> {
        self.meshes.get(key)
    }

    /// 移除网格（O：`drop_in_place<Vec<Slot<RaycastMesh>>>` 面）。
    pub fn remove(&mut self, key: RaycastMeshKey) -> Option<RaycastMesh> {
        self.meshes.remove(key)
    }

    /// 遍历（O：slotmap 迭代）。
    pub fn iter(&self) -> impl Iterator<Item = (RaycastMeshKey, &RaycastMesh)> {
        self.meshes.iter()
    }
}

impl RaycastWorld {
    /// `RaycastWorld::on_level_reload`（0x1408d8710，O）：关卡重载 —— 重建整个碰撞世界。
    ///
    /// fresh 反编译（b1-update-rapier 同批）：
    /// 1. 计数器写 `*[self+0x10] = [self+0x1c]`（+0x10/+0x1c 计数器，flush_rapier_colliders
    ///    同款写；Rust 未建模，登记）；
    /// 2. 重建：`default<RapierSubWorld>`×17（0x390 B/槽）+ 发布器复位 + 空
    ///    `MeshInstanceSources`（0xe0 B 默认副本写 @+0x3dc8、`Vec<Slot<RaycastMesh>>`
    ///    指针 @+0x3ea8/+0x3eb0）。
    ///
    /// Rust 侧等价 = 全量 `RaycastWorld::new()`（结构判等；发布器/槽/源一并复位）。
    pub fn on_level_reload(&mut self) {
        *self = RaycastWorld::new();
    }

    /// `RaycastWorld::into_transformed_shape`（impl$42 0x1408d98c0，O）：
    /// 从子世界 collider 拷贝变换 + 形状 Arc → `OffsetColliderTransform`。
    /// `iter_shape_overlaps<SubworldColliderHandle>` 消费。
    ///
    /// fresh 反编译：`ColliderSet::get_mut_internal` 读活 collider，拷贝
    /// `{trans@+0x34.., quat@+0x40..}` 与 shape Arc（+0x10/+0x18）；未找到 unwrap。
    #[cfg(feature = "native-physics")]
    pub fn into_transformed_shape(
        &self,
        handle: SubworldColliderHandle,
    ) -> OffsetColliderTransform {
        let slot = &self.slots[handle.slot as usize];
        let collider_handle =
            rapier3d::geometry::ColliderHandle::from_raw_parts(handle.index, handle.generation);
        let collider = slot
            .colliders
            .get(collider_handle)
            .unwrap_or_else(|| panic!("into_transformed_shape: collider handle invalid"));
        let iso = collider.position();
        OffsetColliderTransform {
            position: glam::Vec3::new(iso.translation.x, iso.translation.y, iso.translation.z),
            rotation: glam::Quat::from_xyzw(
                iso.rotation.i,
                iso.rotation.j,
                iso.rotation.k,
                iso.rotation.w,
            ),
            shape: collider.shared_shape().0.clone(),
        }
    }
}

/// `into_transformed_shape`（impl$45 0x140b26d20，O）：`(IsoTransform, &RaycastMeshData)`
/// → `OffsetColliderTransform`（`IsoTransform::nested_in` 组合 + 网格 shape Arc）。
/// `iter_shape_overlaps<(IsoTransform, &RaycastMeshData)>` 消费。
///
/// fresh 反编译：`lVar1 = *(in_RDX + 0x20)`（RaycastMeshData shape Arc @+0x20），
/// `nested_in` 组合变换，输出 `{变换, 形状}`。nested_in 精确语义登记（见 utils
/// `IsoTransform::nested_in`）。
#[cfg(feature = "native-physics")]
pub fn into_transformed_shape_iso(
    transform: &utils::iso_transform::IsoTransform,
    mesh: &RaycastMeshData,
    parent: &utils::iso_transform::IsoTransform,
) -> OffsetColliderTransform {
    let nested = transform.nested_in(parent);
    let shape = mesh
        .shape_ref()
        .expect("into_transformed_shape_iso: mesh has no shape")
        .clone_dyn();
    OffsetColliderTransform {
        position: nested.translation,
        rotation: nested.rotation,
        shape: Arc::from(shape),
    }
}

/// `into_transformed_shape`（impl$41 0x14155d340，O）：`Rectangle3d` → 变换后形状参数
/// `{旋转, 平移, 半边长}`。此兼容 helper 返回旋转与半边长；调用方取矩形中心。
///
/// B 0x14155d340: extent*0.5 @+0x1c、center.x/(bottom+top)*0.5/center.y @+0x10、
/// Quat::from_rotation_arc(Vec3::X, axis.x0y()) @+0x00。
#[cfg(feature = "native-physics")]
pub fn rectangle_to_transformed(
    rect: utils::geometry::rectangle::Rectangle3d,
) -> (glam::Quat, glam::Vec3) {
    let extent = rect.extent();
    let half_extents = extent * 0.5;
    let axis = rect.footprint.axis;
    let rotation =
        glam::Quat::from_rotation_arc(glam::Vec3::X, glam::Vec3::new(axis.x, 0.0, axis.y));
    (rotation, half_extents)
}

/// `to_offset_collider_builder`（impl$23 0x1408d0390 / impl$21 0x140b26a80 /
/// impl$22 0x140b26bd0，O）：`OffsetColliderTransform` → rapier `ColliderBuilder`
/// （形状 Arc 增引用计数 + 位置/旋转 + sensor 字节）。
///
/// fresh 反编译：`plVar3 = *(transform + 0x20)`（shape Arc）`LOCK/increment`（克隆），
/// `ColliderBuilder::new` + 拷贝 shape 到 builder +0x60..+0x68、位置/旋转拷贝到
/// +0x00..+0x1f、`*(builder + 0x96) = 1`（sensor）。Rust 以
/// `ColliderBuilder::new(SharedShape).position(...).sensor(true)` 等价装配。
#[cfg(feature = "native-physics")]
pub fn to_offset_collider_builder(
    t: &OffsetColliderTransform,
) -> rapier3d::geometry::ColliderBuilder {
    let shape = t.shape.clone();
    let iso = collider_isometry(t.position, t.rotation);
    rapier3d::geometry::ColliderBuilder::new(SharedShape(shape))
        .position(iso)
        .sensor(true)
}

impl RaycastWorld {
    /// `iter_shape_overlaps`（0x14129b780 / 0x14129bad0 / 0x140254890 / 0x140254aa0，
    /// O）：以 `shape` 形状扫掠 17 槽（mask 门控），对每槽
    /// `QueryPipeline::intersections_with_shape` 回调 `(SubworldColliderHandle,
    /// &RaycastGeoSource)`；回调返回 true 即停止。
    ///
    /// fresh 反编译：`into_transformed_shape` + `as_collider_shape` 取形状 →
    /// 逐槽 `intersections_with_shape`（closure 置 `local_4a=1` 终止外层循环）；
    /// 槽步进 0x390、mask 按槽位门控。Rust 侧等价实现（+0x3d80 计数器面登记）。
    #[cfg(feature = "native-physics")]
    pub fn iter_shape_overlaps<F: FnMut(SubworldColliderHandle, &RaycastGeoSource) -> bool>(
        &self,
        shape: &dyn Shape,
        mask: u32,
        callback: F,
    ) {
        self.iter_shape_overlaps_at(shape, &Isometry::identity(), mask, callback);
    }

    #[cfg(feature = "native-physics")]
    fn iter_shape_overlaps_at<F: FnMut(SubworldColliderHandle, &RaycastGeoSource) -> bool>(
        &self,
        shape: &dyn Shape,
        shape_position: &Isometry<f32>,
        mask: u32,
        mut callback: F,
    ) {
        for (slot_index, slot) in self.slots.iter().enumerate() {
            if (mask >> slot_index) & 1 == 0 {
                continue;
            }
            let mut stop = false;
            slot.pipeline.intersections_with_shape(
                &slot.bodies,
                &slot.colliders,
                shape_position,
                shape,
                QueryFilter::default(),
                |handle| {
                    let (index, generation) = handle.into_raw_parts();
                    let source = RaycastGeoSource::from_slot(slot_index as u8);
                    let result = callback(
                        SubworldColliderHandle {
                            index,
                            generation,
                            slot: slot_index as u8,
                        },
                        &source,
                    );
                    stop = result;
                    // Rapier returns true to continue; this API returns true to stop.
                    !result
                },
            );
            if stop {
                break;
            }
        }
    }

    /// `iter_shape_overlaps<SubworldColliderHandle>`（O：impl$42 变换 + 上族遍历）。
    #[cfg(feature = "native-physics")]
    pub fn iter_shape_overlaps_handle<
        F: FnMut(SubworldColliderHandle, &RaycastGeoSource) -> bool,
    >(
        &self,
        handle: SubworldColliderHandle,
        mask: u32,
        callback: F,
    ) {
        let t = self.into_transformed_shape(handle);
        let shape = t.as_collider_shape();
        let position = collider_isometry(t.position, t.rotation);
        self.iter_shape_overlaps_at(shape.as_ref(), &position, mask, callback);
    }

    /// `iter_shape_overlaps<(IsoTransform, &RaycastMeshData)>`（O：impl$45 变换）。
    #[cfg(feature = "native-physics")]
    pub fn iter_shape_overlaps_iso<F: FnMut(SubworldColliderHandle, &RaycastGeoSource) -> bool>(
        &self,
        transform: &utils::iso_transform::IsoTransform,
        mesh: &RaycastMeshData,
        parent: &utils::iso_transform::IsoTransform,
        mask: u32,
        callback: F,
    ) {
        let t = into_transformed_shape_iso(transform, mesh, parent);
        let shape = t.as_collider_shape();
        let position = collider_isometry(t.position, t.rotation);
        self.iter_shape_overlaps_at(shape.as_ref(), &position, mask, callback);
    }

    /// `shape_overlaps_with<Rectangle3d>`（0x142612d10，O）：矩形 → 变换形状，
    /// 17 槽 mask 门控 `QueryPipeline::intersection_with_shape`，任一命中即 true。
    ///
    /// fresh 反编译：`into_transformed_shape`（impl$41）+ `subworlds` 循环
    /// `(mask>>slot&1)==0` 跳过、`intersection_with_shape` 命中置标志，命中停。
    #[cfg(feature = "native-physics")]
    pub fn shape_overlaps_with(
        &self,
        rect: utils::geometry::rectangle::Rectangle3d,
        mask: u32,
    ) -> bool {
        let (rotation, half) = rectangle_to_transformed(rect);
        let center = glam::Vec3::new(
            rect.footprint.center.x,
            (rect.bottom + rect.top) * 0.5,
            rect.footprint.center.y,
        );
        let position = collider_isometry(center, rotation);
        let shape = SharedShape::cuboid(half.x, half.y, half.z);
        for (slot_index, slot) in self.slots.iter().enumerate() {
            if (mask >> slot_index) & 1 == 0 {
                continue;
            }
            let hit = slot.pipeline.intersection_with_shape(
                &slot.bodies,
                &slot.colliders,
                &position,
                shape.as_ref(),
                QueryFilter::default(),
            );
            if hit.is_some() {
                return true;
            }
        }
        false
    }

    /// `decorator_overlaps_with_others<(OffsetColliderTransform, Arc<dyn Shape>)>`
    /// （0x14129be00，O）：装饰 collider 与其它 collider 是否重叠。
    ///
    /// fresh 反编译：`into_transformed_shape`（impl$43 拷贝）→ `index` 取注册表 →
    /// `as_collider_shape` → `intersection_with_shape`；命中则
    /// `get_mut_internal` + `DecoratorRank::new` 校验（登记）。Rust 侧以
    /// `OffsetColliderTransform` 形状扫掠自身槽 + 注册表 kind 门控近似。
    #[cfg(feature = "native-physics")]
    pub fn decorator_overlaps_with_others(
        &self,
        decorator: &OffsetColliderTransform,
        mask: u32,
    ) -> bool {
        let shape = decorator.as_collider_shape();
        let position = collider_isometry(decorator.position, decorator.rotation);
        // O：单装饰 collider 落其来源槽；Rust 以全 mask 槽扫掠（来源槽映射登记）。
        for (slot_index, slot) in self.slots.iter().enumerate() {
            if (mask >> slot_index) & 1 == 0 {
                continue;
            }
            let hit = slot.pipeline.intersection_with_shape(
                &slot.bodies,
                &slot.colliders,
                &position,
                shape.as_ref(),
                QueryFilter::default(),
            );
            if hit.is_some() {
                return true;
            }
        }
        false
    }
}

impl RaycastGeoSource {
    /// `from_slot`（O：`subworld()` 的反向映射；由 0x142aef03c 表取逆）。
    ///
    /// 槽→来源：`[0,2,3,6,10,11,8,7,4,1,9,12,13,14,5,15,16]`。
    pub fn from_slot(slot: u8) -> RaycastGeoSource {
        const SLOT_TO_SOURCE: [u8; 17] = [0, 2, 3, 6, 10, 11, 8, 7, 4, 1, 9, 12, 13, 14, 5, 15, 16];
        let discriminant = SLOT_TO_SOURCE[slot as usize];
        unsafe { std::mem::transmute(discriminant) }
    }
}

#[cfg(all(test, feature = "native-physics"))]
mod transform_family_tests {
    use super::*;
    use glam::{Quat, Vec3};

    #[test]
    fn offset_transform_builds_rapier_collider_builder() {
        let shape: Arc<dyn Shape> = Arc::new(Cuboid::new(Vector::new(0.5, 0.5, 0.5)));
        let t = OffsetColliderTransform {
            position: Vec3::new(1.0, 2.0, 3.0),
            rotation: Quat::from_rotation_y(0.5),
            shape: shape.clone(),
        };
        let builder = to_offset_collider_builder(&t);
        let collider = builder.build();
        let pos = collider.position();
        assert_eq!(pos.translation.x, 1.0);
        assert_eq!(pos.translation.y, 2.0);
        assert_eq!(pos.translation.z, 3.0);
        assert!(collider.is_sensor());
        // as_collider_shape 取回形状 Arc。
        let back = t.as_collider_shape();
        assert!(Arc::ptr_eq(&back, &shape));
    }

    #[test]
    fn on_level_reload_rebuilds_the_world() {
        let mut world = RaycastWorld::new();
        // 构造一次 collider 注册表 + 网格源，确保重载后复位。
        world.upsert_collider_entry(ColliderRegistryEntry::empty());
        world.on_level_reload();
        assert!(world.collider_registry().is_empty());
        assert_eq!(world.slots.len(), SUBWORLD_SLOT_COUNT as usize);
    }

    #[test]
    fn mesh_sources_insert_get_remove_roundtrip() {
        let mut src = MeshInstanceSources::new();
        let data = Arc::new(RaycastMeshData::default());
        let mesh = RaycastMesh::new(data, RaycastGeoSource::Terrain);
        let key = src.insert(mesh.clone());
        assert!(src.get(key).is_some());
        let removed = src.remove(key);
        assert!(removed.is_some());
        assert!(src.get(key).is_none());
    }

    /// O：source→slot 表 0x142aef03c 与其逆（fresh read_memory，2026-10-17）。
    #[test]
    fn geo_source_from_slot_roundtrips() {
        // 判别 5（旧名 Terrain）按表落槽 14；槽 14 反向映射回判别 5。
        assert_eq!(RaycastGeoSource::Terrain.subworld(), 14);
        assert_eq!(RaycastGeoSource::from_slot(14), RaycastGeoSource::Terrain);
        // 槽 5 ← 判别 11；判别 7 ↔ 槽 7。
        assert_eq!(RaycastGeoSource::from_slot(5), RaycastGeoSource::Slot11);
        assert_eq!(RaycastGeoSource::PlatformStair.subworld(), 7);
        for source in 0..17u8 {
            let slot = RaycastGeoSource::SOURCE_TO_SLOT[source as usize];
            assert_eq!(
                RaycastGeoSource::from_slot(slot) as u8,
                source,
                "source {source} -> slot {slot} must roundtrip"
            );
        }
    }

    /// `ColliderMeshInstance` 机器布局 0x18 B：{gen@0, index@4, handle@8, slot@0x10, pad}。
    #[test]
    fn collider_mesh_instance_machine_layout() {
        assert_eq!(core::mem::size_of::<ColliderMeshInstance>(), 0x18);
        assert_eq!(
            core::mem::offset_of!(ColliderMeshInstance, generation),
            0x00
        );
        assert_eq!(
            core::mem::offset_of!(ColliderMeshInstance, registry_index),
            0x04
        );
        assert_eq!(core::mem::offset_of!(ColliderMeshInstance, collider), 0x08);
        assert_eq!(core::mem::offset_of!(ColliderMeshInstance, slot), 0x10);
    }

    /// `MeshIntersectionArtifacts` 0x40 B 边载荷布局。
    #[test]
    fn mesh_intersection_artifacts_layout() {
        use crate::systems::collision::MeshIntersectionArtifacts;
        assert_eq!(core::mem::size_of::<MeshIntersectionArtifacts>(), 0x40);
        assert_eq!(
            core::mem::offset_of!(MeshIntersectionArtifacts, polylines),
            0x00
        );
        assert_eq!(
            core::mem::offset_of!(MeshIntersectionArtifacts, mesh_ref_a),
            0x18
        );
        assert_eq!(
            core::mem::offset_of!(MeshIntersectionArtifacts, mesh_ref_b),
            0x20
        );
        assert_eq!(core::mem::offset_of!(MeshIntersectionArtifacts, a), 0x28);
        assert_eq!(core::mem::offset_of!(MeshIntersectionArtifacts, b), 0x34);
    }

    /// `RaycastMeshData` 0x78 B 布局（fresh new_indexed/new_unindexed store 序列）：
    /// 顶点 ptr/len @+0x00/+0x08、索引 ptr/len @+0x10/+0x18、shape @+0x20、法线 @+0x30、
    /// uv @+0x40、常量区 @+0x50..+0x68、hash @+0x70。
    #[test]
    fn raycast_mesh_data_machine_layout() {
        assert_eq!(core::mem::size_of::<RaycastMeshData>(), 0x78);
        assert_eq!(core::mem::offset_of!(RaycastMeshData, vertices_ptr), 0x00);
        assert_eq!(core::mem::offset_of!(RaycastMeshData, vertices_len), 0x08);
        assert_eq!(core::mem::offset_of!(RaycastMeshData, indices_ptr), 0x10);
        assert_eq!(core::mem::offset_of!(RaycastMeshData, count), 0x18);
        assert_eq!(core::mem::offset_of!(RaycastMeshData, shape), 0x20);
        assert_eq!(core::mem::offset_of!(RaycastMeshData, normals_ptr), 0x30);
        assert_eq!(core::mem::offset_of!(RaycastMeshData, normals_len), 0x38);
        assert_eq!(core::mem::offset_of!(RaycastMeshData, uvs_ptr), 0x40);
        assert_eq!(core::mem::offset_of!(RaycastMeshData, uvs_len), 0x48);
        assert_eq!(core::mem::offset_of!(RaycastMeshData, scale), 0x68);
        assert_eq!(core::mem::offset_of!(RaycastMeshData, hash), 0x70);
    }

    /// `new_indexed` 的 Option 哈希门控：`None` 与 `Some(empty)` 指纹不同（O：0x1408cee40）。
    #[test]
    fn new_indexed_option_hash_gate_distinguishes_none_from_empty() {
        let indices = [[0u32, 1, 2]];
        let vertices = [
            glam::Vec3::new(0.0, 0.0, 0.0),
            glam::Vec3::new(1.0, 0.0, 0.0),
            glam::Vec3::new(0.0, 1.0, 0.0),
        ];
        let none = RaycastMeshData::new_indexed(&indices, &vertices, None, None);
        let empty_normals = RaycastMeshData::new_indexed(&indices, &vertices, Some(&[]), None);
        let empty_uvs = RaycastMeshData::new_indexed(&indices, &vertices, None, Some(&[]));
        assert_ne!(none.hash, empty_normals.hash);
        assert_ne!(none.hash, empty_uvs.hash);
        // 注：`Some(empty)` 的 normals/uvs 折叠哈希相同（空切片只写 len=0），不额外区分。
        assert_eq!(none.count, 1);
        assert_eq!(none.indices().len(), 1);
        assert_eq!(none.vertices().len(), 3);
        assert_eq!(none.scale, 0x3f80000000000000);
    }

    /// `intersect_newly_added_colliders::closure_env$0` 去重/产物推送。
    #[test]
    fn intersect_newly_added_collider_dedups_and_pushes_artifacts() {
        use crate::systems::collision::MeshIntersectionArtifacts;
        let tri_a = rapier3d::parry::shape::TriMesh::new(
            vec![
                Point::new(-1.0, -1.0, 0.0),
                Point::new(-1.0, 1.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
            ],
            vec![[0u32, 1, 2]],
        );
        let tri_b = rapier3d::parry::shape::TriMesh::new(
            vec![
                Point::new(0.0, -1.0, -1.0),
                Point::new(0.0, -1.0, 1.0),
                Point::new(0.0, 1.0, 0.0),
            ],
            vec![[0u32, 1, 2]],
        );
        let mut world = RaycastWorld::new();
        world.ensure_registry_entry(0, 0, 2);
        world.ensure_registry_entry(1, 0, 2);
        let added = SubworldColliderHandle {
            index: 0,
            generation: 0,
            slot: 0,
        };
        let candidate = SubworldColliderHandle {
            index: 1,
            generation: 0,
            slot: 0,
        };
        let mut processed = std::collections::HashSet::new();
        let mut artifacts: Vec<MeshIntersectionArtifacts> = Vec::new();

        crate::systems::collision::intersect_meshes::intersect_newly_added_collider(
            &world,
            added,
            candidate,
            &tri_a,
            &tri_b,
            &processed,
            &mut artifacts,
        );
        assert_eq!(artifacts.len(), 1);
        assert_eq!(artifacts[0].a, added);
        assert_eq!(artifacts[0].b, candidate);
        assert!(!artifacts[0].polylines.is_empty());

        // 候选已在去重集 → 不再推送。
        processed.insert(candidate);
        crate::systems::collision::intersect_meshes::intersect_newly_added_collider(
            &world,
            added,
            candidate,
            &tri_a,
            &tri_b,
            &processed,
            &mut artifacts,
        );
        assert_eq!(artifacts.len(), 1);
    }

    /// `dispose` 的注册表 free-list 推入 + generation bump（O：0x1408d9590）。
    #[test]
    fn dispose_bumps_registry_generation_and_pushes_free_list() {
        let mut world = RaycastWorld::new();
        let mesh = Arc::new(RaycastMeshData::new_cuboid(
            glam::Vec3::splat(1.0),
            glam::UVec3::ZERO,
        ));
        let source = RaycastGeoSource::Slot0;
        let mut map: std::collections::HashMap<u64, ColliderMeshInstance> =
            std::collections::HashMap::new();
        let upsert = upsert_collider(
            &mut world,
            &mut map,
            7u64,
            source,
            mesh,
            glam::Vec3::ZERO,
            glam::Quat::IDENTITY,
        );
        let handle = match upsert {
            ColliderUpsert::Inserted { handle } => handle,
            other => panic!("expected Inserted, got {other:?}"),
        };
        let before = world.collider_registry()[handle.index as usize].generation;
        let removed = remove_collider(&mut world, &mut map, &7u64).expect("present");
        assert_eq!(removed, handle);
        let entry = world.collider_registry()[handle.index as usize];
        assert_eq!(entry.generation, before + 1);
        assert_eq!(world.registry_free_head, handle.index);
        assert_eq!(world.registry_live, 0);
    }

    #[test]
    fn empty_world_overlaps_report_false() {
        let world = RaycastWorld::new();
        let rect = utils::geometry::rectangle::Rectangle3d::new(
            Vec3::ZERO,
            Vec3::new(1.0, 0.0, 1.0),
            glam::Vec2::new(1.0, 0.0),
        );
        assert!(!world.shape_overlaps_with(rect, u32::MAX));
        // iter_shape_overlaps 空世界不回调。
        let mut calls = 0u32;
        world.iter_shape_overlaps(&Ball::new(0.5), u32::MAX, |_h, _s| {
            calls += 1;
            false
        });
        assert_eq!(calls, 0);
    }
}

#[cfg(all(test, feature = "native-physics"))]
mod collision_query_regressions {
    use super::*;

    fn insert_cuboid(
        world: &mut RaycastWorld,
        slot_index: usize,
        center: glam::Vec3,
        half: glam::Vec3,
    ) -> SubworldColliderHandle {
        let slot = &mut world.slots[slot_index];
        let handle = slot.colliders.insert(
            ColliderBuilder::cuboid(half.x, half.y, half.z)
                .translation(Vector::new(center.x, center.y, center.z))
                .build(),
        );
        slot.pipeline.update(&slot.colliders);
        let (index, generation) = handle.into_raw_parts();
        SubworldColliderHandle {
            index,
            generation,
            slot: slot_index as u8,
        }
    }

    #[test]
    fn ray_queries_keep_nearest_subworld_hit() {
        let mut world = RaycastWorld::new();
        insert_cuboid(
            &mut world,
            0,
            glam::Vec3::new(2.0, 0.0, 0.0),
            glam::Vec3::splat(0.5),
        );
        insert_cuboid(
            &mut world,
            1,
            glam::Vec3::new(6.0, 0.0, 0.0),
            glam::Vec3::splat(0.5),
        );
        let hit = world.raycast(glam::Vec3::ZERO, glam::Vec3::X, 3);
        assert!(hit.is_hit());
        assert_eq!(hit.toi, 1.5);
        assert_eq!(
            world.raycast_simple(glam::Vec3::ZERO, glam::Vec3::X, 3).toi,
            1.5
        );
    }

    #[test]
    fn legacy_parameterized_miss_preserves_integer_tag_bits() {
        let hit = RaycastWorld::new().legacy_raycast_w_params(glam::Vec3::ZERO, glam::Vec3::X, 1);
        assert!(hit.is_miss());
        assert_eq!(hit.toi.to_bits(), 2);
    }

    #[test]
    fn rectangle_overlap_keeps_center_and_orientation() {
        let mut world = RaycastWorld::new();
        insert_cuboid(
            &mut world,
            0,
            glam::Vec3::new(10.0, 2.0, 22.0),
            glam::Vec3::splat(0.1),
        );
        let center = glam::Vec3::new(10.0, 2.0, 20.0);
        let size = glam::Vec3::new(6.0, 2.0, 0.5);
        let rotated = utils::geometry::rectangle::Rectangle3d::new(center, size, glam::Vec2::Y);
        let unrotated = utils::geometry::rectangle::Rectangle3d::new(center, size, glam::Vec2::X);
        assert!(world.shape_overlaps_with(rotated, 1));
        assert!(!world.shape_overlaps_with(unrotated, 1));
    }

    #[test]
    fn overlap_handle_keeps_pose_and_callback_stop_rule() {
        let mut world = RaycastWorld::new();
        let center = glam::Vec3::new(10.0, 3.0, 20.0);
        let query = insert_cuboid(&mut world, 0, center, glam::Vec3::ONE);
        insert_cuboid(&mut world, 0, center, glam::Vec3::splat(0.2));
        let mut hits = 0;
        world.iter_shape_overlaps_handle(query, 1, |_, _| {
            hits += 1;
            false
        });
        assert_eq!(hits, 2, "false continues through all overlaps");
        hits = 0;
        world.iter_shape_overlaps_handle(query, 1, |_, _| {
            hits += 1;
            true
        });
        assert_eq!(hits, 1, "true stops at the first overlap");
    }

    #[test]
    fn sphere_options_keep_limit_target_distance_and_nearest_subworld() {
        use rapier3d::parry::query::ShapeCastOptions;
        let mut world = RaycastWorld::new();
        insert_cuboid(
            &mut world,
            0,
            glam::Vec3::new(2.0, 0.0, 0.0),
            glam::Vec3::splat(0.5),
        );
        insert_cuboid(
            &mut world,
            1,
            glam::Vec3::new(6.0, 0.0, 0.0),
            glam::Vec3::splat(0.5),
        );
        let mut options = ShapeCastOptions {
            max_time_of_impact: 10.0,
            target_distance: 0.0,
            stop_at_penetration: false,
            compute_impact_geometry_on_penetration: false,
        };
        let hit = world.spherecast_with_options(glam::Vec3::ZERO, glam::Vec3::X, 0.5, options, 3);
        assert!(hit.is_hit());
        assert!((hit.toi - 1.0).abs() < 1e-4);
        options.max_time_of_impact = 0.25;
        let miss = world.spherecast_with_options(glam::Vec3::ZERO, glam::Vec3::X, 0.5, options, 3);
        assert!(miss.is_miss());
        assert_eq!(miss.toi, 0.25);
        options.max_time_of_impact = 10.0;
        options.target_distance = 0.5;
        let expanded =
            world.spherecast_with_options(glam::Vec3::ZERO, glam::Vec3::X, 0.5, options, 3);
        assert!(expanded.is_hit());
        assert!((expanded.toi - 0.5).abs() < 1e-4);
    }

    #[test]
    fn sphere_terrain_uses_sweep_limit_and_preserves_existing_entry() {
        use rapier3d::parry::query::ShapeCastOptions;
        let world = Raycasting {
            world: RaycastWorld::new(),
        };
        let terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
        // Outside the terrain grid, the existing terrain owner supplies its y=0 plane.
        let origin = glam::Vec3::new(100.0, 3.0, 0.0);
        let direction = glam::Vec3::NEG_Y;
        let mut options = ShapeCastOptions::with_max_time_of_impact(3.0);
        let hit = world.spherecast_with_options(&terrain, origin, direction, 0.5, options, 0x20);
        assert!(hit.is_hit() && hit.is_terrain());
        assert_eq!(hit.toi, 2.5);
        options.max_time_of_impact = 2.0;
        assert!(
            world
                .spherecast_with_options(&terrain, origin, direction, 0.5, options, 0x20)
                .is_miss()
        );
        assert!(
            world
                .spherecast(&terrain, origin, direction, 0.5, 0x20)
                .is_hit()
        );
    }

    #[test]
    fn sphere_terrain_projection_matches_original_b_wrapper_instructions() {
        // B 0x1408cc390 executed with resolved dependency query boundaries.
        // Each row: origin[3], direction[3], radius, terrain point[3], native TOI.
        let cases: &[[u32; 11]] = &[
            [
                0x40800000, 0x40400000, 0x40000000, 0x00000000, 0xbf800000, 0x00000000, 0x3f000000,
                0x40800000, 0x00000000, 0x40000000, 0x40200000,
            ],
            [
                0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f000000,
                0x00000000, 0x00000000, 0x00000000, 0x602d78ec,
            ],
            [
                0x3f800000, 0x00000000, 0x00000000, 0x0da24260, 0x00000000, 0x00000000, 0x3f000000,
                0x00000000, 0x00000000, 0x00000000, 0x602d78ec,
            ],
            [
                0x3f800000, 0x00000000, 0x00000000, 0x1e3ce508, 0x00000000, 0x00000000, 0x3f000000,
                0x00000000, 0x00000000, 0x00000000, 0x602d78ec,
            ],
            [
                0x3f800000, 0x00000000, 0x00000000, 0x2edbe6ff, 0x00000000, 0x00000000, 0x3f000000,
                0x00000000, 0x00000000, 0x00000000, 0x4f9502f9,
            ],
            [
                0x3f800000, 0x00000000, 0x00000000, 0x40000000, 0x00000000, 0x00000000, 0x3f000000,
                0x00000000, 0x00000000, 0x00000000, 0x3e800000,
            ],
            [
                0x3f800000, 0x00000000, 0x00000000, 0x7fc00000, 0x00000000, 0x00000000, 0x3f000000,
                0x00000000, 0x00000000, 0x00000000, 0x602d78ec,
            ],
            [
                0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000,
                0x00000000, 0x00000000, 0x00000000, 0x3f800000,
            ],
            [
                0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x3f000000,
                0x00000000, 0x00000000, 0x00000000, 0x3f000000,
            ],
            [
                0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x3f800000,
                0x00000000, 0x00000000, 0x00000000, 0x00000000,
            ],
            [
                0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x40000000,
                0x00000000, 0x00000000, 0x00000000, 0x00000000,
            ],
            [
                0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x7fc00000,
                0x00000000, 0x00000000, 0x00000000, 0x00000000,
            ],
        ];
        for row in cases {
            let origin = glam::Vec3::new(
                f32::from_bits(row[0]),
                f32::from_bits(row[1]),
                f32::from_bits(row[2]),
            );
            let direction = glam::Vec3::new(
                f32::from_bits(row[3]),
                f32::from_bits(row[4]),
                f32::from_bits(row[5]),
            );
            let point = glam::Vec3::new(
                f32::from_bits(row[7]),
                f32::from_bits(row[8]),
                f32::from_bits(row[9]),
            );
            let actual =
                terrain_spherecast_time_of_impact(origin, direction, f32::from_bits(row[6]), point);
            assert_eq!(
                actual.to_bits(),
                row[10],
                "original sphere terrain row {row:?}"
            );
        }
    }
}
