//! sheep_walk_animation（0x14231a630，源码 0–；fresh 反编译证据 + RE-ANALYSIS §1.2 跳表解码）。
//!
//! 逐羊（O，fresh 14231a630.c + 反汇编 0x14231a970–0x14231aa27）：
//! - `speed = |target_pos - 记录位(+0x4c/+0x50/+0x54)| / dt`；写 +0x48 = speed；记录位 = 当前位。
//! - 动画状态机（+0x10）：`speed<=0.2`：state==1 → state=2、progress=0；否则保持；
//!   `speed>0.2`：state==1 → 保持 progress；否则 state=1、progress=speed。写 +0x10/+0x14。
//! - `Sheep::being_petted()` 真 → 继续下一羊；假 → 按 state 经跳表 0x14304b0f8 分派动画
//!   handler（JMP 到 handler 后回主循环续下一羊，非返回）。
//!
//! 跳表 0x14304b0f8 解码（RE-ANALYSIS §1.2，index=`*(uint*)(spot+0x10)`=state）：
//! | index | 目标 | 语义 |
//! |---|---|---|
//! | 0 | 0x14231a729 | 主循环续（无 handler） |
//! | 1 | 0x14231aa29 | walk：accum `[+0x5c] += dt` → `fmodf(accum,1.25)`（0x142923f10）→
//!   `idx=clamp_f32_to_i64(accum*24.0)`（0x142934600=24.0）→ `[+0x28]=(idx+n*20+3) mod 30`
//!   （`n=[+0x20]`，除 30 魔数 0x8888888888888889） |
//! | 2 | 0x14231aade | 过渡：同上 accum/fmodf/idx → `[+0x28]=(idx+n*20+3) mod 30`；
//!   `progress += dt/0.1`（0x142925984=0.1）；`progress >= 1.0`（0x142925904）→ state=3；
//!   否则保持 state=2 写 progress |
//! | 3 | 0x14231a729 | 主循环续（无 handler） |
//!
//! 两 handler 均含 `mov dword [r13],ebp` 音频槽写（查询列 +0x28 视图，值=query 派生值，
//! bevy-free 无法复现，不建模）。
//!
//! DIVERGE：`+0x10/+0x14/+0x28/+0x48/+0x4c/+0x50/+0x54/+0x5c` 本在 Sheep 0x68 记录上，
//! 因 components.rs 不可改，落为独立 `SheepWalkRecord` 组件（布局分拆）；`n`(+0x20) 为
//! Sheep.pet_target（u64 种子）。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;
use glam::Vec3;

use crate::components::{Sheep, Transform};

pub use country_core::resources::critters::sheep::SheepWalkRecord;

/// sheep_walk_animation（0x14231a630）。
pub fn sheep_walk_animation(
    time: Res<GameTime>,
    mut query: Query<(&Transform, &Sheep, ComponentWrite<SheepWalkRecord>)>,
) {
    let dt = time.delta;
    for (transform, sheep, mut rec) in &mut query {
        let target_pos = transform.translation;
        // O：speed = |target - 记录位| / dt；记录位更新为当前位。
        let d = target_pos - rec.last_pos;
        rec.speed = d.length() / dt;
        rec.last_pos = target_pos;
        // O：状态机（0.2 = 0x3e4ccccd）。
        let state = rec.state;
        let (new_state, new_progress) = if rec.speed <= 0.2 {
            if state == 1 {
                (2, 0.0)
            } else {
                (state, rec.progress)
            }
        } else if state == 1 {
            (1, rec.progress)
        } else {
            (1, rec.speed)
        };
        rec.state = new_state;
        rec.progress = new_progress;
        // O：being_petted → 跳过动画分派（继续下一羊）。
        if sheep.being_petted() {
            continue;
        }
        // 跳表 0x14304b0f8 分派（handler 完成后回主循环续下一羊，非 return）。
        // `n` = Sheep 记录 +0x20 = pet_target（u64 动画种子）。
        match rec.state {
            1 => sheep_walk_h1(&mut rec, sheep.pet_target, dt),
            2 => sheep_walk_h2(&mut rec, sheep.pet_target, dt),
            _ => {}
        }
    }
}

/// state1 walk handler（0x14231aa29）：
/// `accum += dt`；`accum = fmodf(accum, 1.25)`（call 0x142923f10，XMM1=1.25@0x142a8d600）；
/// `idx = clamp_f32_to_i64(accum * 24.0)`（0x142934600=24.0；cvttss2si 饱和序列
/// 0x5f000000/0x5f7fffff = Rust `as i64` 饱和转换）；
/// `[+0x28] = (idx + n*20 + 3) mod 30`（除 30 魔数 0x8888888888888889 >> 4 带符号 floor）。
fn sheep_walk_h1(rec: &mut SheepWalkRecord, n: u64, dt: f32) {
    // fmodf 语义 = Rust `%`（`x - trunc(x/y)*y`）。
    rec.accum = (rec.accum + dt) % f32::from_bits(0x3fa00000);
    let idx = (rec.accum * f32::from_bits(0x41c00000)) as i64;
    let frame = idx
        .wrapping_add((n as i64).wrapping_mul(20))
        .wrapping_add(3)
        .rem_euclid(30);
    rec.frame_index = frame as u64;
}

/// state2 过渡 handler（0x14231aade）：
/// 同 walk 的 accum/fmodf/frame 更新；`progress += dt / 0.1`（0x142925984=0.1）；
/// `progress >= 1.0`（0x142925904，ucomiss jae → 含 NaN 视为 <1.0）→ state=3（不写 progress）；
/// 否则保持 state=2 并写 progress。
fn sheep_walk_h2(rec: &mut SheepWalkRecord, n: u64, dt: f32) {
    rec.accum = (rec.accum + dt) % f32::from_bits(0x3fa00000);
    let idx = (rec.accum * f32::from_bits(0x41c00000)) as i64;
    let frame = idx
        .wrapping_add((n as i64).wrapping_mul(20))
        .wrapping_add(3)
        .rem_euclid(30);
    rec.frame_index = frame as u64;
    let progress = rec.progress + dt / f32::from_bits(0x3dcccccd);
    if progress >= 1.0 {
        rec.state = 3;
    } else {
        rec.state = 2;
        rec.progress = progress;
    }
}

use country_core::render::extract::ComponentWrite;
