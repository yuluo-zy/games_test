//! B `systems/wall/shapes/shape_placement_signifiers.rs`: placement preview.
//!
//! Evidence: `chapters/systems/wall/shapes/shape_placement_signifiers.rs.md`
//! (O, 0x1408dfc60 / 0x1408dec70). `preview_snapping_line` builds the
//! expanded premade-shape anchor used by snapping_signifiers. The native
//! signature takes the wall state + a parameter u and returns the **point on
//! the sliced curve at u**（`get_coord_at_u` + 3D 插值 + 多载荷字段）——Rust
//! 此前固定 `get_pos_at_u(0.0)`，fresh 证实为「参数 u 处点」（0x1408dfc60
//! tail：`get_coord_at_u<Vec2,2D,CurveU>` 取 slice 曲线 u 处点）。返回载荷
//! 字段（@+0xc/@+0x14 + 2×qword）与 freehand 分支（曲线切片/anyhow）为
//! 宿主/curve 域接线，登记。指示符绘制本身（DebugLine/DebugSphere、实体
//! spawn）是 render-host 待接线；库层写 [`UiAppendSignifier`] 记录。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use crate::resources::walls::public_wall_state::PublicWallState;
use crate::systems::ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream};
use crate::systems::wall::shapes::SelectedWallShape;
use crate::systems::wall::valid_enclosure::{Shape, ShapeParameters};

/// Circle expand radius (0x3ea147ae @0x142aa3280).
const CIRCLE_EXPAND: f32 = 0.315;
/// Rectangle per-axis expand (0x3f2147ae @0x142aa4550/4554) = 2 * CIRCLE_EXPAND.
const RECTANGLE_EXPAND: f32 = 0.63;
/// Preview line half-length scalar (caller stack arg 0x40900000 = 4.5).
const PREVIEW_LINE_LEN: f32 = 4.5;

/// B 0x1408dfc60: point on the expanded premade-shape curve / freehand curve
/// nearest to `position`（原生 `get_approx_u_from_pos` 得 u，再以
/// `slice(u−half, u+half)` + `get_coord_at_u(slice, 0.5)` 取切片中点 = u 处点；
/// 对称切片的中点即投影点）。`half = clamp(4.5 / curve_len, 0, 1) * 0.5`
/// 只影响切片范围，不改变中点。premade 曲线 y=0；freehand 取 3D 路径点。
pub fn preview_snapping_line(state: &PublicWallState, position: Vec2) -> Option<Vec3> {
    if let Some(shape_snapshot) = state.try_get_wall_shape() {
        let params = ShapeParameters::from_snapshot(shape_snapshot);
        let expanded = match params.tag() {
            0 => {
                let circle = params.as_circle();
                Shape::Circle(::utils::geometry::Circle2d {
                    center: circle.center,
                    radius: circle.radius + CIRCLE_EXPAND,
                    rotation: circle.rotation,
                })
            }
            1 => {
                let rect = params.as_rectangle();
                Shape::Rectangle(::utils::geometry::Rectangle2d::with_expanded_size(
                    rect,
                    Vec2::splat(RECTANGLE_EXPAND),
                ))
            }
            _ => return None,
        };
        let u = expanded.get_closest_u_from_pos(position);
        let anchor = expanded.get_pos_at_u(u);
        // 预置形状 as_curve3(max_step, height=0) → 预览点 y=0。
        return Some(Vec3::new(anchor.x, 0.0, anchor.y));
    }
    // tag > 1（freehand）：原生取 3D 曲线在投影 u 处点（切片中点）。
    let curve = state.simplified_curve_2d();
    if curve.points.len() < 2 || curve.length <= 0.0 {
        return None;
    }
    let u = approx_u_from_pos(&curve.points, &curve.points_u, position);
    let half = (PREVIEW_LINE_LEN / curve.length).clamp(0.0, 1.0) * 0.5;
    // tag==2：开曲线切片夹到 [0,1]；tag>2：circular_slice 环绕。中点即切片中点。
    let mid = if state.shape().tag() == 2 {
        let lo = (u - half).clamp(0.0, 1.0);
        let hi = (u + half).clamp(0.0, 1.0);
        (lo + hi) * 0.5
    } else {
        u.rem_euclid(1.0)
    };
    Some(freehand_point_at(state, mid))
}

/// 3D point on the freehand path at parameter `u`（原生 Curve3::get_coord_at_u
/// + 3D 插值）。优先使用 prepared path 的 3D positions；未准备时退回
/// 简化 2D 曲线 + outline（y=0）。
fn freehand_point_at(state: &PublicWallState, u: f32) -> Vec3 {
    if let Some(path) = &state.prepared_path {
        let pts = &path.positions;
        let us = &path.points_u;
        if pts.len() >= 2 && us.len() == pts.len() {
            if u <= us[0] {
                return Vec3::from_array(pts[0]);
            }
            let last = us.len() - 1;
            if u >= us[last] {
                return Vec3::from_array(pts[last]);
            }
            let mut i = 0usize;
            while i + 1 < us.len() && us[i + 1] <= u {
                i += 1;
            }
            let span = us[i + 1] - us[i];
            let t = if span > 0.0 { (u - us[i]) / span } else { 0.0 };
            let a = pts[i];
            let b = pts[i + 1];
            return Vec3::new(
                b[0] * t + a[0] * (1.0 - t),
                b[1] * t + a[1] * (1.0 - t),
                b[2] * t + a[2] * (1.0 - t),
            );
        }
    }
    let curve = state.simplified_curve_2d();
    let p = curve.pos_at_u(u);
    Vec3::new(p[0], 0.0, p[1])
}

/// `Curve::get_approx_u_from_pos`（0x140a6ad80 同类调用）：逐段投影取最近点，
/// 返回最近点处的弧长参数 u。
fn approx_u_from_pos(points: &[[f32; 2]], points_u: &[f32], position: Vec2) -> f32 {
    if points.len() < 2 {
        return 0.0;
    }
    let mut best_d = f32::MAX;
    let mut best_u = 0.0f32;
    for (i, pair) in points.windows(2).enumerate() {
        let a = Vec2::new(pair[0][0], pair[0][1]);
        let b = Vec2::new(pair[1][0], pair[1][1]);
        let seg = b - a;
        let len2 = seg.length_squared();
        let t = if len2 > 0.0 {
            ((position - a).dot(seg) / len2).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let closest = a + seg * t;
        let d = (position - closest).length_squared();
        if d < best_d {
            best_d = d;
            let ua = points_u.get(i).copied().unwrap_or(0.0);
            let ub = points_u.get(i + 1).copied().unwrap_or(1.0);
            best_u = ua + (ub - ua) * t;
        }
    }
    best_u
}

/// B 0x1408dec70: shape placement signifier system — draw the snapping preview
/// line and the shape outline. The outline spawn/mesh is host 待接线; the
/// library records the snapping anchor.
///
/// fresh 门控（2026-10-17）：`SelectedWallShape.tag == 6 && ty ∈ {1,2}` 且公共墙
/// `shape tag != 2`；`mode == 2` / else 两分支分别 format `"snap_signifier_"`
/// / `"snap_signifier_curve_"` 后 push（名字句柄与两分支载荷为 render-host
/// 登记；本函数写 domain 记录）。
pub fn shape_placement_signifiers(
    selected: Res<SelectedWallShape>,
    mode: Res<crate::systems::wall::snapping::SnappingMode>,
    public: Res<crate::resources::walls::public_walls::PublicWalls>,
    mut stream: ResMut<UiSignifierStream>,
) {
    if selected.tag != SelectedWallShape::WALL_SHAPES_TAG
        || (selected.ty != SelectedWallShape::TY_CIRCLE
            && selected.ty != SelectedWallShape::TY_RECTANGLE)
    {
        return;
    }
    if mode.wall.0 == 0 {
        return;
    }
    let Some(wall) = public.get(mode.wall) else {
        return;
    };
    if wall.shape().tag() == 2 {
        return;
    }
    // 原生调用方（shape_placement_signifiers）以参考点（snap 目标）作
    // position 传入 preview_snapping_line；函数内部按 get_approx_u_from_pos
    // 投影到（扩展后的）曲线上。
    let reference = Vec2::new(mode.snap.x, mode.snap.z);
    if let Some(anchor) = preview_snapping_line(wall, reference) {
        stream.write(UiAppendSignifier {
            tag: 3,
            kind: 0,
            position: anchor,
            scale: 0.0,
            name: 0,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roughly_projects_position_onto_polyline() {
        // 折线 (0,0)-(10,0)-(10,10)，points_u = 0/0.5/1。
        let points = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]];
        let points_u = [0.0, 0.5, 1.0];
        // (5,1) 投影到第一段 t=0.5 → u=0.25。
        assert!((approx_u_from_pos(&points, &points_u, Vec2::new(5.0, 1.0)) - 0.25).abs() < 1e-6);
        // (11,5) 投影到第二段 t=0.5 → u=0.75。
        assert!((approx_u_from_pos(&points, &points_u, Vec2::new(11.0, 5.0)) - 0.75).abs() < 1e-6);
    }

    #[test]
    fn freehand_wall_has_no_preview_anchor() {
        let mut world = World::new();
        world.insert_resource(crate::systems::wall::snapping::SnappingMode::new());
        world.insert_resource(crate::resources::walls::public_walls::PublicWalls::default());
        world.insert_resource(crate::systems::wall::shapes::SelectedWallShape {
            tag: crate::systems::wall::shapes::SelectedWallShape::WALL_SHAPES_TAG,
            ty: crate::systems::wall::shapes::SelectedWallShape::TY_CIRCLE,
        });
        world.init_resource::<UiSignifierStream>();
        let mut schedule = Schedule::default();
        schedule.add_systems(shape_placement_signifiers);
        schedule.run(&mut world);
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
    }
}
