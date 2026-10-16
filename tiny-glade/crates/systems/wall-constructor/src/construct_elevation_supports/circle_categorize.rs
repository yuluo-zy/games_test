//! 圆形判定（construct_elevation_supports/circle_categorize.rs）。
//!
//! `circle_categorize`（0x14124af60，re-analyst-C asm↔decomp↔.rdata 三方核对 O）：
//! - `if radius@[2] >= 1.75`（0x142c20388）否则返回 `pillars.len == 0`；
//! - `t = clamp01((r − 1.75)/1.25)`；`s = smoothstep(t) = (3−2t)t²`；
//! - `threshold = 0.6·s + 0.3·(1−s)`；
//! - 2r×2r 网格采样（step = 2r/(u32)(2r)）：形状内点计数 vs 支柱内点计数；
//! - 返回 `支柱占比 < threshold` ⇒ 圆形。
//! panic 源：`crates/systems/wall-constructor/src/construct_elevation_supports/
//! circle_categorize.rs`（0x142c203e8/20400）。
use glam::Vec2;

/// B `circle_categorize`（0x14124af60）：支柱覆盖占比低于阈值 ⇒ 圆形。
pub fn circle_categorize(
    radius: f32,
    pillars: &[Vec2],
    contains_point: impl Fn(Vec2) -> bool,
) -> bool {
    const RADIUS_MIN: f32 = f32::from_bits(0x3fe00000); // 1.75（0x142c20388）
    if radius < RADIUS_MIN {
        return pillars.is_empty();
    }
    let t = ((radius - RADIUS_MIN) / 1.25).clamp(0.0, 1.0);
    let s = (3.0 - 2.0 * t) * t * t;
    let threshold = 0.6 * s + 0.3 * (1.0 - s);
    let side = (radius * 2.0) as u32;
    let step = radius * 2.0 / side.max(1) as f32;
    let mut interior = 0u32;
    let mut pillar_hits = 0u32;
    for ix in 0..side {
        for iz in 0..side {
            let p = Vec2::new((ix as f32 + 0.5) * step, (iz as f32 + 0.5) * step);
            if contains_point(p) {
                interior += 1;
                for pillar in pillars {
                    if (pillar - p).length() <= step {
                        pillar_hits += 1;
                        break;
                    }
                }
            }
        }
    }
    if interior == 0 {
        return true;
    }
    (pillar_hits as f32 / interior as f32) < threshold
}
