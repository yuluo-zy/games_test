//! tree_move_bend.rs — 树移动/弯折插值（B 0x1422689c0，O；2026-10-17 fresh
//! 反编译 + 反汇编复核）。
//!
//! ## fresh 合同（O）
//! - 系统签名：`Query<(&mut TreeMoveBend, &mut PrefabInstanceData)>, Res<GameTime>`。
//!   组件行：`TreeMoveBend` = 8 B `{x@0, y@+4}`；`PrefabInstanceData` = **0x30 B**，
//!   位置 `x@0 / y@+4 / z@+8`，步长 `+0x24/{step_x}`、`+0x28/{step_z}`。
//! - 逐实体：`bend.x = time_continuous_lerp(bend.x, target.x, delta, 5.0)`；
//!   `bend.y = time_continuous_lerp(bend.y, target.z, delta, 5.0)`；
//!   `d = {bend.x-target.x, bend.y-target.z}`、`dist=‖d‖`；
//!   `dist < 0.1` → 保留 lerp 值；否则 `step = min(5.0,dist)·d/dist`、
//!   `bend = target + step`。
//! - 变更检测：`step` 与记录 `+0x24/+0x28` 比较（含 NaN 语义），不同则写
//!   `[+0x24..0x2c] = {step_x, step_z}` 并打变更 tick。
//!
//! Placement owner is canonical resources::prefab::PrefabInstance(48B),
//! distinct from prefab contents PrefabInstanceData(88B). The threshold branch
//! writes zero step (EAX/ECX zeroed at142268cfc), including resetting old bend.
//!
//! `time_continuous_lerp` 本身判等（0x140c904e0 逐指令：`expf(-delta*rate)` →
//! `1-F` → `(b-a)*` → `+a`；rate 0x142925988=5.0）。

use bevy_ecs::prelude::*;

use country_core::components::TreeMoveBend;
use country_core::resources::prefab::PrefabInstance;
use country_core::resources::time::GameTime;

/// `utils::time_continuous_lerp`（0x140c904e0，O 反汇编）：
/// `a + (b - a) * (1 - expf(-delta * rate))`（`XORPS` 取负 → `MULSS` ×rate →
/// `CALL expf` → `1-F` → 差乘 → 加 a）。**判等。**
pub fn time_continuous_lerp(a: f32, b: f32, delta: f32, rate: f32) -> f32 {
    a + (b - a) * (1.0 - (-delta * rate).exp())
}

/// 缓动速率 / 位移钳制上限（O：0x142925988 = 5.0；反汇编 142268a4f
/// `MOVSS XMM6,[0x142925988]`，XMM6 兼作 lerp rate 与 `MINSS XMM3,XMM6`
/// 的位移上限）。
pub const TREE_BEND_RATE: f32 = 5.0;
/// 距离阈值（O：0x142925984 = 0.1；`UCOMISS XMM9,XMM0; JA` → `dist < 0.1` 跳过）。
pub const TREE_BEND_DIST_THRESHOLD: f32 = 0.1;
/// 位移上限（O：与 lerp rate 同源，5.0）。
pub const TREE_BEND_STEP_MAX: f32 = 5.0;

/// B1422689c0, including unordered threshold behavior and actual placement writes.
pub fn tree_move_bend(
    mut trees: Query<(&mut TreeMoveBend, &mut PrefabInstance)>,
    time: Res<GameTime>,
) {
    for (mut bend, mut placement) in &mut trees {
        let target = placement.0.position;
        let bx = time_continuous_lerp(bend.position.x, target.x, time.delta, TREE_BEND_RATE);
        let by = time_continuous_lerp(bend.position.y, target.z, time.delta, TREE_BEND_RATE);
        bend.position = glam::Vec2::new(bx, by);
        let delta = bend.position - glam::Vec2::new(target.x, target.z);
        let distance = (delta.x * delta.x + delta.y * delta.y).sqrt();
        let step = if distance < TREE_BEND_DIST_THRESHOLD {
            glam::Vec2::ZERO
        } else {
            // MINSS(5,dist) selects dist when unordered, unlike f32::min.
            let length = if TREE_BEND_STEP_MAX < distance {
                TREE_BEND_STEP_MAX
            } else {
                distance
            };
            let step = (delta * (1.0 / distance)) * length;
            bend.position = glam::Vec2::new(target.x, target.z) + step;
            step
        };
        if placement.0.bend_vector != step {
            placement.0.bend_vector = step;
        }
    }
}
