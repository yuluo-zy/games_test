//! B `systems/wall/snapping/snapping_to_ends.rs`: wall-end snapping.
//!
//! Evidence: `chapters/systems/wall/snapping/snapping_to_ends.rs.md` (O,
//! 0x140a6ad80) + fresh disasm 2026-10-17（`snapping_to_ends-disasm.txt`）。
//! Only freehand walls participate; the in-progress wall uses its first raw
//! input point as the candidate endpoint. The closest end within 1.5 world
//! units becomes the mode-2 snap target.
//!
//! fresh 反编译/反汇编控制流（471 指令定案）：`mode != 1 → return` 门 +
//! in_progress.wall 必在（unwrap_failed，系统输入边登记不改）+ cursor flags
//! bit0 门（disasm `140a6ae01 TEST byte[RAX],0x1; JZ →{0,0}`）。**无
//! `snapped → break`**：遍历全部墙，仅当 snapped==0 才跳过当前墙；snapped==1
//! 时当前墙也参与（raw 首点 + flags bit0 端标志）。**snap 点 = 墙端点 3D 位置**：
//! `get_coord_at_u<Vec2,CurveCoordSemantic2D>(curve, u∈{0,1})`（disasm
//! `140a6b302 MOVAPS XMM1,XMM6`，XMM6=0.0/1.0 端点标志）→ segment,t →
//! **outline 3D 点按 t 插值**。`mode.end` 极性：起点选中 → 1、终点选中 → 0
//! （disasm `140a6b1e6 MOV AL,0x1` / `140a6b1f2 XOR EAX,EAX`）。

use bevy_ecs::prelude::*;
use glam::Vec2;

use super::{SNAP_MODE_END, SNAP_MODE_SEGMENT_CANDIDATE, SnappingMode};
use crate::resources::{
    cursor_terrain_raycast::CursorTerrainRaycast,
    in_progress_curve::InProgressCurve,
    walls::public_walls::PublicWalls,
};

/// Minimum distance sentinel (0x4cbebc20 @0x142ad4950).
const MIN_DISTANCE_SENTINEL: f32 = 1.0e8;
/// End-snap distance threshold (0x3fc00000 @0x1429258d0).
const SNAP_DISTANCE: f32 = 1.5;

/// B 0x140a6ad80 (4-param system).
pub fn snapping_to_ends(
    mut mode: ResMut<SnappingMode>,
    in_progress: Res<InProgressCurve>,
    cursor: Res<CursorTerrainRaycast>,
    public: Res<PublicWalls>,
) {
    if mode.mode != SNAP_MODE_SEGMENT_CANDIDATE {
        return;
    }
    let Some(_) = in_progress.wall else {
        // fresh：in_progress.wall 必在（unwrap_failed）。系统输入边，Rust return 登记。
        return;
    };
    // fresh disasm `140a6ae01 TEST byte[RAX],0x1; JZ →{0,0}`：cursor flags bit0
    // 未置位时参考点 = (0,0)，置位才取游标 (pos.x, pos.z)。
    let cursor = if cursor.tag & 1 != 0 {
        Vec2::new(cursor.pos.x, cursor.pos.z)
    } else {
        Vec2::ZERO
    };
    let snapped = mode.snapped;
    let mut best: Option<(crate::resources::walls::WallId, u8)> = None;
    let mut min_distance = MIN_DISTANCE_SENTINEL;
    // 端点 u 标志（∈{0,1}）：disasm XMM6 = 0.0(起点)/1.0(终点)。
    let mut best_u = 0.0f32;

    for (id, state) in public.all() {
        state.assert_valid();
        if state.shape().tag() != 2 {
            continue; // only freehand walls expose ends to snap to.
        }
        // fresh 双路径：snapped==0 跳过当前墙；snapped==1 当前墙参与。
        if !snapped && id == in_progress.current_wall {
            continue;
        }
        let (candidate, end_flag, u) = if id == in_progress.current_wall {
            // The in-progress wall reuses its first raw input point; its flag
            // bit 0 forces the occupancy flag and the end flag (u=1.0).
            if in_progress.raw_input_points.is_empty() {
                continue;
            }
            let point = in_progress.first_raw_input_point();
            let end_flag = if in_progress.flags & 1 != 0 { 1u8 } else { 0u8 };
            let u = if in_progress.flags & 1 != 0 { 1.0f32 } else { 0.0f32 };
            (point, end_flag, u)
        } else {
            let points = state.simplified_curve_2d().points.clone();
            if points.len() < 2 {
                continue;
            }
            let start = points[0];
            let end = points[points.len() - 1];
            let d_start = ((start[0] - cursor.x).powi(2) + (start[1] - cursor.y).powi(2)).sqrt();
            let d_end = ((end[0] - cursor.x).powi(2) + (end[1] - cursor.y).powi(2)).sqrt();
            // disasm：`UCOMISS dist_end,dist_start; JBE → 终点`——起点更近 → end_flag=1、
            // u=0；否则（终点 ≤ 起点）→ end_flag=0、u=1。
            if d_start < d_end {
                (Vec2::new(start[0], start[1]), 1u8, 0.0f32)
            } else {
                (Vec2::new(end[0], end[1]), 0u8, 1.0f32)
            }
        };
        let distance = (candidate - cursor).length();
        if distance < min_distance {
            min_distance = distance;
            best = Some((id, end_flag));
            best_u = u;
        }
    }

    if min_distance >= SNAP_DISTANCE {
        return;
    }
    let Some((best_id, best_end)) = best else {
        return;
    };
    let state = public.get(best_id).unwrap();
    // 3D snap 点 = 墙端点（u∈{0,1}）3D 位置：简化 2D 曲线 `coord_at_u(u)` 取
    // (segment,t)，outline 3D 点按 t 插值（原生 get_coord_at_u + 3D 插值语义）。
    let curve = state.simplified_curve_2d();
    let (segment, t) = curve.coord_at_u(best_u);
    let outline = state.geometry.outline.as_slice();
    let a = outline[segment];
    let b = outline[segment + 1];
    let snap = [
        b[0] * t + a[0] * (1.0 - t),
        b[1] * t + a[1] * (1.0 - t),
        b[2] * t + a[2] * (1.0 - t),
    ];

    mode.mode = SNAP_MODE_END;
    mode.snapped = state.building_projection().byte_0x29e() != 0;
    mode.snap = glam::Vec3::new(snap[0], snap[1], snap[2]);
    mode.wall = best_id;
    mode.end = best_end;
}
