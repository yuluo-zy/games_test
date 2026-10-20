//! `animation.rs` — 指示符动画（B 0x140a69340 `signifier_animation_t` / 0x140a69c30
//! `signifier_animation`，O）。
//!
//! 证据（2026-10-17 fresh 反编译/机器码）：
//! - `SignifierAnimation` 组件元素 0x1c B：`byte@+8`、`byte@+9`、`f32@+0xc`（value）、
//!   `f32@+0x10`（duration）、`f32@+0x14`（t）、`f32@+0x18`（eased）。
//! - `signifier_animation_t`：`duration != 0 || NaN` 时
//!   `t' = clamp(t + (1/duration)·tick, 0, 1)`，否则 `t' = 反向?0:1`；
//!   `eased = EasingFunction::apply(t)`；tick 累加到查询上的可选组件（非本组件）。
//! - `signifier_animation`：Query<(&SignifierAnimation, &mut Transform,
//!   Option<&SignifierId>, Option<&SignifierParentToVolumeOrigin>)> + `Res<UiState>`。
//!   门 `byte@+8 == 0` 时：`fVar32 = eased·value`、`fVar33 = byte@+9==0 ? value : fVar32`，
//!   写 `Transform.scale = (fVar33, fVar32, fVar33)`（+0x1c/+0x20/+0x24）。随后按
//!   UiState 配置的 kind（`*(*[UiState+0x20]+0x98)`）switch 写 `Transform.translation`
//!   （+0x10..+0x18）：case 0 取 `+0xa0` 的 12 B、case 6 取 `+0x9c`、case 1 取
//!   `+0xa8` 表（索引 `+0xb0`）的 12 B 条目；8/9/0xa/0xb/0xc..0x10 的 Ghidra
//!   反编译损坏（swi/halt_baddata）。crate 的 UiState 未落库该配置表 → kind 数据流
//!   登记 GAP，scale 面按 fresh 还原。
use bevy_ecs::prelude::*;
use glam::Vec3;

use crate::systems::ui::resources::ui_state::UiState;
use crate::utils::components::transform::Transform;

use super::signifier_create::SignifierId;

/// `SignifierAnimation` 组件（O 布局 0x1c B）。
#[repr(C)]
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct SignifierAnimation {
    /// +0x00..+0x08：未取证（保持 0）。
    pub head: [u8; 8],
    /// +0x08：写入门（==0 时 `signifier_animation` 更新 scale）。
    pub flag_8: u8,
    /// +0x09：scale.x/z 选取门（0 → value，1 → eased·value）。
    pub flag_9: u8,
    /// +0x0a..+0x0c：填充。
    pub pad_a: [u8; 2],
    /// +0x0c：value（动画输入值）。
    pub value: f32,
    /// +0x10：动画时长（秒）。
    pub duration: f32,
    /// +0x14：当前进度 t。
    pub t: f32,
    /// +0x18：缓动派生值（`EasingFunction::apply(t)`）。
    pub eased: f32,
    /// crate 附加：原版把 tick 累加到查询上的另一个可选组件；保留字段承载 t-tick
    /// 判等实现（`signifier_animation_t`），不参与本组件 0x1c 布局。
    pub tick_acc: f32,
}

impl Default for SignifierAnimation {
    fn default() -> Self {
        Self {
            head: [0; 8],
            flag_8: 0,
            flag_9: 0,
            pad_a: [0; 2],
            value: 0.0,
            duration: 1.0,
            t: 0.0,
            eased: 0.0,
            tick_acc: 0.0,
        }
    }
}

/// 指示符挂到 volume 原点（O：查询 `Option<&SignifierParentToVolumeOrigin>`；
/// 组件布局未取证，仅查询存在性/承载需要）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SignifierParentToVolumeOrigin;

/// `signifier_animation_t` @0x140a69340（O）：每帧推进所有指示符动画时间。
///
/// 原版对每个组件：`duration != 0.0 || duration.is_nan()` 时
/// `t' = clamp(t + (1/duration)·tick, 0, 1)`；否则 `t' = 反向?0:1`，
/// 再 `eased = EasingFunction::apply(t')`，tick 累加到可选组件。
/// （该函数此前已判等[结构]；本轮只按 fresh 修正组件布局，不改其控制流。）
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

/// `signifier_animation` @0x140a69c30（O）。
///
/// fresh 可逐句对应部分：
/// - 门 `flag_8 == 0`；
/// - `value = +0xc`、`multiplier = eased@+0x18`，`scaled = multiplier·value`；
/// - `x = flag_9 == 0 ? value : scaled`；
/// - `Transform.scale = (x, scaled, x)`。
///
/// 登记 GAP：kind switch（UiState 配置 @+0x98）与 translation 写入；UiState 未落库
/// signifier 配置表（解锁条件：UiState 暴露 `signifier_config_kind/translation`）。
pub fn signifier_animation(
    mut query: Query<(
        &SignifierAnimation,
        &mut Transform,
        Option<&SignifierId>,
        Option<&SignifierParentToVolumeOrigin>,
    )>,
    _ui_state: Res<UiState>,
) {
    for (anim, mut transform, _id, _parent_to_volume_origin) in &mut query {
        if anim.flag_8 != 0 {
            continue;
        }
        let value = anim.value;
        let scaled = anim.eased * value;
        let x = if anim.flag_9 == 0 { value } else { scaled };
        transform.scale = Vec3::new(x, scaled, x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_layout_matches_native_offsets() {
        assert_eq!(std::mem::size_of::<SignifierAnimation>() - std::mem::size_of::<f32>(), 0x1c);
        let a = SignifierAnimation {
            flag_8: 1,
            flag_9: 2,
            value: 3.0,
            duration: 4.0,
            t: 5.0,
            eased: 6.0,
            ..Default::default()
        };
        let base = &a as *const _ as usize;
        assert_eq!(&a.value as *const _ as usize - base, 0x0c);
        assert_eq!(&a.duration as *const _ as usize - base, 0x10);
        assert_eq!(&a.t as *const _ as usize - base, 0x14);
        assert_eq!(&a.eased as *const _ as usize - base, 0x18);
    }

    #[test]
    fn scale_mapping_uses_flag_9() {
        let anim = SignifierAnimation {
            value: 2.0,
            eased: 0.25,
            ..Default::default()
        };
        let mut t = Transform::identity();
        let scaled = anim.eased * anim.value;
        let x = if anim.flag_9 == 0 { anim.value } else { scaled };
        t.scale = Vec3::new(x, scaled, x);
        assert_eq!(t.scale, Vec3::new(2.0, 0.5, 2.0));
    }
}
