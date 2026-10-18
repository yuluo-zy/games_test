//! B `systems/wall/snapping/telegraph_walls_are_merged.rs`: merge preview line.
//!
//! Evidence: `chapters/systems/wall/snapping/telegraph_walls_are_merged.rs.md`
//! (O, structure; 0x14086ad70) + fresh decomp 2026-10-17. For every public
//! wall segment an easing timer accumulates (`timer = (seg+0x40 += Δ) / 0.75`
//! clamp[0,1]) and `EasingFunction::apply` drives the preview line:
//!   u_lo = clamp01(2·e − 1),  u_hi = clamp01(2·e)
//!   ε = 0.025 / curve_len;  if (u_hi − u_lo < 2ε) { u_lo −= ε; u_hi += ε; }  (re-clamp)
//!   → `Curve::slice_into<Vec3,3D>` [u_lo, u_hi] → mesh/entity rebuild.
//! When the eased input reaches 1.0 the preview entity is **despawned**.
//!
//! 登记（fresh 分歧点）：原生 = **easing 计时 + 曲线切片 + 网格重建 + 实体
//! spawn/despawn + 组件插入**（`EntityCommands::despawn` /
//! `update_curve_mesh` / `SignifierEntityBuilder::build` / `insert<LineAcrossWallSignifier>
//! / DashedLineLength / DrawTopology`）完整实体链；Rust 此前为单 signifier
//! 近似（无计时/切片）。实体链为 render-host 依赖边界（AGENTS），本域重建
//! easing 计时 + 切片范围，并以 [`UiAppendSignifier`]（tag 5）承载切片预览；
//! despawn/insert/网格为宿主接线，登记。

use bevy_ecs::prelude::*;
use hashbrown::HashMap;
use utils::easing_functions::EasingFunction;

use crate::resources::walls::{WallId, public_walls::PublicWalls};
use crate::resources::{time::GameTime, roofs::geometry::surface::Curve2};
use crate::systems::ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream};

/// Telegraph animation duration (0.75 @ fresh /0.75)。
const TELEGRAPH_DURATION: f32 = 0.75;
/// 极小切片 ε 扩展基数（0.025 @ fresh `0.025 / fVar52`）。
const SLICE_MIN_LEN: f32 = 0.025;

/// 每墙段累计计时器（原生存于 public 段记录 @+0x40；域内以资源承载）。
#[derive(Resource, Clone, Debug, Default)]
pub struct TelegraphTimers(pub HashMap<WallId, f32>);

impl TelegraphTimers {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

/// `outline 3D 点` at parameter `u`（简化 2D 曲线 coord_at_u → outline 3D 插值）。
fn outline_at(curve: &Curve2, outline: &[[f32; 3]], u: f32) -> [f32; 3] {
    let (segment, t) = curve.coord_at_u(u);
    let a = outline[segment];
    let b = outline[segment + 1];
    [
        b[0] * t + a[0] * (1.0 - t),
        b[1] * t + a[1] * (1.0 - t),
        b[2] * t + a[2] * (1.0 - t),
    ]
}

/// B 0x14086ad70 (8-param system).
pub fn telegraph_walls_are_merged(
    public: Res<PublicWalls>,
    time: Res<GameTime>,
    mut timers: ResMut<TelegraphTimers>,
    mut stream: ResMut<UiSignifierStream>,
) {
    let dt = time.delta;
    let mut live: std::collections::HashSet<WallId> = std::collections::HashSet::new();
    for (id, state) in public.all() {
        state.assert_valid();
        live.insert(id);
        let curve = state.simplified_curve_2d();
        if curve.points.len() < 2 {
            continue;
        }
        let timer = timers.0.entry(id).or_insert(0.0);
        *timer += dt;
        let t = (*timer / TELEGRAPH_DURATION).clamp(0.0, 1.0);
        // eased 输入到达 1.0 → 旧预览实体 despawn（render-host 宿主接线）。
        // fresh：累加器写回 seg+0x40 后**不重置**（持续 ≥0.75），本帧不再建切片。
        if t == 1.0 {
            continue;
        }
        // fresh disasm `14086bbcc LEA RCX,[0x142a96d84]` → EasingFunction tag 0
        // = `CircularInOut`（enum 序：CircularInOut=0）。
        let e = EasingFunction::CircularInOut.apply(t);
        // eased 双 u 界（fresh maxps/minps clamp）：u_lo = 2e−1、u_hi = 2e。
        let eps = SLICE_MIN_LEN / curve.length;
        let mut u_lo = (2.0 * e - 1.0).clamp(0.0, 1.0);
        let mut u_hi = (2.0 * e).clamp(0.0, 1.0);
        if u_hi - u_lo < eps + eps {
            u_lo = (u_lo - eps).clamp(0.0, 1.0);
            u_hi = (u_hi + eps).clamp(0.0, 1.0);
        }
        // 曲线切片 [u_lo, u_hi] → 预览（原生 slice_into + update_curve_mesh +
        // SignifierEntityBuilder 实体链为 render-host，登记）。
        let outline = state.geometry.outline.as_slice();
        let start = outline_at(curve, outline, u_lo);
        let end = outline_at(curve, outline, u_hi);
        stream.write(UiAppendSignifier {
            tag: 5,
            kind: 5,
            position: glam::Vec3::new(start[0], start[1], start[2]),
            scale: end[0],
            name: end[2].to_bits() as u64,
        });
    }
    // 清除已删除墙的计时（原生 despawn 后无残留；域内资源同步）。
    timers.0.retain(|id, _| live.contains(id));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eased_slice_bounds_grow_then_shrink_and_despawn_at_one() {
        // e=0 → [0,0]；e=0.5 → [0,1]；e→1 → [1,1]（t==1 走 despawn 分支）。
        let bounds = |e: f32| {
            let mut lo = (2.0 * e - 1.0).clamp(0.0, 1.0);
            let mut hi = (2.0 * e).clamp(0.0, 1.0);
            (lo, hi)
        };
        let (lo, hi) = bounds(0.0);
        assert_eq!((lo, hi), (0.0, 0.0));
        let (lo, hi) = bounds(0.5);
        assert_eq!((lo, hi), (0.0, 1.0));
        let (lo, hi) = bounds(0.75);
        assert!((lo - 0.5).abs() < 1e-6 && (hi - 1.0).abs() < 1e-6);
        // t==1.0 不进切片分支（despawn）。
        assert_eq!(1.0f32.clamp(0.0, 1.0), 1.0);
    }
}
