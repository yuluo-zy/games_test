//! autoclutter_animation（B 主基线；0x1422ccef0，size 2238）。
//!
//! 2026-10-17 restorer-clutter-remaining：按 fresh 反编译重建
//! （`evidence/decomp/0x1422ccef0_*.c`）。
//!
//! 控制流（O）：
//! - 对每个动画记录（二进制 stride 0x18：timer@+8、progress@+0xc、target_scale@+0x10、
//!   byte@+0x14）：
//!   1. `forced`（bVar32，wall-index 表第二槽非空）：`delta = dt / -0.3`；
//!   2. 否则 `timer <= 0`：`delta = dt / (has_base ? 0.4 : 0.3)`（常量对
//!      0x3e99999a3ecccccd：非空槽 0.4、空槽 0.3）；
//!   3. 否则 `timer -= dt; delta = 0`，`timer < 0` 时原版推一条 0x18B 事件
//!      （载荷 {u64,u64,f32,u8,u8}；grow_one 符号被 ICF 到
//!      `EventInstance<(str,SettingsMenuState)>`）——U：事件类型/消费方未落库。
//!   4. `progress = clamp01(progress + delta)` 写回 +0xc。
//!   5. 无 base 槽 → `progress == 0.0 && !NaN` → `despawn`。
//!   6. 有 base 槽：
//!      - forced（!bVar20）→ `scale = easing.apply(progress)`；
//!      - 否则 sign = byte@+0x14 ? +1 : -1；
//!        `angle = (eased*sign*60 - (1-eased)*sign*50) * 0.017453292 * 0.5`；
//!        `quat = from_rotation_z(angle) * base`（sin/cos 展开与 0x1422cd4f4 块逐项一致）；
//!        scale = 1。
//!
//! U 登记：原版动画状态存于 wall-index 三张表（+0x100/+0x108/+0x110 选槽），本 crate
//! 无该宿主查询 → 以每实体 [`AutoClutterAnimation`] 组件承载（类型合同 U）；
//! timer 到期事件不伪造（core-type-gap）。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;
use glam::Quat;
use utils::easing_functions::EasingFunction;

/// 自动杂物动画（组件；二进制记录 stride 0x18 的语义承载）。
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct AutoClutterAnimation {
    /// +0x08：倒计时（>0 时只递减）。
    pub timer: f32,
    /// +0x0c：进度（0..1）。
    pub progress: f32,
    /// +0x10：forced 路径的目标缩放。
    pub target_scale: f32,
    /// +0x14：旋转方向字节（非零 = +1，否则 -1）。
    pub flag: u8,
    /// base 四元数（二进制 wall 槽记录）。
    pub base: Quat,
    /// 是否存在 base 槽（bVar25 / pfVar30 非空）。
    pub has_base: bool,
    /// forced 路径（bVar32）。
    pub forced: bool,
    /// Easing 曲线（宿主写入）。
    pub easing: EasingFunction,
}

impl Default for AutoClutterAnimation {
    fn default() -> Self {
        Self {
            timer: 0.0,
            progress: 0.0,
            target_scale: 1.0,
            flag: 0,
            base: Quat::IDENTITY,
            has_base: false,
            forced: false,
            easing: EasingFunction::CubicOut,
        }
    }
}

/// 弧度/度常量（O：0.017453292）。
const DEG_TO_RAD: f32 = 0.017453292;

/// autoclutter_animation（0x1422ccef0）：推进动画；完成 → despawn。
pub fn autoclutter_animation(
    time: Res<GameTime>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut AutoClutterAnimation)>,
) {
    let dt = time.delta;
    for (entity, mut anim) in &mut query {
        // 1–3. 推进增量。
        let (delta, b_var20) = if anim.forced {
            (dt / -0.3, false)
        } else if anim.timer <= 0.0 {
            let div = if anim.has_base { 0.4 } else { 0.3 };
            (dt / div, true)
        } else {
            anim.timer -= dt;
            // O：timer<0 时原版推 0x18B 事件；类型未落库（U），此处不伪造。
            (0.0, true)
        };
        // 4. progress = clamp01(progress + delta)（O：fVar36=0 if delta<0；
        //    再 min(1.0)）。
        let mut p = anim.progress + delta;
        if p < 0.0 {
            p = 0.0;
        }
        if p > 1.0 {
            p = 1.0;
        }
        anim.progress = p;

        // 5. 无 base 槽：progress==0 且非 NaN → despawn。
        if !anim.has_base {
            if anim.progress == 0.0 && !anim.progress.is_nan() {
                commands.entity(entity).despawn();
            }
            continue;
        }

        // 6. 有 base 槽。
        if !b_var20 {
            // forced：scale = easing(progress)。
            let eased = anim.easing.apply(anim.progress);
            anim.target_scale = eased;
        } else {
            let sign = if anim.flag != 0 { 1.0f32 } else { -1.0f32 };
            let eased = anim.easing.apply(anim.progress);
            // O：angle = (eased*sign*60 - (1-eased)*sign*50) * deg2rad * 0.5。
            let angle = (eased * sign * 60.0 - (1.0 - eased) * sign * 50.0) * DEG_TO_RAD * 0.5;
            // O：sin/cos 展开 = from_rotation_z(angle) * base。
            anim.base = Quat::from_rotation_z(angle) * anim.base;
            anim.target_scale = 1.0;
        }
        // 注：写入宿主 scale/quat 组件（ManagedAutoClutterPlacement）由宿主渲染接线；
        // 本组件保留动画状态（U）。
    }
}
