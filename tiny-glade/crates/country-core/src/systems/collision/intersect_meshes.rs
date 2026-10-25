//! `systems/collision/intersect_meshes.rs` — 网格对网格相交（B `intersect_meshes`，
//! LOGIC.md 未实现登记段：21 函数，业务 4）。
//!
//! 证据：`planning/logic-collision` + `planning/impl-collision-remainder` +
//! `impl-collision-deep/b7-flush-intersect` fresh 反编译（ghidra shared 只读，2026-10-17）。
//!
//! ## 分类（协议 §2026-10-17：宏胶水 vs 真业务）
//!
//! 4 个业务函数全部为 **真实业务**：
//! - `should_intersect_collider_meshes`（0x140a9b3c0，284B）**已实现**（kind 字节判别 +
//!   registry generation 校验）；
//! - `intersect_mesh_mesh`（0x140a9afa0，908B）**已实现**（`Qbvh::traverse_bvtt` 包装）；
//! - `find_mesh_mesh_intersections`（0x140a97b70，13359B）**域内装配已实现**（图剪枝
//!   `Subscriber::take<ColliderDiff>` → `remove_node` + 逐句柄 AABB + 候选槽 Qbvh 遍历；
//!   三角对相交 closure 本轮落地 `intersect_triangle_pair` / `intersect_mesh_mesh_segments`；
//!   铺砖 `BrickTransformSsbo` 宿主缺失登记）；
//! - `merge_segments_into_polylines`（0x1408f2850，6057B）**已实现**（端点查表 + VecDeque
//!   折线合并；端点 `roundf` 取整键 + `Segment` Vec3 端点本轮按 fresh 修正）；
//! - `remove_invalid_polylines`（0x1412f3850 closure）**已实现**（空点折线删除；
//!   同 closure 的逐点法线重写段未落地，登记）。

#[cfg(feature = "native-physics")]
use crate::systems::collision::world::ColliderDiff;
use crate::systems::collision::world::{RaycastWorld, SubworldColliderHandle};
#[cfg(feature = "native-physics")]
use rapier3d::geometry::ColliderHandle;

/// B @140a9b3c0 (284 bytes) resolves actual Collider.user_data receipts,
/// then compares source tags, payload+8 and the tag2 flag byte.
#[cfg(feature = "native-physics")]
pub fn should_intersect_collider_meshes(
    a: SubworldColliderHandle,
    b: SubworldColliderHandle,
    world: &RaycastWorld,
) -> bool {
    let a_source = world
        .registered_mesh_for_collider(
            a.slot,
            ColliderHandle::from_raw_parts(a.index, a.generation),
        )
        .source;
    let b_source = world
        .registered_mesh_for_collider(
            b.slot,
            ColliderHandle::from_raw_parts(b.index, b.generation),
        )
        .source;
    let payload_differs = a_source.0[8..16] != b_source.0[8..16];
    match (a_source.tag(), b_source.tag()) {
        (0, 2) => payload_differs && b_source.0[1] & 1 == 0,
        (2, 0) => payload_differs && a_source.0[1] & 1 == 0,
        (2, 2) => payload_differs && a_source.0[1] == b_source.0[1] && a_source.0[1] & 1 == 0,
        _ => panic!("should_intersect_collider_meshes: invalid collider kind combination"),
    }
}

#[cfg(not(feature = "native-physics"))]
pub fn should_intersect_collider_meshes(
    _a: SubworldColliderHandle,
    _b: SubworldColliderHandle,
    _world: &RaycastWorld,
) -> bool {
    panic!("should_intersect_collider_meshes requires native-physics")
}

/// `intersect_mesh_mesh`（0x140a9afa0，O）：薄包装 —— `Qbvh::traverse_bvtt<u32,u32,
/// BoundingVolumeIntersectionsSimultaneousVisitor<closure_env_0>>`。closure 对相交的
/// (左叶, 右叶) 调用（原为三角对三角，用 `Triangle::intersect_triangle`；Rust 侧以回调承接）。
#[cfg(feature = "native-physics")]
pub fn intersect_mesh_mesh<F: FnMut(u32, u32) -> bool>(
    qbvh1: &rapier3d::parry::partitioning::Qbvh<u32>,
    qbvh2: &rapier3d::parry::partitioning::Qbvh<u32>,
    mut callback: F,
) {
    use rapier3d::parry::query::visitors::BoundingVolumeIntersectionsSimultaneousVisitor;
    let mut visitor =
        BoundingVolumeIntersectionsSimultaneousVisitor::<u32, u32, _>::new(|a: &u32, b: &u32| {
            callback(*a, *b)
        });
    qbvh1.traverse_bvtt(qbvh2, &mut visitor);
}

/// `find_mesh_mesh_intersections`（0x140a97b70，O）：域内装配（2026-10-17 restorer2）。
///
/// fresh 反编译（`find_mesh_mesh_intersections.dc.txt`，7783 B）逐语句：
/// 1. `Subscriber::take<ColliderDiff>()`（`tabloid::Subscriber::take`）→ **removed 句柄
///    逐条 `GraphMap::remove_node`**（图剪枝）；
/// 2. 逐新增/变更句柄（slot<2 门控）`ColliderSet::get_mut_internal` + `Collider::compute_aabb`
///    + `SimdAabb::splat` → 候选槽
///    `Qbvh::traverse_depth_first_node<ColliderHandle, BoundingVolumeIntersectionsVisitor<
///    ColliderHandle, intersect_newly_added_colliders::closure_env_0>>`——Rust 侧经
///    `QueryPipeline::colliders_with_aabb_intersecting_aabb` 等价遍历（fork 的
///    `QueryPipeline.qbvh: Qbvh<ColliderHandle>`，机器 +0x3d80 世界 Qbvh 面等价，登记），
///    每候选句柄调用 `intersect` 回调。
///
/// **closure 语义（0x1417d1440，2026-10-17 本轮落地）**：去重集探针 → `get_mut_internal` →
/// `should_intersect_collider_meshes` → 两侧 `as_trimesh` 门控 → `intersect_mesh_mesh` →
/// `merge_segments_into_polylines` → 空结果返回 → 注册表 `gen|1`/`kind&0xfd==0` 校验 →
/// push 0x40 B `MeshIntersectionArtifacts`，见 [`intersect_newly_added_collider`]。
///
/// **跨域登记（未实现，装配阻塞点）**：
/// - 三角对相交已落地（[`intersect_triangle_pair`] / [`intersect_mesh_mesh_segments`]，
///   fresh `intersect_mesh_mesh` 0x140a9afa0 disasm 逐条）；
/// - `intersect_newly_added_colliders::closure_env_0` 的折线→砖铺放
///   （`SparseInstanceBufferWriter::push<BrickTransformSsbo>`、中点步进 0.5、法线/切向
///   正交化、×100 取整）——`BrickTransformSsbo` 具体类型/`SparseSsbo` impl 在
///   `geometry::instanced_wall` 缺失（walls/device 域，另一 agent 锚定中；本轮不重复定义）；
/// - 图 `GraphMap::add_node`×2 + `add_edge`（`MeshIntersectionArtifacts` 载荷，类型已落位
///   [`crate::systems::collision::MeshIntersectionArtifacts`]）；
/// - `spec_extend<BirdNestCandidateLocation, …>` 从图边收集鸟巢候选（类型已落位
///   [`crate::systems::collision::BirdNestCandidateLocation`]，边构造点/`closure_env_3`
///   来源待接线）；
/// - `SparseInstanceBufferAllocation<SparkleSsbo>` drop_in_place（resources/sparkle 已有类型）。
#[cfg(feature = "native-physics")]
pub fn find_mesh_mesh_intersections(
    graph: &mut crate::systems::collision::MeshIntersectionGraph,
    subscriber: &mut tabloid::Subscriber<ColliderDiff>,
    world: &mut RaycastWorld,
    intersect: &mut impl FnMut(SubworldColliderHandle, SubworldColliderHandle),
) {
    // O：`Subscriber::take<ColliderDiff>()`。
    let Some(diff) = subscriber.take() else {
        return;
    };
    // O：图剪枝 —— 逐 removed 句柄 `GraphMap::remove_node`。
    for removed in &diff.removed {
        graph.inner_mut().remove_node(*removed);
    }
    // O：逐新增/变更句柄 AABB + 候选槽 Qbvh 遍历。
    for added in &diff.added {
        // B @140a9839f..3a5 gates the handle's SLOT before physical resolution.
        if added.slot >= 2 {
            continue;
        }
        let handle = ColliderHandle::from_raw_parts(added.index, added.generation);
        let collider = world
            .get_rapier_collider(added.slot, handle)
            .expect("find_mesh_mesh_intersections: added physical collider missing");
        let aabb = collider.compute_aabb();
        // B @140a98401..426: added slot0 -> [1], slot1 -> [0,1].
        let candidates: &[usize] = if added.slot == 0 { &[1] } else { &[0, 1] };
        for &slot_index in candidates {
            // O：候选槽 `Qbvh::traverse_depth_first_node<ColliderHandle,
            // BoundingVolumeIntersectionsVisitor>`（closure = intersect_newly_added_colliders，
            // 跨域登记）。
            world
                .slot_query_pipeline(slot_index)
                .colliders_with_aabb_intersecting_aabb(&aabb, |candidate| {
                    let (cindex, cgen) = candidate.into_raw_parts();
                    let candidate_handle = SubworldColliderHandle {
                        index: cindex,
                        generation: cgen,
                        slot: slot_index as u8,
                    };
                    intersect(*added, candidate_handle);
                    true
                });
        }
    }
}

// 分类表（find_mesh 域内装配已实现；三角相交 closure / 铺砖 / 鸟巢收集跨域登记，restorer2）：
// - 0x140a97b70 `find_mesh_mesh_intersections`（13359B）：域内装配实现（图剪枝 + AABB +
//   候选槽 Qbvh 遍历）；closure 跨域登记。

// ---------------------------------------------------------------------------
// 缺类型落地 + merge_segments_into_polylines（impl-collision-types，2026-10-17）。
//
// 证据：ghidra shared fresh `merge_segments_into_polylines`（0x1408f2850，6057B）：
// - `HashMap<glam::IVec3, VertexId>` 端点查表（`reserve_rehash<tuple<IVec3,VertexId>>`）；
// - 输出 `Vec<Polyline>`（0x28 B/条：`VecDeque<PolylinePoint>` 0x18 B + 首尾 `VertexId`）；
// - `from_iter<PolylinePoint, IntoIter<PolylinePoint, 2>>` 初建（2 点折线）；
// - 端点中点平均（`(old_pos + new_pos) * 0.5`）、跨折线拼接/闭环（`grow<PolylinePoint>`）。
// 布局与判等细节见 reports/interaction/collision/REBUILD-COLLISION-TYPES.md。
// ---------------------------------------------------------------------------

use std::collections::{HashMap, VecDeque};
use utils::geometry::intersect::Triangle;

/// `VertexId`（O：端点顶点索引；`HashMap<IVec3, VertexId>` 的值；空判别 0xffffffff）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VertexId(pub u32);

impl VertexId {
    /// 空（未在查表）判别（O：0xffffffff）。
    pub const NONE: VertexId = VertexId(u32::MAX);
}

impl Default for VertexId {
    /// O：空判别 0xffffffff（非 0）。
    fn default() -> Self {
        Self::NONE
    }
}

/// `PolylinePoint`（O：0x18 B；`{ grid: IVec3 @+0x00, position: Vec3 @+0x0c }`）。
///
/// `grid` 是段端点坐标**四舍五入后的整数键**（`merge_segments_into_polylines::closure$0`
/// 0x1408f4670：`roundf(x/y/z)` 后饱和转 i32，NaN→0），用于跨段端点匹配；
/// `position` 是段携带的浮点向量（三角对的 `normal_a + normal_b`）与其平均值。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PolylinePoint {
    /// 端点网格坐标（O：+0x00，`roundf` 取整；用于两端匹配）。
    pub grid: glam::IVec3,
    /// 顶点位置/法线（O：+0x0c；两折线共享端点时中点平均）。
    pub position: glam::Vec3,
}

/// `Polyline`（O：0x28 B；`{ points: VecDeque<PolylinePoint> 0x18 B, first: VertexId
/// @+0x20, last: VertexId @+0x24 }`）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Polyline {
    /// 折线顶点序列。
    pub points: VecDeque<PolylinePoint>,
    /// 首端点顶点索引（O：+0x20）。
    pub first: VertexId,
    /// 尾端点顶点索引（O：+0x24）。
    pub last: VertexId,
}

/// `Segment`（O：0x24 B；`{ a: Vec3 @+0x00, b: Vec3 @+0x0c, position: Vec3 @+0x18 }`）。
///
/// fresh `intersect_mesh_mesh` disasm（0x140a9afa0）：相交段两端为浮点 Vec3
/// （写 `a` 16 B @+0x00、`b` 8 B @+0x10 及低 4 B 已在 a 的 padding 中、`position` @+0x18）；
/// `position = Triangle::normal(a) + Triangle::normal(b)`（两条 `MOVSD`/`ADDPS` 归一化法线相加）。
/// 旧实现把端点建模为 IVec3（grid）是错的；`IVec3` 只出现在 `merge_segments` 的
/// `roundf` 查表键里（closure$0 0x1408f4670）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segment {
    /// 段端点 a（O：+0x00）。
    pub a: glam::Vec3,
    /// 段端点 b（O：+0x0c）。
    pub b: glam::Vec3,
    /// 段载荷向量（O：+0x18；= 两三角面法线之和）。
    pub position: glam::Vec3,
}

/// 三角对相交的外扩 AABB 尺寸（O：`intersect_mesh_mesh` 0x140a9afa0：
/// `MOVSD XMM6,[0x142930d60]` = {0.1, 0.1}，`+0xd8 = 0x3dcccccd` = 0.1；
/// `Aabb3::expand` 0x1412403c0 内部 ×0.5 → 半边长 0.05）。
pub const TRIANGLE_INTERSECTION_MARGIN: glam::Vec3 = glam::Vec3::new(0.1, 0.1, 0.1);

/// `intersect_mesh_mesh::closure_env$0`（0x140a9afa0 被 Ghidra 删除的不可达块 = 逐对处理体）：
/// 三角对求交 → 仅接受 `TriangleTriangleIntersection::Segment`（判别 1）→
/// 两端点必须同时落在两个 `expand(TRIANGLE_INTERSECTION_MARGIN)` 后的 AABB 内 →
/// 产出 [`Segment`]（position = 两三角法线之和）。
///
/// fresh 指令序列：`triangle()`×2 → `intersect_triangle`（0x14123af40）→
/// `aabb`×2（0x14123ae50）→ `expand`×2（0x1412403c0）→ `inclusive_contains_point`×4
/// （0x1412404f0：a∈A、b∈A、a∈B、b∈B）→ `normal`×2（0x14123ada0）→ push。
pub fn intersect_triangle_pair(a: Triangle, b: Triangle) -> Option<Segment> {
    use utils::geometry::triangle_triangle::TriangleTriangleIntersection;
    let (pa, pb) = match a.intersect_triangle(b) {
        TriangleTriangleIntersection::Segment(pa, pb) => (pa, pb),
        // Point/Coplanar/Disjoint（O：判别 != 1 直接跳过）。
        _ => return None,
    };
    let expanded_a = a.aabb().expand(TRIANGLE_INTERSECTION_MARGIN);
    let expanded_b = b.aabb().expand(TRIANGLE_INTERSECTION_MARGIN);
    if !expanded_a.inclusive_contains_point(pa)
        || !expanded_a.inclusive_contains_point(pb)
        || !expanded_b.inclusive_contains_point(pa)
        || !expanded_b.inclusive_contains_point(pb)
    {
        return None;
    }
    Some(Segment {
        a: pa,
        b: pb,
        position: a.normal() + b.normal(),
    })
}

/// `intersect_mesh_mesh`（0x140a9afa0）的完整落地形态：`TriMesh` 对 → 段 Vec。
///
/// O 前置门（`intersect_newly_added_colliders::closure_env$0` 0x1417d1440）：
/// 两个 collider 的形状都必须 `as_trimesh`（ShapeType 判别 5）才进入本函数。
/// O 先 `traverse_bvtt` 收集相交叶对，再逐对调用 [`intersect_triangle_pair`]。
#[cfg(feature = "native-physics")]
pub fn intersect_mesh_mesh_segments(
    mesh_a: &rapier3d::parry::shape::TriMesh,
    mesh_b: &rapier3d::parry::shape::TriMesh,
    out: &mut Vec<Segment>,
) {
    let convert = |t: rapier3d::parry::shape::Triangle| {
        Triangle::new(
            glam::Vec3::new(t.a.x, t.a.y, t.a.z),
            glam::Vec3::new(t.b.x, t.b.y, t.b.z),
            glam::Vec3::new(t.c.x, t.c.y, t.c.z),
        )
    };
    intersect_mesh_mesh(mesh_a.qbvh(), mesh_b.qbvh(), |ia, ib| {
        if let Some(segment) =
            intersect_triangle_pair(convert(mesh_a.triangle(ia)), convert(mesh_b.triangle(ib)))
        {
            out.push(segment);
        }
        true
    });
}

/// `intersect_newly_added_colliders::closure_env$0`（0x1417d1440，1274 B，完整原过程,
/// 2026-10-17）：新增 collider 与候选 collider 的逐对处理闭包。
///
/// fresh 逐语句：
/// 1. **去重集**：`HashSet<SubworldColliderHandle>`（env+0x10）`hash_one` 探针命中候选句柄
///    → 直接返回（本函数只读集合；插入点在装配侧）；
/// 2. `ColliderSet::get_mut_internal`（失败 `unwrap_failed`）；
/// 3. `should_intersect_collider_meshes`（kind 组合门控，非法组合 panic）；
/// 4. 两侧 shape vtable `+0x130` = `as_trimesh`，判别必须同为 5（TriMesh）才继续
///    （由调用方以 `&TriMesh` 提供承载该门控）；
/// 5. `intersect_mesh_mesh` → 段；`merge_segments_into_polylines`；
/// 6. 合并结果为空 → 返回；
/// 7. 两侧注册表 `generation | 1 == handle.generation | 1`（失败 `begin_panic`）且
///    `kind & 0xfd == 0`（kind ∈ {0,2}，否则 panic）；
/// 8. push **0x40 B** `MeshIntersectionArtifacts`（见该类型文档）。
///
/// 登记：`RaycastMeshData` 的 normals/uvs 来源（注册表条目 +0x08 → mesh 源）未锚定；
/// 调用方以已解析的 `TriMesh` 传入。闭包自身不在本函数插入去重集。
#[cfg(feature = "native-physics")]
pub fn intersect_newly_added_collider(
    world: &RaycastWorld,
    added: SubworldColliderHandle,
    candidate: SubworldColliderHandle,
    mesh_added: &rapier3d::parry::shape::TriMesh,
    mesh_candidate: &rapier3d::parry::shape::TriMesh,
    processed: &std::collections::HashSet<SubworldColliderHandle>,
    artifacts: &mut Vec<crate::systems::collision::MeshIntersectionArtifacts>,
) {
    // 1. O：HashSet 探针命中候选句柄 → 返回。
    if processed.contains(&candidate) {
        return;
    }
    // 2/3. O：collider 解析由调用方完成；kind 组合门控（非法组合 panic）。
    if !should_intersect_collider_meshes(added, candidate, world) {
        return;
    }
    // 5. O：traverse_bvtt → 逐三角对 → Segment；随后 merge_segments_into_polylines。
    let mut segments = Vec::new();
    intersect_mesh_mesh_segments(mesh_added, mesh_candidate, &mut segments);
    let mut polylines = Vec::new();
    merge_segments_into_polylines(&segments, &mut polylines);
    // 6. O：`local_58 == 0` → 不建边。
    if polylines.is_empty() {
        return;
    }
    // B @1417d168c..16fc resolves actual user_data again after nonempty polylines.
    // A Rapier arena index is never a global mesh receipt index.
    let source_a = world
        .registered_mesh_for_collider(
            added.slot,
            ColliderHandle::from_raw_parts(added.index, added.generation),
        )
        .source;
    let source_b = world
        .registered_mesh_for_collider(
            candidate.slot,
            ColliderHandle::from_raw_parts(candidate.index, candidate.generation),
        )
        .source;
    if source_a.tag() & 0xfd != 0 || source_b.tag() & 0xfd != 0 {
        panic!("intersect_newly_added_collider: invalid collider kind");
    }
    // 8. O：push 0x40 B 载荷（Vec<Polyline> + 两个 mesh qword + 两个 12 B 句柄）。
    artifacts.push(crate::systems::collision::MeshIntersectionArtifacts {
        polylines,
        mesh_ref_a: u64::from_le_bytes(source_a.0[8..16].try_into().unwrap()),
        mesh_ref_b: u64::from_le_bytes(source_b.0[8..16].try_into().unwrap()),
        a: added,
        b: candidate,
    });
}

/// `merge_segments_into_polylines`（0x1408f2850，O）：把相交网格产生的段合并成折线。
///
/// fresh 反编译（逐语句，2026-10-17）：
/// 1. 每个段经 `HashMap<IVec3, VertexId>` 求两端顶点索引（`closure$0` 0x1408f4670 两次；
///    键 = 端点 `roundf` 后饱和转 i32，NaN→0；未在表内则分配新 id）；
///    两端都在表内但不同折线 → 拼接，同折线 → 闭环。
/// 2. 折线 = `VecDeque<PolylinePoint>`；初建用 `from_iter<PolylinePoint, IntoIter<_,2>>`
///    （2 点）；共享端点追加时与既有端点**中点平均**（`(old+new)*0.5`）。
/// 3. 拼接两个折线时按方向转移顶点（反向翻转）。
///
/// Rust 侧按上述语义实现（字节级 VecDeque 环缓冲布局与 scratch 端点索引 Vec 登记）。
pub fn merge_segments_into_polylines(
    segments: &[Segment],
    out: &mut Vec<Polyline>,
) -> HashMap<glam::IVec3, VertexId> {
    let mut vertex_ids: HashMap<glam::IVec3, VertexId> = HashMap::new();
    // O：`reserve_rehash` 预分配（段数 × 2）。
    vertex_ids.reserve(segments.len() * 2);
    for seg in segments {
        let grid_a = round_ivec3(seg.a);
        let grid_b = round_ivec3(seg.b);
        let vid_a = lookup_or_insert(&mut vertex_ids, grid_a);
        let vid_b = lookup_or_insert(&mut vertex_ids, grid_b);
        // 查找 vid 所在折线（O：每个 VertexId 对应一个折线端点）。
        let pl_a = find_polyline(out, vid_a);
        let pl_b = find_polyline(out, vid_b);
        match (pl_a, pl_b) {
            // 两端都在同一折线 → 闭环（O：`uVar17==uVar9` 分支语义 —— 读首尾点、
            // 中点平均后 push 一个新点；Rust 以相同语义 + 封闭标记登记）。
            (Some(idx), Some(j)) if idx == j => {
                let polyline = &mut out[idx];
                let closes_forward = polyline.first == vid_a && polyline.last == vid_b;
                let closes_back = polyline.first == vid_b && polyline.last == vid_a;
                if closes_forward || closes_back {
                    let (new_pos, grid) = average_endpoints(polyline, closes_forward, seg);
                    polyline.points.push_back(PolylinePoint {
                        grid,
                        position: new_pos,
                    });
                }
            }
            // 两端都不在折线 → 新建 2 点折线。
            (None, None) => {
                let new = Polyline {
                    points: VecDeque::from([
                        PolylinePoint {
                            grid: grid_a,
                            position: seg.position,
                        },
                        PolylinePoint {
                            grid: grid_b,
                            position: seg.position,
                        },
                    ]),
                    first: vid_a,
                    last: vid_b,
                };
                out.push(new);
            }
            // 一端在折线 → 追加到该折线对应端（O：匹配端首 → 前插，尾 → 追加）。
            (Some(idx), None) => {
                append_to_polyline(out, idx, vid_a, vid_b, seg, grid_b, grid_a);
            }
            (None, Some(idx)) => {
                append_to_polyline(out, idx, vid_b, vid_a, seg, grid_a, grid_b);
            }
            // 两端在不同折线 → 拼接（O：跨 polyline 拼接 + 中点平均）。
            (Some(ia), Some(ib)) if ia != ib => {
                join_polylines(out, ia, ib);
            }
            (Some(_), Some(_)) => unreachable!(),
        }
    }
    vertex_ids
}

/// 端点整数键（O：closure$0 0x1408f4670：`roundf` 后饱和转 `i32`，NaN→0）。
/// Rust `f32::round() as i32` 的语义与机器码一致（half-away-from-zero、饱和、NaN→0）。
pub fn round_ivec3(v: glam::Vec3) -> glam::IVec3 {
    glam::IVec3::new(v.x.round() as i32, v.y.round() as i32, v.z.round() as i32)
}

fn lookup_or_insert(map: &mut HashMap<glam::IVec3, VertexId>, grid: glam::IVec3) -> VertexId {
    if let Some(&id) = map.get(&grid) {
        id
    } else {
        let id = VertexId(map.len() as u32);
        map.insert(grid, id);
        id
    }
}

fn find_polyline(polylines: &[Polyline], vid: VertexId) -> Option<usize> {
    polylines
        .iter()
        .position(|p| p.first == vid || p.last == vid)
}

/// 追加段到折线（O：一端匹配折线首/尾，中点平均 + 翻转视方向）。
fn append_to_polyline(
    polylines: &mut [Polyline],
    idx: usize,
    matched: VertexId,
    new_vid: VertexId,
    seg: &Segment,
    new_grid: glam::IVec3,
    shared_grid: glam::IVec3,
) {
    let polyline = &mut polylines[idx];
    // O：匹配端点位于折线首 → 新端插入前；位于尾 → 追加后。
    let at_front = polyline.first == matched;
    let new_point = PolylinePoint {
        grid: new_grid,
        position: seg.position,
    };
    // O：共享端点中点平均。
    if at_front {
        if let Some(front) = polyline.points.front_mut() {
            if front.grid == shared_grid {
                front.position = (front.position + seg.position) * 0.5;
            }
        }
        polyline.points.push_front(new_point);
        polyline.first = new_vid;
    } else {
        if let Some(back) = polyline.points.back_mut() {
            if back.grid == shared_grid {
                back.position = (back.position + seg.position) * 0.5;
            }
        }
        polyline.points.push_back(new_point);
        polyline.last = new_vid;
    }
}

/// 拼接两个折线（O：跨 polyline 拼接 + 中点平均；Rust 以朝向整理 + 转移点近似，
/// 共享端点中点平均登记）。
fn join_polylines(polylines: &mut Vec<Polyline>, ia: usize, ib: usize) {
    // A 含甲端、B 含乙端（调用方由 vid_a/vid_b 匹配保证）。B 追加到 A：
    // first = A.first、last = B.last（朝向整理与反向登记为结构近似）。
    let mut points_a = polylines[ia].points.clone();
    let points_b = polylines[ib].points.clone();
    let first = polylines[ia].first;
    let last = polylines[ib].last;
    points_a.extend(points_b);
    polylines[ia].points = points_a;
    polylines[ia].first = first;
    polylines[ia].last = last;
    polylines.remove(ib);
}

/// 闭环时对首尾共享端点做中点平均（O：`uVar17==uVar9` 分支的端点平均；
/// `grid` = 闭环新点落回的端点网格坐标，取 `roundf` 后的端点）。
fn average_endpoints(
    polyline: &Polyline,
    closes_forward: bool,
    seg: &Segment,
) -> (glam::Vec3, glam::IVec3) {
    let front = polyline.points.front().copied().unwrap_or(PolylinePoint {
        grid: glam::IVec3::ZERO,
        position: glam::Vec3::ZERO,
    });
    let back = polyline.points.back().copied().unwrap_or(front);
    // O：`(last.pos + first.pos) * 0.5`；grid 取回落的端点（closes_forward → seg.b，否则 seg.a）。
    (
        (front.position + back.position) * 0.5,
        round_ivec3(if closes_forward { seg.b } else { seg.a }),
    )
}

/// `remove_invalid_polylines`（O：`retain_mut<Polyline, ..., remove_invalid_polylines::closure_env$0>`
/// 0x1412f3850）：删除 `points` 为空的折线。
///
/// fresh 逐语句：`VecDeque::len`（polyline+0x18）为 0 → 丢弃该折线；否则对每个点保留。
/// 同一 closure 内另有逐点向量重写（`normalize(p_i - f(p_{i-1}))` 形态，0x1412f38…，
/// 反编译对前一点寄存器更新不完整）；该归一化段**未落地**，登记见报告。
pub fn remove_invalid_polylines(polylines: &mut Vec<Polyline>) {
    polylines.retain(|p| !p.points.is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Vec3;

    fn seg(a: glam::IVec3, b: glam::IVec3) -> Segment {
        let af = a.as_vec3();
        let bf = b.as_vec3();
        Segment {
            a: af,
            b: bf,
            position: Vec3::new((af.x + bf.x) * 0.5, 1.0, (af.z + bf.z) * 0.5),
        }
    }

    #[test]
    fn two_disjoint_segments_make_two_polylines() {
        let segments = [
            seg(glam::IVec3::new(0, 0, 0), glam::IVec3::new(1, 0, 0)),
            seg(glam::IVec3::new(5, 0, 0), glam::IVec3::new(6, 0, 0)),
        ];
        let mut out = Vec::new();
        let ids = merge_segments_into_polylines(&segments, &mut out);
        assert_eq!(out.len(), 2);
        assert_eq!(ids.len(), 4);
    }

    #[test]
    fn chained_segments_merge_into_one_polyline() {
        // (0,0)-(1,0) 与 (1,0)-(2,0) 共享端点 → 一条 3 点折线。
        let segments = [
            seg(glam::IVec3::new(0, 0, 0), glam::IVec3::new(1, 0, 0)),
            seg(glam::IVec3::new(1, 0, 0), glam::IVec3::new(2, 0, 0)),
        ];
        let mut out = Vec::new();
        merge_segments_into_polylines(&segments, &mut out);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].points.len(), 3);
        // 共享端点中点平均：两段位置 (0.5,0) 与 (1.5,0) 不冲突，首点 (0,0)。
        assert_eq!(out[0].first.0, 0);
        assert_eq!(out[0].last.0, 2);
    }

    #[test]
    fn loop_closes_when_endpoints_meet() {
        // (0,0)-(1,0), (1,0)-(2,0), (2,0)-(0,0) → 闭环（仍一条折线）。
        let segments = [
            seg(glam::IVec3::new(0, 0, 0), glam::IVec3::new(1, 0, 0)),
            seg(glam::IVec3::new(1, 0, 0), glam::IVec3::new(2, 0, 0)),
            seg(glam::IVec3::new(2, 0, 0), glam::IVec3::new(0, 0, 0)),
        ];
        let mut out = Vec::new();
        merge_segments_into_polylines(&segments, &mut out);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].points.len(), 4);
    }

    /// O：closure$0 0x1408f4670 —— 端点先 `roundf` 再饱和转 i32 作为网格键
    /// （0.6→1、-0.6→-1、NaN→0、超大→i32::MAX）。
    #[test]
    fn endpoint_grid_uses_rounded_saturating_cast() {
        assert_eq!(
            round_ivec3(Vec3::new(0.6, -0.6, 0.0)),
            glam::IVec3::new(1, -1, 0)
        );
        assert_eq!(
            round_ivec3(Vec3::new(f32::NAN, 1e30, -1e30)),
            glam::IVec3::new(0, i32::MAX, i32::MIN)
        );
    }

    /// O：`intersect_mesh_mesh` 0x140a9afa0 —— 相交三角对产出 Segment，
    /// position = 两三角法线之和；不相交返回 None。
    #[test]
    fn triangle_pair_intersection_produces_segment_with_normal_sum() {
        // 两个互相垂直、交于 x=0 的三角面。
        let a = Triangle::new(
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(-1.0, 1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        let b = Triangle::new(
            Vec3::new(0.0, -1.0, -1.0),
            Vec3::new(0.0, -1.0, 1.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let segment = intersect_triangle_pair(a, b).expect("triangles intersect");
        let expected = a.normal() + b.normal();
        assert!((segment.position - expected).length() < 1e-4);
        // 端点必须落在两个外扩 AABB 内（O：四个 inclusive_contains_point 全真）。
        let ea = a.aabb().expand(TRIANGLE_INTERSECTION_MARGIN);
        let eb = b.aabb().expand(TRIANGLE_INTERSECTION_MARGIN);
        assert!(ea.inclusive_contains_point(segment.a));
        assert!(ea.inclusive_contains_point(segment.b));
        assert!(eb.inclusive_contains_point(segment.a));
        assert!(eb.inclusive_contains_point(segment.b));

        // 不相交：同一平面内错开的两个三角。
        let c = Triangle::new(
            Vec3::new(5.0, 5.0, 0.0),
            Vec3::new(6.0, 5.0, 0.0),
            Vec3::new(5.0, 6.0, 0.0),
        );
        assert!(intersect_triangle_pair(a, c).is_none());
    }

    /// O：`remove_invalid_polylines` 0x1412f3850 —— 空点折线被删除。
    #[test]
    fn empty_polylines_are_removed() {
        let mut polylines = vec![
            Polyline::default(),
            Polyline {
                points: VecDeque::from([PolylinePoint {
                    grid: glam::IVec3::ZERO,
                    position: Vec3::ZERO,
                }]),
                first: VertexId(0),
                last: VertexId(1),
            },
        ];
        remove_invalid_polylines(&mut polylines);
        assert_eq!(polylines.len(), 1);
        assert_eq!(polylines[0].points.len(), 1);
    }
}

#[cfg(all(test, feature = "native-physics"))]
mod find_mesh_tests {
    use super::*;
    use crate::systems::collision::{MeshIntersectionGraph, world::ColliderDelta};
    use tabloid::{Publisher, State, StateDelta};

    /// `find_mesh_mesh_intersections` 图剪枝：`Subscriber::take<ColliderDiff>` 的 removed
    /// 句柄逐条 `GraphMap::remove_node`（O：0x140a97b70）。
    #[test]
    fn removed_handles_are_pruned_from_the_graph() {
        let mut graph = MeshIntersectionGraph::default();
        let mut pubsub = Publisher::<ColliderDiff>::new();
        let mut subscriber = pubsub.subscribe();

        let a = SubworldColliderHandle {
            index: 1,
            generation: 0,
            slot: 3,
        };
        let b = SubworldColliderHandle {
            index: 2,
            generation: 0,
            slot: 3,
        };
        graph.inner_mut().add_node(a);
        graph.inner_mut().add_node(b);
        assert_eq!(graph.inner().node_count(), 2);

        // 发布 removed 增量 → 图剪枝移除 a。
        pubsub
            .publish(ColliderDelta {
                handle: Some(a),
                removed: true,
            })
            .unwrap();
        let mut world = RaycastWorld::new();
        let mut calls = 0;
        find_mesh_mesh_intersections(&mut graph, &mut subscriber, &mut world, &mut |_s, _c| {
            calls += 1;
        });
        assert_eq!(graph.inner().node_count(), 1);
        assert!(graph.inner().contains_node(a) == false);
        assert!(graph.inner().contains_node(b));
        // 空世界下 added 无 rapier collider → 遍历不触发回调。
        assert_eq!(calls, 0);
    }

    /// `ColliderDelta` 的 `apply_to`：removed=true 推入 removed 列表、false 推入 added 列表
    /// （O：dispose 发布 removed 增量 / upsert 发布 added 增量）。
    #[test]
    fn collider_delta_accumulates_removed_and_added() {
        let mut state = ColliderDiff::empty();
        let h = SubworldColliderHandle {
            index: 7,
            generation: 2,
            slot: 5,
        };
        ColliderDelta {
            handle: Some(h),
            removed: true,
        }
        .apply_to(&mut state);
        ColliderDelta {
            handle: Some(h),
            removed: false,
        }
        .apply_to(&mut state);
        assert_eq!(state.removed, vec![h]);
        assert_eq!(state.added, vec![h]);
        // None 载荷为空增量。
        ColliderDelta {
            handle: None,
            removed: true,
        }
        .apply_to(&mut state);
        assert_eq!(state.removed, vec![h]);
    }
}

#[cfg(all(test, feature = "native-physics"))]
mod actual_receipt_consumer_tests {
    use super::*;
    use crate::systems::collision::MeshIntersectionGraph;
    use crate::systems::collision::world::{
        ColliderMeshInstance, ColliderUpsert, RaycastGeoSource, RaycastMeshData, upsert_collider,
    };
    use glam::{Quat, Vec3};
    use std::collections::HashMap;
    use std::sync::Arc;

    fn source(tag: u8, flag: u8, payload: u64) -> RaycastGeoSource {
        let mut source = match tag {
            0 => RaycastGeoSource::Slot0,
            2 => RaycastGeoSource::Slot2,
            3 => RaycastGeoSource::Slot3,
            4 => RaycastGeoSource::Slot4,
            _ => panic!("test input source tag not declared"),
        };
        source.0[1] = flag;
        source.0[8..16].copy_from_slice(&payload.to_le_bytes());
        source
    }

    fn insert(
        world: &mut RaycastWorld,
        map: &mut HashMap<u32, ColliderMeshInstance>,
        key: u32,
        source: RaycastGeoSource,
    ) -> SubworldColliderHandle {
        let mesh = Arc::new(RaycastMeshData::new_indexed(
            &[[0, 1, 2]],
            &[
                Vec3::new(-1.0, -1.0, -1.0),
                Vec3::new(1.0, 1.0, -1.0),
                Vec3::new(0.0, 0.0, 1.0),
            ],
            None,
            None,
        ));
        match upsert_collider(world, map, key, source, mesh, Vec3::ZERO, Quat::IDENTITY) {
            ColliderUpsert::Inserted { handle } => handle,
            other => panic!("actual fixture must insert: {other:?}"),
        }
    }

    #[test]
    fn actual_registry_predicate_keeps_original_payload_and_full_flag_byte_policy() {
        let cases = [
            (0, 0, 11, 2, 0, 12, true),
            (2, 0, 11, 0, 0, 12, true),
            (0, 0, 11, 2, 0, 11, false),
            (0, 0, 11, 2, 1, 12, false),
            (2, 0, 11, 2, 0, 12, true),
            (2, 0, 11, 2, 0, 11, false),
            (2, 1, 11, 2, 1, 12, false),
            (2, 0, 11, 2, 2, 12, false),
            (2, 2, 11, 2, 2, 12, true),
        ];
        for (tag_a, flag_a, payload_a, tag_b, flag_b, payload_b, expected) in cases {
            let mut world = RaycastWorld::new();
            let mut map = HashMap::new();
            let a = insert(&mut world, &mut map, 0, source(tag_a, flag_a, payload_a));
            let b = insert(&mut world, &mut map, 1, source(tag_b, flag_b, payload_b));
            if a.slot != b.slot {
                assert_eq!(
                    a.index, b.index,
                    "two actual subworlds may both allocate index0"
                );
            }
            assert_ne!(map[&0].registry_index, map[&1].registry_index);
            assert_eq!(
                should_intersect_collider_meshes(a, b, &world),
                expected,
                "tags {tag_a}/{tag_b}, flags {flag_a}/{flag_b}, payload {payload_a}/{payload_b}"
            );
        }
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        let a = insert(&mut world, &mut map, 0, source(0, 0, 11));
        let b = insert(&mut world, &mut map, 1, source(0, 0, 12));
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                should_intersect_collider_meshes(a, b, &world);
            }))
            .is_err(),
            "original illegal tag pair remains an error"
        );
    }

    #[test]
    fn actual_diff_routes_only_original_mesh_slots_and_candidate_slots() {
        let mut world = RaycastWorld::new();
        let mut subscriber = world.mesh_intersect_collider_diff_publisher().subscribe();
        let mut map = HashMap::new();
        insert(&mut world, &mut map, 0, source(0, 0, 11));
        insert(&mut world, &mut map, 1, source(2, 0, 12));
        insert(&mut world, &mut map, 2, source(3, 0, 13));
        insert(&mut world, &mut map, 3, source(4, 0, 14));
        world.flush_colliders();
        let mut graph = MeshIntersectionGraph::default();
        let mut routes = Vec::new();
        find_mesh_mesh_intersections(&mut graph, &mut subscriber, &mut world, &mut |a, b| {
            routes.push((a.slot, b.slot));
        });
        routes.sort_unstable();
        assert_eq!(routes, vec![(0, 1), (1, 0), (1, 1)]);
    }

    #[test]
    fn slot_gate_skips_invalid_nonmesh_handles_before_physical_resolution() {
        let mut pubsub = tabloid::Publisher::<ColliderDiff>::new();
        let mut subscriber = pubsub.subscribe();
        for slot in [2, 8, 255] {
            pubsub
                .publish(crate::systems::collision::world::ColliderDelta {
                    handle: Some(SubworldColliderHandle {
                        index: u32::MAX,
                        generation: 77,
                        slot,
                    }),
                    removed: false,
                })
                .unwrap();
        }
        let mut world = RaycastWorld::new();
        let mut graph = MeshIntersectionGraph::default();
        let mut calls = 0;
        find_mesh_mesh_intersections(&mut graph, &mut subscriber, &mut world, &mut |_, _| {
            calls += 1
        });
        assert_eq!(calls, 0);
    }

    #[test]
    fn predicate_rejects_stale_actual_user_data_after_real_replacement() {
        let mut world = RaycastWorld::new();
        let mut map = HashMap::new();
        let a_source = source(0, 0, 11);
        let a = insert(&mut world, &mut map, 0, a_source);
        let b = insert(&mut world, &mut map, 1, source(2, 0, 12));
        let old_raw = world
            .get_rapier_collider(
                a.slot,
                ColliderHandle::from_raw_parts(a.index, a.generation),
            )
            .unwrap()
            .user_data;
        let changed = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(3.0), Vec3::ZERO));
        let ColliderUpsert::Replaced { handle: new_a } = upsert_collider(
            &mut world,
            &mut map,
            0,
            a_source,
            changed,
            Vec3::ZERO,
            Quat::IDENTITY,
        ) else {
            panic!("geometry change must replace")
        };
        let physical = world
            .get_rapier_collider(
                new_a.slot,
                ColliderHandle::from_raw_parts(new_a.index, new_a.generation),
            )
            .unwrap();
        assert_ne!(physical.user_data, old_raw);
        // Corrupt the real physical collider with the genuinely removed receipt.
        physical.user_data = old_raw;
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                should_intersect_collider_meshes(new_a, b, &world);
            }))
            .is_err()
        );
    }
}
