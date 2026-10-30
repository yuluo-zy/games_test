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
//!   `try_convex_hull` + `ConvexPolyhedron::from_convex_mesh` → `SharedShape` Arc@+0x20/+0x28、
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
//! 以下为2026-10-17历史登记，不能当作当前未完成清单；2026-10-27已恢复十二参更新与
//! 五实体表、真实几何及Reload验收，见reports/module-recovery-2026-10-23/
//! m01-rapier-producer-recovery-2026-10-27.md。历史项：`update_rapier_colliders`、`on_level_reload`、
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

/// 原1408d7ed0：对heights/positions的zip相邻窗口追加两面；RDX法线、R8三角，均非索引。
/// 保留原不等长度截短、短段不输出与已有Vec前缀，不读取该104B段的其余字段。
pub fn construct_arch_geo(
    segment: &crate::systems::wall::construction::structures::MaterializedWallSegment,
    normals: &mut Vec<[glam::Vec3; 3]>,
    triangles: &mut Vec<utils::geometry::intersect::Triangle>,
) {
    let count = segment.heights.len().min(segment.positions.len());
    for index in 1..count {
        let before = glam::Vec3::from_array(segment.positions[index - 1]);
        let after = glam::Vec3::from_array(segment.positions[index]);
        let heights = [segment.heights[index - 1], segment.heights[index]];
        let mut output = arch_collision_pair::<false>(before, after, heights);
        // 有限输入保持原常规浮点性能；异常结果局部恢复原SSE NaN来源与invalid负位型。
        if output.iter().any(|(triangle, normal)| !normal.is_finite() || triangle.points.iter().any(|p| !p.is_finite())) {
            output = arch_collision_pair::<true>(before, after, heights);
        }
        for (triangle, normal) in output {
            normals.push([normal; 3]);
            triangles.push(triangle);
        }
    }
}

fn arch_collision_pair<const SSE: bool>(before: glam::Vec3, after: glam::Vec3, heights: [f32; 2])
    -> [(utils::geometry::intersect::Triangle, glam::Vec3); 2]
{
    #[inline]
    fn original<const SSE: bool>(a: f32, b: f32, result: f32) -> f32 {
        if !SSE { return result; }
        if a.is_nan() { f32::from_bits(a.to_bits() | 0x0040_0000) }
        else if b.is_nan() { f32::from_bits(b.to_bits() | 0x0040_0000) }
        else if result.is_nan() { f32::from_bits(0xffc0_0000) }
        else { result }
    }
    let add = |a: f32, b: f32| original::<SSE>(a, b, a + b);
    let sub = |a: f32, b: f32| original::<SSE>(a, b, a - b);
    let mul = |a: f32, b: f32| original::<SSE>(a, b, a * b);
    let div = |a: f32, b: f32| original::<SSE>(a, b, a / b);
    let lift = |point: glam::Vec3, height| glam::Vec3::new(add(mul(height, 0.0), point.x), add(height, point.y), add(point.z, mul(height, 0.0)));
    let raised_before = lift(before, heights[0]);
    let raised_after = lift(after, heights[1]);
    [utils::geometry::intersect::Triangle::new(before, after, raised_before),
        utils::geometry::intersect::Triangle::new(after, raised_after, raised_before)].map(|triangle| {
        // 原14123ada0使用(p1-p0)×(p2-p1)，保留SIMD每个乘法的左右来源及Z先加顺序。
        let [p0, p1, p2] = triangle.points;
        let a = glam::Vec3::from_array(std::array::from_fn(|i| sub(p1[i], p0[i])));
        let b = glam::Vec3::from_array(std::array::from_fn(|i| sub(p2[i], p1[i])));
        let normal = glam::Vec3::new(sub(mul(a.y, b.z), mul(b.y, a.z)),
            sub(mul(a.z, b.x), mul(b.z, a.x)), sub(mul(b.y, a.x), mul(b.x, a.y)));
        let squared = add(mul(normal.z, normal.z), add(mul(normal.y, normal.y), mul(normal.x, normal.x)));
        let length = original::<SSE>(squared, 0.0, squared.sqrt());
        let inverse = div(1.0, length);
        (triangle, normal.map(|component| mul(inverse, component)))
    })
}

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

/// 原 GraphMap::add_edge（B 0x14093a9ca..0x14093aa0c）先比较槽，
/// 槽相同时依次比较 collider 索引和 generation，用完整顺序规范化无向边。
impl PartialOrd for SubworldColliderHandle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SubworldColliderHandle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.slot, self.index, self.generation).cmp(&(other.slot, other.index, other.generation))
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

/// `RaycastWorldResult`（O：~0x40B）。判别语义：+0x34 = 0 无UV命中 / 1 有UV命中 / 2 未命中；地形命中时 +0x00
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
    /// +0x34：0 = 无UV命中，1 = 有UV命中，2 = 未命中。
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
        matches!(self.discriminant, 0 | 1)
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
/// `Rapier::raycast` / `Rapier::raycast_simple` 命中的**结果句柄区**。
///
/// fresh（0x1408cdc60 / 0x1408ce180）：命中后先 `ColliderSet::get_mut_internal` 取句柄，
/// 再以 `(index, generation)` 校验注册表条目（`entry.generation | 1 == generation | 1`，
/// 失败 `begin_panic`），随后把**注册表条目的前 24 字节**整块拷进结果 +0x00..+0x18
/// （`in_RCX[0..2] = puVar1[0..2]`，条目 stride 0x28）。因此句柄区不是 rapier 句柄打包，
/// 而是 `ColliderRegistryEntry` 的前 24 字节（kind/meta 区）。
///
/// Resolve the actual physical collider's user_data key through the sole mesh owner.
/// Missing or stale receipts fail explicitly; no source is synthesized from its slot.
#[cfg(feature = "native-physics")]
fn registry_handle_region(world: &RaycastWorld, slot: u8, handle: ColliderHandle) -> [u64; 3] {
    let source = world.registered_mesh_for_collider(slot, handle).source;
    std::array::from_fn(|i| u64::from_le_bytes(source.0[i * 8..i * 8 + 8].try_into().unwrap()))
}

/// Raw receipts must round-trip before lookup: KeyData::from_ffi alone ORs generation with 1.
#[cfg(feature = "native-physics")]
fn strict_mesh_key(raw: u64) -> RaycastMeshKey {
    use slotmap::Key;
    let key = RaycastMeshKey::from(slotmap::KeyData::from_ffi(raw));
    assert_eq!(
        key.data().as_ffi(),
        raw,
        "collider mesh receipt has invalid generation bits"
    );
    key
}

/// 地形命中结果（O：+0x00 首字节 0xb、position@+0x18 = origin + dir*toi、normal=(0,1,0)@+0x24、
/// toi@+0x30、判别 0@+0x34）。
#[cfg(feature = "native-physics")]
fn terrain_result(toi: f32, origin: glam::Vec3, dir: glam::Vec3) -> RaycastWorldResult {
    terrain_result_at_position(toi, origin + dir * toi)
}

/// 原raycast_w_terrain复制地形服务返回的真实point，不能重算丢掉该字段。
#[cfg(feature = "native-physics")]
fn terrain_result_at_position(toi: f32, position: glam::Vec3) -> RaycastWorldResult {
    RaycastWorldResult {
        handle: [u64::from(RAYCAST_WORLD_RESULT_TERRAIN), 0, 0],
        position,
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
    pub removed: std::collections::HashSet<SubworldColliderHandle>,
    /// 新增/变更的 collider 句柄（O：find_mesh 逐句柄 AABB + Qbvh 遍历消费）。
    pub added: std::collections::HashSet<SubworldColliderHandle>,
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
                // 原 141453d98..ded：先抵消未消费的新增，找不到才记录删除。
                if !state.added.remove(&h) { state.removed.insert(h); }
            } else {
                state.added.insert(h);
            }
        }
    }
}

/// `RaycastMeshData`（O：0x78B；布局按 fresh `new_indexed` 0x1408cee40 /
/// `new_unindexed` 0x1408cf7d0 的输出 store 序列重新锚定，2026-10-17）：
///
/// | +0x00 | +0x08 | +0x10 | +0x18 | +0x20/+0x28 | +0x30 | +0x38 | +0x40 | +0x48 | +0x50..+0x68 | +0x70 |
/// |---|---|---|---|---|---|---|---|---|---|---|---|
/// | 顶点 ptr | 顶点 len | 索引 ptr | 索引 len（计数） | shape SharedShape（内部 Arc<dyn Shape>） | 法线 ptr | 法线 len | uv ptr | uv len | 常量区（0/0/0/1.0） | hash |
///
/// 关键修正（fresh）：
/// - 缓冲区是 **ptr+len**（无 cap 槽）；原输入 Vec 在 store 前被 shrink 到 cap==len；
/// - `+0x10` 是**索引缓冲 ptr**（旧实现误作顶点 cap）；`+0x18` = 索引数；
/// - `+0x40/+0x48` 是 uv ptr/len（旧实现把 uv 放在 +0x48/+0x50）；
/// - `+0x50/+0x58/+0x60 = 0`、`+0x68 = 0x3f80000000000000`（常量 0x142aa3ec0/0x142aa3ed0
///   已 read_memory 核对）。
///
/// Primitive variants reuse the same layout: +0x50..+0x6f is mesh-local rigid pose,
/// not dimensions/hash scratch; the geometry fingerprint remains at +0x70.
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
    /// +0x20/+0x28：原 SharedShape（内部 Arc ptr + dyn Shape metadata）。
    pub shape: Option<SharedShape>,
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
            scale: 0x3f80000000000000,
            hash: 0,
        }
    }
}

/// 原trap内联Clone：四个boxed缓冲独立复制，内部SharedShape只递增同一Arc。
#[cfg(feature = "native-physics")]
impl Clone for RaycastMeshData {
    fn clone(&self) -> Self {
        // 先由Box持有全部副本；后续复制失败时，前面的分配仍能正常析构。
        // ptr0与ptr4分别表示None与Some(empty)，不能只按空slice统一处理。
        let vertices: Option<Box<[glam::Vec3]>> =
            (self.vertices_ptr != 0).then(|| self.vertices().into());
        let indices: Option<Box<[[u32; 3]]>> =
            (self.indices_ptr != 0).then(|| self.indices().into());
        let normals: Option<Box<[glam::Vec3]>> =
            (self.normals_ptr != 0).then(|| self.normals().into());
        let uvs: Option<Box<[glam::Vec2]>> = (self.uvs_ptr != 0).then(|| self.uvs().into());
        let shape = self.shape.clone();
        Self {
            vertices_ptr: into_owned_mesh_buffer_ptr(vertices),
            vertices_len: self.vertices_len,
            indices_ptr: into_owned_mesh_buffer_ptr(indices),
            count: self.count,
            shape,
            normals_ptr: into_owned_mesh_buffer_ptr(normals),
            normals_len: self.normals_len,
            uvs_ptr: into_owned_mesh_buffer_ptr(uvs),
            uvs_len: self.uvs_len,
            _pad0: self._pad0,
            _pad1: self._pad1,
            _pad2: self._pad2,
            scale: self.scale,
            hash: self.hash,
        }
    }
}

/// 把已成功复制的Box交给现有Drop；空Box保持元素对齐悬垂哨兵4。
#[cfg(feature = "native-physics")]
fn into_owned_mesh_buffer_ptr<T>(buffer: Option<Box<[T]>>) -> u64 {
    buffer.map_or(0, |buffer| Box::into_raw(buffer) as *mut T as u64)
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

    /// 原共享形状借用（+0x20/+0x28）；Deref 复用锁定 parry 的 SharedShape。
    #[cfg(feature = "native-physics")]
    pub fn shape_ref(&self) -> Option<&dyn Shape> {
        self.shape.as_deref()
    }

    /// B data+0x50..0x6f is a rigid pose, including primitive constructors.
    /// Decode fields explicitly; the reconstructed IsoTransform has a different size.
    pub fn local_transform(&self) -> utils::iso_transform::IsoTransform {
        utils::iso_transform::IsoTransform::from_translation_rotation(
            glam::Vec3::new(
                f32::from_bits(self._pad0 as u32),
                f32::from_bits((self._pad0 >> 32) as u32),
                f32::from_bits(self._pad1 as u32),
            ),
            glam::Quat::from_xyzw(
                f32::from_bits(self._pad2 as u32),
                f32::from_bits((self._pad2 >> 32) as u32),
                f32::from_bits(self.scale as u32),
                f32::from_bits((self.scale >> 32) as u32),
            ),
        )
    }

    #[cfg(feature = "native-physics")]
    fn set_translation_pose(&mut self, translation: glam::Vec3) {
        self._pad0 =
            u64::from(translation.x.to_bits()) | (u64::from(translation.y.to_bits()) << 32);
        // B from_translation@140c91b30 SHUFPS 0x58 produces [x,y,z,z].
        self._pad1 =
            u64::from(translation.z.to_bits()) | (u64::from(translation.z.to_bits()) << 32);
        self._pad2 = 0;
        self.scale = 0x3f80000000000000;
    }

    /// `RaycastMeshData::new_convex`（0x1408cec10，O）：`calculate_hash<&[UVec3]>` 存 hash、
    /// `try_convex_hull` + `ConvexPolyhedron::from_convex_mesh` → `SharedShape`；
    /// None 写判别 0（O：`*out = 0`）。输出：tag=4@+0x00、kind=4@+0x10、shape@+0x20/+0x28、
    /// scale@+0x68=1.0、hash@+0x70（其余写 0，disasm 0x1408cec10 核对）。
    #[cfg(feature = "native-physics")]
    pub fn new_convex(points: &[glam::Vec3]) -> Option<Self> {
        // B 1408cec10 hashes the float words through its UVec3 bit view;
        // 1407dec90 passes eight contiguous Vec3 values directly. Numeric
        // u32->f32 conversion would turn negative/fractional world coordinates
        // into huge positive values and corrupt the convex hull.
        let point_bits: &[glam::UVec3] = bytemuck::cast_slice(points);
        let hash = crate::render::utils::calculate_hash(&point_bits);
        let points_f: Vec<Point<f32>> = points
            .iter()
            .map(|p| Point::new(p.x, p.y, p.z))
            .collect();
        // O：`parry3d::transformation::convex_hull3::convex_hull::try_convex_hull`。
        let (hull_points, indices) =
            rapier3d::parry::transformation::try_convex_hull(&points_f).ok()?;
        // O：`ConvexPolyhedron::from_convex_mesh`。
        let convex = ConvexPolyhedron::from_convex_mesh(hull_points, &indices)?;
        let shape = SharedShape::new(convex);
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

    /// Compatibility entry: the second UVec3 carries translation float bits.
    /// B @1408cfff0 hashes the raw vectors and passes the second to from_translation.
    #[cfg(feature = "native-physics")]
    pub fn new_cuboid(dimensions: glam::Vec3, translation_bits: glam::UVec3) -> Self {
        Self::new_cuboid_at(
            dimensions,
            glam::Vec3::new(
                f32::from_bits(translation_bits.x),
                f32::from_bits(translation_bits.y),
                f32::from_bits(translation_bits.z),
            ),
        )
    }

    /// Full dimensions and mesh-local translation, B @1408cfff0 (290 bytes).
    /// The constructor itself performs the one original dimensions*0.5 conversion.
    #[cfg(feature = "native-physics")]
    pub fn new_cuboid_at(dimensions: glam::Vec3, translation: glam::Vec3) -> Self {
        let hash = crate::render::utils::calculate_hash(&(
            [
                dimensions.x.to_bits(),
                dimensions.y.to_bits(),
                dimensions.z.to_bits(),
            ],
            glam::UVec3::new(
                translation.x.to_bits(),
                translation.y.to_bits(),
                translation.z.to_bits(),
            ),
        ));
        let half = dimensions * 0.5;
        let shape = SharedShape::cuboid(half.x, half.y, half.z);
        let mut mesh = Self {
            vertices_ptr: 4,
            vertices_len: 0,
            indices_ptr: 4,
            count: 0,
            shape: Some(shape),
            normals_ptr: 0,
            normals_len: 0,
            uvs_ptr: 0,
            uvs_len: 0,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
            scale: 0,
            hash,
        };
        mesh.set_translation_pose(translation);
        mesh
    }

    /// B @1408d0120 uses the raw second vector as translation, and hashes its bits.
    #[cfg(feature = "native-physics")]
    pub fn new_sphere(radius: f32, translation_bits: glam::UVec3) -> Self {
        let hash = crate::render::utils::calculate_hash(&(radius.to_bits(), translation_bits));
        let shape = SharedShape::ball(radius);
        let mut mesh = Self {
            vertices_ptr: 4,
            vertices_len: 0,
            indices_ptr: 4,
            count: 0,
            shape: Some(shape),
            normals_ptr: 0,
            normals_len: 0,
            uvs_ptr: 0,
            uvs_len: 0,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
            scale: 0,
            hash,
        };
        mesh.set_translation_pose(glam::Vec3::new(
            f32::from_bits(translation_bits.x),
            f32::from_bits(translation_bits.y),
            f32::from_bits(translation_bits.z),
        ));
        mesh
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
            .map(|mut child| {
                // B new_compound's map closure reads child+0x50/+0x60.
                let local = child.local_transform();
                let shape = child
                    .shape
                    .take()
                    .expect("new_compound: child has no shape");
                (
                    collider_isometry(local.translation, local.rotation),
                    shape,
                )
            })
            .collect();
        let shape = SharedShape::compound(parts);
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
        let shape = SharedShape::trimesh(points, indices.to_vec());
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
        let shape = SharedShape::trimesh(points, indices.clone());
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

/// The existing 24-byte source projection is the canonical source receipt.
/// B source tags/payload are copied to ray results and compared selectively.
/// Legacy named constants retain their existing tags; decorator factories carry identity.
pub type RaycastGeoSource = crate::systems::collision::query_policy::RaycastGeoSourceProjection;

#[allow(non_upper_case_globals)]
impl RaycastGeoSource {
    pub const SOURCE_TO_SLOT: [u8; 17] = [0, 9, 1, 2, 8, 14, 3, 7, 6, 10, 4, 5, 11, 12, 13, 15, 16];
    const fn tag_only(tag: u8) -> Self {
        let mut bytes = [0; 24];
        bytes[0] = tag;
        Self(bytes)
    }
    pub const Slot0: Self = Self::tag_only(0);
    pub const Slot1: Self = Self::tag_only(1);
    pub const Slot2: Self = Self::tag_only(2);
    pub const Slot3: Self = Self::tag_only(3);
    pub const Slot4: Self = Self::tag_only(4);
    pub const Terrain: Self = Self::tag_only(5);
    pub const Slot6: Self = Self::tag_only(6);
    pub const Slot7: Self = Self::tag_only(7);
    pub const PlatformStair: Self = Self::Slot7;
    pub const Slot8: Self = Self::tag_only(8);
    pub const Slot9: Self = Self::tag_only(9);
    pub const Slot10: Self = Self::tag_only(10);
    pub const Slot11: Self = Self::tag_only(11);
    pub const Slot12: Self = Self::tag_only(12);
    pub const Slot13: Self = Self::tag_only(13);
    pub const Slot14: Self = Self::tag_only(14);
    pub const Slot15: Self = Self::tag_only(15);
    pub const Slot16: Self = Self::tag_only(16);

    pub fn subworld(self) -> u8 {
        Self::SOURCE_TO_SLOT[self.tag() as usize]
    }

    pub fn decorator(
        id: crate::resources::walls::decorator::DecoratorId,
        storage: crate::resources::walls::decorator_storage::DecoratorStorageAddr,
    ) -> Self {
        Self::decorator_source(3, id, storage)
    }

    /// 原Roof来源tag2：墙身份@8，flat/gable为两个独立字节，非实体索引。
    pub fn roof(wall: crate::resources::walls::WallId, flat: bool, gable: bool) -> Self {
        let mut source = Self::tag_only(2);
        source.0[1] = u8::from(flat);
        source.0[2] = u8::from(gable);
        source.0[8..16].copy_from_slice(&wall.0.to_le_bytes());
        source
    }

    /// 原自动杂物来源tag12的载荷是完整实体身份，与Roof的WallId载荷不同。
    pub fn auto_clutter(entity: bevy_ecs::entity::Entity) -> Self {
        let mut source = Self::tag_only(12);
        source.0[8..16].copy_from_slice(&entity.to_bits().to_le_bytes());
        source
    }

    /// 原墙、地板和拱/过梁来源都携带WallId，三个实体表保持独立。
    fn wall_owned(tag: u8, wall: crate::resources::walls::WallId) -> Self {
        let mut source = Self::tag_only(tag);
        source.0[8..16].copy_from_slice(&wall.0.to_le_bytes());
        source
    }

    pub fn decorator_interaction(
        id: crate::resources::walls::decorator::DecoratorId,
        storage: crate::resources::walls::decorator_storage::DecoratorStorageAddr,
        flags: u8,
    ) -> Self {
        let tag = flags | 4;
        assert!(
            matches!(tag, 4 | 5),
            "invalid decorator interaction source flags"
        );
        Self::decorator_source(tag, id, storage)
    }

    fn decorator_source(
        tag: u8,
        id: crate::resources::walls::decorator::DecoratorId,
        storage: crate::resources::walls::decorator_storage::DecoratorStorageAddr,
    ) -> Self {
        use crate::resources::walls::decorator_storage::DecoratorStorageAddr;
        let (disc, payload) = match storage {
            DecoratorStorageAddr::Wall(wall) => (0u64, wall.0),
            DecoratorStorageAddr::Terrain => (1, 0),
            DecoratorStorageAddr::Stair => (2, 0),
        };
        let mut source = Self::tag_only(tag);
        source.0[4..8].copy_from_slice(&id.0.to_le_bytes());
        source.0[8..16].copy_from_slice(&disc.to_le_bytes());
        source.0[16..24].copy_from_slice(&payload.to_le_bytes());
        source
    }

    /// Compatibility method retains the original selected-field comparison.
    pub fn eq(self, other: Self) -> bool {
        self.same_native_source(other)
    }

    /// B impl18::eq selected fields; raw projection padding is not equality.
    fn same_native_source(self, other: Self) -> bool {
        if self.tag() != other.tag() {
            return false;
        }
        match self.tag() {
            0 | 1 | 6 | 7 | 8 | 9 | 12 => self.0[8..16] == other.0[8..16],
            2 => self.0[8..16] == other.0[8..16] && self.0[1..3] == other.0[1..3],
            3 | 4 | 5 => {
                self.0[4..8] == other.0[4..8]
                    && self.0[8..16] == other.0[8..16]
                    && (self.0[8..16] != [0; 8] || self.0[16..24] == other.0[16..24])
            }
            13 | 14 => self.0[4..12] == other.0[4..12],
            15 | 16 => self.0[4..16] == other.0[4..16],
            _ => true,
        }
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
/// - 命中点先减平移并逆旋转到网格局部，再经 `barycentric(local, v0, v1, v2)`；
/// - **法线经旋转四元数（O：in_R9）旋转** `n' = q * n`；uv 旋转不变；
/// - 法线缓冲为空 → 用命中法线（O：`in_R8+0x0c` 的 fallback Vec3，已是世界空间，不旋转）；
/// - 有 uv → 判别 1，extra = uv；无 uv → 判别 0，extra = {bc.z, bc.z}。
/// 当前公开旧签名仅供零平移兼容；真实带姿态查询使用下方原完整内核。
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
    rapier_hit_to_raycast_world_result_at_pose(
        mesh,
        origin,
        dir,
        toi,
        hit_normal,
        tri_index,
        [
            u64::from(handle.index) | (u64::from(handle.generation) << 32),
            u64::from(handle.slot),
            0,
        ],
        glam::Vec3::ZERO,
        rotation,
    )
}

/// 原1408ce4a0的姿态转换。此处保留原逐lane运算顺序；先还原局部命中坐标，
/// 才能对原网格顶点插值；旧公开签名通过零平移wrapper保持兼容。
#[cfg(feature = "native-physics")]
#[allow(clippy::too_many_arguments)]
pub(crate) fn rapier_hit_to_raycast_world_result_at_pose(
    mesh: &crate::systems::collision::world::RaycastMeshData,
    origin: glam::Vec3,
    direction: glam::Vec3,
    toi: f32,
    normal: glam::Vec3,
    triangle: Option<u32>,
    source: [u64; 3],
    translation: glam::Vec3,
    q: glam::Quat,
) -> crate::systems::collision::world::RaycastWorldResult {
    use crate::systems::collision::world::RaycastWorldResult;
    let position = direction * toi + origin;
    let mut result = RaycastWorldResult {
        handle: source,
        position,
        normal,
        toi,
        discriminant: 0,
        extra: [0.0; 2],
    };
    let indices = mesh.indices();
    if indices.is_empty() {
        return result;
    }
    let Some(mut triangle) = triangle.map(|x| x as usize) else {
        result.discriminant = 2;
        return result;
    };
    if triangle >= indices.len() {
        triangle -= indices.len();
    }
    let [a, b, c] = indices[triangle];
    let d = position - translation;
    let tx = (d.y * q.z - d.z * q.y) * 2.0;
    let ty = (d.z * q.x - d.x * q.z) * 2.0;
    let tz = (q.y * d.x - d.y * q.x) * 2.0;
    let local = glam::Vec3::new(
        (tx * q.w + (ty * q.z - tz * q.y)) + d.x,
        (ty * q.w + (tz * q.x - tx * q.z)) + d.y,
        ((q.y * tx - ty * q.x) + q.w * tz) + d.z,
    );
    let vertices = mesh.vertices();
    let bc = ::utils::geometry::barycentric::barycentric(
        local,
        vertices[a as usize],
        vertices[b as usize],
        vertices[c as usize],
    );
    let normals = mesh.normals();
    // 原按Option指针判别；Some(empty)须继续原索引bounds检查，不能当作None fallback。
    if mesh.normals_ptr != 0 {
        let (na, nb, nc) = (
            normals[a as usize],
            normals[b as usize],
            normals[c as usize],
        );
        let n = glam::Vec3::new(
            nc.x * bc.z + (nb.x * bc.y + bc.x * na.x),
            nc.y * bc.z + (nb.y * bc.y + bc.x * na.y),
            nc.z * bc.z + (na.z * bc.x + bc.y * nb.z),
        );
        let tx = (n.z * q.y - n.y * q.z) * 2.0;
        let ty = (n.x * q.z - n.z * q.x) * 2.0;
        let tz = (n.y * q.x - q.y * n.x) * 2.0;
        result.normal = glam::Vec3::new(
            (q.w * tx + (tz * q.y - ty * q.z)) + n.x,
            (q.w * ty + (tx * q.z - tz * q.x)) + n.y,
            ((ty * q.x - q.y * tx) + q.w * tz) + n.z,
        );
    }
    let uvs = mesh.uvs();
    if mesh.uvs_ptr == 0 {
        result.extra = [bc.z, bc.z];
    } else {
        let (a, b, c) = (uvs[a as usize], uvs[b as usize], uvs[c as usize]);
        result.extra = [
            c.x * bc.z + (bc.y * b.x + bc.x * a.x),
            c.y * bc.z + (bc.y * b.y + bc.x * a.y),
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
    /// 每槽一个脏瓦片发布器（O：槽内联 `Publisher<DirtyTiles>` @slot+0x150 = world+0x20+0x390*slot+0x150
    /// = `world + slot*0x390 + 0x170`）。机器码 `RaycastWorld::subscribe_to_changes` 0x1408cdc20 为
    /// `movzx eax,dl; imul rax,rax,0x390; add rcx,rax; add rcx,0x170; jmp Publisher::subscribe<DirtyTiles>`，
    /// 即按槽号取发布器；`flush_colliders` 0x1408cd5c0 逐槽 take-and-clear 后向**该槽**发布器
    /// `publish<DirtyTiles,DirtyTiles>`。（旧实现用单个 world 级发布器，已纠正。）
    dirty_tiles_publishers: [Publisher<crate::utils::dirty_tiles::DirtyTiles>; crate::systems::collision::SUBWORLD_SLOT_COUNT as usize],
    /// The existing SlotMap is the sole mesh receipt allocator/owner.
    /// Collider.user_data carries its key; the colliders Vec is only its public projection.
    #[cfg(feature = "native-physics")]
    mesh_sources: MeshInstanceSources,
    /// 原World+3df8：屋顶实体完整generation→实际物理实例，供指定屋顶查询与卸载共用。
    #[cfg(feature = "native-physics")]
    roof_colliders: std::collections::HashMap<bevy_ecs::entity::Entity, ColliderMeshInstance>,
    /// 原+3dc8、+3e28、+3e58：主墙面、地板、拱/过梁，各自保留真实实体generation。
    #[cfg(feature = "native-physics")]
    wall_colliders: std::collections::HashMap<bevy_ecs::entity::Entity, ColliderMeshInstance>,
    #[cfg(feature = "native-physics")]
    floor_colliders: std::collections::HashMap<bevy_ecs::entity::Entity, ColliderMeshInstance>,
    #[cfg(feature = "native-physics")]
    arch_colliders: std::collections::HashMap<bevy_ecs::entity::Entity, ColliderMeshInstance>,
    /// 原World+3e88：可转换自动杂物的独立实体表，与屋顶共享同一物理receipt分配器。
    #[cfg(feature = "native-physics")]
    auto_clutter_colliders: std::collections::HashMap<bevy_ecs::entity::Entity, ColliderMeshInstance>,
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
            dirty_tiles_publishers: std::array::from_fn(|_| Publisher::new()),
            #[cfg(feature = "native-physics")]
            mesh_sources: MeshInstanceSources::new(),
            #[cfg(feature = "native-physics")]
            roof_colliders: std::collections::HashMap::new(),
            #[cfg(feature = "native-physics")]
            wall_colliders: std::collections::HashMap::new(),
            #[cfg(feature = "native-physics")]
            floor_colliders: std::collections::HashMap::new(),
            #[cfg(feature = "native-physics")]
            arch_colliders: std::collections::HashMap::new(),
            #[cfg(feature = "native-physics")]
            auto_clutter_colliders: std::collections::HashMap::new(),
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
        for (slot, publisher) in self.slots.iter_mut().zip(self.dirty_tiles_publishers.iter_mut()) {
            if slot.dirty {
                let tiles = slot.dirty_tiles.take_and_clear();
                // O：`publish<DirtyTiles,DirtyTiles>`（`take_and_clear` 的快照即 delta，
                // `DirtyTiles` 的 `StateDelta::apply_to` 为逐字 OR）。
                let _ = publisher.publish(tiles);
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

    /// Existing projection-only compatibility helper. It does not register an owned mesh.
    /// Canonical physics insertion obtains its independent key from MeshInstanceSources.
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

    /// B @1408cdbd0 returns the mesh after exact full-generation validation.
    pub fn get_collider_mesh(&self, index: u32, generation: u32) -> &RaycastMesh {
        #[cfg(feature = "native-physics")]
        {
            let raw = (u64::from(generation) << 32) | u64::from(index);
            self.mesh_sources
                .get(strict_mesh_key(raw))
                .expect("get_collider_mesh: collider mesh receipt missing or stale")
        }
        #[cfg(not(feature = "native-physics"))]
        {
            let _ = (index, generation);
            panic!("get_collider_mesh requires native-physics")
        }
    }

    #[cfg(feature = "native-physics")]
    pub(crate) fn registered_mesh_for_collider(
        &self,
        slot: u8,
        handle: ColliderHandle,
    ) -> &RaycastMesh {
        let collider = self.slots[slot as usize]
            .colliders
            .get(handle)
            .expect("collider source: physical collider missing");
        let key = strict_mesh_key(collider.user_data as u64);
        self.mesh_sources
            .get(key)
            .expect("collider source: mesh receipt missing or stale")
    }

    /// 原完整射线predicate1408cab80及winner1408cdeea均先将raw generation按位OR1。
    /// 这是该原查询的读取规则；不改变其他owner显式generation的严格接口。
    #[cfg(feature = "native-physics")]
    fn raycast_mesh_for_collider(&self, slot: u8, handle: ColliderHandle) -> (&RaycastMesh, &Collider) {
        let collider = self.slots[slot as usize].colliders.get(handle)
            .expect("raycast: physical collider missing");
        let raw = collider.user_data as u64;
        let mesh = self.get_collider_mesh(raw as u32, (raw >> 32) as u32 | 1);
        (mesh, collider)
    }

    /// 原1408cab80/1408ca9a0共用注册校验，但完整/简单射线的排墙位图不同。
    #[cfg(feature = "native-physics")]
    fn ray_collider_is_allowed(
        &self,
        slot: u8,
        handle: ColliderHandle,
        excluded_wall: Option<crate::resources::walls::WallId>,
        wall_kinds: u32,
    ) -> bool {
        // 即使没有排墙条件，也先验证真实collider的完整mesh receipt generation。
        let source = self.registered_mesh_for_collider(slot, handle).source;
        Self::source_passes_wall_exclusion(source, excluded_wall, wall_kinds)
    }

    #[cfg(feature = "native-physics")]
    fn source_passes_wall_exclusion(source: RaycastGeoSource,
        excluded_wall: Option<crate::resources::walls::WallId>, wall_kinds: u32) -> bool {
        match excluded_wall {
            Some(wall) if source.0[0] <= 8 && wall_kinds & (1 << source.0[0]) != 0 => {
                u64::from_le_bytes(source.0[8..16].try_into().unwrap()) != wall.0
            }
            _ => true,
        }
    }

    /// `RaycastWorld::subscribe_to_changes`（0x1408cdc20，O）：`(&mut self, slot: u8)` →
    /// `Publisher::subscribe<DirtyTiles>` on `self.slot[slot]` 的内联发布器
    /// （`slot * 0x390 + 0x170`，不做边界检查；本实现越界 panic）。
    pub fn subscribe_to_changes(
        &mut self,
        slot: u8,
    ) -> tabloid::Subscriber<crate::utils::dirty_tiles::DirtyTiles> {
        self.dirty_tiles_publishers[slot as usize].subscribe()
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

    /// 指定屋顶只查原实体表，不以同位置或同槽的其它碰撞体代替。
    #[cfg(feature = "native-physics")]
    pub fn roof_collider_instance(&self, entity: bevy_ecs::entity::Entity) -> Option<&ColliderMeshInstance> {
        self.roof_colliders.get(&entity)
    }

    /// Inspect the real wall collider owner for an existing wall entity.
    #[cfg(feature = "native-physics")]
    pub fn wall_collider_instance(&self, entity: bevy_ecs::entity::Entity) -> Option<&ColliderMeshInstance> {
        self.wall_colliders.get(&entity)
    }

    /// 原get_rapier_collider的只读借用投影；调用者负责保留missing的原失败先后。
    #[cfg(feature = "native-physics")]
    pub fn get_rapier_collider_ref(&self, slot: u8, handle: rapier3d::geometry::ColliderHandle)
        -> Option<&rapier3d::geometry::Collider> {
        self.slots[slot as usize].colliders.get(handle)
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

    /// 原Raycasting显式距离上限。方向保持原长度，TOI传入QBVH剪枝，
    /// 不做无限射线之后再丢弃远处命中的替代。
    #[cfg(feature = "native-physics")]
    pub fn raycast_with_max_toi(&self, origin: glam::Vec3, dir: glam::Vec3,
        max_toi: f32, mask: u32) -> RaycastWorldResult {
        Rapier::raycast_with_max_toi(self, origin, dir, max_toi, mask)
    }

    /// `RaycastWorld::raycast`（非 native-physics 壳：查询不落地）。
    #[cfg(not(feature = "native-physics"))]
    pub fn raycast(&self, _origin: glam::Vec3, _dir: glam::Vec3, _mask: u32) -> RaycastWorldResult {
        RaycastWorldResult::miss()
    }

    #[cfg(not(feature = "native-physics"))]
    pub fn raycast_with_max_toi(&self, _origin: glam::Vec3, _dir: glam::Vec3,
        _max_toi: f32, _mask: u32) -> RaycastWorldResult {
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
        self.raycast_w_terrain_with_max_toi(terrain, origin, dir, f32::MAX, mask)
    }

    /// B1408cc2e0有界转发；地形同样受调用方上限约束，之后仅收紧距离。
    #[cfg(feature = "native-physics")]
    pub fn raycast_w_terrain_with_max_toi(
        &self, terrain: &TerrainHeightsData<TerrainHeightSsbo>,
        origin: glam::Vec3, dir: glam::Vec3, max_toi: f32, mask: u32,
    ) -> RaycastWorldResult {
        let mut result = Rapier::raycast_with_max_toi(self, origin, dir, max_toi, mask);
        if mask & 0x20 != 0 {
            let ray = TerrainRay {
                origin,
                direction: dir,
            };
            // B 0x1408cd508..0x1408cd51f passes the Rapier hit's TOI to terrain.
            let maximum_toi = if result.is_hit() {
                result.toi
            } else {
                max_toi
            };
            if let Some(terrain_hit) = terrain.intersect_ray(ray, maximum_toi) {
                result = terrain_result_at_position(terrain_hit.t, terrain_hit.position);
            }
        }
        result
    }

    /// Current borrowing adapter over the recovered B Raycasting terrain sweep.
    /// RaycastWorld remains the sole physics owner; the owned wrapper delegates
    /// here so camera motion and existing consumers share the same algorithm.
    #[cfg(feature = "native-physics")]
    pub fn spherecast_w_terrain(
        &self,
        terrain: &TerrainHeightsData<TerrainHeightSsbo>,
        origin: glam::Vec3,
        dir: glam::Vec3,
        radius: f32,
        options: rapier3d::parry::query::ShapeCastOptions,
        mask: u32,
    ) -> RaycastWorldResult {
        let mut result = self.spherecast_with_options(origin, dir, radius, options, mask);
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

    /// `mesh_intersect_collider_diff_publisher`（0x1408cc670）：返回 Publisher 的地址（O：
    /// `lea rax,[rcx+0x3cb0]; ret`，即 &self+0x3cb0）。机器码不区分共享/独占借用；
    /// `Publisher::subscribe` 需要 `&mut`，所以这里给出独占借用。
    pub fn mesh_intersect_collider_diff_publisher(&mut self) -> &mut Publisher<ColliderDiff> {
        &mut self.publisher
    }

    /// Unbounded compatibility form of B `RaycastWorld::intersects_any`.
    /// The original caller-provided ray bound is exposed by the method below.
    #[cfg(feature = "native-physics")]
    pub fn intersects_any(&self, origin: glam::Vec3, dir: glam::Vec3, mask: u32) -> bool {
        Rapier::intersects_any(self, origin, dir, f32::MAX, mask)
    }

    /// B @0x1408cd5b0 -> 0x1408cdf80: preserve the caller's parametric ray bound.
    /// The direction is passed unchanged; physical length is `|dir| * max_toi`.
    #[cfg(feature = "native-physics")]
    pub fn intersects_any_with_max_toi(
        &self,
        origin: glam::Vec3,
        dir: glam::Vec3,
        max_toi: f32,
        mask: u32,
    ) -> bool {
        Rapier::intersects_any(self, origin, dir, max_toi, mask)
    }

    /// 原 `Raycasting::intersects_any`（B 0x1408cc320）的借用接口。
    /// 地形和已有Rapier owner共用调用方上限；方向不归一化。
    #[cfg(feature = "native-physics")]
    pub fn intersects_any_w_terrain(
        &self,
        terrain: &TerrainHeightsData<TerrainHeightSsbo>,
        origin: glam::Vec3,
        dir: glam::Vec3,
        max_toi: f32,
        mask: u32,
    ) -> bool {
        if mask & 0x20 != 0 {
            let ray = TerrainRay {
                origin,
                direction: dir,
            };
            if terrain.intersect_ray(ray, max_toi).is_some() {
                return true;
            }
        }
        self.intersects_any_with_max_toi(origin, dir, max_toi, mask)
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
        Rapier::raycast_simple(self, origin, dir, f32::MAX, mask, None)
    }

    /// B `raycast_simple` accepts an explicit parametric TOI bound. The actual
    /// Snap caller passes 1.0; direction and bound retain their original scale.
    #[cfg(feature = "native-physics")]
    pub fn raycast_simple_with_max_toi(
        &self,
        origin: glam::Vec3,
        dir: glam::Vec3,
        max_toi: f32,
        mask: u32,
    ) -> RaycastSimpleResult {
        Rapier::raycast_simple(self, origin, dir, max_toi, mask, None)
    }

    /// 原tips三次有界simple查询：保留原方向长度、TOI与指定墙排除，不另建射线实现。
    #[cfg(feature = "native-physics")]
    pub fn raycast_simple_with_max_toi_excluding_wall_id(
        &self, origin: glam::Vec3, dir: glam::Vec3, max_toi: f32, mask: u32,
        wall: crate::resources::walls::WallId,
    ) -> RaycastSimpleResult {
        Rapier::raycast_simple(self, origin, dir, max_toi, mask, Some(wall))
    }

    #[cfg(not(feature = "native-physics"))]
    pub fn raycast_simple_with_max_toi_excluding_wall_id(
        &self, _origin: glam::Vec3, _dir: glam::Vec3, _max_toi: f32, _mask: u32,
        _wall: crate::resources::walls::WallId,
    ) -> RaycastSimpleResult {
        panic!("有界排墙simple射线需要native-physics")
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

    /// 原0x1408ccc50把有效Option<WallId>转交0x1408ce180，区别于ccc20的None。
    /// 旧SubworldColliderHandle入参混淆了墙身份与碰撞句柄；统一复用实际墙排除查询。
    pub fn raycast_simple_exclude_wall_id(
        &self,
        origin: glam::Vec3,
        dir: glam::Vec3,
        mask: u32,
        excluded: crate::resources::walls::WallId,
    ) -> RaycastSimpleResult {
        self.raycast_simple_excluding_wall_id(origin, dir, mask, excluded)
    }

    /// 原 B 1408ce180 的真实 Option<WallId> 排除面；移动建筑使用 mask 0x202。
    /// 复用原十七槽查询，只排除 source kind 0/1/2/8 中同一墙的几何。
    #[cfg(feature = "native-physics")]
    pub fn raycast_simple_excluding_wall_id(
        &self,
        origin: glam::Vec3,
        dir: glam::Vec3,
        mask: u32,
        wall: crate::resources::walls::WallId,
    ) -> RaycastSimpleResult {
        Rapier::raycast_simple(self, origin, dir, f32::MAX, mask, Some(wall))
    }

    /// B 0x1408ca9a0: exclude source tags 0/1/2/8 whose wall id at +8
    /// matches the selected wall. Filtering occurs inside each Rapier query.
    #[cfg(feature = "native-physics")]
    pub fn raycast_simple_excluding_wall(
        &self, origin: glam::Vec3, dir: glam::Vec3, mask: u32,
        excluded: crate::resources::walls::WallId,
    ) -> RaycastSimpleResult {
        Rapier::raycast_simple_excluding_wall(self, origin, dir, f32::MAX, mask, excluded)
    }

    /// `RaycastWorld::raycast_simple_exclude_wall_id`（非 native-physics 壳）。
    #[cfg(not(feature = "native-physics"))]
    pub fn raycast_simple_excluding_wall_id(
        &self,
        _origin: glam::Vec3,
        _dir: glam::Vec3,
        _mask: u32,
        _wall: crate::resources::walls::WallId,
    ) -> RaycastSimpleResult {
        panic!("建筑移动的屋顶吸附需要 native-physics 碰撞查询")
    }

    #[cfg(not(feature = "native-physics"))]
    pub fn raycast_simple_excluding_wall(
        &self, _origin: glam::Vec3, _dir: glam::Vec3, _mask: u32,
        _excluded: crate::resources::walls::WallId,
    ) -> RaycastSimpleResult { RaycastSimpleResult::miss() }

    /// `RaycastWorld::legacy_raycast_w_params`（0x1408cc770）：转发 `Rapier::raycast`；
    /// 未命中把判别 2 写 +0x30（toi 槽，legacy 特例，同 `legacy_raycast`）。
    /// 原1408cc770：基础mask为0xf3，再OR调用者的额外位；不能把Option判别当mask。
    /// 未命中保留legacy整数判别2的位模式，命中使用安全拥有型结果。
    #[cfg(feature = "native-physics")]
    pub fn legacy_raycast_w_params(
        &self,
        origin: glam::Vec3,
        dir: glam::Vec3,
        additional_mask: u32,
    ) -> RaycastWorldResult {
        let mut result = Rapier::raycast(self, origin, dir, 0xf3 | additional_mask);
        if result.is_miss() {
            result.toi = f32::from_bits(RAYCAST_WORLD_RESULT_MISS);
        }
        result
    }

    /// 原旗帜140800844：Some(当前墙)，额外mask0，射线方向(+0,-1,-0)。
    /// 仍调用完整Rapier射线和法线路径，不能换成simple查询后拼装结果。
    #[cfg(feature = "native-physics")]
    pub fn legacy_raycast_w_params_excluding_wall_id(
        &self,
        origin: glam::Vec3,
        dir: glam::Vec3,
        additional_mask: u32,
        wall: crate::resources::walls::WallId,
    ) -> RaycastWorldResult {
        let mut result = Rapier::raycast_with_params(
            self, origin, dir, f32::MAX, 0xf3 | additional_mask, Some(wall),
        );
        if result.is_miss() { result.toi = f32::from_bits(RAYCAST_WORLD_RESULT_MISS); }
        result
    }

    #[cfg(not(feature = "native-physics"))]
    pub fn legacy_raycast_w_params_excluding_wall_id(
        &self,
        _origin: glam::Vec3,
        _dir: glam::Vec3,
        _additional_mask: u32,
        _wall: crate::resources::walls::WallId,
    ) -> RaycastWorldResult {
        panic!("旗帜排除当前墙的完整射线需要 native-physics 碰撞查询")
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
                        handle: registry_handle_region(self, slot_index as u8, handle),
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

/// 原版系统 `country_core::systems::collision::world::flush_rapier_colliders`（0x1408d8360，757B）。
///
/// SystemParam 面：`ResMut<RaycastWorld>`（单参数：`*param_1` 为资源值、`param_1[2]` 变更 tick
/// 槽、`+0x1c` this_run；`deref_mut` 写 tick）。体：puffin 作用域
/// `country_core::systems::collision::world::flush_rapier_colliders`（仅 scopes 开启时，非业务）+
/// `RaycastWorld::flush_colliders()`。
pub fn flush_rapier_colliders(mut world: bevy_ecs::prelude::ResMut<RaycastWorld>) {
    world.flush_colliders();
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

    /// The query/publication handle is the distinct per-slot Rapier handle.
    pub fn handle(&self) -> SubworldColliderHandle {
        let (index, generation) = self.collider.into_raw_parts();
        SubworldColliderHandle {
            index,
            generation,
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

#[cfg(feature = "native-physics")]
impl RaycastWorld {
    /// 借出同一原实体表，继续复用已恢复的泛型upsert/remove；panic也归还表的所有权。
    fn with_entity_collider_map<R>(&mut self,
        select: fn(&mut Self) -> &mut std::collections::HashMap<bevy_ecs::entity::Entity, ColliderMeshInstance>,
        run: impl FnOnce(&mut Self,
        &mut std::collections::HashMap<bevy_ecs::entity::Entity, ColliderMeshInstance>) -> R) -> R {
        struct Borrowed<'a> {
            world: &'a mut RaycastWorld,
            map: std::collections::HashMap<bevy_ecs::entity::Entity, ColliderMeshInstance>,
            select: fn(&mut RaycastWorld) -> &mut std::collections::HashMap<bevy_ecs::entity::Entity, ColliderMeshInstance>,
        }
        impl Drop for Borrowed<'_> {
            fn drop(&mut self) { *(self.select)(self.world) = std::mem::take(&mut self.map); }
        }
        let map = std::mem::take(select(self));
        let mut borrowed = Borrowed { world: self, map, select };
        run(borrowed.world, &mut borrowed.map)
    }
}

/// 原1408d075e..1408d13a7：Changed<Transform>先写，再消费Removed转换组件。
/// 原只用包围盒宽/深和top-bottom，高度半值作为local平移；不加bottom，不读center/axis或Transform.scale。
#[cfg(feature = "native-physics")]
fn auto_clutter_collider_geometry(bounds: &utils::geometry::rectangle::Rectangle3d) -> (glam::Vec3, glam::Vec3) {
    let height = bounds.top - bounds.bottom;
    let zero = height * 0.0;
    (glam::Vec3::new(bounds.footprint.size.x, height, bounds.footprint.size.y),
        glam::Vec3::new(zero * 0.5, height * 0.5, zero * 0.5))
}

/// 原Changed/Removed两部分共用原+3e88实体表；这里不替代完整12参父系统。
#[cfg(feature = "native-physics")]
pub fn update_auto_clutter_rapier_colliders(
    mut world: bevy_ecs::prelude::ResMut<RaycastWorld>,
    changed: bevy_ecs::prelude::Query<(bevy_ecs::entity::Entity,
        &crate::utils::components::transform::Transform,
        &crate::components::drawable::AutoClutterConvertableToDecoClutter),
        bevy_ecs::prelude::Changed<crate::utils::components::transform::Transform>>,
    mut removed: bevy_ecs::prelude::RemovedComponents<crate::components::drawable::AutoClutterConvertableToDecoClutter>,
) {
    update_auto_clutter_colliders_inner(&mut world, &changed, removed.read());
}

#[cfg(feature = "native-physics")]
fn update_auto_clutter_colliders_inner<'a>(
    world: &mut RaycastWorld,
    changed: impl IntoIterator<Item = (bevy_ecs::entity::Entity,
        &'a crate::utils::components::transform::Transform,
        &'a crate::components::drawable::AutoClutterConvertableToDecoClutter)>,
    removed: impl Iterator<Item = bevy_ecs::entity::Entity>,
) {
    world.with_entity_collider_map(|world| &mut world.auto_clutter_colliders, |world, instances| {
        for (entity, transform, clutter) in changed {
            let (dimensions, center) = auto_clutter_collider_geometry(&clutter.bounds);
            let mesh = Arc::new(RaycastMeshData::new_cuboid_at(dimensions, center));
            upsert_collider(world, instances, entity, RaycastGeoSource::auto_clutter(entity),
                mesh, transform.translation, transform.rotation);
        }
        for entity in removed { remove_collider(world, instances, &entity); }
    });
}

/// 原1408d45a7..1408d5b4f的完整Roof分支；完整12参主系统的其它分支仍独立恢复。
/// Added删除→Changed触发体/屋顶→Removed触发体，不能将移除提前或过滤为只看mesh变更。
#[cfg(feature = "native-physics")]
pub fn update_roof_rapier_colliders(
    mut world: bevy_ecs::prelude::ResMut<RaycastWorld>,
    walls: bevy_ecs::prelude::Res<crate::resources::walls::public_walls::PublicWalls>,
    changed: bevy_ecs::prelude::Query<(
        bevy_ecs::entity::Entity,
        &crate::resources::roofs::RoofTriggerVolume,
        &crate::resources::roofs::Roof,
    ), (
        bevy_ecs::prelude::Or<(
            bevy_ecs::prelude::Changed<crate::resources::roofs::RoofTriggerVolume>,
            bevy_ecs::prelude::Changed<crate::resources::roofs::Roof>,
        )>,
        bevy_ecs::prelude::Without<crate::resources::roofs::RoofMarkedForDeletion>,
    )>,
    mut removed: bevy_ecs::prelude::RemovedComponents<crate::resources::roofs::RoofTriggerVolume>,
    deleted: bevy_ecs::prelude::Query<bevy_ecs::entity::Entity, (
        bevy_ecs::prelude::With<crate::resources::roofs::RoofTriggerVolume>,
        bevy_ecs::prelude::Added<crate::resources::roofs::RoofMarkedForDeletion>,
    )>,
) {
    update_roof_colliders_inner(&mut world, &walls, &changed, removed.read(), deleted.iter());
}

#[cfg(feature = "native-physics")]
fn update_roof_colliders_inner<'a>(
    world: &mut RaycastWorld,
    walls: &crate::resources::walls::public_walls::PublicWalls,
    changed: impl IntoIterator<Item = (bevy_ecs::entity::Entity,
        &'a crate::resources::roofs::RoofTriggerVolume, &'a crate::resources::roofs::Roof)>,
    removed: impl Iterator<Item = bevy_ecs::entity::Entity>,
    deleted: impl Iterator<Item = bevy_ecs::entity::Entity>,
) {
    world.with_entity_collider_map(|world| &mut world.roof_colliders, |world, instances| {
        for entity in deleted { remove_collider(world, instances, &entity); }
        for (entity, volume, roof) in changed {
            // 原先检查真实PublicWalls：缺墙或正在推倒时删除既有实例，不读取触发体数组。
            if !walls.get(roof.wall_id()).is_some_and(|wall| !wall.is_being_bulldozed()) {
                remove_collider(world, instances, &entity);
                continue;
            }
            let triangles: Vec<_> = volume.triangles_at_floor_y(roof.floor_y()).collect();
            let mesh = Arc::new(RaycastMeshData::new_unindexed(&triangles, volume.normals(), Some(volume.uvs())));
            upsert_collider(world, instances, entity,
                RaycastGeoSource::roof(roof.wall_id(), roof.is_flat(), roof.is_gable()),
                mesh, glam::Vec3::ZERO, glam::Quat::IDENTITY);
        }
        for entity in removed { remove_collider(world, instances, &entity); }
    });
}

/// 原1408d0490的十二组SystemParam。内部严格依次消费杂物、墙三域、墙移除、屋顶；
/// Query保留原Changed/Added/Without过滤，不依赖平行系统的偶然运行次序。
#[cfg(feature = "native-physics")]
pub fn update_rapier_colliders(
    mut world: bevy_ecs::prelude::ResMut<RaycastWorld>,
    changed_walls: bevy_ecs::prelude::Query<(bevy_ecs::entity::Entity,
        &crate::resources::walls::wall_trigger_volume::WallTriggerVolume,
        &crate::geometry::instanced_wall::InstancedWall<crate::systems::wall::construction::bricks::WallBrickInstance>),
        bevy_ecs::prelude::Changed<crate::resources::walls::wall_trigger_volume::WallTriggerVolume>>,
    mut removed_walls: bevy_ecs::prelude::RemovedComponents<crate::resources::walls::wall_trigger_volume::WallTriggerVolume>,
    roofs: bevy_ecs::prelude::Query<&crate::resources::roofs::Roof>,
    changed_roofs: bevy_ecs::prelude::Query<(bevy_ecs::entity::Entity,
        &crate::resources::roofs::RoofTriggerVolume, &crate::resources::roofs::Roof),
        (bevy_ecs::prelude::Or<(bevy_ecs::prelude::Changed<crate::resources::roofs::RoofTriggerVolume>,
            bevy_ecs::prelude::Changed<crate::resources::roofs::Roof>)>,
        bevy_ecs::prelude::Without<crate::resources::roofs::RoofMarkedForDeletion>)>,
    mut removed_roofs: bevy_ecs::prelude::RemovedComponents<crate::resources::roofs::RoofTriggerVolume>,
    deleted_roofs: bevy_ecs::prelude::Query<bevy_ecs::entity::Entity,
        (bevy_ecs::prelude::With<crate::resources::roofs::RoofTriggerVolume>,
        bevy_ecs::prelude::Added<crate::resources::roofs::RoofMarkedForDeletion>)>,
    arches: bevy_ecs::prelude::Res<crate::resources::walls::arches::ArchSegments>,
    lintels: bevy_ecs::prelude::Res<crate::resources::walls::arches::LintelSegments>,
    walls: bevy_ecs::prelude::Res<crate::resources::walls::public_walls::PublicWalls>,
    changed_auto: bevy_ecs::prelude::Query<(bevy_ecs::entity::Entity,
        &crate::utils::components::transform::Transform,
        &crate::components::drawable::AutoClutterConvertableToDecoClutter),
        bevy_ecs::prelude::Changed<crate::utils::components::transform::Transform>>,
    mut removed_auto: bevy_ecs::prelude::RemovedComponents<crate::components::drawable::AutoClutterConvertableToDecoClutter>,
) {
    use crate::resources::walls::{WallId, wall_trigger_volume::WallTriggerVolumeParams};
    use crate::systems::wall::valid_enclosure::ShapeParameters;
    update_auto_clutter_colliders_inner(&mut world, &changed_auto, removed_auto.read());
    for (entity, volume, instanced) in &changed_walls {
        let wall_id = WallId(instanced.id);
        // 原183d/18ce→5c37为required unwrap；不同于Roof的可缺失墙门控。
        // 有效性检查同样发生在空触发体和几何访问之前，不能吞掉发布/调度错误。
        let wall = walls.get(wall_id).expect("墙碰撞更新缺少已发布的PublicWallState");
        if wall.is_being_bulldozed() {
            remove_wall_owned_colliders(&mut world, entity);
            continue;
        }
        let source_flag = wall.authority.source_flag_65();
        // 原1b08读取param9 Lintel（RBP+4b0→stack290）；else 1c50读取param8 Arch。
        // 两表元素同为104B，不能根据反编译临时名或几何形状反推表身份。
        let segments = if source_flag == 1 { lintels.0.get(&wall_id) } else { arches.0.get(&wall_id) };
        world.with_entity_collider_map(|world| &mut world.arch_colliders, |world, instances| {
            let mut normals = Vec::new();
            let mut triangles = Vec::new();
            if let Some(segments) = segments {
                for segment in segments { construct_arch_geo(segment, &mut normals, &mut triangles); }
            }
            if triangles.is_empty() { remove_collider(world, instances, &entity); }
            else {
                let mesh = Arc::new(RaycastMeshData::new_unindexed(&triangles, &normals, None));
                upsert_collider(world, instances, entity, RaycastGeoSource::wall_owned(9, wall_id),
                    mesh, glam::Vec3::ZERO, glam::Quat::IDENTITY);
            }
        });
        // 关联屋顶只查询原Roof组件；缺失/失效Entity视为非平顶，不扫描其他屋顶。
        let flat_roof = walls.roof_component(wall_id, |entity| roofs.get(entity).ok())
            .is_some_and(|roof| roof.is_flat());
        world.with_entity_collider_map(|world| &mut world.wall_colliders, |world, instances| {
            if volume.quads.is_empty() { remove_collider(world, instances, &entity); }
            else {
                let geometry = volume.get_collision_geo(WallTriggerVolumeParams {
                    u_range: None,
                    normal_offset: Some(if wall.shape().tag() < 2 { f32::from_bits(0x3ea1_47ae) } else { 0.0 }),
                    minimum_height: 0.0,
                    offset_from_top: if flat_roof { f32::from_bits(0xbeeb_851f) } else { 0.0 },
                    apply: true,
                });
                let mesh = Arc::new(RaycastMeshData::new_unindexed(&geometry.triangles, &geometry.normals, Some(&geometry.uvs)));
                // 原字节左移3位，不把原flag 65与其他freehand字段混淆。
                upsert_collider(world, instances, entity, RaycastGeoSource::wall_owned(source_flag.wrapping_shl(3), wall_id),
                    mesh, glam::Vec3::ZERO, glam::Quat::IDENTITY);
            }
        });
        world.with_entity_collider_map(|world| &mut world.floor_colliders, |world, instances| {
            // 原JBE也排除NaN；地形相对最小高度用于门控，实际平面Y取authority.elevation。
            let shape = ShapeParameters::from_observed(*wall.shape().bytes()).get_shape();
            if !(wall.geometry.metrics.minimum_relative_height > 0.0) || shape.is_none() {
                remove_collider(world, instances, &entity);
            } else {
                let mesh = Arc::new(floor_mesh(shape.unwrap(), wall.authority.elevation()));
                upsert_collider(world, instances, entity, RaycastGeoSource::wall_owned(1, wall_id),
                    mesh, glam::Vec3::ZERO, glam::Quat::IDENTITY);
            }
        });
    }
    for entity in removed_walls.read() { remove_wall_owned_colliders(&mut world, entity); }
    update_roof_colliders_inner(&mut world, &walls, &changed_roofs, removed_roofs.read(), deleted_roofs.iter());
}

#[cfg(feature = "native-physics")]
fn remove_wall_owned_colliders(world: &mut RaycastWorld, entity: bevy_ecs::entity::Entity) {
    // 原墙移除顺序：主面+3dc8，再地板+3e28，最后拱/过梁+3e58。
    world.with_entity_collider_map(|world| &mut world.wall_colliders,
        |world, instances| { remove_collider(world, instances, &entity); });
    world.with_entity_collider_map(|world| &mut world.floor_colliders,
        |world, instances| { remove_collider(world, instances, &entity); });
    world.with_entity_collider_map(|world| &mut world.arch_colliders,
        |world, instances| { remove_collider(world, instances, &entity); });
}

/// 原内联floor_mesh（1408d3014..398c）：四角矩形，或0.5步长整圆加中心的完整扇面。
/// 第二份二维点列沿原求值次序保留；原new_indexed明确传None UV，不拿该点列补造UV。
#[cfg(feature = "native-physics")]
fn floor_mesh(shape: crate::systems::wall::valid_enclosure::Shape, elevation: f32) -> RaycastMeshData {
    use crate::systems::wall::valid_enclosure::Shape;
    let normal = glam::Vec3::new(-0.0, -1.0, -0.0);
    match shape {
        Shape::Rectangle(rectangle) => {
            let vertices = std::array::from_fn::<_, 4, _>(|index| {
                let p = rectangle.get_point(index);
                glam::Vec3::new(p.x, elevation, p.y)
            });
            let _projection_points = std::array::from_fn::<_, 4, _>(|index| rectangle.get_point(index));
            RaycastMeshData::new_indexed(&[[0,1,2],[0,2,3]], &vertices, Some(&[normal;4]), None)
        }
        Shape::Circle(circle) => {
            // 原as_points2/as_points3将ceil结果饱和转usize后取至少3点。
            // 这里复用现有逐点算法，避免其它用途的try采样接口提前拒绝非有限半径。
            let mut vertices: Vec<_> = circle.as_points2_ptcount(floor_circle_points_count(circle.radius))
                .map(|p| glam::Vec3::new(p.x, elevation, p.y)).collect();
            let mut projection_points: Vec<_> = circle.as_points2_ptcount(floor_circle_points_count(circle.radius)).collect();
            vertices.insert(0, glam::Vec3::new(circle.center.x, elevation, circle.center.y));
            projection_points.insert(0, circle.center);
            let indices: Vec<_> = (1..vertices.len()).zip(2..vertices.len())
                .map(|(first,second)| [0,first as u32,second as u32]).collect();
            let normals = vec![normal;vertices.len()];
            let mesh = RaycastMeshData::new_indexed(&indices, &vertices, Some(&normals), None);
            drop(projection_points);
            mesh
        }
    }
}

#[cfg(feature = "native-physics")]
fn floor_circle_points_count(radius: f32) -> usize {
    // 原140c97bc0/140c97c50的固定step=0.5调用，包含NaN→0、+∞→usize::MAX。
    (((radius * core::f32::consts::TAU / 0.5).ceil()) as usize).max(3)
}

/// Canonical upsert, B DecoratorId monomorph @141294980.
/// Kept requires the composed live pose, geometry hash and selected source fields to match.
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
    use slotmap::Key;
    let slot = source.subworld() as usize;
    let external =
        utils::iso_transform::IsoTransform::from_translation_rotation(position, rotation);
    // B tuple builder impl21: rdx=mesh.local, r8=external => external*local.
    let final_pose = mesh.local_transform().nested_in(&external);
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
                pos == final_pose.translation && rot == final_pose.rotation
            })
            .unwrap_or(false);
        if pose_matches {
            let registered = world.get_collider_mesh(existing.registry_index, existing.generation);
            if registered.data.hash == mesh.hash && registered.source.same_native_source(source) {
                return ColliderUpsert::Kept {
                    handle: existing.handle(),
                };
            }
        }
        dispose_collider_mesh_instance(world, &existing);
    }
    // 原转换共享同一个内部形状，不能通过 clone_dyn 重新分配几何。
    let shape = Arc::clone(
        &mesh
            .shape
            .as_ref()
            .expect("upsert_collider: mesh has no shape")
            .0,
    );
    let transformed = OffsetColliderTransform {
        position: final_pose.translation,
        rotation: final_pose.rotation,
        shape,
    };
    let registry_key = world.mesh_sources.insert(RaycastMesh::new(mesh, source));
    let raw = registry_key.data().as_ffi();
    let registry_index = raw as u32;
    let generation = (raw >> 32) as u32;
    // This Vec is the existing source/generation projection, never an allocator.
    while world.colliders.len() <= registry_index as usize {
        world.colliders.push(ColliderRegistryEntry::empty());
    }
    let mut projection = ColliderRegistryEntry::empty();
    projection.first = u32::from_le_bytes(source.0[..4].try_into().unwrap());
    projection.meta[..20].copy_from_slice(&source.0[4..24]);
    projection.generation = generation;
    world.colliders[registry_index as usize] = projection;
    world.registry_live += 1;

    let mut collider = to_offset_collider_builder(&transformed).build();
    collider.user_data = u128::from(raw);
    world.slots[slot].dirty = true;
    let aabb = collider.compute_aabb();
    let flat = xz_flatten_aabb(&Aabb3 {
        min: glam::Vec3::new(aabb.mins.x, aabb.mins.y, aabb.mins.z),
        max: glam::Vec3::new(aabb.maxs.x, aabb.maxs.y, aabb.maxs.z),
    });
    world.slots[slot]
        .dirty_tiles
        .rasterize_world_rect([flat.min.x, flat.min.y, flat.max.x, flat.max.y]);
    let collider_handle = world.slots[slot].colliders.insert(collider);
    let instance =
        ColliderMeshInstance::new(registry_index, generation, collider_handle, slot as u8);
    let handle = instance.handle();
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
    // The same SlotMap owns the mesh and validates the original full generation.
    let raw = (u64::from(generation) << 32) | u64::from(instance.registry_index);
    let _removed_mesh = world
        .mesh_sources
        .remove(strict_mesh_key(raw))
        .expect("dispose_collider_mesh_instance: mesh receipt missing or stale");
    let old_head = world.registry_free_head;
    world.colliders[index].set_free_next(old_head);
    world.registry_free_head = index as u32;
    world.registry_live -= 1;
    world.colliders[index].generation = generation.wrapping_add(1);
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
        Self::raycast_with_max_toi(world, origin, dir, f32::MAX, mask)
    }

    fn raycast_with_max_toi(
        world: &RaycastWorld, origin: glam::Vec3, dir: glam::Vec3,
        max_toi: f32, mask: u32,
    ) -> RaycastWorldResult {
        Self::raycast_with_params(world, origin, dir, max_toi, mask, None)
    }

    fn raycast_with_params(
        world: &RaycastWorld, origin: glam::Vec3, dir: glam::Vec3,
        max_toi: f32, mask: u32,
        excluded_wall: Option<crate::resources::walls::WallId>,
    ) -> RaycastWorldResult {
        let ray = Ray::new(
            Point::new(origin.x, origin.y, origin.z),
            Vector::new(dir.x, dir.y, dir.z),
        );
        let mut selected = None;
        let mut maximum_toi = max_toi;
        for (slot_index, slot) in world.slots.iter().enumerate() {
            if (mask >> slot_index) & 1 == 0 {
                continue;
            }
            // 原完整射线predicate的0x105只含0/2/8，不排除kind1；simple是0x107。
            let predicate = |handle: ColliderHandle, _: &Collider| {
                let (mesh, _) = world.raycast_mesh_for_collider(slot_index as u8, handle);
                RaycastWorld::source_passes_wall_exclusion(mesh.source, excluded_wall, 0x105)
            };
            if let Some((handle, intersection)) = slot.pipeline.cast_ray_and_get_normal(
                &slot.bodies,
                &slot.colliders,
                &ray,
                maximum_toi,
                true,
                QueryFilter::default().predicate(&predicate),
            ) {
                // B 0x1408cde07: the next slot receives this hit's TOI.
                maximum_toi = intersection.time_of_impact;
                // 原循环只保留最终winner；不能提前插值导致被后来命中覆盖的mesh先panic。
                selected = Some((slot_index as u8, handle, intersection));
            }
        }
        let Some((slot, handle, hit)) = selected else { return RaycastWorldResult::miss(); };
        let (mesh, collider) = world.raycast_mesh_for_collider(slot, handle);
        let position = collider.position();
        let source = std::array::from_fn(|i| u64::from_le_bytes(mesh.source.0[i*8..i*8+8].try_into().unwrap()));
        let triangle = match hit.feature { rapier3d::parry::shape::FeatureId::Face(index) => Some(index), _ => None };
        rapier_hit_to_raycast_world_result_at_pose(&mesh.data, origin, dir, hit.time_of_impact,
            glam::Vec3::new(hit.normal.x, hit.normal.y, hit.normal.z), triangle, source,
            glam::Vec3::new(position.translation.x, position.translation.y, position.translation.z),
            glam::Quat::from_xyzw(position.rotation.i, position.rotation.j, position.rotation.k, position.rotation.w))
    }

    /// O：0x1408cdf80。17 槽 Qbvh `traverse_depth_first_node` + `RayIntersectionsVisitor`：
    /// 每槽 closure 置 found 并终止遍历，命中即返回 true。
    fn intersects_any(
        world: &RaycastWorld,
        origin: glam::Vec3,
        dir: glam::Vec3,
        max_toi: f32,
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
                max_toi,
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
                    handle: registry_handle_region(world, slot_index as u8, handle),
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
        max_toi: f32,
        mask: u32,
        excluded_wall: Option<crate::resources::walls::WallId>,
    ) -> RaycastSimpleResult {
        Self::raycast_simple_filtered(world, origin, dir, max_toi, mask, excluded_wall)
    }

    fn raycast_simple_filtered(
        world: &RaycastWorld, origin: glam::Vec3, dir: glam::Vec3, max_toi: f32,
        mask: u32, excluded: Option<crate::resources::walls::WallId>,
    ) -> RaycastSimpleResult {
        let ray = Ray::new(
            Point::new(origin.x, origin.y, origin.z),
            Vector::new(dir.x, dir.y, dir.z),
        );
        let mut result = RaycastSimpleResult::miss();
        let mut maximum_toi = max_toi;
        for (slot_index, slot) in world.slots.iter().enumerate() {
            if (mask >> slot_index) & 1 == 0 {
                continue;
            }
            // 1408ca9a0：先通过真实 collider receipt 校验注册记录，再按墙ID过滤。
            let predicate = |handle: ColliderHandle, _: &Collider| {
                world.ray_collider_is_allowed(slot_index as u8, handle, excluded, 0x107)
            };
            if let Some((handle, toi)) = slot.pipeline.cast_ray(
                &slot.bodies,
                &slot.colliders,
                &ray,
                maximum_toi,
                true,
                QueryFilter::default().predicate(&predicate),
            ) {
                // B 0x1408ce180: a hit shortens the next slot's maximum TOI.
                maximum_toi = toi;
                result = RaycastSimpleResult {
                    handle: registry_handle_region(world, slot_index as u8, handle),
                    position: origin + dir * toi,
                    toi,
                };
            }
        }
        result
    }

    fn raycast_simple_excluding_wall(
        world: &RaycastWorld, origin: glam::Vec3, dir: glam::Vec3, max_toi: f32,
        mask: u32, excluded: crate::resources::walls::WallId,
    ) -> RaycastSimpleResult {
        Self::raycast_simple_filtered(world, origin, dir, max_toi, mask, Some(excluded))
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
                    handle: registry_handle_region(world, slot_index as u8, handle),
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
        Rapier::intersects_any(&self.world, origin, dir, f32::MAX, mask)
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
        self.world
            .spherecast_w_terrain(terrain, origin, dir, radius, options, mask)
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
    /// 1. 系统参数变更 tick 写 `*[param+0x10] = [param+0x1c]`；由 Bevy ResMut 管理；
    /// 2. 重建：`default<RapierSubWorld>`×17（0x390 B/槽）+ 空
    ///    `MeshInstanceSources`（0xe0 B 默认副本写 @+0x3dc8、`Vec<Slot<RaycastMesh>>`
    ///    指针 @+0x3ea8/+0x3eb0）。
    ///
    /// 3. 原 1408d8b48–6a 先保存旧世界再写入新世界；1408d8c36–d20
    ///    按槽取旧 +0x150 的 7 qword，析构新发布器并搬回旧发布器；
    ///    1408d8c69–87 同时保留槽内待发布 DirtyTiles 与脏标志，
    ///    1408d8dbb–e41 对 +0x3cb0 的 ColliderDiff 发布器同样处理。
    ///    保留旧订阅及未读取的通知，不发布伪造的重置差量。
    pub fn on_level_reload(&mut self) {
        let mut fresh = RaycastWorld::new();
        std::mem::swap(&mut fresh.publisher, &mut self.publisher);
        std::mem::swap(&mut fresh.dirty_tiles_publishers, &mut self.dirty_tiles_publishers);
        #[cfg(feature = "native-physics")]
        for (new, old) in fresh.slots.iter_mut().zip(self.slots.iter_mut()) {
            std::mem::swap(&mut new.dirty_tiles, &mut old.dirty_tiles);
            new.dirty = old.dirty;
        }
        *self = fresh;
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
    // B impl45 uses rdx=external, r8=mesh.local: local*external.
    // The reconstructed explicit parent remains outside that original composition.
    let nested = transform
        .nested_in(&mesh.local_transform())
        .nested_in(parent);
    // 原140b26d4d只增加shape Arc强计数，保持几何和metadata身份。
    let shape = Arc::clone(
        &mesh
            .shape
            .as_ref()
            .expect("into_transformed_shape_iso: mesh has no shape")
            .0,
    );
    OffsetColliderTransform {
        position: nested.translation,
        rotation: nested.rotation,
        shape,
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
        callback: F,
    ) {
        self.iter_shape_overlaps_at_filtered(shape, shape_position, mask, |_, _| true, callback);
    }

    /// 原 predicate 先于形状精测运行；保留同一 Rapier 管线及句柄/mesh generation 校验。
    #[cfg(feature = "native-physics")]
    fn iter_shape_overlaps_at_filtered<
        P: Fn(SubworldColliderHandle, &RaycastGeoSource) -> bool,
        F: FnMut(SubworldColliderHandle, &RaycastGeoSource) -> bool,
    >(
        &self,
        shape: &dyn Shape,
        shape_position: &Isometry<f32>,
        mask: u32,
        predicate: P,
        mut callback: F,
    ) {
        for (slot_index, slot) in self.slots.iter().enumerate() {
            if (mask >> slot_index) & 1 == 0 {
                continue;
            }
            let mut stop = false;
            let filter = |handle: rapier3d::geometry::ColliderHandle, _: &rapier3d::geometry::Collider| {
                let (index, generation) = handle.into_raw_parts();
                let mesh = self.registered_mesh_for_collider(slot_index as u8, handle);
                predicate(SubworldColliderHandle::new(index, generation, slot_index as u8), &mesh.source)
            };
            slot.pipeline.intersections_with_shape(
                &slot.bodies,
                &slot.colliders,
                shape_position,
                shape,
                QueryFilter { predicate: Some(&filter), ..QueryFilter::default() },
                |handle| {
                    let (index, generation) = handle.into_raw_parts();
                    let source = self
                        .registered_mesh_for_collider(slot_index as u8, handle)
                        .source;
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
        self.iter_shape_overlaps_handle_filtered(handle, mask, |_, _| true, callback);
    }

    /// 楼梯结构图的两组原闭包：先排除同边/本轮已处理端，再做真实形状相交。
    #[cfg(feature = "native-physics")]
    pub fn iter_shape_overlaps_handle_filtered<
        P: Fn(SubworldColliderHandle, &RaycastGeoSource) -> bool,
        F: FnMut(SubworldColliderHandle, &RaycastGeoSource) -> bool,
    >(
        &self,
        handle: SubworldColliderHandle,
        mask: u32,
        predicate: P,
        callback: F,
    ) {
        let t = self.into_transformed_shape(handle);
        let shape = t.as_collider_shape();
        let position = collider_isometry(t.position, t.rotation);
        self.iter_shape_overlaps_at_filtered(shape.as_ref(), &position, mask, predicate, callback);
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

    /// B `decorator_overlaps_with_others` @0x14129be00: query decorator slot2
    /// and return the actual registered source of an accepted overlap.
    /// The predicate @0x14129c3d0 keeps same-wall decorators and excludes self;
    /// ordered mode further accepts only IDs lower than the current ID.
    #[cfg(feature = "native-physics")]
    pub fn decorator_overlaps_with_others(
        &self,
        decorator: &OffsetColliderTransform,
        current_id: crate::resources::walls::decorator::DecoratorId,
        wall: crate::resources::walls::WallId,
        ordered: bool,
    ) -> Option<&RaycastGeoSource> {
        // B passes slice length17 and SubworldIndex(2), not a subworld mask.
        const DECORATOR_SLOT: u8 = 2;
        let slot = &self.slots[DECORATOR_SLOT as usize];
        let predicate = |handle: ColliderHandle, _: &Collider| {
            let source = &self.registered_mesh_for_collider(DECORATOR_SLOT, handle).source;
            assert_eq!(source.tag(), 3, "decorator overlap: unexpected source kind");
            let candidate_id = u32::from_le_bytes(source.0[4..8].try_into().unwrap());
            let is_wall = source.0[8..16] == [0; 8];
            let accepted_id = if ordered {
                current_id.0 > candidate_id
            } else {
                current_id.0 != candidate_id
            };
            is_wall
                && accepted_id
                && u64::from_le_bytes(source.0[16..24].try_into().unwrap()) == wall.0
        };
        let shape = decorator.as_collider_shape();
        let position = collider_isometry(decorator.position, decorator.rotation);
        let handle = slot.pipeline.intersection_with_shape(
            &slot.bodies,
            &slot.colliders,
            &position,
            shape.as_ref(),
            QueryFilter::default().predicate(&predicate),
        )?;
        // Resolve the accepted physical handle again, preserving receipt checks
        // and returning a borrow of the canonical mesh owner's source.
        Some(&self.registered_mesh_for_collider(DECORATOR_SLOT, handle).source)
    }
}

impl RaycastGeoSource {
    /// `from_slot`（O：`subworld()` 的反向映射；由 0x142aef03c 表取逆）。
    ///
    /// 槽→来源：`[0,2,3,6,10,11,8,7,4,1,9,12,13,14,5,15,16]`。
    pub fn from_slot(slot: u8) -> RaycastGeoSource {
        const SLOT_TO_SOURCE: [u8; 17] = [0, 2, 3, 6, 10, 11, 8, 7, 4, 1, 9, 12, 13, 14, 5, 15, 16];
        Self::tag_only(SLOT_TO_SOURCE[slot as usize])
    }
}

#[cfg(all(test, feature = "native-physics"))]
mod global_raycast_recovery_tests {
    use super::*;
    use glam::{Quat,Vec2,Vec3};
    use std::collections::HashMap;

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("../../../../../tests/fixtures/native/m01-global-ray.json")).unwrap()
    }
    fn floats(row:&serde_json::Value,name:&str) -> Vec<f32> {
        row[name].as_array().unwrap().iter().map(|n|n.as_f64().unwrap() as f32).collect()
    }
    fn source(row:&serde_json::Value) -> RaycastGeoSource {
        let values=row["source"].as_array().unwrap();crate::systems::collision::query_policy::RaycastGeoSourceProjection(
            std::array::from_fn(|i|values[i].as_u64().unwrap() as u8))
    }
    fn data(row:&serde_json::Value) -> RaycastMeshData {
        if row["primitive"]==true {return RaycastMeshData::new_cuboid_at(Vec3::splat(2.0),Vec3::NEG_Y);}
        let vertices=floats(row,"vertices").chunks_exact(3).map(|p|Vec3::from_slice(p)).collect::<Vec<_>>();
        let normals=floats(row,"normal_values").chunks_exact(3).map(|p|Vec3::from_slice(p)).collect::<Vec<_>>();
        let uv=floats(row,"uv_values").chunks_exact(2).map(|p|Vec2::from_slice(p)).collect::<Vec<_>>();
        let mesh=RaycastMeshData::new_indexed(&[[0,1,2]],&vertices,
            (row["normals"]==true).then_some(normals.as_slice()),(row["uv"]==true).then_some(uv.as_slice()));
        if let Some(indices)=row.get("indices") {
            let bad=std::array::from_fn(|i|indices[i].as_u64().unwrap() as u32);
            // 故障夹具：唯一owned三索引缓冲保持原分配长度，物理Shape已拥有独立有效副本。
            // 只改变转换用源索引，验证原winner后置读取；不伪造实际物理相交记录。
            unsafe {*(mesh.indices_ptr as *mut [u32;3])=bad;}
        }
        mesh
    }
    fn actual_world(case:&serde_json::Value) -> (RaycastWorld,HashMap<usize,ColliderMeshInstance>) {
        let mut world=RaycastWorld::new();let mut instances=HashMap::new();
        for (id,row) in case["colliders"].as_array().unwrap().iter().enumerate() {
            let translation=floats(row,"translation");let rotation=floats(row,"rotation");
            upsert_collider(&mut world,&mut instances,id,source(row),Arc::new(data(row)),Vec3::from_slice(&translation),Quat::from_slice(&rotation));
            if let Some(generation)=row.get("raw_generation") {
                let instance=instances[&id];let raw=(generation.as_u64().unwrap()<<32)|u64::from(instance.registry_index);
                world.get_rapier_collider(instance.slot,instance.collider).unwrap().user_data=u128::from(raw);
            }
        }
        world.flush_colliders();(world,instances)
    }
    fn active_words(hit:RaycastWorldResult) -> Vec<u32> {
        hit.handle.into_iter().flat_map(|word|[word as u32,(word>>32)as u32])
            .chain(hit.position.to_array().map(f32::to_bits)).chain(hit.normal.to_array().map(f32::to_bits))
            .chain([hit.toi.to_bits(),hit.discriminant]).chain(hit.extra.map(f32::to_bits)).collect()
    }

    #[test]
    fn original_full_global_outputs_match_actual_rapier_world_for_all_slots_layers_and_yaw_poses() {
        let fixture=fixture();let mut compared=0;
        for case in fixture["cases"].as_array().unwrap() {
            if !case["panic"].is_null() || case["name"]=="no_face" || case["name"]=="back_face" {continue;}
            // 非Face/反面feature的显式契约在下个resolved原差分测试中核验；这里使用真实QBVH返回。
            let (world,_instances)=actual_world(case);let ray=case.get("ray").map(|_|floats(case,"ray")).unwrap_or(vec![0.,12.,0.,0.,-1.,0.]);
            let limit=case.get("max_toi").map(|n|n.as_f64().unwrap() as f32).unwrap_or(f32::MAX);
            let hit=world.raycast_with_max_toi(Vec3::from_slice(&ray[..3]),Vec3::from_slice(&ray[3..]),limit,case["mask"].as_u64().unwrap() as u32);
            let expected=case["output"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap() as u32).collect::<Vec<_>>();
            assert_eq!(hit.discriminant,expected[13],"原global判别{}",case["name"]);
            if hit.is_hit() {
                let actual=active_words(hit);
                assert_eq!(&actual[..14],&expected[..14],"原global完整活动结果{}",case["name"]);
                // 原primitive无UV分支不写extra；三角网格无UV仍写barycentric尾分量。
                let selected=case["physical_trace"][0].as_u64().unwrap();
                let primitive=case["colliders"].as_array().unwrap().iter().find(|c|c["slot"]==selected).unwrap()["primitive"]==true;
                if !primitive {assert_eq!(&actual[14..],&expected[14..],"原global UV/barycentric{}",case["name"]);}
            }
            compared+=1;
        }
        assert_eq!(compared,70);
    }

    #[test]
    fn original_global_predicate_normalizes_raw_generation_then_filters_only_0_2_8_wall_kinds() {
        let fixture=fixture();let rows=fixture["predicates"].as_array().unwrap();assert_eq!(rows.len(),204);
        for case in rows {
            let row=&case["colliders"][0];let mut world=RaycastWorld::new();let mut instances=HashMap::new();
            upsert_collider(&mut world,&mut instances,0,source(row),Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE,Vec3::ZERO)),Vec3::Y*5.,Quat::IDENTITY);
            let instance=instances[&0];let raw=(row["raw_generation"].as_u64().unwrap()<<32)|u64::from(instance.registry_index);
            world.get_rapier_collider(instance.slot,instance.collider).unwrap().user_data=u128::from(raw);world.flush_colliders();
            let excluded=case.get("excluded_wall").map(|n|crate::resources::walls::WallId(n.as_u64().unwrap()));
            let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||Rapier::raycast_with_params(&world,Vec3::Y*12.,Vec3::NEG_Y,
                f32::MAX,1<<instance.slot,excluded)));
            if !case["panic"].is_null() {assert!(result.is_err(),"原predicate应失败{case}");}
            else {assert_eq!(result.unwrap().is_hit(),case["allowed"].as_bool().unwrap(),"原predicate{case}");}
        }
    }

    #[test]
    fn original_final_feature_and_bounds_paths_use_the_same_complete_pose_converter() {
        let fixture=fixture();let mut checked=0;
        for case in fixture["cases"].as_array().unwrap() {
            if case["physical_trace"].as_array().unwrap().is_empty() || matches!(case["panic"].as_str(),Some("receipt"|"physical")) {continue;}
            let slot=case["physical_trace"][0].as_u64().unwrap();let row=case["colliders"].as_array().unwrap().iter().find(|row|row["slot"]==slot).unwrap();
            let mesh=data(row);let translation=floats(row,"translation");let rotation=floats(row,"rotation");
            let src=source(row);let words=std::array::from_fn(|i|u64::from_le_bytes(src.0[i*8..i*8+8].try_into().unwrap()));
            let feature=row.get("feature").and_then(|v|v.as_u64()).unwrap_or(2);
            let face=row.get("face").and_then(|v|v.as_u64()).unwrap_or(0);
            let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||rapier_hit_to_raycast_world_result_at_pose(
                &mesh,Vec3::Y*12.,Vec3::NEG_Y,row["toi"].as_f64().unwrap() as f32,Vec3::Y,
                (feature==2).then_some(face as u32),words,Vec3::from_slice(&translation),Quat::from_slice(&rotation))));
            if case["panic"]=="bounds" {assert!(result.is_err(),"原bounds{case}");}
            else {
                let actual=result.unwrap();let expected=case["output"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap() as u32).collect::<Vec<_>>();
                assert_eq!(actual.discriminant,expected[13]);
                if actual.is_hit() {let actual=active_words(actual);assert_eq!(&actual[..14],&expected[..14]);
                    if row["primitive"]!=true {assert_eq!(&actual[14..],&expected[14..]);}}
            }
            checked+=1;
        }
        assert_eq!(checked,75);
    }

    #[test]
    fn only_final_winner_reads_corrupted_conversion_indices_in_a_real_world() {
        let fixture=fixture();
        for near in [true,false] {
            let name=if near {"deferred_bad_vertices_True"}else{"deferred_bad_vertices_False"};
            let case=fixture["cases"].as_array().unwrap().iter().find(|case|case["name"]==name).unwrap();
            let (world,_)=actual_world(case);
            let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||world.raycast(Vec3::Y*12.,Vec3::NEG_Y,3)));
            if near {let result=result.unwrap();assert_eq!(active_words(result),case["output"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap() as u32).collect::<Vec<_>>());}
            else {assert!(result.is_err());}
        }
    }

    #[test]
    fn bounded_simple_adapter_preserves_wall_exclusion_max_toi_and_mask() {
        use crate::resources::walls::WallId;
        let mut world=RaycastWorld::new();let mut instances=HashMap::new();
        upsert_collider(&mut world,&mut instances,0,RaycastGeoSource::wall_owned(0,WallId(7)),
            Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE,Vec3::ZERO)),Vec3::X*0.1,Quat::IDENTITY);
        upsert_collider(&mut world,&mut instances,1,RaycastGeoSource::wall_owned(0,WallId(8)),
            Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE,Vec3::ZERO)),Vec3::X*1.5,Quat::IDENTITY);
        world.flush_colliders();
        assert!(world.raycast_simple_with_max_toi_excluding_wall_id(Vec3::ZERO,Vec3::X*2.,0.49,1,WallId(7)).is_miss());
        let hit=world.raycast_simple_with_max_toi_excluding_wall_id(Vec3::ZERO,Vec3::X*2.,0.5,1,WallId(7));
        assert!(!hit.is_miss());assert_eq!(hit.toi,0.5);assert_eq!(hit.handle[1],8);
        assert!(world.raycast_simple_with_max_toi_excluding_wall_id(Vec3::ZERO,Vec3::X*2.,1.,0,WallId(7)).is_miss());
    }

    #[test]
    fn original_terrain_wrapper_limits_and_copied_point_keep_the_service_contract() {
        let fixture=fixture();let rows=fixture["terrain_cases"].as_array().unwrap();assert_eq!(rows.len(),16);
        for case in rows {
            assert!(case["panic"].is_null());let mask=case["mask"].as_u64().unwrap() as u32;
            let maximum=case.get("max_toi").map(|v|v.as_f64().unwrap()as f32).unwrap_or(f32::MAX);
            if mask&0x20==0 {assert!(case["terrain_limit"].is_null());continue;}
            let limit=if mask&2!=0 && 4.0<=maximum {4.0f32}else{maximum};
            assert_eq!(case["terrain_limit"].as_u64().unwrap()as u32,limit.to_bits(),"原UV与无UV hit都收紧terrain上限");
            let expected=case["output"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u32).collect::<Vec<_>>();
            if expected[0]&255==11 {
                let toi=case["terrain_toi"].as_f64().unwrap()as f32;
                let point=case.get("terrain_position").map(|_|Vec3::from_slice(&floats(case,"terrain_position"))).unwrap_or(Vec3::new(0.,12.-toi,0.));
                let actual=active_words(terrain_result_at_position(toi,point));
                assert_eq!(&actual[6..14],&expected[6..14]);assert_eq!(actual[0]&255,11);
            }
        }
    }

    #[test]
    fn actual_global_receipts_obey_mask_normalization_staleness_and_retirement() {
        use crate::resources::walls::WallId;
        let mut world=RaycastWorld::new();let mut instances=HashMap::new();
        let mesh=Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE,Vec3::ZERO));let weak=Arc::downgrade(&mesh);
        upsert_collider(&mut world,&mut instances,0,RaycastGeoSource::wall_owned(0,WallId(7)),mesh,Vec3::Y*5.,Quat::IDENTITY);
        world.flush_colliders();let original=instances[&0];
        let receipt=(u64::from(original.generation)<<32)|u64::from(original.registry_index);
        for raw in [receipt.wrapping_add(2<<32),(u64::from(original.generation)<<32)|u64::from(u32::MAX)] {
            world.get_rapier_collider(original.slot,original.collider).unwrap().user_data=u128::from(raw);
            assert!(world.raycast(Vec3::Y*12.,Vec3::NEG_Y,0).is_miss(),"未选slot不触发receipt读取");
            assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||world.raycast(Vec3::Y*12.,Vec3::NEG_Y,1))).is_err());
        }
        world.get_rapier_collider(original.slot,original.collider).unwrap().user_data=u128::from(receipt);
        remove_collider(&mut world,&mut instances,&0).unwrap();world.flush_colliders();assert!(weak.upgrade().is_none());
        assert!(world.raycast(Vec3::Y*12.,Vec3::NEG_Y,1).is_miss());
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||world.raycast_mesh_for_collider(original.slot,original.collider))).is_err());
        upsert_collider(&mut world,&mut instances,1,RaycastGeoSource::wall_owned(0,WallId(8)),
            Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE,Vec3::ZERO)),Vec3::Y*5.,Quat::IDENTITY);
        world.flush_colliders();let next=instances[&1];assert_eq!(next.registry_index,original.registry_index);assert_ne!(next.generation,original.generation);
        world.get_rapier_collider(next.slot,next.collider).unwrap().user_data=u128::from(receipt);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||world.raycast(Vec3::Y*12.,Vec3::NEG_Y,1))).is_err());
        let even=(u64::from(next.generation&!1)<<32)|u64::from(next.registry_index);
        world.get_rapier_collider(next.slot,next.collider).unwrap().user_data=u128::from(even);
        assert_eq!(world.raycast(Vec3::Y*12.,Vec3::NEG_Y,1).handle[1],8,"新世代raw偶数也只OR1，不固定到首世代");
    }

    #[test]
    fn actual_twelve_param_roofs_global_nearest_uv_terrain_merge_and_removal_are_continuous() {
        use crate::resources::{roofs::{Roof,RoofMarkedForDeletion},walls::{WallId,public_walls::PublicWalls}};
        let (mut world,mut schedule,_,_)=super::full_rapier_collider_producer_tests::setup();
        let mut low=super::full_rapier_collider_producer_tests::roof(42,0.);
        let mut high=super::full_rapier_collider_producer_tests::roof(43,1.);
        low.observed_mut()[0x4c..0x50].copy_from_slice(&3.0f32.to_le_bytes());high.observed_mut()[0x4c..0x50].copy_from_slice(&7.0f32.to_le_bytes());
        let low=world.spawn((low,super::full_rapier_collider_producer_tests::roof_volume())).id();
        let high=world.spawn((high,super::full_rapier_collider_producer_tests::roof_volume())).id();
        world.resource_mut::<PublicWalls>().associate_roof(WallId(42),low);world.resource_mut::<PublicWalls>().associate_roof(WallId(43),high);
        schedule.run(&mut world);world.resource_mut::<RaycastWorld>().flush_colliders();
        let origin=Vec3::new(-0.25,12.,-0.25);
        let hit=world.resource::<RaycastWorld>().raycast(origin,Vec3::NEG_Y,2);
        assert!(hit.is_hit());assert_eq!(hit.discriminant,1);assert_eq!(hit.extra,[0.375,0.375]);assert_eq!(hit.position.y,7.);
        assert_eq!(hit.handle[1],43);
        let instance=*world.resource::<RaycastWorld>().roof_collider_instance(high).unwrap();
        let weak=Arc::downgrade(&world.resource::<RaycastWorld>().get_collider_mesh(instance.registry_index,instance.generation).data);
        world.entity_mut(high).insert(RoofMarkedForDeletion);schedule.run(&mut world);world.resource_mut::<RaycastWorld>().flush_colliders();
        assert!(weak.upgrade().is_none());assert_eq!(world.resource::<RaycastWorld>().raycast(origin,Vec3::NEG_Y,2).position.y,3.);
        let mut terrain=TerrainHeightsData::<TerrainHeightSsbo>::new();terrain.set_height_range(0.,0.);
        let hit=world.resource::<RaycastWorld>().raycast_w_terrain(&terrain,origin,Vec3::NEG_Y,2|0x20);
        assert_eq!(hit.discriminant,1);assert_eq!(hit.position.y,3.,"近UV命中必须限制更远地形查询");
        assert!(world.resource::<RaycastWorld>().raycast_w_terrain_with_max_toi(&terrain,origin,Vec3::NEG_Y,8.,2|0x20).is_miss());
        assert_eq!(world.resource::<RaycastWorld>().raycast_w_terrain_with_max_toi(&terrain,origin,Vec3::NEG_Y,9.,2|0x20).discriminant,1);
        for (height,terrain_wins) in [(0.0f32,true),(-3.,true),(3.,false)] {
            world.get_mut::<Roof>(low).unwrap().observed_mut()[0x4c..0x50].copy_from_slice(&height.to_le_bytes());
            schedule.run(&mut world);world.resource_mut::<RaycastWorld>().flush_colliders();
            let hit=world.resource::<RaycastWorld>().raycast_w_terrain(&terrain,origin,Vec3::NEG_Y,2|0x20);
            assert_eq!(hit.is_terrain(),terrain_wins);assert!(hit.is_hit());
            assert_eq!(hit.discriminant,if terrain_wins {0}else{1});
        }
        world.despawn(low);schedule.run(&mut world);world.resource_mut::<RaycastWorld>().flush_colliders();
        assert!(world.resource::<RaycastWorld>().raycast(origin,Vec3::NEG_Y,2).is_miss());
    }
}

#[cfg(all(test, feature = "native-physics"))]
mod full_rapier_collider_producer_tests {
    use super::*;
    use bevy_ecs::prelude::*;
    use glam::{Quat, Vec2, Vec3};
    use crate::resources::{roofs::{Roof, RoofTriggerVolume, RoofMarkedForDeletion, geometry::surface::Curve2},
        walls::{WallId, WallState, InnerWalls, arches::{ArchSegments, LintelSegments},
            wall_trigger_volume::{Quad, WallTriggerVolume}, publication_fields::WallShapeSnapshot,
            public_walls::{PublicWalls, WallPublicationDirty, WallSpatialHash}, publication_source::TerrainHeightGrid512}};
    use crate::geometry::instanced_wall::InstancedWall;
    use crate::systems::wall::{construction::{bricks::WallBrickInstance,
        structures::MaterializedWallSegment, segmentation::WallLoopRange}, valid_enclosure::Shape};
    use crate::components::drawable::{AutoClutterConvertableToDecoClutter, AutoClutterDestination, AutoClutterSource};
    use crate::utils::components::transform::Transform;

    fn segment(x: f32) -> MaterializedWallSegment {
        MaterializedWallSegment {heights:vec![1.0,2.0],positions:vec![[x,3.0,1.0],[x+1.0,3.0,1.0]],
            points_u:vec![0.0,1.0],source_indices:[0,2],range:WallLoopRange {start:0,end:1,modulus:2}}
    }
    fn volume() -> WallTriggerVolume {
        let mut volume=WallTriggerVolume::default();
        volume.quads.push(Quad {wall_uv:[Vec2::ZERO,Vec2::X,Vec2::Y,Vec2::ONE],
            positions:[Vec3::new(-2.0,3.0,-2.0),Vec3::new(2.0,3.0,-2.0),Vec3::new(-2.0,7.0,-2.0),Vec3::new(2.0,7.0,-2.0)],
            prepared_u:[0.0,1.0],endpoint_normals:[Vec3::Z;2],segment_heights:[3.0;2]});volume
    }
    pub(super) fn roof(id:u64,height:f32) -> Roof {
        let mut bytes=[0;88];bytes[..8].copy_from_slice(&id.to_le_bytes());
        bytes[0x30..0x34].copy_from_slice(&height.to_le_bytes());bytes[0x4c..0x50].copy_from_slice(&3.0f32.to_le_bytes());
        Roof::from_observed(bytes)
    }
    pub(super) fn roof_volume() -> RoofTriggerVolume {
        let mut volume=RoofTriggerVolume::default();
        volume.set_triangles(vec![[Vec3::new(-1.0,0.0,-1.0),Vec3::new(-1.0,0.0,1.0),Vec3::new(1.0,0.0,-1.0)]]);
        volume.set_normals(vec![[Vec3::Y;3]]);volume.set_uvs(vec![[Vec2::ZERO,Vec2::Y,Vec2::X]]);volume
    }
    pub(super) fn setup() -> (World,Schedule,Entity,Entity) {
        let mut inner=InnerWalls::new();
        for (id,flag,shape) in [(42,0,Shape::Rectangle(utils::geometry::Rectangle2d {axis:Vec2::X,center:Vec2::ZERO,size:Vec2::splat(4.0)})),
            (43,1,Shape::Circle(utils::geometry::Circle2d {center:Vec2::new(8.0,0.0),radius:2.0,rotation:0.3}))] {
            let curve=Curve2::try_new(vec![[-2.0,-2.0],[2.0,-2.0],[2.0,2.0],[-2.0,2.0]],true).unwrap();
            let state=WallState::new(curve,2.0,0,flag,false);let (mut snapshot,freehand)=state.into_snapshot();
            snapshot.source.terrain_floor=3.0;snapshot.source.shape=WallShapeSnapshot::from_observed(shape.to_record());
            snapshot.source.curve_kind_tag=shape.tag();
            inner.new_wall_with_id(WallId(id),WallState::from_snapshot(snapshot,freehand));
        }
        let mut walls=PublicWalls::default();walls.consume_with_paths(&mut inner,&TerrainHeightGrid512 {
            encoded:vec![0;512*512].into(),minimum_y:0.0,maximum_y:0.0,sampling_fix:false},None,
            &WallPublicationDirty::default(),&WallSpatialHash::default());
        assert!(walls.get(WallId(42)).unwrap().geometry.metrics.minimum_relative_height>0.0);
        assert!(walls.get(WallId(43)).unwrap().geometry.metrics.minimum_relative_height>0.0);
        let mut world=World::new();world.insert_resource(RaycastWorld::new());world.insert_resource(walls);
        // 对侧表故意带不同几何，验收必须证明原分派，不能靠另一表为空掩盖倒置或fallback。
        world.insert_resource(ArchSegments(std::collections::HashMap::from([
            (WallId(42),vec![segment(0.0)]),(WallId(43),vec![segment(-100.0)])])));
        world.insert_resource(LintelSegments(std::collections::HashMap::from([
            (WallId(43),vec![segment(8.0)]),(WallId(42),vec![segment(100.0)])])));
        let a=world.spawn((volume(),InstancedWall::<WallBrickInstance>::new(42,Vec::new()))).id();
        let b=world.spawn((volume(),InstancedWall::<WallBrickInstance>::new(43,Vec::new()))).id();
        let mut schedule=Schedule::default();schedule.add_systems(update_rapier_colliders);(world,schedule,a,b)
    }
    fn mesh(world:&World,instance:ColliderMeshInstance) -> &RaycastMeshData {
        &world.resource::<RaycastWorld>().get_collider_mesh(instance.registry_index,instance.generation).data
    }
    fn wall_owned(world:&World,entity:Entity) -> [ColliderMeshInstance;3] {
        let world=world.resource::<RaycastWorld>();[world.wall_colliders[&entity],world.floor_colliders[&entity],world.arch_colliders[&entity]]
    }
    fn weak(world:&World,instance:ColliderMeshInstance) -> std::sync::Weak<RaycastMeshData> {
        Arc::downgrade(&world.resource::<RaycastWorld>().get_collider_mesh(instance.registry_index,instance.generation).data)
    }
    fn touch(world:&mut World,entity:Entity) { world.get_mut::<WallTriggerVolume>(entity).unwrap().set_changed(); }

    #[test]
    fn twelve_params_update_all_five_real_entity_maps_and_keep_64_clean_frames() {
        let (mut world,mut schedule,a,b)=setup();
        let roof_a=world.spawn((roof(42,0.0),roof_volume())).id();
        let roof_b=world.spawn((roof(43,1.0),roof_volume())).id();
        world.resource_mut::<PublicWalls>().associate_roof(WallId(42),roof_a);
        world.resource_mut::<PublicWalls>().associate_roof(WallId(43),roof_b);
        let clutter=world.spawn((Transform {translation:Vec3::new(1.0,2.0,3.0),rotation:Quat::from_rotation_y(0.7),scale:Vec3::splat(9.0)},
            AutoClutterConvertableToDecoClutter {source:AutoClutterSource::None,destination:AutoClutterDestination::Terrain,
                mesh_name:"real-collider-query",bounds:utils::geometry::rectangle::Rectangle3d::new(Vec3::ZERO,Vec3::splat(2.0),Vec2::X)})).id();
        schedule.run(&mut world);
        let first=wall_owned(&world,a);let second=wall_owned(&world,b);
        assert_eq!(world.resource::<RaycastWorld>().registry_live,9);
        for (entity,instances,tag,id) in [(a,first,0,42u64),(b,second,8,43)] {
            for (instance,expected_tag) in instances.into_iter().zip([tag,1,9]) {
                let owner=world.resource::<RaycastWorld>().get_collider_mesh(instance.registry_index,instance.generation);
                assert_eq!(owner.source.tag(),expected_tag);assert_eq!(&owner.source.0[8..16],&id.to_le_bytes());
            }
            assert!(!mesh(&world,instances[0]).uvs().is_empty());assert!(mesh(&world,instances[1]).uvs().is_empty());
            assert!(mesh(&world,instances[2]).uvs().is_empty());
            assert_eq!(mesh(&world,instances[2]).vertices()[0].x,if id==42 {0.0}else{8.0},"原typed参数栈身份决定Arch/Lintel表");
            assert_eq!(world.resource::<RaycastWorld>().wall_colliders[&entity],instances[0]);
        }
        assert_eq!(mesh(&world,first[0]).vertices()[0].z.to_bits(),(-2.0+f32::from_bits(0x3ea147ae)).to_bits());
        assert_eq!(mesh(&world,first[0]).vertices()[2].y.to_bits(),(7.0+f32::from_bits(0xbeeb851f)).to_bits());
        assert_eq!(mesh(&world,second[0]).vertices()[2].y,7.0);
        assert!(mesh(&world,first[1]).vertices().iter().all(|p|p.y==3.0));
        assert_eq!(mesh(&world,second[1]).vertices()[0],Vec3::new(8.0,3.0,0.0));
        let owner_weak=weak(&world,first[0]);
        for _ in 0..64 {schedule.run(&mut world);assert_eq!(wall_owned(&world,a),first);assert_eq!(wall_owned(&world,b),second);}
        assert_eq!(world.resource::<RaycastWorld>().registry_live,9);
        // 原只监听触发体Changed；纯Arch/Lintel资源变更不能偷偷提前重建。
        world.resource_mut::<ArchSegments>().0.get_mut(&WallId(42)).unwrap()[0].heights[0]=2.5;
        schedule.run(&mut world);assert_eq!(wall_owned(&world,a),first);
        world.get_mut::<WallTriggerVolume>(a).unwrap().quads[0].positions[0].x=-2.5;
        schedule.run(&mut world);let changed=wall_owned(&world,a);
        assert_ne!(changed[0],first[0]);assert_eq!(changed[1],first[1]);assert_ne!(changed[2],first[2]);assert!(owner_weak.upgrade().is_none());
        world.entity_mut(roof_a).insert(RoofMarkedForDeletion);world.entity_mut(clutter).remove::<AutoClutterConvertableToDecoClutter>();
        schedule.run(&mut world);assert!(!world.resource::<RaycastWorld>().roof_colliders.contains_key(&roof_a));
        assert!(world.resource::<RaycastWorld>().auto_clutter_colliders.is_empty());
        assert_eq!(world.resource::<RaycastWorld>().registry_live,7);
    }

    #[test]
    fn original_floor_and_wall_gates_keep_independent_tables_and_only_the_associated_roof() {
        let (mut world,mut schedule,a,b)=setup();
        let unrelated=world.spawn(roof(42,0.0)).id();schedule.run(&mut world);
        let initial=wall_owned(&world,a);assert_eq!(mesh(&world,initial[0]).vertices()[2].y,7.0);
        world.resource_mut::<PublicWalls>().associate_roof(WallId(42),unrelated);touch(&mut world,a);schedule.run(&mut world);
        let flat=wall_owned(&world,a);assert_ne!(flat[0],initial[0]);assert_eq!(flat[1],initial[1]);
        world.despawn(unrelated);touch(&mut world,a);schedule.run(&mut world);
        assert_eq!(mesh(&world,wall_owned(&world,a)[0]).vertices()[2].y,7.0,"失效关联不扫描其他Roof");
        for minimum in [0.0,-0.0,-1.0,f32::NAN] {
            world.resource_mut::<PublicWalls>().get_mut(WallId(42)).unwrap().geometry.metrics.minimum_relative_height=minimum;
            touch(&mut world,a);schedule.run(&mut world);let physics=world.resource::<RaycastWorld>();
            assert!(!physics.floor_colliders.contains_key(&a));assert!(physics.wall_colliders.contains_key(&a));assert!(physics.arch_colliders.contains_key(&a));
        }
        let mut shape=[0;28];shape[..4].copy_from_slice(&2u32.to_le_bytes());
        {let mut walls=world.resource_mut::<PublicWalls>();let wall=walls.get_mut(WallId(42)).unwrap();
            wall.geometry.metrics.minimum_relative_height=3.0;wall.authority.snapshot.source.shape=WallShapeSnapshot::from_observed(shape);}
        touch(&mut world,a);schedule.run(&mut world);assert!(!world.resource::<RaycastWorld>().floor_colliders.contains_key(&a));
        let primary=world.resource::<RaycastWorld>().wall_colliders[&a];assert_eq!(mesh(&world,primary).vertices()[0].z,-2.0);
        world.get_mut::<WallTriggerVolume>(a).unwrap().quads.clear();schedule.run(&mut world);
        assert!(!world.resource::<RaycastWorld>().wall_colliders.contains_key(&a));assert!(world.resource::<RaycastWorld>().arch_colliders.contains_key(&a));
        world.resource_mut::<ArchSegments>().0.remove(&WallId(42));touch(&mut world,a);schedule.run(&mut world);
        assert!(!world.resource::<RaycastWorld>().arch_colliders.contains_key(&a),"选定表缺失时移除，不能转读非空的另一表");
        world.resource_mut::<LintelSegments>().0.get_mut(&WallId(43)).unwrap()[0].heights.truncate(1);
        touch(&mut world,b);schedule.run(&mut world);assert!(!world.resource::<RaycastWorld>().arch_colliders.contains_key(&b),"原zip少于两点不生成mesh");
        world.resource_mut::<PublicWalls>().get_mut(WallId(43)).unwrap().set_being_bulldozed(true);
        touch(&mut world,b);schedule.run(&mut world);let physics=world.resource::<RaycastWorld>();
        assert!(!physics.wall_colliders.contains_key(&b));assert!(!physics.floor_colliders.contains_key(&b));assert!(!physics.arch_colliders.contains_key(&b));
    }

    #[test]
    fn required_missing_or_invalid_public_wall_panics_before_an_empty_trigger_can_skip_it() {
        for invalid in [false,true] {
            let (mut world,mut schedule,a,b)=setup();
            world.entity_mut(a).remove::<WallTriggerVolume>();world.entity_mut(b).remove::<WallTriggerVolume>();
            if invalid {world.resource_mut::<PublicWalls>().get_mut(WallId(42)).unwrap().authority.snapshot.source.curve.length=0.0;}
            world.spawn((WallTriggerVolume::default(),InstancedWall::<WallBrickInstance>::new(if invalid {42}else{999},Vec::new())));
            assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||schedule.run(&mut world))).is_err());
            assert_eq!(world.resource::<RaycastWorld>().registry_live,0);
        }
    }

    #[test]
    fn component_removal_generation_reuse_and_reload_release_every_original_map_owner() {
        let (mut world,mut schedule,a,b)=setup();schedule.run(&mut world);
        let first=wall_owned(&world,a);let weak_meshes=first.map(|instance|weak(&world,instance));
        world.entity_mut(a).remove::<WallTriggerVolume>();schedule.run(&mut world);
        assert!(weak_meshes.iter().all(|mesh|mesh.upgrade().is_none()));assert_eq!(world.resource::<RaycastWorld>().registry_live,3);
        world.despawn(a);let reused=world.spawn((volume(),InstancedWall::<WallBrickInstance>::new(42,Vec::new()))).id();
        assert_ne!(reused,a);schedule.run(&mut world);assert!(!world.resource::<RaycastWorld>().wall_colliders.contains_key(&a));
        let owners=wall_owned(&world,reused).into_iter().chain(wall_owned(&world,b)).map(|instance|weak(&world,instance)).collect::<Vec<_>>();
        world.resource_mut::<RaycastWorld>().on_level_reload();assert!(owners.iter().all(|mesh|mesh.upgrade().is_none()));
        {let physics=world.resource::<RaycastWorld>();assert_eq!(physics.registry_live,0);assert!(physics.wall_colliders.is_empty());
            assert!(physics.floor_colliders.is_empty());assert!(physics.arch_colliders.is_empty());assert!(physics.roof_colliders.is_empty());assert!(physics.auto_clutter_colliders.is_empty());}
        touch(&mut world,reused);touch(&mut world,b);schedule.run(&mut world);assert_eq!(world.resource::<RaycastWorld>().registry_live,6);
        world.despawn(reused);world.despawn(b);schedule.run(&mut world);world.resource_mut::<RaycastWorld>().flush_colliders();
        assert_eq!(world.resource::<RaycastWorld>().registry_live,0);
        assert!(world.resource::<RaycastWorld>().slots.iter().all(|slot|slot.colliders.is_empty()));
    }

    #[test]
    fn failed_primary_geometry_preserves_original_arch_before_primary_before_floor_side_effects() {
        let (mut world,mut schedule,a,_)=setup();schedule.run(&mut world);let before=wall_owned(&world,a);
        let old_arch=weak(&world,before[2]);let old_wall=weak(&world,before[0]);let old_floor=weak(&world,before[1]);
        world.resource_mut::<ArchSegments>().0.get_mut(&WallId(42)).unwrap()[0].heights[0]=3.5;
        world.get_mut::<WallTriggerVolume>(a).unwrap().quads[0].prepared_u=[0.5,0.5];
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||schedule.run(&mut world))).is_err());
        // 原先提交新的拱，再因所有quad退化失败；旧主面及尚未处理的地板不被清掉。
        let after=wall_owned(&world,a);assert_ne!(after[2],before[2]);assert_eq!(after[0],before[0]);assert_eq!(after[1],before[1]);
        assert!(old_arch.upgrade().is_none());assert!(old_wall.upgrade().is_some());assert!(old_floor.upgrade().is_some());
    }
}

#[cfg(all(test, feature = "native-physics"))]
mod floor_collider_geometry_tests {
    use super::*;
    use crate::systems::wall::valid_enclosure::Shape;
    use glam::Vec2;

    #[test]
    fn original_circle_sampling_count_keeps_saturating_nonfinite_and_signed_boundary_rules() {
        let fixture:serde_json::Value=serde_json::from_str(include_str!(
            "../../../../../tests/fixtures/native/m01-floor-mesh.json")).unwrap();
        let cases=fixture["count_cases"].as_array().unwrap();assert_eq!(cases.len(),256);
        for case in cases {
            let radius=f32::from_bits(case["radius"].as_u64().unwrap() as u32);
            assert_eq!(floor_circle_points_count(radius) as u64,case["expected"].as_u64().unwrap(),"radius{:08x}",radius.to_bits());
        }
    }

    #[test]
    fn complete_rectangle_and_circle_floor_geometry_matches_96_original_branch_outputs() {
        let fixture:serde_json::Value = serde_json::from_str(include_str!(
            "../../../../../tests/fixtures/native/m01-floor-mesh.json")).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            assert_eq!(case["panic"],false);
            let input:Vec<_> = case["input"].as_array().unwrap().iter().map(|v|f32::from_bits(v.as_u64().unwrap() as u32)).collect();
            let shape = if case["kind"] == 1 {
                Shape::Rectangle(utils::geometry::Rectangle2d {axis:Vec2::new(input[0],input[1]),
                    center:Vec2::new(input[2],input[3]),size:Vec2::new(input[4],input[5])})
            } else {
                Shape::Circle(utils::geometry::Circle2d {center:Vec2::new(input[0],input[1]),radius:input[2],rotation:input[3]})
            };
            let mesh = floor_mesh(shape,input[6]);
            let expected = |name:&str|case[name].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u32).collect::<Vec<_>>();
            assert_eq!(mesh.indices().iter().flatten().copied().collect::<Vec<_>>(),expected("indices"),"原floor索引case{}",case["id"]);
            assert_eq!(mesh.vertices().iter().flat_map(|point|point.to_array().map(f32::to_bits)).collect::<Vec<_>>(),expected("vertices"),"原floor顶点case{}",case["id"]);
            assert_eq!(mesh.normals().iter().flat_map(|point|point.to_array().map(f32::to_bits)).collect::<Vec<_>>(),expected("normals"),"原floor法线case{}",case["id"]);
            assert!(mesh.uvs().is_empty(),"原显式None，不能把projection_points伪接UV");
            assert!(mesh.shape.is_some());
        }
        assert_eq!(fixture["cases"].as_array().unwrap().len(),96);
    }
}

#[cfg(all(test, feature = "native-physics"))]
mod auto_clutter_collider_producer_tests {
    use super::*;
    use bevy_ecs::prelude::*;
    use crate::components::drawable::{AutoClutterConvertableToDecoClutter, AutoClutterDestination, AutoClutterSource};
    use crate::utils::components::transform::Transform;
    use utils::geometry::rectangle::Rectangle3d;
    use glam::{Quat, Vec2, Vec3};

    #[test]
    fn original_256_dimensions_and_center_match_native_bits_except_cross_arch_nan_payload() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../../tests/fixtures/native/m01-auto-clutter-collider.json")).unwrap();
        let mut exact = 0;
        for case in fixture["cases"].as_array().unwrap() {
            let b: Vec<_> = case["bounds"].as_array().unwrap().iter().map(|v|f32::from_bits(v.as_u64().unwrap() as u32)).collect();
            let mut bounds = Rectangle3d::new(Vec3::splat(77.0),Vec3::ONE,Vec2::Y);
            bounds.footprint.size = Vec2::new(b[0],b[1]);bounds.bottom = b[2];bounds.top = b[3];
            let (dimensions,center) = auto_clutter_collider_geometry(&bounds);
            for (actual,expected) in dimensions.to_array().into_iter().chain(center.to_array())
                .zip(case["expected"].as_array().unwrap()) {
                let expected = expected.as_u64().unwrap() as u32;
                if f32::from_bits(expected).is_nan() { assert!(actual.is_nan()); }
                else { assert_eq!(actual.to_bits(),expected,"case{}",case["id"]);exact+=1; }
            }
        }
        assert_eq!(fixture["cases"].as_array().unwrap().len(),256);assert!(exact>900);
    }

    #[test]
    fn real_transform_query_ignores_scale_and_bounds_pose_keeps_clean_frames_and_removes_exact_entity() {
        let mut world = World::new();world.insert_resource(RaycastWorld::new());
        let mut bounds = Rectangle3d::new(Vec3::new(66.0,11.0,-99.0),Vec3::new(2.0,4.0,6.0),Vec2::Y);
        bounds.bottom = -3.0;bounds.top = 5.0;
        let entity = world.spawn((Transform {translation:Vec3::new(1.0,3.0,4.0),rotation:Quat::from_rotation_z(core::f32::consts::FRAC_PI_2),
            scale:Vec3::new(-9.0,0.0,0.001)},AutoClutterConvertableToDecoClutter {source:AutoClutterSource::None,
                destination:AutoClutterDestination::Terrain,mesh_name:"source-field-is-not-collider-geometry",bounds})).id();
        let mut schedule = Schedule::default();schedule.add_systems(update_auto_clutter_rapier_colliders);
        schedule.run(&mut world);
        let first = world.resource::<RaycastWorld>().auto_clutter_colliders[&entity];
        let weak = Arc::downgrade(&world.resource::<RaycastWorld>().get_collider_mesh(first.registry_index,first.generation).data);
        {
            let physics = world.resource::<RaycastWorld>();
            assert_eq!(first.slot,11);
            let mesh = physics.get_collider_mesh(first.registry_index,first.generation);
            assert_eq!(mesh.source.0[0],12);assert_eq!(u64::from_le_bytes(mesh.source.0[8..16].try_into().unwrap()),entity.to_bits());
            let collider = physics.get_rapier_collider_ref(first.slot,first.collider).unwrap();
            assert_eq!(collider.shape().as_cuboid().unwrap().half_extents,Vector::new(1.0,4.0,3.0));
            // 原external*local，局部中心只有高度半值，且不消费Transform.scale。
            let expected = Vec3::new(1.0,3.0,4.0)+Quat::from_rotation_z(core::f32::consts::FRAC_PI_2)*Vec3::new(0.0,4.0,0.0);
            let actual = collider.position().translation.vector;
            assert!((Vec3::new(actual.x,actual.y,actual.z)-expected).length()<1e-5);
        }
        world.clear_trackers();schedule.run(&mut world);
        assert_eq!(world.resource::<RaycastWorld>().auto_clutter_colliders[&entity],first);
        world.get_mut::<AutoClutterConvertableToDecoClutter>(entity).unwrap().bounds.top = 7.0;
        schedule.run(&mut world);
        assert_eq!(world.resource::<RaycastWorld>().auto_clutter_colliders[&entity],first,"原Changed仅看Transform");
        world.get_mut::<Transform>(entity).unwrap().set_changed();schedule.run(&mut world);
        let replacement = world.resource::<RaycastWorld>().auto_clutter_colliders[&entity];
        assert_ne!(replacement,first);assert!(weak.upgrade().is_none());
        let weak = Arc::downgrade(&world.resource::<RaycastWorld>().get_collider_mesh(replacement.registry_index,replacement.generation).data);
        world.clear_trackers();world.entity_mut(entity).remove::<AutoClutterConvertableToDecoClutter>();schedule.run(&mut world);
        assert!(world.resource::<RaycastWorld>().auto_clutter_colliders.is_empty());assert!(weak.upgrade().is_none());
        world.resource_mut::<RaycastWorld>().flush_colliders();assert_eq!(world.resource::<RaycastWorld>().registry_live,0);
    }
}

#[cfg(all(test, feature = "native-physics"))]
mod roof_collider_producer_tests {
    use super::*;
    use bevy_ecs::prelude::*;
    use crate::resources::{roofs::{Roof, RoofTriggerVolume, RoofMarkedForDeletion, geometry::surface::Curve2},
        walls::{WallId, WallState, InnerWalls, public_walls::{PublicWalls, WallPublicationDirty, WallSpatialHash},
            publication_source::TerrainHeightGrid512}};
    use glam::{Vec2, Vec3};

    fn walls() -> PublicWalls {
        let mut inner = InnerWalls::new();
        for id in [42,43] {
            let curve = Curve2::try_new(vec![[-2.0,-2.0],[2.0,-2.0],[2.0,2.0],[-2.0,2.0]], true).unwrap();
            inner.new_wall_with_id(WallId(id), WallState::new(curve, 2.0, 0, 0, false));
        }
        let mut result = PublicWalls::default();
        result.consume_with_paths(&mut inner, &TerrainHeightGrid512 {encoded:vec![0;512*512].into(),
            minimum_y:0.0,maximum_y:0.0,sampling_fix:false}, None,
            &WallPublicationDirty::default(), &WallSpatialHash::default());
        result
    }
    fn roof(wall: u64, floor: f32, height: f32, gable: f32) -> Roof {
        let mut bytes = [0;88];bytes[..8].copy_from_slice(&wall.to_le_bytes());
        for (offset,value) in [(0x30,height),(0x34,gable),(0x4c,floor)] {
            bytes[offset..offset+4].copy_from_slice(&value.to_le_bytes());
        }
        Roof::from_observed(bytes)
    }
    fn volume() -> RoofTriggerVolume {
        let mut volume = RoofTriggerVolume::default();
        volume.set_triangles(vec![[Vec3::new(-1.0,0.0,-1.0),Vec3::new(-1.0,0.0,1.0),Vec3::new(1.0,0.0,-1.0)]]);
        volume.set_normals(vec![[Vec3::Y;3]]);volume.set_uvs(vec![[Vec2::ZERO,Vec2::Y,Vec2::X]]);volume
    }
    fn setup() -> (World, Schedule) {
        let mut world = World::new();world.insert_resource(RaycastWorld::new());world.insert_resource(walls());
        let mut schedule = Schedule::default();schedule.add_systems(update_roof_rapier_colliders);(world,schedule)
    }
    fn instance(world: &World, entity: Entity) -> ColliderMeshInstance {
        *world.resource::<RaycastWorld>().roof_collider_instance(entity).unwrap()
    }
    fn weak_mesh(world: &World, instance: ColliderMeshInstance) -> std::sync::Weak<RaycastMeshData> {
        Arc::downgrade(&world.resource::<RaycastWorld>().get_collider_mesh(instance.registry_index,instance.generation).data)
    }

    #[test]
    fn real_roof_query_keeps_unchanged_owner_replaces_floor_and_removes_added_deleted_or_trigger_removed() {
        let (mut world,mut schedule) = setup();
        let a = world.spawn((roof(42,3.0,0.0,0.0),volume())).id();
        let b = world.spawn((roof(43,7.0,0.8,1.0),volume())).id();
        schedule.run(&mut world);
        let first = instance(&world,a);let first_weak = weak_mesh(&world,first);
        let other = instance(&world,b);let other_weak = weak_mesh(&world,other);
        assert_eq!(first.slot,1);assert_eq!(other.slot,1);
        {
            let physics = world.resource::<RaycastWorld>();
            let mesh = physics.get_collider_mesh(first.registry_index,first.generation);
            assert_eq!(mesh.data.vertices()[0],Vec3::new(-1.0,3.0,-1.0));
            assert_eq!(mesh.data.normals(),&[Vec3::Y;3]);
            assert_eq!(mesh.data.uvs(),&[Vec2::ZERO,Vec2::Y,Vec2::X]);
            assert_eq!(&mesh.source.0[..3],&[2,1,0]);
            assert_eq!(&physics.get_collider_mesh(other.registry_index,other.generation).source.0[..3],&[2,0,1]);
            assert_eq!(physics.get_rapier_collider_ref(first.slot,first.collider).unwrap().position().translation.vector.y,0.0);
        }
        world.clear_trackers();schedule.run(&mut world);
        assert_eq!(instance(&world,a),first,"干净帧不重建网格、receipt或物理实例");
        world.get_mut::<Roof>(a).unwrap().observed_mut()[0x4c..0x50].copy_from_slice(&5.0f32.to_le_bytes());
        schedule.run(&mut world);let changed = instance(&world,a);
        assert_ne!(changed,first);assert!(first_weak.upgrade().is_none());
        assert_eq!(world.resource::<RaycastWorld>().get_collider_mesh(changed.registry_index,changed.generation).data.vertices()[0].y,5.0);
        let changed_weak = weak_mesh(&world,changed);
        world.clear_trackers();world.entity_mut(a).insert(RoofMarkedForDeletion);schedule.run(&mut world);
        assert!(world.resource::<RaycastWorld>().roof_collider_instance(a).is_none());
        assert!(changed_weak.upgrade().is_none());
        world.clear_trackers();world.entity_mut(b).remove::<RoofTriggerVolume>();schedule.run(&mut world);
        assert!(world.resource::<RaycastWorld>().roof_collider_instance(b).is_none());
        assert!(other_weak.upgrade().is_none());
        world.resource_mut::<RaycastWorld>().flush_colliders();
        assert_eq!(world.resource::<RaycastWorld>().registry_live,0);
    }

    #[test]
    fn roof_original_public_wall_gate_precedes_empty_geometry_and_reload_drops_entity_owners() {
        let (mut world,mut schedule) = setup();
        let missing = world.spawn((roof(99,0.0,0.0,0.0),RoofTriggerVolume::default())).id();
        schedule.run(&mut world);
        assert!(world.resource::<RaycastWorld>().roof_collider_instance(missing).is_none(),"缺真实墙时不读取空触发体");
        let a = world.spawn((roof(42,2.0,f32::NAN,1.0),volume())).id();schedule.run(&mut world);
        let old = instance(&world,a);let old_weak = weak_mesh(&world,old);
        assert_eq!(&world.resource::<RaycastWorld>().get_collider_mesh(old.registry_index,old.generation).source.0[..3],&[2,0,1],"原无序高度仍进入gable门");
        world.clear_trackers();world.resource_mut::<PublicWalls>().get_mut(WallId(42)).unwrap().set_being_bulldozed(true);
        world.get_mut::<Roof>(a).unwrap().set_changed();schedule.run(&mut world);
        assert!(world.resource::<RaycastWorld>().roof_collider_instance(a).is_none());assert!(old_weak.upgrade().is_none());
        world.clear_trackers();world.resource_mut::<PublicWalls>().get_mut(WallId(42)).unwrap().set_being_bulldozed(false);
        world.get_mut::<Roof>(a).unwrap().set_changed();schedule.run(&mut world);
        let live = instance(&world,a);let live_weak = weak_mesh(&world,live);
        world.resource_mut::<RaycastWorld>().on_level_reload();
        assert!(world.resource::<RaycastWorld>().roof_colliders.is_empty());assert!(live_weak.upgrade().is_none());
        assert!(world.resource::<RaycastWorld>().mesh_sources.meshes.is_empty());
        world.clear_trackers();world.get_mut::<RoofTriggerVolume>(a).unwrap().set_changed();schedule.run(&mut world);
        assert!(world.resource::<RaycastWorld>().roof_collider_instance(a).is_some(),"Reload重建写出的真实Changed触发体重新注册");
    }
}

#[cfg(all(test, feature = "native-physics"))]
mod transform_family_tests {
    use super::*;
    use glam::{Quat, Vec3};

    #[test]
    fn bounded_rollercoaster_rays_keep_parametric_limit_and_terrain_limit() {
        let mut world = RaycastWorld::new();
        let mut map = std::collections::HashMap::new();
        upsert_collider(&mut world, &mut map, 7_u32, RaycastGeoSource::Slot0,
            Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.0), Vec3::ZERO)),
            Vec3::new(5.0, 0.0, 0.0), Quat::IDENTITY);
        world.flush_colliders();
        let miss = world.raycast_with_max_toi(Vec3::ZERO, Vec3::X * 2.0, 1.99, 1);
        assert!(miss.is_miss());
        let hit = world.raycast_with_max_toi(Vec3::ZERO, Vec3::X * 2.0, 2.0, 1);
        assert!(hit.is_hit());
        assert_eq!(hit.toi, 2.0);
        assert_eq!(hit.position, Vec3::new(4.0, 0.0, 0.0));
        let terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
        let origin = Vec3::new(100.0, 3.0, 100.0);
        assert!(world.raycast_w_terrain_with_max_toi(&terrain, origin, Vec3::NEG_Y,
            2.99, 0x20).is_miss());
        let hit = world.raycast_w_terrain_with_max_toi(&terrain, origin, Vec3::NEG_Y,
            3.0, 0x20);
        assert!(hit.is_terrain());
        assert_eq!(hit.toi, 3.0);
    }

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
    fn mesh_data_clone_copies_four_owned_buffers_and_keeps_one_inner_shape() {
        let vertices = [Vec3::ZERO, Vec3::X, Vec3::Z];
        let indices = [[0, 1, 2]];
        let normals = [Vec3::Y; 3];
        let uvs = [glam::Vec2::ZERO, glam::Vec2::X, glam::Vec2::Y];
        let mut original =
            RaycastMeshData::new_indexed(&indices, &vertices, Some(&normals), Some(&uvs));
        original.set_translation_pose(Vec3::new(-2.0, 0.5, 3.0));
        let weak = Arc::downgrade(&original.shape.as_ref().unwrap().0);
        let cloned = original.clone();
        assert_eq!(cloned.vertices(), original.vertices());
        assert_eq!(cloned.indices(), original.indices());
        assert_eq!(cloned.normals(), original.normals());
        assert_eq!(cloned.uvs(), original.uvs());
        assert_ne!(cloned.vertices_ptr, original.vertices_ptr);
        assert_ne!(cloned.indices_ptr, original.indices_ptr);
        assert_ne!(cloned.normals_ptr, original.normals_ptr);
        assert_ne!(cloned.uvs_ptr, original.uvs_ptr);
        assert_eq!(cloned.hash, original.hash);
        assert_eq!(
            [cloned._pad0, cloned._pad1, cloned._pad2, cloned.scale],
            [
                original._pad0,
                original._pad1,
                original._pad2,
                original.scale
            ],
        );
        assert!(Arc::ptr_eq(
            &cloned.shape.as_ref().unwrap().0,
            &original.shape.as_ref().unwrap().0,
        ));
        assert_eq!(Arc::strong_count(&original.shape.as_ref().unwrap().0), 2);
        // 两份真实CPU缓冲独立；修改clone缓冲不会修改原条目。
        // SAFETY: clone独占已验证非空的Vec3副本；这里仅改它的第一个元素。
        unsafe {
            *(cloned.vertices_ptr as *mut Vec3) = Vec3::new(9.0, 8.0, 7.0);
        }
        assert_eq!(original.vertices()[0], Vec3::ZERO);
        drop(original);
        assert_eq!(cloned.vertices()[0], Vec3::new(9.0, 8.0, 7.0));
        assert_eq!(cloned.indices(), &indices);
        assert_eq!(cloned.normals(), &normals);
        assert_eq!(cloned.uvs(), &uvs);
        assert_eq!(Arc::strong_count(&cloned.shape.as_ref().unwrap().0), 1);
        drop(cloned);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn mesh_data_clone_preserves_none_empty_and_primitive_sentinels() {
        let vertices = [Vec3::ZERO, Vec3::X, Vec3::Z];
        for present in [false, true] {
            let original = RaycastMeshData::new_indexed(
                &[[0, 1, 2]],
                &vertices,
                present.then_some(&[][..]),
                present.then_some(&[][..]),
            );
            let cloned = original.clone();
            let expected = if present { 4 } else { 0 };
            assert_eq!((cloned.normals_ptr, cloned.uvs_ptr), (expected, expected));
            assert_eq!((cloned.normals_len, cloned.uvs_len), (0, 0));
            assert_eq!(cloned.hash, original.hash);
            drop(cloned);
            assert_eq!(original.vertices(), &vertices);
            assert_eq!(Arc::strong_count(&original.shape.as_ref().unwrap().0), 1);
        }
        let original =
            RaycastMeshData::new_cuboid_at(Vec3::new(1.3, 0.5, 1.3), Vec3::new(1.0, 2.0, 3.0));
        let collision = Arc::new(original.clone());
        let interaction = Arc::new(original);
        assert!(!Arc::ptr_eq(&collision, &interaction));
        assert!(Arc::ptr_eq(
            &collision.shape.as_ref().unwrap().0,
            &interaction.shape.as_ref().unwrap().0,
        ));
        assert_eq!((collision.vertices_ptr, collision.indices_ptr), (4, 4));
        assert_eq!((collision.normals_ptr, collision.uvs_ptr), (0, 0));
        assert_eq!(collision.local_transform(), interaction.local_transform());
        assert_eq!(collision.hash, interaction.hash);
        let empty = RaycastMeshData::default().clone();
        assert_eq!(
            (
                empty.vertices_ptr,
                empty.indices_ptr,
                empty.normals_ptr,
                empty.uvs_ptr
            ),
            (0, 0, 0, 0)
        );
        assert!(empty.shape.is_none());
    }

    #[test]
    fn compound_and_transform_adapters_retain_the_original_child_shape_arc() {
        let child = RaycastMeshData::new_cuboid_at(Vec3::splat(2.0), Vec3::X);
        let inner = Arc::clone(&child.shape.as_ref().unwrap().0);
        let compound =
            RaycastMeshData::new_compound(vec![child], [0, 0, 0, 0, 0, 0, 0, 1.0f32.to_bits()]);
        let shape = compound.shape_ref().unwrap().as_compound().unwrap();
        assert!(Arc::ptr_eq(&shape.shapes()[0].1.0, &inner));
        assert_eq!(shape.shapes()[0].0.translation.x, 1.0);
        let mesh = Arc::new(compound);
        let transformed = into_transformed_shape_iso(
            &utils::iso_transform::IsoTransform::identity(),
            &mesh,
            &utils::iso_transform::IsoTransform::identity(),
        );
        assert!(Arc::ptr_eq(
            &transformed.shape,
            &mesh.shape.as_ref().unwrap().0
        ));
        let mut world = RaycastWorld::new();
        let mut map = std::collections::HashMap::new();
        let handle = match upsert_collider(
            &mut world,
            &mut map,
            (),
            RaycastGeoSource::Slot0,
            Arc::clone(&mesh),
            Vec3::ZERO,
            Quat::IDENTITY,
        ) {
            ColliderUpsert::Inserted { handle } => handle,
            other => panic!("expected actual insertion, got {other:?}"),
        };
        let registered = world.into_transformed_shape(handle);
        assert!(Arc::ptr_eq(
            &registered.shape,
            &mesh.shape.as_ref().unwrap().0
        ));
        drop(registered);
        drop(transformed);
        drop(world);
        drop(mesh);
        assert_eq!(Arc::strong_count(&inner), 1);
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
                RaycastGeoSource::from_slot(slot).tag(),
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
        use crate::systems::collision::PendingMeshIntersection;
        assert_eq!(core::mem::size_of::<PendingMeshIntersection>(), 0x40);
        assert_eq!(
            core::mem::offset_of!(PendingMeshIntersection, polylines),
            0x00
        );
        assert_eq!(
            core::mem::offset_of!(PendingMeshIntersection, mesh_ref_a),
            0x18
        );
        assert_eq!(
            core::mem::offset_of!(PendingMeshIntersection, mesh_ref_b),
            0x20
        );
        assert_eq!(core::mem::offset_of!(PendingMeshIntersection, a), 0x28);
        assert_eq!(core::mem::offset_of!(PendingMeshIntersection, b), 0x34);
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
        use crate::systems::collision::PendingMeshIntersection;
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
        // Real registered sources in two actual subworlds; both Rapier indices may be 0.
        let vertices_a: Vec<_> = tri_a
            .vertices()
            .iter()
            .map(|p| Vec3::new(p.x, p.y, p.z))
            .collect();
        let vertices_b: Vec<_> = tri_b
            .vertices()
            .iter()
            .map(|p| Vec3::new(p.x, p.y, p.z))
            .collect();
        let data_a = Arc::new(RaycastMeshData::new_indexed(
            tri_a.indices(),
            &vertices_a,
            None,
            None,
        ));
        let data_b = Arc::new(RaycastMeshData::new_indexed(
            tri_b.indices(),
            &vertices_b,
            None,
            None,
        ));
        let mut source_a = RaycastGeoSource::Slot0;
        source_a.0[8..16].copy_from_slice(&11u64.to_le_bytes());
        let mut source_b = RaycastGeoSource::Slot2;
        source_b.0[8..16].copy_from_slice(&12u64.to_le_bytes());
        let mut map_a = std::collections::HashMap::new();
        let mut map_b = std::collections::HashMap::new();
        let ColliderUpsert::Inserted { handle: added } = upsert_collider(
            &mut world,
            &mut map_a,
            (),
            source_a,
            data_a,
            Vec3::ZERO,
            Quat::IDENTITY,
        ) else {
            panic!("fixture A must insert")
        };
        let ColliderUpsert::Inserted { handle: candidate } = upsert_collider(
            &mut world,
            &mut map_b,
            (),
            source_b,
            data_b,
            Vec3::ZERO,
            Quat::IDENTITY,
        ) else {
            panic!("fixture B must insert")
        };
        assert_eq!(added.index, candidate.index);
        assert_ne!(map_a[&()].registry_index, map_b[&()].registry_index);
        let mut processed = std::collections::HashSet::new();
        let mut artifacts: Vec<PendingMeshIntersection> = Vec::new();

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
        assert_eq!(artifacts[0].mesh_ref_a, 11);
        assert_eq!(artifacts[0].mesh_ref_b, 12);

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
        let registry_index = map[&7u64].registry_index;
        let before = world.collider_registry()[registry_index as usize].generation;
        let removed = remove_collider(&mut world, &mut map, &7u64).expect("present");
        assert_eq!(removed, handle);
        let entry = world.collider_registry()[registry_index as usize];
        assert_eq!(entry.generation, before + 1);
        assert_eq!(world.registry_free_head, registry_index);
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
        // Use the real registry producer; a physical collider without a receipt is invalid.
        let mut map = std::collections::HashMap::new();
        let result = upsert_collider(
            world,
            &mut map,
            (),
            RaycastGeoSource::from_slot(slot_index as u8),
            Arc::new(RaycastMeshData::new_cuboid_at(half * 2.0, glam::Vec3::ZERO)),
            center,
            glam::Quat::IDENTITY,
        );
        world.flush_colliders();
        match result {
            ColliderUpsert::Inserted { handle } => handle,
            other => panic!("fixture must insert: {other:?}"),
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
        // Cuboid/ball casts use the pinned dependency's iterative GJK solver.
        // Compare forwarding/slot selection to its actual result, not an assumed
        // analytic TOI tolerance that this adapter does not guarantee.
        let dependency_toi = |slot_index: usize, options: ShapeCastOptions| {
            let slot = &world.slots[slot_index];
            slot.pipeline
                .cast_shape(
                    &slot.bodies,
                    &slot.colliders,
                    &Isometry::identity(),
                    &Vector::x(),
                    &Ball::new(0.5),
                    options,
                    QueryFilter::default(),
                )
                .expect("isolated dependency cast should hit")
                .1
                .time_of_impact
        };
        let near_toi = dependency_toi(0, options);
        assert!(dependency_toi(1, options) > near_toi);
        let hit = world.spherecast_with_options(glam::Vec3::ZERO, glam::Vec3::X, 0.5, options, 3);
        assert!(hit.is_hit());
        assert_eq!(
            hit.toi, near_toi,
            "nearest dependency TOI was not preserved"
        );
        options.max_time_of_impact = 0.25;
        let miss = world.spherecast_with_options(glam::Vec3::ZERO, glam::Vec3::X, 0.5, options, 3);
        assert!(miss.is_miss());
        assert_eq!(miss.toi, 0.25);
        options.max_time_of_impact = 10.0;
        options.target_distance = 0.5;
        let expanded =
            world.spherecast_with_options(glam::Vec3::ZERO, glam::Vec3::X, 0.5, options, 3);
        assert!(expanded.is_hit());
        let expanded_toi = dependency_toi(0, options);
        assert!(
            expanded_toi < near_toi,
            "target distance should advance contact"
        );
        assert_eq!(
            expanded.toi, expanded_toi,
            "target_distance was not forwarded"
        );
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

#[cfg(all(test, feature = "native-physics"))]
mod clutter_physics_regressions {
    use super::*;
    use crate::resources::walls::WallId;
    use crate::resources::walls::decorator::DecoratorId;
    use crate::resources::walls::decorator_collision::DecoratorPhysicsColliders;
    use crate::resources::walls::decorator_storage::DecoratorStorageAddr;
    use glam::{Quat, Vec3};

    fn inserted(result: ColliderUpsert) -> SubworldColliderHandle {
        match result {
            ColliderUpsert::Inserted { handle } => handle,
            other => panic!("expected actual insertion, got {other:?}"),
        }
    }

    fn assert_vec_near(actual: Vec3, expected: Vec3) {
        assert!(
            (actual - expected).length() < 1e-5,
            "{actual:?} != {expected:?}"
        );
    }

    #[test]
    fn cuboid_dimensions_and_translation_reach_real_primitive() {
        let dimensions = Vec3::new(2.0, 4.0, 6.0);
        let center = Vec3::new(1.0, 2.0, 3.0);
        let mesh = RaycastMeshData::new_cuboid_at(dimensions, center);
        let bits = glam::UVec3::new(center.x.to_bits(), center.y.to_bits(), center.z.to_bits());
        let compatibility = RaycastMeshData::new_cuboid(dimensions, bits);
        assert_eq!(mesh.hash, compatibility.hash);
        assert_eq!(mesh.local_transform(), compatibility.local_transform());
        assert_eq!((mesh._pad1 >> 32) as u32, center.z.to_bits());
        let half = mesh
            .shape_ref()
            .unwrap()
            .as_cuboid()
            .expect("must remain a Cuboid")
            .half_extents;
        assert_eq!(half, Vector::new(1.0, 2.0, 3.0));
        let local = mesh.local_transform();
        let aabb = mesh
            .shape_ref()
            .unwrap()
            .compute_aabb(&collider_isometry(local.translation, local.rotation));
        assert_eq!(aabb.mins, Point::new(0.0, 0.0, 0.0));
        assert_eq!(aabb.maxs, Point::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn two_original_transform_adapters_keep_their_distinct_order() {
        let mesh = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.0), Vec3::X));
        let external = utils::iso_transform::IsoTransform::from_translation_rotation(
            Vec3::new(10.0, 0.0, 0.0),
            Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
        );
        let iso_shape = into_transformed_shape_iso(
            &external,
            &mesh,
            &utils::iso_transform::IsoTransform::identity(),
        );
        assert_vec_near(iso_shape.position, Vec3::new(11.0, 0.0, 0.0));
        let mut world = RaycastWorld::new();
        let mut map = std::collections::HashMap::new();
        let handle = inserted(upsert_collider(
            &mut world,
            &mut map,
            (),
            RaycastGeoSource::Slot0,
            mesh,
            external.translation,
            external.rotation,
        ));
        let tuple_shape = world.into_transformed_shape(handle);
        assert_vec_near(tuple_shape.position, Vec3::new(10.0, 0.0, -1.0));
    }

    #[test]
    fn shared_pose_tail_is_valid_for_sphere_and_real_compound_children() {
        let sphere = RaycastMeshData::new_sphere(
            1.0,
            glam::UVec3::new(0.0f32.to_bits(), 0.0f32.to_bits(), 3.0f32.to_bits()),
        );
        assert_eq!(
            sphere.local_transform().translation,
            Vec3::new(0.0, 0.0, 3.0)
        );
        assert_eq!(sphere.local_transform().rotation, Quat::IDENTITY);
        let child = RaycastMeshData::new_cuboid_at(Vec3::splat(2.0), Vec3::new(2.0, 0.0, 0.0));
        let compound =
            RaycastMeshData::new_compound(vec![child], [0, 0, 0, 0, 0, 0, 0, 1.0f32.to_bits()]);
        let aabb = compound
            .shape_ref()
            .unwrap()
            .compute_aabb(&Isometry::identity());
        assert_eq!(aabb.mins, Point::new(1.0, -1.0, -1.0));
        assert_eq!(aabb.maxs, Point::new(3.0, 1.0, 1.0));
        assert_eq!(compound.local_transform().rotation, Quat::IDENTITY);
    }

    #[test]
    fn real_same_key_upsert_checks_geometry_and_selected_source_fields() {
        let id = DecoratorId(17);
        let source = RaycastGeoSource::decorator(id, DecoratorStorageAddr::Wall(WallId(23)));
        let mesh = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.0), Vec3::Y));
        let mut world = RaycastWorld::new();
        let mut maps = DecoratorPhysicsColliders::default();
        let first = inserted(upsert_collider(
            &mut world,
            &mut maps.first,
            id,
            source,
            mesh.clone(),
            Vec3::ZERO,
            Quat::IDENTITY,
        ));
        let mut padding_changed = source;
        padding_changed.0[3] = 99;
        assert!(
            matches!(upsert_collider(&mut world, &mut maps.first, id, padding_changed, mesh.clone(), Vec3::ZERO, Quat::IDENTITY),
            ColliderUpsert::Kept { handle } if handle == first)
        );
        let changed_geometry = Arc::new(RaycastMeshData::new_cuboid_at(
            Vec3::new(4.0, 2.0, 2.0),
            Vec3::Y,
        ));
        assert!(matches!(
            upsert_collider(
                &mut world,
                &mut maps.first,
                id,
                source,
                changed_geometry.clone(),
                Vec3::ZERO,
                Quat::IDENTITY
            ),
            ColliderUpsert::Replaced { .. }
        ));
        let changed_address =
            RaycastGeoSource::decorator(id, DecoratorStorageAddr::Wall(WallId(24)));
        assert!(matches!(
            upsert_collider(
                &mut world,
                &mut maps.first,
                id,
                changed_address,
                changed_geometry,
                Vec3::ZERO,
                Quat::IDENTITY
            ),
            ColliderUpsert::Replaced { .. }
        ));
        // For non-Wall storage the unused payload lane is not an equality condition.
        let terrain = RaycastGeoSource::decorator(id, DecoratorStorageAddr::Terrain);
        let mut ignored_payload = terrain;
        ignored_payload.0[16..24].copy_from_slice(&99u64.to_le_bytes());
        assert!(terrain.same_native_source(ignored_payload));
    }

    #[test]
    fn dual_decorator_maps_share_world_but_keep_independent_real_receipts() {
        let id = DecoratorId(31);
        let storage = DecoratorStorageAddr::Terrain;
        let collision_source = RaycastGeoSource::decorator(id, storage);
        let interaction_source = RaycastGeoSource::decorator_interaction(id, storage, 0);
        let center = Vec3::new(0.0, 2.0, 0.0);
        let mut world = RaycastWorld::new();
        let mut maps = DecoratorPhysicsColliders::default();
        // Original set_colliders argument order is shrink first, full second.
        let first = inserted(upsert_collider(
            &mut world,
            &mut maps.first,
            id,
            collision_source,
            Arc::new(RaycastMeshData::new_cuboid_at(
                Vec3::new(1.5, 3.5, 5.5),
                center,
            )),
            Vec3::ZERO,
            Quat::IDENTITY,
        ));
        let second = inserted(upsert_collider(
            &mut world,
            &mut maps.second,
            id,
            interaction_source,
            Arc::new(RaycastMeshData::new_cuboid_at(
                Vec3::new(2.0, 4.0, 6.0),
                center,
            )),
            Vec3::ZERO,
            Quat::IDENTITY,
        ));
        assert_eq!(first.index, 0);
        assert_eq!(second.index, 0);
        assert_ne!(first.slot, second.slot);
        assert_ne!(
            maps.first[&id].registry_index,
            maps.second[&id].registry_index
        );
        assert_eq!(
            world
                .get_collider_mesh(maps.first[&id].registry_index, maps.first[&id].generation)
                .source,
            collision_source
        );
        assert_eq!(
            world
                .get_collider_mesh(maps.second[&id].registry_index, maps.second[&id].generation)
                .source,
            interaction_source
        );
        world.flush_colliders();
        let ray_origin = Vec3::new(0.0, 10.0, 0.0);
        let first_hit = world.raycast(ray_origin, Vec3::NEG_Y, 1 << first.slot);
        let second_hit = world.raycast(ray_origin, Vec3::NEG_Y, 1 << second.slot);
        assert_eq!(first_hit.toi, 6.25);
        assert_eq!(second_hit.toi, 6.0);
        let source_words = |source: RaycastGeoSource| -> [u64; 3] {
            std::array::from_fn(|i| {
                u64::from_le_bytes(source.0[i * 8..i * 8 + 8].try_into().unwrap())
            })
        };
        assert_eq!(first_hit.handle, source_words(collision_source));
        assert_eq!(second_hit.handle, source_words(interaction_source));
        let mut sources = Vec::new();
        world.iter_shape_overlaps_handle(
            second,
            (1 << first.slot) | (1 << second.slot),
            |_, source| {
                sources.push(*source);
                false
            },
        );
        assert!(sources.contains(&collision_source) && sources.contains(&interaction_source));
        remove_collider(&mut world, &mut maps.first, &id).expect("first present");
        world.flush_colliders();
        assert!(
            world
                .raycast(ray_origin, Vec3::NEG_Y, 1 << first.slot)
                .is_miss()
        );
        assert_eq!(
            world
                .raycast(ray_origin, Vec3::NEG_Y, 1 << second.slot)
                .handle,
            source_words(interaction_source)
        );
    }

    #[test]
    fn explicit_receipts_stay_strict_global_raycast_normalizes_even_and_retirement_releases_owner() {
        let mut world = RaycastWorld::new();
        let mut map = std::collections::HashMap::new();
        let mesh = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.0), Vec3::ZERO));
        let weak = Arc::downgrade(&mesh);
        inserted(upsert_collider(
            &mut world,
            &mut map,
            (),
            RaycastGeoSource::Slot0,
            mesh,
            Vec3::ZERO,
            Quat::IDENTITY,
        ));
        let old = map[&()];
        let fails = |world: &RaycastWorld, generation| {
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                world.get_collider_mesh(old.registry_index, generation);
            }))
            .is_err()
        };
        assert!(fails(&world, old.generation & !1));
        assert!(fails(&world, old.generation.wrapping_add(2)));
        world.flush_colliders();
        let good_raw = (u64::from(old.generation) << 32) | u64::from(old.registry_index);
        world.slots[old.slot as usize]
            .colliders
            .get_mut(old.collider)
            .unwrap()
            .user_data =
            u128::from((u64::from(old.generation & !1) << 32) | u64::from(old.registry_index));
        assert!(world.raycast(Vec3::new(0.0, 3.0, 0.0), Vec3::NEG_Y, 1).is_hit(),
            "原global predicate和winner都读取raw generation|1，显式receipt getter仍严格");
        world.slots[old.slot as usize]
            .colliders
            .get_mut(old.collider)
            .unwrap()
            .user_data = u128::from(good_raw);
        remove_collider(&mut world, &mut map, &()).expect("present");
        assert!(weak.upgrade().is_none());
        assert!(fails(&world, old.generation));
        let mesh = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.0), Vec3::ZERO));
        let new_weak = Arc::downgrade(&mesh);
        inserted(upsert_collider(
            &mut world,
            &mut map,
            (),
            RaycastGeoSource::Slot0,
            mesh,
            Vec3::ZERO,
            Quat::IDENTITY,
        ));
        let new = map[&()];
        assert_eq!(new.registry_index, old.registry_index);
        assert_ne!(new.generation, old.generation);
        assert!(fails(&world, old.generation));
        world.on_level_reload();
        assert!(new_weak.upgrade().is_none());
    }

    #[test]
    fn physical_collider_without_registered_receipt_fails_at_actual_hit() {
        let mut world = RaycastWorld::new();
        let slot = &mut world.slots[0];
        slot.colliders
            .insert(ColliderBuilder::cuboid(1.0, 1.0, 1.0).build());
        slot.pipeline.update(&slot.colliders);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                world.raycast(Vec3::new(0.0, 3.0, 0.0), Vec3::NEG_Y, 1);
            }))
            .is_err()
        );
    }
}


#[cfg(all(test, feature = "native-physics"))]
mod decorator_overlap_regressions {
    use super::*;
    use crate::resources::walls::WallId;
    use crate::resources::walls::decorator::DecoratorId;
    use crate::resources::walls::decorator_storage::DecoratorStorageAddr;
    use glam::{Quat, Vec3};
    use std::collections::HashMap;

    fn insert_source(
        world: &mut RaycastWorld,
        map: &mut HashMap<DecoratorId, ColliderMeshInstance>,
        id: DecoratorId,
        source: RaycastGeoSource,
        position: Vec3,
        dimensions: Vec3,
    ) -> ColliderMeshInstance {
        assert!(matches!(
            upsert_collider(
                world,
                map,
                id,
                source,
                Arc::new(RaycastMeshData::new_cuboid_at(dimensions, Vec3::ZERO)),
                position,
                Quat::IDENTITY,
            ),
            ColliderUpsert::Inserted { .. }
        ));
        map[&id]
    }

    fn query(position: Vec3) -> OffsetColliderTransform {
        OffsetColliderTransform {
            position,
            rotation: Quat::IDENTITY,
            shape: SharedShape::ball(0.25).0,
        }
    }

    #[test]
    fn rejects_self_other_wall_and_nonwall_then_returns_canonical_source() {
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        let current = DecoratorId(50);
        let wall = WallId(9);
        let query = query(Vec3::ZERO);
        assert!(world.decorator_overlaps_with_others(&query, current, wall, false).is_none());
        for (id, storage) in [
            (current, DecoratorStorageAddr::Wall(wall)),
            (DecoratorId(1), DecoratorStorageAddr::Wall(WallId(10))),
            (DecoratorId(2), DecoratorStorageAddr::Terrain),
            (DecoratorId(3), DecoratorStorageAddr::Stair),
        ] {
            insert_source(&mut world, &mut map, id, RaycastGeoSource::decorator(id, storage), Vec3::ZERO, Vec3::splat(2.0));
        }
        insert_source(&mut world, &mut map, DecoratorId(99), RaycastGeoSource::Slot0, Vec3::ZERO, Vec3::splat(2.0));
        world.flush_colliders();
        assert!(world.decorator_overlaps_with_others(&query, current, wall, false).is_none());
        assert!(world.decorator_overlaps_with_others(&query, current, wall, true).is_none());

        let id = DecoratorId(22);
        let expected = RaycastGeoSource::decorator(id, DecoratorStorageAddr::Wall(wall));
        let live = insert_source(&mut world, &mut map, id, expected, Vec3::ZERO, Vec3::splat(2.0));
        world.flush_colliders();
        let hit = world.decorator_overlaps_with_others(&query, current, wall, false).unwrap();
        assert_eq!(*hit, expected);
        assert!(std::ptr::eq(hit, &world.get_collider_mesh(live.registry_index, live.generation).source));
    }

    #[test]
    fn ordered_mode_uses_unsigned_id_comparison() {
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        let wall = WallId(9);
        let query = query(Vec3::ZERO);
        let larger = DecoratorId(u32::MAX);
        let expected = RaycastGeoSource::decorator(larger, DecoratorStorageAddr::Wall(wall));
        insert_source(&mut world, &mut map, larger, expected, Vec3::ZERO, Vec3::splat(2.0));
        world.flush_colliders();
        assert_eq!(world.decorator_overlaps_with_others(&query, DecoratorId(9), wall, false), Some(&expected));
        assert!(world.decorator_overlaps_with_others(&query, DecoratorId(9), wall, true).is_none());
        assert!(remove_collider(&mut world, &mut map, &larger).is_some());
        let smaller = DecoratorId(7);
        let expected = RaycastGeoSource::decorator(smaller, DecoratorStorageAddr::Wall(wall));
        insert_source(&mut world, &mut map, smaller, expected, Vec3::ZERO, Vec3::splat(2.0));
        world.flush_colliders();
        assert_eq!(world.decorator_overlaps_with_others(&query, DecoratorId(0x8000_0000), wall, true), Some(&expected));
    }

    #[test]
    fn overlap_query_preserves_translation_and_rotation() {
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        let wall = WallId(9);
        let id = DecoratorId(1);
        let center = Vec3::new(8.0, 2.0, 3.0);
        let expected = RaycastGeoSource::decorator(id, DecoratorStorageAddr::Wall(wall));
        insert_source(&mut world, &mut map, id, expected, center + Vec3::Z * 1.5, Vec3::splat(0.2));
        world.flush_colliders();
        let mut query = OffsetColliderTransform {
            position: center,
            rotation: Quat::IDENTITY,
            shape: SharedShape::cuboid(2.0, 0.2, 0.2).0,
        };
        assert!(world.decorator_overlaps_with_others(&query, DecoratorId(20), wall, false).is_none());
        query.rotation = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
        assert_eq!(world.decorator_overlaps_with_others(&query, DecoratorId(20), wall, false), Some(&expected));
        query.position = Vec3::ZERO;
        assert!(world.decorator_overlaps_with_others(&query, DecoratorId(20), wall, false).is_none());
    }

    #[test]
    fn removed_and_reused_receipts_return_only_the_new_live_owner() {
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        let wall = WallId(9);
        let id = DecoratorId(1);
        let query = query(Vec3::ZERO);
        let source = RaycastGeoSource::decorator(id, DecoratorStorageAddr::Wall(wall));
        let old = insert_source(&mut world, &mut map, id, source, Vec3::ZERO, Vec3::splat(2.0));
        world.flush_colliders();
        assert_eq!(world.decorator_overlaps_with_others(&query, DecoratorId(20), wall, false), Some(&source));
        assert!(remove_collider(&mut world, &mut map, &id).is_some());
        world.flush_colliders();
        assert!(world.decorator_overlaps_with_others(&query, DecoratorId(20), wall, false).is_none());
        let next = DecoratorId(2);
        let expected = RaycastGeoSource::decorator(next, DecoratorStorageAddr::Wall(wall));
        let live = insert_source(&mut world, &mut map, next, expected, Vec3::ZERO, Vec3::splat(2.0));
        assert_ne!((old.registry_index, old.generation), (live.registry_index, live.generation));
        world.flush_colliders();
        let hit = world.decorator_overlaps_with_others(&query, DecoratorId(20), wall, false).unwrap();
        assert_eq!(*hit, expected);
        assert!(std::ptr::eq(hit, &world.get_collider_mesh(live.registry_index, live.generation).source));
    }

    #[test]
    fn live_query_rejects_stale_receipts_and_unexpected_source_kind() {
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        let wall = WallId(9);
        let id = DecoratorId(1);
        let source = RaycastGeoSource::decorator(id, DecoratorStorageAddr::Wall(wall));
        let live = insert_source(&mut world, &mut map, id, source, Vec3::ZERO, Vec3::splat(2.0));
        world.flush_colliders();
        let query = query(Vec3::ZERO);
        let original = (u64::from(live.generation) << 32) | u64::from(live.registry_index);
        for generation in [live.generation & !1, live.generation.wrapping_add(2)] {
            world.get_rapier_collider(live.slot, live.collider).unwrap().user_data =
                u128::from((u64::from(generation) << 32) | u64::from(live.registry_index));
            assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                world.decorator_overlaps_with_others(&query, DecoratorId(20), wall, false);
            })).is_err());
        }
        world.get_rapier_collider(live.slot, live.collider).unwrap().user_data = u128::from(original);
        world.mesh_sources.meshes.get_mut(strict_mesh_key(original)).unwrap().source = RaycastGeoSource::Slot0;
        world.colliders[live.registry_index as usize].set_kind(0);
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            world.decorator_overlaps_with_others(&query, DecoratorId(20), wall, false);
        })).is_err());
    }
}


#[cfg(all(test, feature = "native-physics"))]
mod bounded_ray_regressions {
    use super::*;
    use glam::{Quat, Vec3};
    use std::collections::HashMap;

    fn insert_box(
        world: &mut RaycastWorld,
        map: &mut HashMap<u32, ColliderMeshInstance>,
        key: u32,
        source: RaycastGeoSource,
        center: Vec3,
    ) {
        assert!(matches!(
            upsert_collider(
                world,
                map,
                key,
                source,
                Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
                center,
                Quat::IDENTITY,
            ),
            ColliderUpsert::Inserted { .. }
        ));
    }

    #[test]
    fn finite_bound_ignores_geometry_beyond_endpoint() {
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        insert_box(&mut world, &mut map, 1, RaycastGeoSource::Slot0, Vec3::Z * 5.0);
        world.flush_colliders();
        assert!(world.intersects_any(Vec3::ZERO, Vec3::Z, 1));
        assert!(!world.intersects_any_with_max_toi(Vec3::ZERO, Vec3::Z, 4.49, 1));
        assert!(world.intersects_any_with_max_toi(Vec3::ZERO, Vec3::Z, 4.5, 1));
    }

    #[test]
    fn finite_bound_and_mask_reach_last_subworld() {
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        insert_box(&mut world, &mut map, 1, RaycastGeoSource::Slot0, Vec3::Z);
        insert_box(&mut world, &mut map, 2, RaycastGeoSource::Slot16, Vec3::Z * 5.0);
        world.flush_colliders();
        let last = 1 << 16;
        assert!(!world.intersects_any_with_max_toi(Vec3::ZERO, Vec3::Z, 2.0, last));
        assert!(world.intersects_any_with_max_toi(Vec3::ZERO, Vec3::Z, 10.0, last));
        assert!(!world.intersects_any_with_max_toi(Vec3::ZERO, Vec3::Z, 10.0, 0));
        assert!(!world.intersects_any_with_max_toi(Vec3::ZERO, Vec3::Z, 10.0, 1 << 31));
    }

    #[test]
    fn ray_direction_keeps_its_parametric_scale() {
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        insert_box(&mut world, &mut map, 1, RaycastGeoSource::Slot0, Vec3::Z * 4.0);
        world.flush_colliders();
        let direction = Vec3::Z * 2.0;
        assert!(!world.intersects_any_with_max_toi(Vec3::ZERO, direction, 1.5, 1));
        assert!(world.intersects_any_with_max_toi(Vec3::ZERO, direction, 1.75, 1));
        assert!(world.intersects_any(Vec3::ZERO, direction, 1));
    }

    #[test]
    fn solid_ray_starts_inside_at_zero_toi() {
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        insert_box(&mut world, &mut map, 1, RaycastGeoSource::Slot0, Vec3::ZERO);
        world.flush_colliders();
        assert!(world.intersects_any_with_max_toi(Vec3::ZERO, Vec3::Z, 0.0, 1));
    }

    #[test]
    fn terrain_aware_ray_uses_the_same_finite_bound_and_mask() {
        let world = RaycastWorld::new();
        let mut terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
        terrain.set_height_range(-2.5, -2.5);
        let origin = Vec3::Y * 10.0;
        let direction = -Vec3::Y * 10.0;
        // 地面在t=1.25；有限线段到t=1时不应把后面的地形算作遮挡。
        assert!(!world.intersects_any_w_terrain(&terrain, origin, direction, 1.0, 0x20));
        assert!(world.intersects_any_w_terrain(&terrain, origin, direction, 2.0, 0x20));
        assert!(!world.intersects_any_w_terrain(&terrain, origin, direction, 2.0, 0));
    }

    #[test]
    fn terrain_aware_ray_keeps_the_live_collider_owner_and_bound() {
        let mut world = RaycastWorld::new();
        let mut instances = HashMap::new();
        insert_box(&mut world, &mut instances, 1, RaycastGeoSource::Slot0, Vec3::Y * 15.0);
        world.flush_colliders();
        let mut terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
        terrain.set_height_range(-2.5, -2.5);
        let origin = Vec3::ZERO;
        let direction = Vec3::Y * 10.0;
        assert!(world.intersects_any(origin, direction, 1));
        assert!(!world.intersects_any_w_terrain(&terrain, origin, direction, 1.0, 0x21));
        assert!(world.intersects_any_w_terrain(&terrain, origin, direction, 2.0, 0x21));
        assert!(!world.intersects_any_w_terrain(&terrain, origin, direction, 2.0, 0x20));
    }
}


#[cfg(all(test, feature = "native-physics"))]
mod finite_simple_ray_regressions {
    use super::*;
    use glam::{Quat, Vec3};
    use std::collections::HashMap;

    fn insert_box(
        world: &mut RaycastWorld,
        map: &mut HashMap<u32, ColliderMeshInstance>,
        key: u32,
        source: RaycastGeoSource,
        center: Vec3,
    ) {
        assert!(matches!(
            upsert_collider(
                world,
                map,
                key,
                source,
                Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
                center,
                Quat::IDENTITY,
            ),
            ColliderUpsert::Inserted { .. }
        ));
    }

    fn assert_source(hit: &RaycastSimpleResult, expected: RaycastGeoSource) {
        assert!(!hit.is_miss());
        let words = std::array::from_fn(|i| {
            u64::from_le_bytes(expected.0[i * 8..i * 8 + 8].try_into().unwrap())
        });
        assert_eq!(hit.handle, words);
    }

    fn original_legacy_evidence() -> serde_json::Value {
        let value: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"), "/../../reports/module-recovery-2026-10-23/m01-legacy-wall-ray-native.json"
        ))).unwrap();
        assert_eq!(value["binary_sha256"], "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03");
        value
    }
    fn source_with_wall(kind: u8, wall: crate::resources::walls::WallId) -> RaycastGeoSource {
        let mut bytes = [0;24]; bytes[0] = kind;
        bytes[8..16].copy_from_slice(&wall.0.to_le_bytes());
        crate::systems::collision::query_policy::RaycastGeoSourceProjection(bytes)
    }
    fn assert_full_source(hit: &RaycastWorldResult, expected: RaycastGeoSource) {
        assert!(hit.is_hit());
        assert_eq!(hit.handle, std::array::from_fn(|i|u64::from_le_bytes(expected.0[i*8..i*8+8].try_into().unwrap())));
    }

    #[test]
    fn legacy_wall_exclusion_matches_original_full_predicate_for_all_source_kinds() {
        use crate::resources::walls::WallId;
        let evidence = original_legacy_evidence();
        let rows = evidence["rows"].as_array().unwrap();
        assert_eq!(rows.len(),144);
        for row in rows.iter().filter(|row| row["query"] == "full" && row["panic"].is_null()) {
            let mut world = RaycastWorld::new(); let mut instances = HashMap::new();
            let own = source_with_wall(row["kind"].as_u64().unwrap() as u8, WallId(row["source_wall"].as_u64().unwrap()));
            let other = source_with_wall(own.0[0], WallId(row["source_wall"].as_u64().unwrap() ^ 1));
            insert_box(&mut world,&mut instances,1,own,Vec3::Y*5.0);
            insert_box(&mut world,&mut instances,2,other,Vec3::Y);
            world.flush_colliders();
            let additional = 1u32 << own.subworld();
            let direction = Vec3::new(0.0,-1.0,-0.0);
            let hit = if row["option_tag"] == 1 {
                world.legacy_raycast_w_params_excluding_wall_id(Vec3::Y*10.0,direction,additional,WallId(row["excluded_wall"].as_u64().unwrap()))
            } else { world.legacy_raycast_w_params(Vec3::Y*10.0,direction,additional) };
            let own_allowed = row["allowed"].as_bool().unwrap();
            assert_full_source(&hit,if own_allowed { own } else { other });
            assert_eq!(hit.toi,if own_allowed { 4.5 } else { 8.5 });
            assert_eq!(hit.position.y,if own_allowed { 5.5 } else { 1.5 });
            assert_eq!(hit.normal,Vec3::Y,"完整射线必须返回实际Rapier法线，不能以simple结果补零");
        }
        // 原完整0x105保留kind1，原simple0x107排kind1；两个路径确实不同。
        let mut world=RaycastWorld::new(); let mut instances=HashMap::new();
        let wall=WallId(77);let own=source_with_wall(1,wall);let other=source_with_wall(1,WallId(88));
        insert_box(&mut world,&mut instances,1,own,Vec3::Y*5.0);
        insert_box(&mut world,&mut instances,2,other,Vec3::Y);world.flush_colliders();
        let mask=1<<own.subworld();
        assert_full_source(&world.legacy_raycast_w_params_excluding_wall_id(Vec3::Y*10.0,-Vec3::Y,mask,wall),own);
        assert_source(&world.raycast_simple_excluding_wall_id(Vec3::Y*10.0,-Vec3::Y,mask,wall),other);
    }

    #[test]
    fn legacy_wrapper_uses_original_base_mask_and_extra_bits_in_real_rapier_world() {
        use crate::resources::walls::WallId;
        let evidence=original_legacy_evidence();let rows=evidence["wrappers"].as_array().unwrap();assert_eq!(rows.len(),24);
        let mut world=RaycastWorld::new();let mut instances=HashMap::new();
        let near=RaycastGeoSource::from_slot(3);let far=RaycastGeoSource::from_slot(7);
        insert_box(&mut world,&mut instances,1,near,Vec3::Z*2.0);
        insert_box(&mut world,&mut instances,2,far,Vec3::Z*8.0);world.flush_colliders();
        for row in rows {
            let input=&row["input"];let mask=row["callee"]["mask"].as_u64().unwrap() as u32;
            assert_eq!(row["callee"]["max_toi_bits"].as_u64().unwrap() as u32,f32::MAX.to_bits());
            assert_eq!(row["legacy_miss_bits"],2);
            let extra=input["additional_mask"].as_u64().unwrap() as u32;
            let hit=if input["option_tag"]==1 {
                world.legacy_raycast_w_params_excluding_wall_id(Vec3::ZERO,Vec3::Z,extra,WallId(input["wall"].as_u64().unwrap()))
            } else { world.legacy_raycast_w_params(Vec3::ZERO,Vec3::Z,extra) };
            assert_full_source(&hit,if mask & (1<<3)!=0 { near } else { far });
            assert_eq!(hit.toi,if mask & (1<<3)!=0 { 1.5 } else { 7.5 });
        }
    }

    #[test]
    fn legacy_filtered_ray_preserves_nearest_bound_equal_later_slot_and_miss_tag() {
        use crate::resources::walls::WallId;
        let wall=WallId(7);let mut world=RaycastWorld::new();let mut instances=HashMap::new();
        let excluded=source_with_wall(0,wall);
        let other=source_with_wall(2,WallId(8));let tie=RaycastGeoSource::from_slot(4);let last=RaycastGeoSource::from_slot(16);
        insert_box(&mut world,&mut instances,1,excluded,Vec3::Y*8.0);
        insert_box(&mut world,&mut instances,2,other,Vec3::Y*2.0);
        insert_box(&mut world,&mut instances,3,tie,Vec3::Y*2.0);
        insert_box(&mut world,&mut instances,4,last,Vec3::Y);world.flush_colliders();
        let cast=|world:&RaycastWorld,extra|world.legacy_raycast_w_params_excluding_wall_id(Vec3::Y*10.0,Vec3::new(0.0,-1.0,-0.0),extra,wall);
        assert_full_source(&cast(&world,0),tie);
        assert_full_source(&cast(&world,1<<16),tie); // 后槽更远，不能覆盖之前的上限。
        remove_collider(&mut world,&mut instances,&4).unwrap();
        insert_box(&mut world,&mut instances,5,last,Vec3::Y*2.0);world.flush_colliders();
        let hit=cast(&world,1<<16);assert_full_source(&hit,last);assert_eq!(hit.toi,7.5);
        for id in [2,3,5] {remove_collider(&mut world,&mut instances,&id).unwrap();}
        world.flush_colliders();let miss=cast(&world,1<<16);
        assert!(miss.is_miss());assert_eq!(miss.toi.to_bits(),2);
    }

    #[test]
    fn legacy_predicate_validates_receipt_before_wall_rejection_and_obeys_slot_mask() {
        use crate::resources::walls::WallId;
        let wall=WallId(7);let mut world=RaycastWorld::new();let mut instances=HashMap::new();
        insert_box(&mut world,&mut instances,1,source_with_wall(0,wall),Vec3::Y*5.0);world.flush_colliders();
        let live=instances[&1];
        let original=(u64::from(live.generation)<<32)|u64::from(live.registry_index);
        world.get_rapier_collider(live.slot,live.collider).unwrap().user_data=u128::from(original^(1<<32));
        assert!(world.legacy_raycast_w_params_excluding_wall_id(Vec3::Y*10.0,-Vec3::Y,0,wall).is_miss(),
            "原完整predicate先OR1验证成功，再排除同墙；不能沿用显式receipt的偶数拒绝断言");
        for raw in [original.wrapping_add(2<<32), (u64::from(live.generation)<<32)|u64::from(u32::MAX)] {
            world.get_rapier_collider(live.slot,live.collider).unwrap().user_data=u128::from(raw);
            assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                world.legacy_raycast_w_params_excluding_wall_id(Vec3::Y*10.0,-Vec3::Y,0,wall))).is_err());
        }
        world.get_rapier_collider(live.slot,live.collider).unwrap().user_data=u128::from(original);
        remove_collider(&mut world,&mut instances,&1).unwrap();
        insert_box(&mut world,&mut instances,2,source_with_wall(1,wall),Vec3::Y*5.0);world.flush_colliders();
        let live=instances[&2];
        world.get_rapier_collider(live.slot,live.collider).unwrap().user_data=u128::from((u64::from(live.generation.wrapping_add(2))<<32)|u64::from(live.registry_index));
        assert!(world.legacy_raycast_w_params_excluding_wall_id(Vec3::Y*10.0,-Vec3::Y,0,wall).is_miss());
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
            world.legacy_raycast_w_params_excluding_wall_id(Vec3::Y*10.0,-Vec3::Y,1<<9,wall))).is_err());
    }

    #[test]
    fn original_simple_ray_predicate_excludes_all_own_wall_kinds() {
        use crate::resources::walls::WallId;
        let source = |kind, wall: WallId| {
            let mut bytes = [0; 24];
            bytes[0] = kind;
            bytes[8..16].copy_from_slice(&wall.0.to_le_bytes());
            crate::systems::collision::query_policy::RaycastGeoSourceProjection(bytes)
        };
        let wall = WallId(0x1234_5678_9abc_def0);
        let other = WallId(0xfedc_ba98_7654_3210);
        // 原predicate常量0x107只包含0/1/2/8，不能按subworld或其他类型推断墙。
        for kind in 0u8..17 {
            let mut world = RaycastWorld::new();
            let mut instances = HashMap::new();
            let own = source(kind, wall);
            let target = source(kind, other);
            insert_box(&mut world, &mut instances, 1, own, Vec3::Y * 5.0);
            insert_box(&mut world, &mut instances, 2, target, Vec3::Y);
            world.flush_colliders();
            let mask = 1 << own.subworld();
            let ordinary = world.raycast_simple(Vec3::Y * 10.0, -Vec3::Y, mask);
            assert_source(&ordinary, own);
            let filtered = world.raycast_simple_excluding_wall_id(
                Vec3::Y * 10.0, -Vec3::Y, mask, wall,
            );
            assert_eq!(world.raycast_simple_exclude_wall_id(Vec3::Y * 10.0, -Vec3::Y, mask, wall), filtered);
            assert_source(&filtered, if matches!(kind, 0 | 1 | 2 | 8) { target } else { own });
            assert_eq!(filtered.toi, if matches!(kind, 0 | 1 | 2 | 8) { 8.5 } else { 4.5 });
            let filtered_other = world.raycast_simple_excluding_wall_id(
                Vec3::Y * 10.0, -Vec3::Y, mask, other,
            );
            assert_source(&filtered_other, own);
            assert_eq!(filtered_other.toi,4.5);
        }
    }

    #[test]
    fn selected_wall_filter_skips_nearest_and_keeps_farther_hit_in_same_slot() {
        for tag in [0u8, 1, 2, 8] {
            let mut world = RaycastWorld::new();
            let mut map = HashMap::new();
            let mut near = RaycastGeoSource::tag_only(tag);
            near.0[8..16].copy_from_slice(&7u64.to_le_bytes());
            let mut far = RaycastGeoSource::tag_only(tag);
            far.0[8..16].copy_from_slice(&8u64.to_le_bytes());
            insert_box(&mut world, &mut map, 1, near, Vec3::Z * 2.0);
            insert_box(&mut world, &mut map, 2, far, Vec3::Z * 4.0);
            world.flush_colliders();
            let hit = world.raycast_simple_excluding_wall(Vec3::ZERO, Vec3::Z, 1 << near.subworld(), crate::resources::walls::WallId(7));
            assert_source(&hit, far);
            assert_eq!(hit.toi, 3.5);
        }
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        let mut allowed = RaycastGeoSource::tag_only(3);
        allowed.0[8..16].copy_from_slice(&7u64.to_le_bytes());
        let mut other_slot = RaycastGeoSource::tag_only(1);
        other_slot.0[8..16].copy_from_slice(&8u64.to_le_bytes());
        insert_box(&mut world, &mut map, 1, allowed, Vec3::Z * 2.0);
        insert_box(&mut world, &mut map, 2, other_slot, Vec3::Z * 4.0);
        world.flush_colliders();
        let mask = (1 << allowed.subworld()) | (1 << other_slot.subworld());
        assert_source(&world.raycast_simple_excluding_wall(Vec3::ZERO, Vec3::Z, mask, crate::resources::walls::WallId(7)), allowed);
    }

    #[test]
    fn bounded_source13_miss_and_endpoint_keep_old_max_entry() {
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        let source = RaycastGeoSource::Slot13;
        insert_box(&mut world, &mut map, 1, source, Vec3::Y * -1.5);
        world.flush_colliders();
        let origin = Vec3::Y;
        let direction = Vec3::new(-0.0, -1.0, -0.0);
        let mask = 0x1000;
        assert!(world.raycast_simple_with_max_toi(origin, direction, 1.0, mask).is_miss());
        let old = world.raycast_simple(origin, direction, mask);
        assert_source(&old, source);
        assert_eq!(old.toi, 2.0);
        let hit = world.raycast_simple_with_max_toi(origin, direction, 2.0, mask);
        assert_source(&hit, source);
        assert_eq!(hit.position, Vec3::Y * -1.0);
        assert_eq!(hit.toi, 2.0);
        // source13不携带墙身份，排除任一墙都保持此命中，不把句柄误作WallId。
        assert_eq!(world.raycast_simple_exclude_wall_id(origin, direction, mask,
            crate::resources::walls::WallId(1)), old);
    }

    #[test]
    fn finite_hit_returns_all_canonical_source_words_and_actual_pose() {
        use crate::resources::walls::decorator::DecoratorId;
        use crate::resources::walls::decorator_storage::DecoratorStorageAddr;
        use crate::resources::walls::WallId;
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        let source = RaycastGeoSource::decorator(
            DecoratorId(0xfedc_ba98), DecoratorStorageAddr::Wall(WallId(0x1234_5678_9abc_def0)));
        insert_box(&mut world, &mut map, 1, source, Vec3::new(8.0, 2.0, 5.0));
        world.flush_colliders();
        let origin = Vec3::new(8.0, 2.0, 1.0);
        let hit = world.raycast_simple_with_max_toi(origin, Vec3::Z, 4.0, 1 << source.subworld());
        assert_source(&hit, source);
        assert_eq!(hit.toi, 3.5);
        assert_eq!(hit.position, Vec3::new(8.0, 2.0, 4.5));
    }

    #[test]
    fn nearest_hit_tightens_bound_and_equal_later_slot_replaces() {
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        insert_box(&mut world, &mut map, 1, RaycastGeoSource::Slot0, Vec3::Z * 3.0);
        insert_box(&mut world, &mut map, 2, RaycastGeoSource::Slot16, Vec3::Z * 6.0);
        world.flush_colliders();
        let mask = 1 | (1 << 16);
        let hit = world.raycast_simple_with_max_toi(Vec3::ZERO, Vec3::Z, 10.0, mask);
        assert_source(&hit, RaycastGeoSource::Slot0);
        assert_eq!(hit.toi, 2.5);
        assert_eq!(hit.position, Vec3::Z * 2.5);
        assert!(remove_collider(&mut world, &mut map, &2).is_some());
        insert_box(&mut world, &mut map, 3, RaycastGeoSource::Slot16, Vec3::Z * 3.0);
        world.flush_colliders();
        let tie = world.raycast_simple_with_max_toi(Vec3::ZERO, Vec3::Z, 10.0, mask);
        assert_source(&tie, RaycastGeoSource::Slot16);
        assert_eq!(tie.toi, 2.5);
        assert_eq!(tie.position, hit.position);
    }

    #[test]
    fn bound_and_mask_select_last_subworld_with_parametric_direction() {
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        insert_box(&mut world, &mut map, 1, RaycastGeoSource::Slot0, Vec3::Z);
        insert_box(&mut world, &mut map, 2, RaycastGeoSource::Slot16, Vec3::Z * 4.0);
        world.flush_colliders();
        let direction = Vec3::Z * 2.0;
        let mask = 1 << 16;
        assert!(world.raycast_simple_with_max_toi(Vec3::ZERO, direction, 1.5, mask).is_miss());
        let hit = world.raycast_simple_with_max_toi(Vec3::ZERO, direction, 1.75, mask);
        assert_source(&hit, RaycastGeoSource::Slot16);
        assert_eq!(hit.toi, 1.75);
        assert_eq!(hit.position, Vec3::Z * 3.5);
        assert!(world.raycast_simple_with_max_toi(Vec3::ZERO, direction, 10.0, 0).is_miss());
        assert!(world.raycast_simple_with_max_toi(Vec3::ZERO, direction, 10.0, 1 << 31).is_miss());
    }

    #[test]
    fn solid_query_starts_inside_at_zero_bound() {
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        insert_box(&mut world, &mut map, 1, RaycastGeoSource::Slot0, Vec3::ZERO);
        world.flush_colliders();
        let hit = world.raycast_simple_with_max_toi(Vec3::ZERO, Vec3::Z, 0.0, 1);
        assert_source(&hit, RaycastGeoSource::Slot0);
        assert_eq!(hit.toi, 0.0);
        assert_eq!(hit.position, Vec3::ZERO);
    }
}

#[cfg(test)]
mod slot_publisher_tests {
    use super::*;

    #[cfg(feature = "native-physics")]
    #[test]
    fn reload_preserves_pending_and_future_slot_and_mesh_publications() {
        let mut world = RaycastWorld::new();
        let mut dirty = world.subscribe_to_changes(0);
        let mut diff = world.mesh_intersect_collider_diff_publisher().subscribe();
        let mut owners = std::collections::HashMap::new();
        let mesh = || std::sync::Arc::new(RaycastMeshData::new_cuboid_at(glam::Vec3::ONE, glam::Vec3::ZERO));
        upsert_collider(&mut world, &mut owners, 1u64, RaycastGeoSource::Slot0,
            mesh(), glam::Vec3::ZERO, glam::Quat::IDENTITY);
        world.flush_colliders();
        upsert_collider(&mut world, &mut owners, 2u64, RaycastGeoSource::Slot0,
            mesh(), glam::Vec3::X * 4.0, glam::Quat::IDENTITY);
        world.on_level_reload();
        assert!(dirty.take().is_some(), "重载前尚未读取的脏区必须保留");
        assert!(!diff.take().expect("重载前差量必须保留").added.is_empty());
        world.flush_colliders();
        assert!(dirty.take().is_some(), "重载前尚未 flush 的槽脏区必须保留");
        owners.clear();
        upsert_collider(&mut world, &mut owners, 3u64, RaycastGeoSource::Slot0,
            mesh(), glam::Vec3::ZERO, glam::Quat::IDENTITY);
        world.flush_colliders();
        assert!(dirty.take().is_some(), "原订阅必须接收新世界脏区");
        assert!(diff.take().expect("原订阅必须接收新世界差量").added.iter().any(|handle| handle.slot == 0));
    }

    /// 0x1408cdc20：按槽号取内联发布器（`slot*0x390+0x170`）；每个槽独立，且不检查越界（Rust 侧 panic）。
    #[test]
    fn subscribe_to_changes_is_per_slot() {
        let mut world = RaycastWorld::new();
        for slot in 0..crate::systems::collision::SUBWORLD_SLOT_COUNT {
            let _ = world.subscribe_to_changes(slot);
        }
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut world = RaycastWorld::new();
            let _ = world.subscribe_to_changes(crate::systems::collision::SUBWORLD_SLOT_COUNT);
        }));
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod native_arch_collision_recovery_tests {
    use super::*;
    #[test]
    fn original_arch_full_entry_preserves_zip_order_prefix_and_nonfinite_bits() {
        let report: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../reports/module-recovery-2026-10-23/m01-arch-collision-rosetta.json"))).unwrap();
        assert_eq!(report["rne_sha256"], "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03");
        let rows = report["rows"].as_array().unwrap(); assert_eq!(rows.len(), 315);
        let mut differences = Vec::new();
        for (case, row) in rows.iter().enumerate() {
            let f = |value: &serde_json::Value| f32::from_bits(value.as_u64().unwrap() as u32);
            let segment = crate::systems::wall::construction::structures::MaterializedWallSegment {
                heights: row["heights"].as_array().unwrap().iter().map(f).collect(),
                positions: row["positions"].as_array().unwrap().iter().map(|p| std::array::from_fn(|i| f(&p[i]))).collect(),
                points_u: vec![f32::NAN], source_indices: [91, 94],
                range: crate::systems::wall::construction::segmentation::WallLoopRange { start: 0, end: 0, modulus: 1 },
            };
            let seed = row["seed"].as_u64().unwrap() as usize;
            let mut normals: Vec<_> = (0..seed).map(|i| std::array::from_fn(|p| glam::Vec3::from_array(std::array::from_fn(|j| f32::from_bits(0x3f80_0000 + (i * 9 + p * 3 + j) as u32))))).collect();
            let mut triangles: Vec<_> = (0..seed).map(|i| utils::geometry::intersect::Triangle { points: std::array::from_fn(|p| glam::Vec3::from_array(std::array::from_fn(|j| f32::from_bits(0xbf80_0000 + (i * 9 + p * 3 + j) as u32)))) }).collect();
            construct_arch_geo(&segment, &mut normals, &mut triangles);
            for (name, actual) in [("normals", normals.iter().flatten().flat_map(|v| v.to_array()).map(f32::to_bits).collect::<Vec<_>>()),
                ("triangles", triangles.iter().flat_map(|t| t.points).flat_map(|v| v.to_array()).map(f32::to_bits).collect())] {
                let expected: Vec<u32> = row[name].as_array().unwrap().iter().flat_map(|a| a.as_array().unwrap()).map(|v| v.as_u64().unwrap() as u32).collect();
                if actual != expected { differences.push(format!("case{case} {name}: {:?}", actual.iter().zip(&expected).enumerate().filter(|(_, (a,b))| a != b).take(4).collect::<Vec<_>>())); }
            }
        }
        assert!(differences.is_empty(), "原完整拱碰撞几何逐位差异：{differences:?}");
    }
}
