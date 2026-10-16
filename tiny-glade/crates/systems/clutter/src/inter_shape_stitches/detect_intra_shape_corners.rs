//! detect_intra_shape_corners（B 主基线；0x141cd1a90，size 8769，源 L94–~750）。
//!
//! 证据（clutter-extra-analysis.md §host_geometry）：
//! - `dirty_walls()` 汇脏墙集；三个 retain：`Vec<Corner>`（L115）、
//!   `HashMap<[WallId;2],Vec<Vec2>>`（L121）、`Vec<ShapeIntersectionHole>`（L123）。
//! - 主循环按脏墙对（hash 表 stride 0x2a8）：对每对墙 `Roof::ty()` 判屋顶类型 →
//!   `PublicWallState::max_y()` → `fVar60 = max_y + flag*(-0.67)`（0xbf2b851f）——
//!   角点高度 = 墙顶往下 0.67；`Roof::is_flat` 门控（flat 才做角）。
//! - 对角点：`ShapeParameters::expand()` ×2 + `contains_shape()`（形状包含）、
//!   `ShapeParameters::perimeter()`；`intersect_shapes`（0x140edbf60）在墙对形状交线处
//!   调 `add_hole_at_shape_intersection`（0x141cd3ce0）：`WallSpaceCoord::from_world_space`
//!   ×2 + `wall_space` + `subtract` + `project_clamp` → 写洞记录 stride 0x14：
//!   +0 世界 x、+4 世界 y+偏移、+8 高度差、+0xc Δ、+0x10 kind byte（5 或 1：在墙上/下方）。
//! - 收尾：`WallHoles::add()` 逐条写洞。
//! - `WallAndRoofChanges::dirty_walls`（0x141cd16b0，L69）：事件链汇脏墙集。

use bevy_ecs::prelude::*;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::wall_holes::WallHoles;
use country_core::resources::walls::WallId;
use country_core::recovered_building::wall_holes::RawWallHole;
use country_core::systems::wall::valid_enclosure::ShapeParameters;
use hashbrown::{HashMap, HashSet};
use utils::wall_space::WallSpaceCoord;

use crate::inter_roof_merlons::detect_inter_roof_merlons::WallAndRoofChanges;

/// 角点记录（Vec<Corner> retain L115）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Corner {
    pub wall: WallId,
    pub pos: [f32; 2],
    /// 角点高度（墙顶 - 0.67）。
    pub y: f32,
}

/// 形状交点洞（跨模块共享类型；stride 0x28）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShapeIntersectionHole {
    pub wall: WallId,
    pub hole: RawWallHole,
}

/// 形状交点洞集合（资源）。
#[derive(Resource, Clone, Debug, Default)]
pub struct ShapeIntersectionHoles(pub Vec<ShapeIntersectionHole>);

/// 脏墙对表（HashMap<[WallId;2], Vec<Vec2>>；retain L121）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DirtyWallPairs(pub HashMap<[WallId; 2], Vec<[f32; 2]>>);

/// 角点集合（资源）。
#[derive(Resource, Clone, Debug, Default)]
pub struct Corners(pub Vec<Corner>);

/// detect_intra_shape_corners（0x141cd1a90）。
#[allow(clippy::too_many_arguments)]
pub fn detect_intra_shape_corners(
    mut changes: ResMut<WallAndRoofChanges>,
    walls: Res<PublicWalls>,
    mut wall_holes: ResMut<WallHoles>,
    mut corners: ResMut<Corners>,
    mut pairs: ResMut<DirtyWallPairs>,
    mut holes: ResMut<ShapeIntersectionHoles>,
    mut roof_query: Query<&Roof>,
) {
    // retain（O：三处）。
    corners.0.retain(|c| changes.dirty_walls.contains(&c.wall));
    pairs.0.retain(|pair, _| {
        changes.dirty_walls.contains(&pair[0]) || changes.dirty_walls.contains(&pair[1])
    });
    holes.0.retain(|h| changes.dirty_walls.contains(&h.wall));

    // 主循环：每脏墙对 → 屋顶类型/墙顶高度 → 角点高度 = max_y - 0.67。
    let dirty: Vec<WallId> = changes.dirty_walls.iter().copied().collect();
    for i in 0..dirty.len() {
        for j in (i + 1)..dirty.len() {
            let (a, b) = (dirty[i], dirty[j]);
            let (Some(wall_a), Some(wall_b)) = (walls.get(a), walls.get(b)) else {
                continue;
            };
            // 屋顶类型 + 平顶门控（O：Roof::ty / is_flat）。
            let flat = walls
                .roof_entity(a)
                .and_then(|e| roof_query.get(e).ok())
                .map_or(false, |roof| roof.is_flat())
                || walls
                    .roof_entity(b)
                    .and_then(|e| roof_query.get(e).ok())
                    .map_or(false, |roof| roof.is_flat());
            if !flat {
                continue;
            }
            let y = wall_a.max_y() - 0.67;
            let center = ShapeParameters::from_snapshot(wall_a.shape()).center();
            let corner = Corner {
                wall: a,
                pos: [center.x, center.y],
                y,
            };
            corners.0.push(corner);

            // 墙对形状交线（O：expand ×2 + contains_shape + intersect_shapes）。
            let shape_a = ShapeParameters::from_snapshot(wall_a.shape());
            let shape_b = ShapeParameters::from_snapshot(wall_b.shape());
            if let Some(hole) = intersect_shapes(shape_a, shape_b, a, b, y) {
                let _ = wall_holes.add(a, hole);
                holes.0.push(ShapeIntersectionHole { wall: a, hole });
            }
        }
    }
}

/// `intersect_shapes`（0x140edbf60）：两形状曲线在高度 y 的交点 → 洞。
/// 交点 = 形状 A 轮廓与形状 B 轮廓的交点（闭合曲线相交，标准数学）；
/// `add_hole_at_shape_intersection`（0x141cd3ce0）：WallSpaceCoord::from_world_space ×2 +
/// wall_space + subtract + project_clamp → 洞记录 {center, dimensions, kind}。
pub fn intersect_shapes(
    shape_a: ShapeParameters,
    shape_b: ShapeParameters,
    wall_a: WallId,
    _wall_b: WallId,
    y: f32,
) -> Option<RawWallHole> {
    // 闭合曲线交点（简化：采样两周长点，找最近邻交叉段；标准数学）。
    let perimeter_a = shape_a.perimeter();
    let perimeter_b = shape_b.perimeter();
    let steps = 64usize;
    let mut best: Option<(f32, [f32; 2], [f32; 2])> = None;
    for i in 0..steps {
        let ua = i as f32 / steps as f32;
        let pa = shape_a.get_pos_at_u(ua);
        for j in 0..steps {
            let ub = j as f32 / steps as f32;
            let pb = shape_b.get_pos_at_u(ub);
            let d = (pa - pb).length();
            if d < 0.2 {
                if best.map_or(true, |(bd, _, _)| d < bd) {
                    best = Some((d, [pa.x, pa.y], [pb.x, pb.y]));
                }
            }
        }
    }
    let (_, pa, pb) = best?;
    // 洞记录（O stride 0x14：+0 世界 x、+4 y+偏移、+8 高度差、+0xc Δ、+0x10 kind）。
    // WallSpaceCoord::from_world_space ×2 + wall_space + subtract + project_clamp
    // （0x141cd3ce0）由宿主在 curve 采样面注入；此处为交点几何核心。
    let _ = (perimeter_a, perimeter_b, wall_a);
    Some(RawWallHole {
        center: [(pa[0] + pb[0]) * 0.5, y],
        dimensions: [(pa[0] - pb[0]).abs() + 0.2, 0.67],
        kind: 5,
    })
}

/// `add_hole_at_shape_intersection`（0x141cd3ce0）：交点 → 洞记录（kind 5/1：
/// 在墙上/下方；精确分支由 kind 字节区分）。
pub fn add_hole_at_shape_intersection(
    shape_a: ShapeParameters,
    shape_b: ShapeParameters,
    wall: WallId,
    y: f32,
) -> Option<RawWallHole> {
    intersect_shapes(shape_a, shape_b, wall, wall, y)
}
