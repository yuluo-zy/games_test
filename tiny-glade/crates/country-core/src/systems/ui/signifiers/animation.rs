//! `animation.rs` — 指示符动画（B 0x140a69340 `signifier_animation_t` / 0x140a69c30
//! `signifier_animation`，O）。
//!
//! 证据：`chapters/systems/ui/signifiers/animation.rs.md`（wp15 b7）+ 反编译
//! `ui-input/decomp/decomp-0x140a69340.txt` / `decomp-0x140a69c30.txt`：
//! - `signifier_animation_t` 每帧 tick：Query 遍历 signifier 实体（组件表 stride 0xa0，
//!   源索引@+0x94），对每个 `SignifierAnimation` 推进时间 `t' = clamp(t + tick/duration,
//!   0, 1.0)`（O：`(1.0/fVar28)*fVar38 + t`，clamp `[0, 1.0]`），写回 `t@+0x14`，再把
//!   `tick` 累加到 `@+0x28`；
//! - `signifier_animation` 同 Query 迭代：按 `byte@+0x98` 判别（switch 0..0x10，含 mesh
//!   变体分发）把 t 映射到实体 scale/alpha 目标 `@+0x1c/+0x20/+0x24`；父挂体积原点取
//!   `WidgetVolumeShape::get_origin`（I）。
//!
//! scale/alpha 目标映射的 switch 0..0x10 细节 U（登记待取证）；本实现落地组件 + t-tick
//! 系统（O 核心），动画应用按 t 缓动落到 `eased`。
use bevy_ecs::prelude::*;

/// `SignifierAnimation` 组件（O 布局：`duration@+0x10`、`t@+0x14`、`eased@+0x18`、
/// `scale/alpha 目标@+0x1c/+0x20/+0x24`、`tick@+0x28`、`kind@+0x98`；组件表元素 0xa0 B）。
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct SignifierAnimation {
    /// 动画时长（秒；0 时保持不动）。
    pub duration: f32,
    /// 当前进度 t（0..1）。
    pub t: f32,
    /// 缓动派生值（scale/alpha，t-tick 后写）。
    pub eased: f32,
    /// scale 目标。
    pub scale: f32,
    /// alpha 目标。
    pub alpha: f32,
    /// 帧 tick 累计（O：`@+0x28 += tick`）。
    pub tick_acc: f32,
    /// 判别字节（switch 0..0x10 分发；U）。
    pub kind: u8,
}

impl Default for SignifierAnimation {
    fn default() -> Self {
        Self {
            duration: 1.0,
            t: 0.0,
            eased: 0.0,
            scale: 1.0,
            alpha: 1.0,
            tick_acc: 0.0,
            kind: 0,
        }
    }
}

/// `signifier_animation_t` @0x140a69340（O）：每帧推进所有指示符动画时间。
///
/// `t' = clamp(t + tick/duration, 0, 1.0)`（`_UNK_142925904` = 1.0 钳上界）；`tick_acc` 累加。
pub fn signifier_animation_t(
    time: Res<crate::resources::time::GameTime>,
    mut query: Query<&mut SignifierAnimation>,
) {
    let tick = time.delta;
    for mut anim in &mut query {
        if anim.duration != 0.0 && !anim.duration.is_nan() {
            anim.t = (anim.t + tick / anim.duration).clamp(0.0, 1.0);
        }
        anim.tick_acc += tick;
    }
}

/// `signifier_animation` @0x140a69c30（O 结构）：按 `kind` 判别把 t 映射到 scale/alpha。
///
/// switch 0..0x10 精确目标表 U；本实现以 t 做线性缓动落到 `eased`（I），
/// 实体 Transform scale / 父挂 WidgetVolumeShape 原点待宿主接线。
pub fn signifier_animation(mut query: Query<&mut SignifierAnimation>) {
    for mut anim in &mut query {
        let t = anim.t;
        anim.eased = match anim.kind {
            // 缓动形态（I：按 t 的平滑插值；精确曲线 U）。
            0 => t,
            _ => t * t * (3.0 - 2.0 * t),
        };
        anim.scale = anim.eased;
        anim.alpha = anim.eased;
    }
}
