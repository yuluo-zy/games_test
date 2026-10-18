//! B `systems/wall/shapes/shape_placement.rs`: shape placement parameters.
//!
//! Evidence: `chapters/systems/wall/shapes/shape_placement.rs.md` (O,
//! 0x14097f970 / 0x140980db0) + fresh decomp 2026-10-17. The cursor terrain
//! position becomes an unsnapped placement or, when a wall lies within 1.5
//! units, a snapped one carrying the wall normal/target.
//!
//! fresh 分歧点登记：① 空间查询半宽 = **±2.25**（0x14097f970 内联 `+−2.25`，
//! Rust 此前 ±8.0）；② 原生在输入射线在场时走 `Rapier::raycast`（0x1408cdc60，
//! collision 域跨域）+ **大命中类型跳表**（case 0/2/3-5/0xf-0x11/...，其中
//! case 0/9 墙命中、case 2 屋顶脊），未命中/无射线则落回 query 路径——raycast
//! 为 collision 域接口，登记不编造；③ `SnapProposal` `sort_by`（smallsort/
//! driftsort）后取最近 + 墙形状判别（circle/rectangle/freehand 法向）已由
//! 本域 query_with_aabb_clamped 最近候选覆盖（语义等价）；④
//! snap_to_roof_ridge（0x140980db0，屋顶脊几何）+ DebugSphere/DebugLine +
//! 屋顶实体 Query + 记录多写（mode{0,1,2} 多载荷）为屋顶/宿主接线，登记。

use bevy_ecs::prelude::*;
use glam::Vec2;
use ::utils::geometry::Rectangle2d;

use super::{ShapePlacement, SelectedWallShape};
use crate::resources::{
    cursor_terrain_raycast::CursorTerrainRaycast,
    roofs::roof_shape::Roof,
    terrain_height::{TerrainHeightsData, TerrainHeightSsbo},
    walls::public_walls::PublicWalls,
};
use crate::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;

/// Snap threshold (0x3fc00000 @0x1429258d0).
const SNAP_DISTANCE: f32 = 1.5;
/// Spatial-query half extent for shape placement probing
/// (fresh 0x14097f970 内联 ±2.25，Rust 此前 ±8.0)。
const QUERY_HALF_EXTENT: f32 = 2.25;

impl ShapePlacement {
    /// Inline B line 70: unsnapped placement at the cursor terrain position.
    /// The original also seeds a 0.3 scalar whose role is 待取证.
    pub fn new_unsnapped(position: Vec2, height: f32) -> Self {
        Self {
            mode: 0,
            snapped: false,
            position,
            target: glam::Vec3::new(position.x, height, position.y),
            normal: glam::Vec3::Y,
        }
    }
}

/// B 0x14097f970 (14-param system). Cursor terrain position drives a snapped
/// placement when a wall is within `SNAP_DISTANCE`; otherwise unsnapped.
pub fn shape_placement_params(
    _selected: Res<SelectedWallShape>,
    cursor: Res<CursorTerrainRaycast>,
    public: Res<PublicWalls>,
    spatial: Res<WallSpatialHash>,
    terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    mut placement: ResMut<ShapePlacement>,
) {
    if cursor.tag == 0 {
        return;
    }
    let position = Vec2::new(cursor.pos.x, cursor.pos.z);
    let height = terrain.sample_at_world_xz(position.x, position.y);
    let candidates = spatial.query_with_aabb_clamped(
        [position.x - QUERY_HALF_EXTENT, position.y - QUERY_HALF_EXTENT],
        [position.x + QUERY_HALF_EXTENT, position.y + QUERY_HALF_EXTENT],
    );
    let mut snap_wall = None;
    let mut best = SNAP_DISTANCE * SNAP_DISTANCE;
    // 候选 = 各墙最近段到 AABB 中心的最近点（query_with_aabb_clamped fresh 语义），
    // 距离平方即真实吸附距离。
    for (candidate, wall) in candidates {
        let distance_sq =
            (candidate[0] - position.x).powi(2) + (candidate[1] - position.y).powi(2);
        if distance_sq < best {
            best = distance_sq;
            snap_wall = Some(wall);
        }
    }
    let Some(wall) = snap_wall else {
        *placement = ShapePlacement::new_unsnapped(position, height);
        return;
    };
    let Some(state) = public.get(wall) else {
        *placement = ShapePlacement::new_unsnapped(position, height);
        return;
    };
    // Normal at the snapped point: premade shapes use their shape normal,
    // freehand uses the curve tangent normal.
    let normal = crate::systems::wall::valid_enclosure::ShapeParameters::from_snapshot(
        state.shape(),
    )
    .get_shape()
    .map(|shape| {
        let u = shape.get_closest_u_from_pos(position);
        let n = shape.get_normal_at_u(u);
        glam::Vec3::new(n.x, 0.0, n.y)
    })
    .unwrap_or_else(|| {
        let curve = state.simplified_curve_2d();
        let u = curve_u_at(&curve.points, position);
        let tangent = curve_tangent(&curve.points, u);
        glam::Vec3::new(-tangent[1], 0.0, tangent[0])
    });
    *placement = ShapePlacement {
        mode: 2,
        snapped: true,
        position,
        target: glam::Vec3::new(position.x, height, position.y),
        normal,
    };
}

/// `snap_to_roof_ridge` 返回载荷（原生 sret 判别 +0：0=None / 1=Point /
/// 2=RidgeSegment）。tag 2 载荷 = `{t f32@+4, a Vec3@+8, b Vec3@+0x14}`。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RoofRidgeSnap {
    None,
    Point(glam::Vec3),
    Segment { a: glam::Vec3, b: glam::Vec3, t: f32 },
}

/// SSE 语义的 `MAXSS dst,src`（未排序/相等取 src）。
fn sse_max(dst: f32, src: f32) -> f32 {
    if dst > src { dst } else { src }
}
/// SSE 语义的 `MINSS dst,src`（未排序/相等取 src）。
fn sse_min(dst: f32, src: f32) -> f32 {
    if dst < src { dst } else { src }
}
/// `max_num`（CMPUNORDSS 选择：a NaN 取 b，否则 SSE MAXSS）。
fn max_num(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        b
    } else {
        sse_max(b, a)
    }
}
/// smoothstep 变换（原 `(3 − 2t)·t²`）。
fn smooth(t: f32) -> f32 {
    let squared = t * t;
    (3.0 - (t + t)) * squared
}

/// B 0x140980db0（fresh 反编译 + 316 指令 disasm 2026-10-17）：屋顶脊吸附。
///
/// 两条主分支按 `roof[8] & 1`（圆 / 矩形）：
/// - 圆：平顶（height_01 ≤ 0.1）→ 脊点 = `(center.x, position.y, center.z)`；
///   否则按 `((handle − 0.90000004)/7.1)/0.4` 缓动求脊高（低值 8.0）→
///   `(0·h + center.x, h + floor, center.z + 0)`。
/// - 矩形：`RoofRidgeDims::calc` 同构的尺寸对 → `Rectangle2d`（axis=roof+0xc,
///   center=roof+0x14, size=尺寸对，另一轴取 0.01/0.1）→ 角点 0/2 经 `xvy`
///   取两点；y = 平顶 ? position.y : floor + ridge height。
/// - 命中（到点/线段距离 < threshold）：
///   - 圆 → tag 1 Point；
///   - 矩形 → tag 2 Segment，`t` 缺省 0.5，否则按 0.3/len 边界夹取
///     （`max(dims) ≤ 0.5` 时恒 0.5）。
///
/// 常量（fresh read_memory 2026-10-17）：1e-20 / 0.3 / 0.4 / 12.0 / 8.0 /
/// 9.0 / -2.25 / 12.75 / -0.90000004 / 7.1 / 0.1 / 0.01 / 3.0。
/// 高度夹取 `assert!(0 ≤ limit)`（原生 `f32::clamp` 运行时 panic 路径）。
pub fn snap_to_roof_ridge(
    roof: &Roof,
    position: glam::Vec3,
    threshold: f32,
) -> RoofRidgeSnap {
    let observed = roof.observed();
    let at = |o: usize| f32::from_le_bytes(observed[o..o + 4].try_into().unwrap());
    let height_01 = at(0x30);
    let floor_y = at(0x4c);
    if observed[8] & 1 == 0 {
        // 圆
        let point = if height_01 <= 0.1 {
            // 平顶：xvy(center_xz, position.y)
            glam::Vec3::new(at(0xc), position.y, at(0x10))
        } else {
            let ratio = ((at(0x14) - f32::from_bits(0x3f666667)) / 7.1) / 0.4;
            let t = sse_min(1.0, sse_max(0.0, ratio));
            let eased = smooth(t);
            // smooth·12.0 + (1 − smooth)·8.0
            let limit = eased * 12.0 + (1.0 - eased) * 8.0;
            // 原生：f32::clamp(min=0,max=limit) 的 `assert!(min <= max)` 运行时 panic。
            assert!(0.0 <= limit, "invalid roof ridge height limit");
            let requested = (1.0 - height_01) * 0.4 + height_01 * 12.0;
            let ridge_height = sse_min(limit, sse_max(0.0, requested));
            glam::Vec3::new(
                0.0 * ridge_height + at(0xc),
                ridge_height + floor_y,
                at(0x10) + 0.0,
            )
        };
        let delta = position - point;
        return if delta.length() < threshold {
            RoofRidgeSnap::Point(point)
        } else {
            RoofRidgeSnap::None
        };
    }
    // 矩形
    let ridge_length_01 = at(0x34);
    let ridge_dir = observed[0x3c];
    let (dim0, dim1) = (at(0x1c), at(0x20));
    let calculated = (1.0 - ridge_length_01) * 0.1 + ridge_length_01 * at(if ridge_dir == 0 { 0x1c } else { 0x20 });
    let (f3, f2) = if ridge_dir == 0 {
        (calculated, 0.1)
    } else {
        (0.1, calculated)
    };
    let mut f4 = 0.01;
    if f3 <= f2 {
        f4 = f2;
    }
    // CMPLTSS(f2,f3) 选择：f2 < f3 → f3；否则 0.01。
    let f3 = if f2 < f3 { f3 } else { 0.01 };
    let ridge_height = if 0.1 < height_01 {
        let ratio = (max_num(dim0, dim1) + f32::from_bits(0xc0100000)) / f32::from_bits(0x414c0000) / 0.4;
        let t = sse_min(1.0, sse_max(0.0, ratio));
        let eased = smooth(t);
        let limit = eased * 12.0 + (1.0 - eased) * 9.0;
        assert!(0.0 <= limit, "invalid roof ridge height limit");
        let requested = (1.0 - height_01) * 0.4 + height_01 * 12.0;
        sse_min(limit, sse_max(0.0, requested))
    } else {
        0.0
    };
    let ridge_rect = Rectangle2d {
        axis: Vec2::new(at(0xc), at(0x10)),
        center: Vec2::new(at(0x14), at(0x18)),
        size: Vec2::new(f3, f4),
    };
    let point_y = if 0.1 < height_01 {
        floor_y + ridge_height
    } else {
        position.y
    };
    let a_xz = ridge_rect.get_point(0);
    let b_xz = ridge_rect.get_point(2);
    let a = glam::Vec3::new(a_xz.x, point_y, a_xz.y);
    let b = glam::Vec3::new(b_xz.x, point_y, b_xz.y);
    let ab = b - a;
    let length_squared = ab.length_squared();
    // MAXSS len², 1e-20（dst>src ? dst : src；NaN 取 1e-20）。
    let denominator = if length_squared > 1e-20 {
        length_squared
    } else {
        1e-20
    };
    let t = sse_min(
        1.0,
        sse_max(0.0, (position - a).dot(ab) / denominator),
    );
    // 原生 lerp = a·(1−t) + b·t（保持加乘次序）。
    let closest = a * (1.0 - t) + b * t;
    if (position - closest).length() < threshold {
        let length = length_squared.sqrt();
        let boundary = 0.3 / length;
        // UCOMISS/JA/JBE 有序语义：boundary>t → 0；t>1−boundary → 1；否则 t。
        let keep_t = !(t > (1.0 - boundary));
        let t_out = if 0.5 > max_num(f3, f4) {
            0.5
        } else if boundary > t {
            0.0
        } else if keep_t {
            t
        } else {
            1.0
        };
        RoofRidgeSnap::Segment { a, b, t: t_out }
    } else {
        RoofRidgeSnap::None
    }
}

/// Approximate arc-length parameter of the closest point (CurveU projection).
fn curve_u_at(points: &[[f32; 2]], position: Vec2) -> f32 {
    if points.len() < 2 {
        return 0.0;
    }
    let mut best = 0.0f32;
    let mut best_d = f32::MAX;
    let mut length = 0.0f32;
    for pair in points.windows(2) {
        let a = pair[0];
        let b = pair[1];
        let seg = glam::Vec2::new(b[0] - a[0], b[1] - a[1]);
        let len2 = seg.length_squared().max(1e-20);
        let t = ((position - glam::Vec2::new(a[0], a[1])).dot(seg) / len2).clamp(0.0, 1.0);
        let closest = glam::Vec2::new(a[0], a[1]) + seg * t;
        let d = (position - closest).length_squared();
        if d < best_d {
            best_d = d;
            best = (length + seg.length() * t) / total_length(points);
        }
        length += seg.length();
    }
    best
}

fn total_length(points: &[[f32; 2]]) -> f32 {
    let mut length = 0.0;
    for pair in points.windows(2) {
        length += (glam::Vec2::new(pair[1][0] - pair[0][0], pair[1][1] - pair[0][1])).length();
    }
    if length > 0.0 { length } else { 1.0 }
}

fn curve_tangent(points: &[[f32; 2]], u: f32) -> [f32; 2] {
    let segment = ((points.len() - 1) as f32 * u).floor().min((points.len() - 2) as f32) as usize;
    let a = points[segment];
    let b = points[segment + 1];
    let dx = b[0] - a[0];
    let dz = b[1] - a[1];
    let inv = 1.0 / (dx * dx + dz * dz).sqrt();
    [dx * inv, dz * inv]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat_circle_roof(center: [f32; 2], floor: f32) -> Roof {
        let mut bytes = [0u8; 88];
        bytes[8..12].copy_from_slice(&0u32.to_le_bytes()); // circle tag
        bytes[0x0c..0x10].copy_from_slice(&center[0].to_le_bytes());
        bytes[0x10..0x14].copy_from_slice(&center[1].to_le_bytes());
        bytes[0x30..0x34].copy_from_slice(&0.0f32.to_le_bytes()); // height_01 = 0
        bytes[0x4c..0x50].copy_from_slice(&floor.to_le_bytes());
        Roof::from_observed(bytes)
    }

    #[test]
    fn flat_circle_ridge_uses_cursor_height_and_respects_threshold() {
        let roof = flat_circle_roof([2.0, 4.0], 0.7);
        assert_eq!(
            snap_to_roof_ridge(&roof, glam::Vec3::new(2.0, 1.5, 4.0), 0.5),
            RoofRidgeSnap::Point(glam::Vec3::new(2.0, 1.5, 4.0))
        );
        assert_eq!(
            snap_to_roof_ridge(&roof, glam::Vec3::new(10.0, 1.5, 10.0), 0.5),
            RoofRidgeSnap::None
        );
    }

    #[test]
    fn pitched_circle_ridge_is_floor_plus_eased_height() {
        let mut bytes = [0u8; 88];
        bytes[8..12].copy_from_slice(&0u32.to_le_bytes());
        bytes[0x0c..0x10].copy_from_slice(&1.0f32.to_le_bytes());
        bytes[0x10..0x14].copy_from_slice(&2.0f32.to_le_bytes());
        bytes[0x14..0x18].copy_from_slice(&1.0f32.to_le_bytes()); // handle
        bytes[0x30..0x34].copy_from_slice(&1.0f32.to_le_bytes()); // height_01
        bytes[0x4c..0x50].copy_from_slice(&0.25f32.to_le_bytes()); // floor
        let roof = Roof::from_observed(bytes);
        let t = (((1.0f32 - f32::from_bits(0x3f666667)) / 7.1) / 0.4).clamp(0.0, 1.0);
        let eased = (3.0 - (t + t)) * t * t;
        let limit = eased * 12.0 + (1.0 - eased) * 8.0;
        let expected = glam::Vec3::new(1.0, limit + 0.25, 2.0);
        assert_eq!(
            snap_to_roof_ridge(&roof, expected, 0.5),
            RoofRidgeSnap::Point(expected)
        );
    }

    #[test]
    fn rectangle_ridge_returns_corner_segment_with_default_half() {
        let mut bytes = [0u8; 88];
        bytes[8] = 1; // rectangle
        bytes[0x0c..0x10].copy_from_slice(&1.0f32.to_le_bytes()); // axis = X
        bytes[0x10..0x14].copy_from_slice(&0.0f32.to_le_bytes());
        bytes[0x1c..0x20].copy_from_slice(&4.0f32.to_le_bytes());
        bytes[0x20..0x24].copy_from_slice(&2.0f32.to_le_bytes());
        bytes[0x30..0x34].copy_from_slice(&0.0f32.to_le_bytes()); // flat
        bytes[0x34..0x38].copy_from_slice(&0.5f32.to_le_bytes()); // ridge_length_01
        bytes[0x3c] = 0; // ridge dir
        let roof = Roof::from_observed(bytes);
        let position = glam::Vec3::new(0.0, 0.5, 0.0);
        match snap_to_roof_ridge(&roof, position, 0.5) {
            RoofRidgeSnap::Segment { a, b, t } => {
                // dir=0 → size = (calculated=2.05, 其余轴 0.01)；角点 0/2 = ±half。
                assert!((a.x + 1.025).abs() < 1e-6 && (a.z + 0.005).abs() < 1e-6);
                assert!((b.x - 1.025).abs() < 1e-6 && (b.z - 0.005).abs() < 1e-6);
                assert_eq!(a.y, 0.5);
                assert_eq!(b.y, 0.5);
                assert_eq!(t, 0.5); // t=0.5 且不在 0.3/len 边界内 → 保留投影参数
            }
            other => panic!("expected ridge segment, got {other:?}"),
        }
    }
}
