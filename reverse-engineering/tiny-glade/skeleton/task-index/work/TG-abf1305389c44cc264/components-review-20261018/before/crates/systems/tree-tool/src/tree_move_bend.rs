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
//! ## 未实现（core-type-gap）
//! country-core `systems::prefab::PrefabInstanceData` 是 0x58 B Vec 布局
//! （`gpu_instances@+8 / rt_meshes@+0x20 / audio_triggers@+0x38 / prefab_name@+0x48`），
//! 不存在 native 的 `position@0` 与 `step@+0x24/+0x28`。本实现以同时挂在树实体上的
//! `Transform.translation` 作目标位置投影，且不写 step 字段（无承载位）；因此
//! **verdict = 未实现（近似体，已登记）**，解锁条件 = core 落 native 0x30 B 放置
//! 记录（或给 `PrefabInstanceData` 补 position/step 字段）。
//!
//! `time_continuous_lerp` 本身判等（0x140c904e0 逐指令：`expf(-delta*rate)` →
//! `1-F` → `(b-a)*` → `+a`；rate 0x142925988=5.0）。

use bevy_ecs::prelude::*;

use country_core::components::TreeMoveBend;
use country_core::resources::time::GameTime;
use country_core::utils::components::transform::Transform;

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

/// `tree_move_bend` @0x1422689c0 — **未实现（近似体：位置源/step 写面缺 core 字段）**。
pub fn tree_move_bend(
    mut trees: Query<(&mut TreeMoveBend, &Transform)>,
    time: Res<GameTime>,
) {
    let delta = time.delta;
    for (mut bend, data) in &mut trees {
        // 投影：native 目标 = PrefabInstanceData[0..0xc]（0x30 B 放置记录）；
        // core 无该字段，用 Transform.translation 承载（登记 I）。
        let target = data.translation;
        // TreeMoveBend 是 8 B 组件 {x f32 @+0, y f32 @+4}（原生迭代步长 8）——
        // core 型以 `_gap: u64` 承载，按位解包读写。
        let mut bx = f32::from_bits(bend._gap as u32);
        let mut by = f32::from_bits((bend._gap >> 32) as u32);
        bx = time_continuous_lerp(bx, target.x, delta, TREE_BEND_RATE);
        by = time_continuous_lerp(by, target.z, delta, TREE_BEND_RATE);
        let dx = bx - target.x;
        let dz = by - target.z;
        let dist = (dx * dx + dz * dz).sqrt();
        if dist >= TREE_BEND_DIST_THRESHOLD {
            // O：step = min(5.0, dist) * {dx,dz}/dist；写回 bend = target + step
            // （反汇编 142268d09-142268d3c）。
            let step_len = dist.min(TREE_BEND_STEP_MAX);
            let sx = step_len * dx / dist;
            let sz = step_len * dz / dist;
            bx = target.x + sx;
            by = target.z + sz;
            // O：变更检测 + 写 PrefabInstanceData +0x24/+0x28 = {step_x, step_z}
            // （反汇编 142268d47-142268d68）。core 无这些字段 → 未实现面。
        }
        bend._gap = (bx.to_bits() as u64) | ((by.to_bits() as u64) << 32);
    }
}
