//! intersect_meshes.rs — 网格-网格相交检测（O）。
//!
//! 源 SHA256（B skeleton）`systems/collision/intersect_meshes.rs`。PDB 实体：
//! `MeshIntersectionArtifacts` / `MeshIntersectionGraph` /
//! `MeshMeshIntersectionSubscriber` / `IntersectionOutput` / `polyline::{Polyline,
//! PolylineMergeScratch, PolylinePoint, VertexId}`。
//!
//! 功能（O）：新 collider 与既有网格做两两三角形相交（rapier/parry QBVH
//! `traverse_bvtt`），把交点段合并成折线（`merge_segments_into_polylines`
//! 0x1408f2850），生成"网格相交伪影"（房间/拱门界线）；结果经 tabloid 通道
//! 发布。`should_intersect_collider_meshes`（0x140a9b3c0）按几何源过滤。
//! 业务合同见 `contracts/misc-systems.md` §collision。

#![allow(dead_code)]

use crate::typed_gap::Gap;

/// 网格相交伪影（O：`MeshIntersectionArtifacts`；petgraph GraphMap 边权，
/// 两 collider 间交点集合）。
#[derive(Clone, Debug, Default)]
pub struct MeshIntersectionArtifacts {
    pub points: Vec<[f32; 3]>,
    pub _gap: Gap,
}

/// 网格相交图（O：`MeshIntersectionGraph`；`GraphMap<SubworldColliderHandle,
/// MeshIntersectionArtifacts>`）。
#[derive(Clone, Debug, Default)]
pub struct MeshIntersectionGraph {
    pub nodes: Vec<crate::systems::collision::world::SubworldColliderHandle>,
    pub edges: Vec<(
        crate::systems::collision::world::SubworldColliderHandle,
        crate::systems::collision::world::SubworldColliderHandle,
        MeshIntersectionArtifacts,
    )>,
}

/// 相交订阅者（O：`MeshMeshIntersectionSubscriber`；tabloid 消费者）。
#[derive(Clone, Debug, Default)]
pub struct MeshMeshIntersectionSubscriber(pub Gap);

/// 相交输出（O：`IntersectionOutput` 实体）。
#[derive(Clone, Copy, Debug)]
pub struct IntersectionOutput {
    pub a: crate::systems::collision::world::SubworldColliderHandle,
    pub b: crate::systems::collision::world::SubworldColliderHandle,
    pub _point_count: u32,
}

// --- polyline（交点段合并为折线） ---

/// 折线顶点 id（O：`polyline::VertexId`；`as_usize` 内联）。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct VertexId(pub u32);

impl VertexId {
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }
}

/// 折线点（O：`polyline::PolylinePoint`）。
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PolylinePoint {
    pub position: [f32; 3],
    pub vertex: VertexId,
}

/// 折线（O：`polyline::Polyline`）。
#[derive(Clone, Debug, Default)]
pub struct Polyline {
    pub points: Vec<PolylinePoint>,
}

/// 折线合并暂存（O：`polyline::PolylineMergeScratch`）。
#[derive(Clone, Debug, Default)]
pub struct PolylineMergeScratch {
    pub polylines: Vec<Polyline>,
}

/// `merge_segments_into_polylines`（O 0x1408f2850）：把离散交点段按端点
/// 连通性合并成连续折线。
pub fn merge_segments_into_polylines(
    _segments: &[([f32; 3], [f32; 3])],
) -> Vec<Polyline> {
    Vec::new()
}

/// `find_mesh_mesh_intersections`（O 0x140a97b70）：新加 collider 与相关网格
/// 相交 → 伪影图更新 + 折线化 + 发布。
pub fn find_mesh_mesh_intersections(_world: &mut bevy_ecs::world::World) {
    // U：相交候选对（typed gap）。
}

/// `intersect_mesh_mesh`（O 0x140a9afa0）：两网格三角形两两相交
/// （parry `traverse_bvtt`；`from_parry_triangle` 内联）。
pub fn intersect_mesh_mesh(
    _a: &crate::systems::collision::world::RaycastMeshData,
    _b: &crate::systems::collision::world::RaycastMeshData,
) -> Vec<([f32; 3], [f32; 3])> {
    Vec::new()
}

/// `should_intersect_collider_meshes`（O 0x140a9b3c0）：按几何源过滤哪些
/// 网格对需要相交检测。
pub fn should_intersect_collider_meshes(
    _a: crate::systems::collision::world::RaycastGeoSource,
    _b: crate::systems::collision::world::RaycastGeoSource,
) -> bool {
    false
}
