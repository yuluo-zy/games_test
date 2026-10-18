//! detect_intra_shape_corners（B 主基线；0x141cd1a90，size 8769）。
//!
//! 2026-10-17 restorer-clutter-remaining：按 fresh 反编译重建
//! （`evidence/decomp/0x141cd1a90_*.c` + `0x141cd3ce0_*.c`、`evidence/disasm1.txt`）。
//!
//! 控制流（O）：
//! - `dirty_walls()`（0x141cd16b0）汇脏墙集；三个 retain：`Vec<Corner>`、
//!   `HashMap<[WallId;2],Vec<Vec2>>`、`Vec<ShapeIntersectionHole>`。
//! - 脏墙对循环：`Roof::ty()` → 平顶门控（`is_flat`；两墙任一 flat 即处理）→
//!   `max_y()`；角点高度 `y = max_y + (ty ^ 1) * -0.67`（0xbf2b851f = -0.67）。
//! - 形状交线：`ShapeParameters::expand ×2` + `contains_shape`；
//!   交点经 `add_hole_at_shape_intersection`（0x141cd3ce0）：
//!   `WallSpaceCoord::from_world_space ×2` → `wall_space/subtract/project_clamp`，
//!   洞记录 stride 0x14：+0 世界 x、+4 世界 y、+8 高度差、+0xc Δ、+0x10 kind
//!   （5=墙下，1=墙上；仅 `0.0 < 高度差` 时写字段并置 1）。
//! - 收尾逐条 `WallHoles::add()`。
//!
//! U 登记：`intersect_shapes` 的曲线交点采样（0x140edbf60 未逐行反编译）为 I；
//! `WallSpaceCoord` 的世界→墙空间投影按 utils 语义，精确 u/y 取整为 I。

use bevy_ecs::prelude::*;
use country_core::resources::walls::wall_holes::geometry::RawWallHole;
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::wall_holes::WallHoles;
use country_core::resources::walls::WallId;
use country_core::systems::wall::valid_enclosure::ShapeParameters;
use glam::{Vec2, Vec3};
use hashbrown::HashMap;
use utils::wall_space::{WallSpace, WallSpaceCoord};

use crate::inter_roof_merlons::detect_inter_roof_merlons::WallAndRoofChanges;

/// 角点记录（Vec<Corner> retain L115）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Corner {
    pub wall: WallId,
    pub pos: [f32; 2],
    /// 角点高度（墙顶 + (ty^1)×-0.67）。
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

    // 主循环：每脏墙对。
    let dirty: Vec<WallId> = changes.dirty_walls.iter().copied().collect();
    for i in 0..dirty.len() {
        for j in (i + 1)..dirty.len() {
            let (a, b) = (dirty[i], dirty[j]);
            let (Some(wall_a), Some(wall_b)) = (walls.get(a), walls.get(b)) else {
                continue;
            };
            // 平顶门控（O：两墙任一 Roof::is_flat）。
            let roof_a = walls.roof_entity(a).and_then(|e| roof_query.get(e).ok());
            let roof_b = walls.roof_entity(b).and_then(|e| roof_query.get(e).ok());
            let flat = roof_a.map_or(false, |r| r.is_flat())
                || roof_b.map_or(false, |r| r.is_flat());
            if !flat {
                continue;
            }
            // O：flag = ty ^ 1（ty = Roof::is_pitched，0x1408e... `ty_observed`）；y = max_y + flag × -0.67。
            let ty = roof_a.map_or(0u8, |r| r.is_pitched() as u8);
            let flag = (ty ^ 1) & 0xff;
            let y = wall_a.max_y() + flag as f32 * -0.67;
            let center = ShapeParameters::from_snapshot(wall_a.shape()).center();
            corners.0.push(Corner {
                wall: a,
                pos: [center.x, center.y],
                y,
            });

            // 墙对形状交线（O：expand ×2 + contains_shape + intersect_shapes）。
            let shape_a = ShapeParameters::from_snapshot(wall_a.shape());
            let shape_b = ShapeParameters::from_snapshot(wall_b.shape());
            if let Some(hole) = intersect_shapes(shape_a, shape_b, a, b, y) {
                holes.0.push(ShapeIntersectionHole { wall: a, hole });
            }
        }
    }
    // O：收尾逐条 WallHoles::add()。
    for h in holes.0.iter() {
        let _ = wall_holes.add(h.wall, h.hole);
    }
}

/// `intersect_shapes`（0x140edbf60）：两形状曲线在高度 y 的交点 → 洞。
///
/// U：曲线交点采样（双周长最近邻、阈值 0.2）为 I；洞字段按 0x141cd3ce0 的
/// wall-space 语义构造。
pub fn intersect_shapes(
    shape_a: ShapeParameters,
    shape_b: ShapeParameters,
    wall_a: WallId,
    _wall_b: WallId,
    y: f32,
) -> Option<RawWallHole> {
    let steps = 64usize;
    let mut best: Option<(f32, Vec2, Vec2)> = None;
    for i in 0..steps {
        let ua = i as f32 / steps as f32;
        let pa = shape_a.get_pos_at_u(ua);
        for j in 0..steps {
            let ub = j as f32 / steps as f32;
            let pb = shape_b.get_pos_at_u(ub);
            let d = (pa - pb).length();
            if d < 0.2 && best.map_or(true, |(bd, _, _)| d < bd) {
                best = Some((d, pa, pb));
            }
        }
    }
    let (_, pa, pb) = best?;
    let record = wall_space_hole(&shape_a, pa, pb, y);
    if record.kind != 5 {
        Some(RawWallHole {
            center: [record.x, record.y],
            dimensions: [record.delta, record.height],
            kind: record.kind,
        })
    } else {
        None
    }
}

/// 0x141cd3ce0 洞记录（stride 0x14）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShapeIntersectionRecord {
    /// +0x00：世界 x。
    pub x: f32,
    /// +0x04：世界 y。
    pub y: f32,
    /// +0x08：高度差。
    pub height: f32,
    /// +0x0c：Δ。
    pub delta: f32,
    /// +0x10：kind（5=墙下，1=墙上）。
    pub kind: u8,
}

/// `add_hole_at_shape_intersection`（0x141cd3ce0）：两世界点 → 墙空间 → 洞记录。
///
/// O：`from_world_space` ×2 → `wall_space().subtract()`；若 y<0 加墙宽
/// （`wall+0x30`）后再 `project_clamp`；`0.0 < 高度差` 才写字段并置 kind=1，
/// 否则 kind=5。
pub fn wall_space_hole(
    shape: &ShapeParameters,
    pa: Vec2,
    pb: Vec2,
    world_y: f32,
) -> ShapeIntersectionRecord {
    let curve = shape.as_curve2();
    let space = WallSpace {
        is_loop: shape.tag() == 0,
        length: shape.perimeter(),
    };
    let a = WallSpaceCoord::from_world_space(Vec3::new(pa.x, world_y, pa.y), &curve);
    let b = WallSpaceCoord::from_world_space(Vec3::new(pb.x, world_y, pb.y), &curve);
    let diff = a.subtract(&space, b);
    let mut height = diff.y;
    let mut du = diff.x;
    if height < 0.0 {
        // O：`fVar5 += wall+0x30; fVar4 += wall+0x30`（墙宽补偿）。
        let wall_width = space.length * 0.5;
        height += wall_width;
        du += wall_width;
    }
    // O：project_clamp（WallSpaceCoord::new 走 clamp）。
    let clamped = WallSpaceCoord::new(&space, du, height);
    let mut rec = ShapeIntersectionRecord {
        x: pa.x,
        y: world_y,
        height: clamped.y,
        delta: pa.y - pb.y,
        kind: 5,
    };
    if 0.0 < clamped.y {
        rec.x = pa.x;
        rec.y = clamped.y;
        rec.height = clamped.y;
        rec.delta = pa.y - pb.y;
        rec.kind = 1;
    }
    rec
}

/// `add_hole_at_shape_intersection`（0x141cd3ce0）形状入口（供 merlon 检测复用）。
pub fn add_hole_at_shape_intersection(
    shape_a: ShapeParameters,
    shape_b: ShapeParameters,
    wall: WallId,
    y: f32,
) -> Option<RawWallHole> {
    intersect_shapes(shape_a, shape_b, wall, wall, y)
}
